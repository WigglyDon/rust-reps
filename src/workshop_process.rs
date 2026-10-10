//! Narrow Unix process adapter. No shell, no inherited stdin, no blocking pipes.
use std::{
    ffi::OsString,
    io::{self, Read},
    os::{
        fd::AsRawFd,
        unix::process::{CommandExt, ExitStatusExt},
    },
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};

pub const COMPILE_TIMEOUT: Duration = Duration::from_secs(30);
pub const RUN_TIMEOUT: Duration = Duration::from_secs(5);
pub const OUTPUT_LIMIT: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Exited {
        code: Option<i32>,
        signal: Option<i32>,
    },
    MissingTool,
    SpawnFailed(String),
    Cancelled,
    TimedOut,
    OutputLimit,
    IoFailed(String),
}

impl Outcome {
    pub fn success(&self) -> bool {
        matches!(self, Self::Exited { code: Some(0), .. })
    }
}

#[derive(Clone, Debug)]
pub struct Capture {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub truncated: bool,
}

pub struct Invocation {
    pub executable: PathBuf,
    pub args: Vec<OsString>,
    pub directory: PathBuf,
    pub compiler: bool,
}

impl Invocation {
    // Debug quoting is a display of argv, never shell input.
    pub fn display(&self) -> String {
        format!(
            "{:?} {}\nworking directory: {}",
            self.executable,
            self.args
                .iter()
                .map(|arg| format!("{arg:?}"))
                .collect::<Vec<_>>()
                .join(" "),
            self.directory.display()
        )
    }
}

pub struct Job {
    child: Child,
    started: Instant,
    timeout: Duration,
    forced: Option<Outcome>,
    pub capture: Capture,
    finished: bool,
}

impl Job {
    pub fn start(invocation: &Invocation, timeout: Duration) -> Result<Self, Outcome> {
        let mut command = Command::new(&invocation.executable);
        command
            .args(&invocation.args)
            .current_dir(&invocation.directory)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env_clear()
            .env("LANG", "C")
            .env("LC_ALL", "C")
            .env("TMPDIR", &invocation.directory)
            .process_group(0);
        if invocation.compiler {
            command.env("RUSTUP_AUTO_INSTALL", "0");
            // rustup's rustc shim and the linker need installed toolchain lookup.
            // Do not inherit arbitrary compiler flags or credential variables.
            for key in [
                "PATH",
                "HOME",
                "RUSTUP_HOME",
                "CARGO_HOME",
                "RUSTUP_TOOLCHAIN",
            ] {
                if let Some(value) = std::env::var_os(key) {
                    command.env(key, value);
                }
            }
        } else {
            command.env("PATH", "/usr/bin:/bin");
        }
        let child = command.spawn().map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                Outcome::MissingTool
            } else {
                Outcome::SpawnFailed(error.to_string())
            }
        })?;
        let job = Self {
            child,
            started: Instant::now(),
            timeout,
            forced: None,
            capture: Capture {
                stdout: vec![],
                stderr: vec![],
                truncated: false,
            },
            finished: false,
        };
        for fd in [
            job.child.stdout.as_ref().unwrap().as_raw_fd(),
            job.child.stderr.as_ref().unwrap().as_raw_fd(),
        ] {
            // SAFETY: these descriptors belong to live child pipes. fcntl changes
            // only their nonblocking flag; ownership stays with Child.
            let result = unsafe {
                let flags = libc::fcntl(fd, libc::F_GETFL);
                if flags < 0 {
                    -1
                } else {
                    libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK)
                }
            };
            if result < 0 {
                // Drop kills/reaps the child without attempting any pipe reads.
                return Err(Outcome::IoFailed(io::Error::last_os_error().to_string()));
            }
        }
        Ok(job)
    }

    pub fn cancel(&mut self) {
        self.stop(Outcome::Cancelled);
    }

    fn stop(&mut self, outcome: Outcome) {
        if self.forced.is_none() {
            self.forced = Some(outcome);
        }
        self.kill_group();
    }

    fn kill_group(&mut self) {
        // SAFETY: process_group(0) creates a group whose id is the child's pid.
        // The direct child remains unreaped until after this call.
        unsafe {
            libc::kill(-(self.child.id() as i32), libc::SIGKILL);
        }
        let _ = self.child.kill();
    }

    pub fn poll(&mut self) -> Option<Outcome> {
        if self.finished {
            return None;
        }
        self.drain();
        if self.started.elapsed() >= self.timeout {
            self.stop(Outcome::TimedOut);
        }
        match self.child.try_wait() {
            Ok(Some(status)) => {
                // Even a normally exited program may have left descendants with
                // inherited pipes. Never wait for EOF from those pipes.
                // SAFETY: this is the group we created; called immediately on reap.
                unsafe {
                    libc::kill(-(self.child.id() as i32), libc::SIGKILL);
                }
                self.drain();
                self.finished = true;
                Some(self.forced.take().unwrap_or_else(|| exit_outcome(status)))
            }
            Ok(None) => None,
            Err(error) => {
                self.stop(Outcome::IoFailed(error.to_string()));
                let _ = self.child.wait();
                self.finished = true;
                self.forced.take()
            }
        }
    }

    fn drain(&mut self) {
        let result = drain_pipe(
            self.child.stdout.as_mut().unwrap(),
            &mut self.capture.stdout,
        )
        .and_then(|a| {
            drain_pipe(
                self.child.stderr.as_mut().unwrap(),
                &mut self.capture.stderr,
            )
            .map(|b| a || b)
        });
        match result {
            Ok(true) => {
                self.capture.truncated = true;
                self.stop(Outcome::OutputLimit);
            }
            Ok(false) => {}
            Err(error) => self.stop(Outcome::IoFailed(error.to_string())),
        }
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        if !self.finished {
            self.kill_group();
            let _ = self.child.wait();
        }
    }
}

