mod context;
mod development;
mod evidence;
mod hash;
mod lane;
mod packet;
mod queue;
mod respawn;

use std::{env, path::PathBuf, process};

pub(crate) type AppResult<T> = Result<T, String>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutputMode {
    Human,
    Machine,
}

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("fleet error: {error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> AppResult<()> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage());
    };
    let rest = &args[1..];

    match command {
        "verify" => development::verify(&development::repository(rest)?),
        "context" => {
            let repo = development::repository(rest)?;
            match rest.first().map(String::as_str) {
                Some("current") => development::current(&repo, output_mode(rest)?, rest.iter().any(|arg| arg == "--verify")),
                Some("verify") => context::command_verify(&repo, output_mode(rest)?),
                Some("manifest") => context::command_manifest(&repo, output_mode(rest)?),
                _ => Err("usage: cargo xtask context <current|verify|manifest> [--repo PATH] [--human|--machine] [--verify]".to_owned()),
            }
        }
        "context-verify" => {
            let repo = required_path(rest, "--repo")?;
            context::command_verify(&repo, output_mode(rest)?)
        }
        "context-manifest" => {
            let repo = required_path(rest, "--repo")?;
            context::command_manifest(&repo, output_mode_or(rest, OutputMode::Human)?)
        }
        "lane-doctor" => {
            let worktree = required_path(rest, "--worktree")?;
            let branch = required_value(rest, "--branch")?;
            let base = required_value(rest, "--base")?;
            let overlap_file = optional_value(rest, "--overlap-file").map(PathBuf::from);
            lane::command_doctor(
                &worktree,
                &branch,
                &base,
                overlap_file.as_deref(),
                output_mode(rest)?,
            )
        }
        "packet-verify" => {
            let file = required_path(rest, "--file")?;
            packet::command_verify(&file, output_mode(rest)?)
        }
        "respawn-bundle" => {
            let repo = required_path(rest, "--repo")?;
            let lane = required_value(rest, "--lane")?;
            let output = required_path(rest, "--output")?;
            respawn::command_generate(&repo, &lane, &output)
        }
        "queue-render" => {
            let repo = required_path(rest, "--repo")?;
            queue::command_render(&repo, output_mode_or(rest, OutputMode::Human)?)
        }
        "queue-verify" => {
            let repo = required_path(rest, "--repo")?;
            queue::command_verify(&repo, output_mode(rest)?)
        }
        "evidence-manifest" => {
            let input = required_path(rest, "--input")?;
            let output = required_path(rest, "--output")?;
            evidence::command_generate(&input, &output)
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: cargo xtask <verify|context|context-verify|context-manifest|lane-doctor|packet-verify|respawn-bundle|queue-render|queue-verify|evidence-manifest> [arguments]".to_owned()
}

pub(crate) fn required_value(args: &[String], name: &str) -> AppResult<String> {
    optional_value(args, name).ok_or_else(|| format!("missing required argument {name}"))
}

pub(crate) fn required_path(args: &[String], name: &str) -> AppResult<PathBuf> {
    required_value(args, name).map(PathBuf::from)
}

pub(crate) fn optional_value(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn output_mode(args: &[String]) -> AppResult<OutputMode> {
    output_mode_or(args, OutputMode::Human)
}

fn output_mode_or(args: &[String], default: OutputMode) -> AppResult<OutputMode> {
    let human = args.iter().any(|arg| arg == "--human");
    let machine = args.iter().any(|arg| arg == "--machine");
    match (human, machine) {
        (true, true) => Err("choose exactly one of --human or --machine".to_owned()),
        (true, false) => Ok(OutputMode::Human),
        (false, true) => Ok(OutputMode::Machine),
        (false, false) => Ok(default),
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn repo_path(relative: &str) -> PathBuf {
        let root = option_env!("CARGO_MANIFEST_DIR")
            .map(|path| {
                std::path::Path::new(path)
                    .parent()
                    .expect("xtask workspace root")
            })
            .unwrap_or_else(|| std::path::Path::new("."));
        root.join(relative)
    }

    pub(crate) struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        pub(crate) fn new(label: &str) -> Self {
            loop {
                let number = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
                let path = std::env::temp_dir().join(format!(
                    "rust-reps-fleet-test-{}-{label}-{number}",
                    std::process::id()
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("create test directory: {error}"),
                }
            }
        }

        pub(crate) fn path(&self) -> &std::path::Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
