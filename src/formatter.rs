use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const RUSTFMT_COMMAND: &str = "rustfmt";
const RUSTFMT_TIMEOUT: Duration = Duration::from_millis(900);
const RUSTFMT_POLL_INTERVAL: Duration = Duration::from_millis(10);
const MAX_FORMAT_INPUT_BYTES: usize = 32 * 1024;
const WRAPPER_PREFIX: &str = "fn __rust_reps_scratch() {\n";
const WRAPPER_SUFFIX: &str = "\n}\n";
const RUSTFMT_INDENT: &str = "    ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatResult {
    Formatted(String),
    Unavailable,
    Failed(String),
    TimedOut,
}

pub fn format_rust_source(input: &str) -> FormatResult {
    format_rust_source_with_command(RUSTFMT_COMMAND, input, RUSTFMT_TIMEOUT)
}

fn format_rust_source_with_command(command: &str, input: &str, timeout: Duration) -> FormatResult {
    if input.len() > MAX_FORMAT_INPUT_BYTES {
        return FormatResult::Failed("input too large".to_string());
    }

    match run_rustfmt(command, input, timeout) {
        FormatResult::Formatted(text) => {
            FormatResult::Formatted(strip_one_final_line_ending(&text))
        }
        FormatResult::Failed(_) => format_wrapped_snippet(command, input, timeout),
        other => other,
    }
}

fn format_wrapped_snippet(command: &str, input: &str, timeout: Duration) -> FormatResult {
    let wrapped = format!("{WRAPPER_PREFIX}{input}{WRAPPER_SUFFIX}");
    match run_rustfmt(command, &wrapped, timeout) {
        FormatResult::Formatted(text) => extract_wrapped_body(&text)
            .map(FormatResult::Formatted)
            .unwrap_or_else(|| {
                FormatResult::Failed("unable to extract formatted snippet".to_string())
            }),
        other => other,
    }
}

fn run_rustfmt(command: &str, input: &str, timeout: Duration) -> FormatResult {
    let mut child = match Command::new(command)
        .arg("--emit")
        .arg("stdout")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return FormatResult::Unavailable,
        Err(err) => return FormatResult::Failed(err.to_string()),
    };

    let Some(mut stdin) = child.stdin.take() else {
        let _ = child.kill();
        let _ = child.wait();
        return FormatResult::Failed("unable to open rustfmt stdin".to_string());
    };
    if let Err(err) = stdin.write_all(input.as_bytes()) {
        let _ = child.kill();
        let _ = child.wait();
        return FormatResult::Failed(err.to_string());
    }
    drop(stdin);

    let started_at = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = read_child_pipe(child.stdout.take());
                let stderr = read_child_pipe(child.stderr.take());
                let _ = child.wait();
                if status.success() {
                    return FormatResult::Formatted(stdout);
                }
                return FormatResult::Failed(first_error_line(&stderr));
            }
            Ok(None) if started_at.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return FormatResult::TimedOut;
            }
            Ok(None) => thread::sleep(RUSTFMT_POLL_INTERVAL),
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return FormatResult::Failed(err.to_string());
            }
        }
    }
}

fn read_child_pipe<R: Read>(pipe: Option<R>) -> String {
    let Some(mut pipe) = pipe else {
        return String::new();
    };
    let mut output = String::new();
    let _ = pipe.read_to_string(&mut output);
    output
}

fn first_error_line(stderr: &str) -> String {
    stderr
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("rustfmt failed")
        .to_string()
}

fn strip_one_final_line_ending(text: &str) -> String {
    if let Some(stripped) = text.strip_suffix("\r\n") {
        stripped.to_string()
    } else if let Some(stripped) = text.strip_suffix('\n') {
        stripped.to_string()
    } else {
        text.to_string()
    }
}

fn extract_wrapped_body(formatted: &str) -> Option<String> {
    let stripped = strip_one_final_line_ending(formatted);
    let mut lines = stripped.lines();
    let first = lines.next()?;
    if first.trim() != "fn __rust_reps_scratch() {" {
        return None;
    }

    let mut body = lines.collect::<Vec<_>>();
    if body.last().map(|line| line.trim()) != Some("}") {
        return None;
    }
    body.pop();

    let dedented = body
        .into_iter()
        .map(|line| line.strip_prefix(RUSTFMT_INDENT).unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");
    Some(dedented)
}

#[cfg(test)]
mod tests {
    use super::{
        FormatResult, extract_wrapped_body, format_rust_source, format_rust_source_with_command,
        strip_one_final_line_ending,
    };
    use std::time::Duration;

    #[test]
    fn strips_exactly_one_final_line_ending() {
        assert_eq!(
            strip_one_final_line_ending("fn main() {}\n"),
            "fn main() {}"
        );
        assert_eq!(
            strip_one_final_line_ending("fn main() {}\r\n"),
            "fn main() {}"
        );
        assert_eq!(
            strip_one_final_line_ending("fn main() {}\n\n"),
            "fn main() {}\n"
        );
    }

    #[test]
    fn extracts_wrapper_body_and_dedents_one_rustfmt_indent() {
        let formatted = "fn __rust_reps_scratch() {\n    score += 1;\n}\n";

        assert_eq!(
            extract_wrapped_body(formatted),
            Some("score += 1;".to_string())
        );
    }

    #[test]
    fn unavailable_rustfmt_reports_unavailable() {
        let result = format_rust_source_with_command(
            "rust-reps-missing-rustfmt-for-test",
            "fn main(){}",
            Duration::from_millis(50),
        );

        assert_eq!(result, FormatResult::Unavailable);
    }

    #[test]
    fn direct_source_formatting_handles_function_items_when_rustfmt_is_available() {
        if matches!(format_rust_source("fn main(){}"), FormatResult::Unavailable) {
            return;
        }

        assert_eq!(
            format_rust_source("fn main(){}"),
            FormatResult::Formatted("fn main() {}".to_string())
        );
    }

    #[test]
    fn wrapper_formatting_handles_statement_snippets_when_rustfmt_is_available() {
        if matches!(format_rust_source("score+=1;"), FormatResult::Unavailable) {
            return;
        }

        assert_eq!(
            format_rust_source("score+=1;"),
            FormatResult::Formatted("score += 1;".to_string())
        );
        assert_eq!(
            format_rust_source("let answer=42;"),
            FormatResult::Formatted("let answer = 42;".to_string())
        );
    }

    #[test]
    fn invalid_snippet_fails_without_formatted_output_when_rustfmt_is_available() {
        if matches!(format_rust_source("let = ;"), FormatResult::Unavailable) {
            return;
        }

        assert!(matches!(
            format_rust_source("let = ;"),
            FormatResult::Failed(_)
        ));
    }
}