fn exit_outcome(status: ExitStatus) -> Outcome {
    Outcome::Exited {
        code: status.code(),
        signal: status.signal(),
    }
}

fn drain_pipe(pipe: &mut impl Read, captured: &mut Vec<u8>) -> io::Result<bool> {
    let mut bytes = [0; 4096];
    // Bound work per frame as well as retained bytes, even for an endless writer.
    for _ in 0..32 {
        match pipe.read(&mut bytes) {
            Ok(0) => return Ok(false),
            Ok(count) => {
                let remaining = OUTPUT_LIMIT.saturating_sub(captured.len());
                captured.extend_from_slice(&bytes[..count.min(remaining)]);
                if count > remaining {
                    return Ok(true);
                }
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(false),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(false)
}

pub fn compiler_invocation(compiler: &Path, directory: &Path) -> Invocation {
    Invocation {
        executable: compiler.into(),
        args: vec![
            "--edition=2024".into(),
            "--crate-name".into(),
            "workshop".into(),
            "--color=never".into(),
            "source.rs".into(),
            "-o".into(),
            "program".into(),
        ],
        directory: directory.into(),
        compiler: true,
    }
}

/// Escape terminal controls and invisible direction/formatting characters.
pub fn safe_text(bytes: &[u8]) -> String {
    let mut text = String::new();
    for ch in String::from_utf8_lossy(bytes).chars() {
        match ch {
            '\n' => text.push('\n'),
            '\t' => text.push_str("    "),
            ch if ch.is_control()
                || matches!(ch, '\u{200b}'..='\u{200f}' | '\u{2028}'..='\u{202e}' | '\u{2060}'..='\u{206f}' | '\u{feff}') =>
            {
                text.push_str(&format!("\\u{{{:x}}}", ch as u32));
            }
            ch => text.push(ch),
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_spawn_timeout_and_controls_are_distinct() {
        let root = crate::workshop::tests::Fixture::new();
        let mut invocation = Invocation {
            executable: root.root.join("missing"),
            args: vec![],
            directory: root.root.clone(),
            compiler: true,
        };
        assert!(matches!(
            Job::start(&invocation, COMPILE_TIMEOUT),
            Err(Outcome::MissingTool)
        ));
        invocation.executable = root.root.clone();
        assert!(matches!(
            Job::start(&invocation, COMPILE_TIMEOUT),
            Err(Outcome::SpawnFailed(_))
        ));
        invocation.executable = "/bin/sleep".into();
        invocation.args = vec!["10".into()];
        let mut job = Job::start(&invocation, Duration::from_millis(50))
            .unwrap_or_else(|outcome| panic!("{outcome:?}"));
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            if let Some(outcome) = job.poll() {
                assert_eq!(outcome, Outcome::TimedOut);
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
        let safe = safe_text(b"\x1b]52;clipboard\x07\r\x08\n\t");
        assert!(!safe.contains('\x1b'));
        assert!(!safe.contains('\r'));
        assert!(safe.contains("\\u{7}"));
        assert_eq!(safe_text("\u{202e}abc".as_bytes()), "\\u{202e}abc");
    }
}
