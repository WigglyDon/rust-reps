//! Actual Unix terminal route: key input, real rustc, resize, resume and restoration.
#![cfg(target_os = "linux")]

use std::{
    fs::{self, File},
    io::{self, Read, Write},
    mem::MaybeUninit,
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::process::CommandExt,
    },
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Workspace(PathBuf);
impl Workspace {
    fn new() -> Self {
        let base =
            Path::new(env!("CARGO_MANIFEST_DIR")).join(".rust-reps-local/workshop-1-pty-tests");
        fs::create_dir_all(&base).unwrap();
        let path = base.join(format!(
            "rust-reps-workshop-pty-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn source(&self) -> PathBuf {
        self.0.join(".rust-reps-local/workshop/main.rs")
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Pty {
    master: File,
    slave: File,
    child: Child,
    original: libc::termios,
    bytes: Vec<u8>,
}

impl Pty {
    fn open(directory: &Path) -> Self {
        let mut master = -1;
        let mut slave = -1;
        let size = libc::winsize {
            ws_row: 45,
            ws_col: 160,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: openpty initializes both descriptors on success; the size is
        // valid and no terminal name buffer or input termios is requested.
        let result = unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null(),
                &size,
            )
        };
        assert_eq!(result, 0, "PTY unavailable: {}", io::Error::last_os_error());
        // SAFETY: we take each newly opened descriptor exactly once.
        let master = unsafe { File::from_raw_fd(master) };
        let slave = unsafe { File::from_raw_fd(slave) };
        let original = termios(&slave);
        // SAFETY: the descriptor is valid; keep the File as the sole owner.
        unsafe {
            let flags = libc::fcntl(master.as_raw_fd(), libc::F_GETFL);
            assert!(flags >= 0);
            assert_eq!(
                libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK),
                0
            );
        }
        let mut command = Command::new(env!("CARGO_BIN_EXE_rust-reps"));
        command
            .arg("workshop")
            .current_dir(directory)
            .env("TERM", "xterm-256color")
            .stdin(Stdio::from(slave.try_clone().unwrap()))
            .stdout(Stdio::from(slave.try_clone().unwrap()))
            .stderr(Stdio::from(slave.try_clone().unwrap()));
        // SAFETY: pre_exec only invokes async-signal-safe libc calls using
        // inherited fd 0, to establish this child's own controlling terminal.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() < 0 || libc::ioctl(0, libc::TIOCSCTTY, 0) < 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command.spawn().unwrap();
        Self {
            master,
            slave,
            child,
            original,
            bytes: vec![],
        }
    }

    fn send(&mut self, bytes: &[u8]) {
        self.master.write_all(bytes).unwrap();
    }

    fn pump(&mut self) {
        let mut buffer = [0; 8192];
        loop {
            match self.master.read(&mut buffer) {
                Ok(0) => break,
                Ok(count) => self.bytes.extend_from_slice(&buffer[..count]),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(error) => panic!("PTY read: {error}"),
            }
        }
        assert!(
            self.bytes.len() < 4 * 1024 * 1024,
            "unexpected terminal output flood"
        );
    }

    fn wait_for(&mut self, from: usize, text: &str) {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            self.pump();
            if self.bytes.len() > from && screen_text(&self.bytes).contains(text) {
                return;
            }
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "workshop exited early: {}",
                screen_text(&self.bytes)
            );
            assert!(
                Instant::now() < deadline,
                "terminal did not show {text:?}: {}",
                screen_text(&self.bytes)
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn resize(&mut self, columns: u16, rows: u16) {
        let size = libc::winsize {
            ws_row: rows,
            ws_col: columns,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: live PTY descriptor, initialized winsize and our live child pid.
        unsafe {
            assert_eq!(
                libc::ioctl(self.slave.as_raw_fd(), libc::TIOCSWINSZ, &size),
                0
            );
            assert_eq!(libc::kill(self.child.id() as i32, libc::SIGWINCH), 0);
        }
    }

    fn quit_and_check_restoration(&mut self) {
        self.send(b"\x1b");
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            self.pump();
            if let Some(status) = self.child.try_wait().unwrap() {
                assert!(status.success());
                break;
            }
            assert!(Instant::now() < deadline, "Quit hung");
            thread::sleep(Duration::from_millis(10));
        }
        self.pump();
        let restored = termios(&self.slave);
        assert_eq!(restored.c_lflag, self.original.c_lflag);
        assert_eq!(restored.c_iflag, self.original.c_iflag);
        assert_eq!(restored.c_oflag, self.original.c_oflag);
        assert_eq!(restored.c_cflag, self.original.c_cflag);
        assert_eq!(restored.c_cc, self.original.c_cc);
        assert!(
            self.bytes
                .windows(b"\x1b[?1049l".len())
                .any(|bytes| bytes == b"\x1b[?1049l"),
            "alternate screen was not restored"
        );
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        // Best-effort graceful cleanup even if an assertion fails during a job.
        let _ = self.master.write_all(b"\x1b[19~\x1b");
        let deadline = Instant::now() + Duration::from_secs(2);
        while self.child.try_wait().ok().flatten().is_none() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(10));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn termios(file: &File) -> libc::termios {
    let mut settings = MaybeUninit::uninit();
    // SAFETY: tcgetattr fills the termios structure when it succeeds.
    unsafe {
        assert_eq!(libc::tcgetattr(file.as_raw_fd(), settings.as_mut_ptr()), 0);
        settings.assume_init()
    }
}

// Interpret only the cursor/erase operations emitted by CrosstermBackend so
// assertions inspect the current displayed frame, not adjacent raw byte chunks.
// This is a test observer, not a product terminal emulator.
fn screen_text(bytes: &[u8]) -> String {
    let mut grid = vec![vec![' '; 240]; 80];
    let (mut row, mut column) = (0usize, 0usize);
    let decoded = String::from_utf8_lossy(bytes);
    let mut chars = decoded.chars();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.next() == Some('[') {
                let mut params = String::new();
                for next in chars.by_ref() {
                    if ('@'..='~').contains(&next) {
                        let values: Vec<usize> = params
                            .split(';')
                            .map(|value| value.parse().unwrap_or(0))
                            .collect();
                        let value = values.first().copied().unwrap_or(0);
                        match next {
                            'H' | 'f' => {
                                row = value.max(1) - 1;
                                column = values.get(1).copied().unwrap_or(1).max(1) - 1;
                            }
                            'G' => column = value.max(1) - 1,
                            'd' => row = value.max(1) - 1,
                            'A' => row = row.saturating_sub(value.max(1)),
                            'B' => row += value.max(1),
                            'C' => column += value.max(1),
                            'D' => column = column.saturating_sub(value.max(1)),
                            'J' if value == 2 => grid.iter_mut().for_each(|line| line.fill(' ')),
                            'K' if row < grid.len() => {
                                let line = &mut grid[row];
                                let start = column.min(line.len());
                                match value {
                                    0 => line[start..].fill(' '),
                                    1 => line[..start].fill(' '),
                                    2 => line.fill(' '),
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        break;
                    }
                    params.push(next);
                }
            }
        } else if ch == '\r' {
            column = 0;
        } else if ch == '\n' {
            row += 1;
        } else if !ch.is_control() {
            if let Some(cell) = grid.get_mut(row).and_then(|line| line.get_mut(column)) {
                *cell = ch;
            }
            column += 1;
        }
    }
    grid.into_iter()
        .map(|line| line.into_iter().collect::<String>().trim_end().to_owned())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn workshop_real_terminal_type_compile_run_resize_save_and_resume() {
    let workspace = Workspace::new();
    let mut pty = Pty::open(&workspace.0);
    pty.wait_for(0, "Guidance");
    assert_eq!(fs::read_to_string(workspace.source()).unwrap(), "");
    // The editor inserts the final brace and indentation; these are typed keys.
    pty.send(b"fn main() {\rprintln!(\"pty first\");");
    pty.send(b"\x1b[15~"); // F5 Compile
    pty.wait_for(0, "Compiled successfully");
    assert_eq!(
        fs::read_to_string(workspace.source()).unwrap(),
        "fn main() {\n    println!(\"pty first\");\n}"
    );
    let from = pty.bytes.len();
    pty.send(b"\x1b[17~"); // F6 Run
    pty.wait_for(from, "Program exited with status 0");
    pty.resize(120, 35);
    pty.wait_for(from, "stdout (observed)");
    // Focus and navigation operate through the actual crossterm event loop.
    pty.send(b"\t\x1b[6~\x1b[12~\x1b[14~\t");
    let from = pty.bytes.len();
    pty.resize(160, 45);
    pty.wait_for(from, "Guidance");
    // Add a deliberate trailing-token syntax error and repair with Backspace.
    pty.send(b"\x1b[F@");
    let from = pty.bytes.len();
    pty.send(b"\x1b[17~");
    pty.wait_for(from, "Unsaved edits");
    pty.send(b"\x1b[15~");
    pty.wait_for(from, "Compiler rejected source");
    pty.send(b"\x7f\x1b[15~");
    pty.wait_for(from, "Compiled successfully");
    pty.quit_and_check_restoration();
    let saved = fs::read_to_string(workspace.source()).unwrap();
    assert_eq!(saved, "fn main() {\n    println!(\"pty first\");\n}");
    drop(pty);
    let mut pty = Pty::open(&workspace.0);
    pty.wait_for(0, "pty first");
    pty.send(b"\x1b[17~");
    pty.wait_for(0, "No successful build");
    assert_eq!(fs::read_to_string(workspace.source()).unwrap(), saved);
    pty.quit_and_check_restoration();
}

#[test]
fn workshop_real_terminal_stop_navigation_and_quit_during_run() {
    let workspace = Workspace::new();
    fs::create_dir_all(workspace.source().parent().unwrap()).unwrap();
    let source = "fn main() { loop { std::thread::sleep(std::time::Duration::from_millis(10)); } }";
    fs::write(workspace.source(), source).unwrap();
    let mut pty = Pty::open(&workspace.0);
    pty.wait_for(0, "Guidance");
    pty.send(b"\x1b[15~");
    pty.wait_for(0, "Compiled successfully");
    let from = pty.bytes.len();
    pty.send(b"\x1b[17~");
    pty.wait_for(from, "Run active");
    // Navigate while the real program is alive, then Stop without waiting 5s.
    pty.send(b"\t\x1b[12~\x1b[6~");
    pty.resize(140, 40);
    pty.wait_for(from, "Guidance");
    let start = Instant::now();
    pty.send(b"\x1b[19~\x1b[12~\x1b[12~");
    pty.wait_for(from, "Cancelled by Stop/Quit");
    assert!(start.elapsed() < Duration::from_secs(2));
    let from = pty.bytes.len();
    pty.send(b"\x1b[17~");
    pty.wait_for(from, "Run active");
    pty.quit_and_check_restoration();
    assert_eq!(fs::read_to_string(workspace.source()).unwrap(), source);
}
