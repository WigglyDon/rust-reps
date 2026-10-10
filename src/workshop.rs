//! Single owner of learner source, saved revision and build/run eligibility.
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::{
        fd::AsRawFd,
        unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    },
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::workshop_process::{self as process, Capture, Invocation, Job, Outcome};

pub const SOURCE_LIMIT: usize = 256 * 1024;
const ARTIFACT_LIMIT: usize = 32 * 1024 * 1024;
static SERIAL: AtomicU64 = AtomicU64::new(0);

fn unique_name(prefix: &str) -> String {
    format!(
        "{prefix}-{}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    )
}

pub struct Workspace {
    pub source_path: PathBuf,
    root: PathBuf,
    _lock: File,
    directories: Vec<OwnedDirectory>,
}

#[derive(Clone)]
struct OwnedDirectory {
    path: PathBuf,
    device: u64,
    inode: u64,
}
impl OwnedDirectory {
    fn inspect(path: &Path) -> io::Result<Self> {
        let meta = fs::symlink_metadata(path)?;
        if !meta.is_dir() || meta.file_type().is_symlink() {
            return Err(io::Error::other(
                "workspace/build path is no longer an ordinary directory",
            ));
        }
        Ok(Self {
            path: path.into(),
            device: meta.dev(),
            inode: meta.ino(),
        })
    }
    fn verify(&self) -> io::Result<()> {
        let now = Self::inspect(&self.path)?;
        if now.device != self.device || now.inode != self.inode {
            return Err(io::Error::other(
                "workspace/build directory was replaced; refusing to use the new location",
            ));
        }
        Ok(())
    }
}

impl Workspace {
    pub fn open(root: &Path) -> io::Result<Self> {
        let root = if root.is_absolute() {
            root.to_path_buf()
        } else {
            std::env::current_dir()?.join(root)
        };
        // Check every component before creating/opening dedicated output paths.
        let mut checked = PathBuf::new();
        let mut directories = Vec::new();
        for component in root.components() {
            if matches!(component, std::path::Component::ParentDir) {
                return Err(io::Error::other(
                    "workspace cannot contain parent components",
                ));
            }
            checked.push(component);
            match fs::symlink_metadata(&checked) {
                Ok(meta) if meta.is_dir() && !meta.file_type().is_symlink() => {}
                Ok(_) => {
                    return Err(io::Error::other(format!(
                        "workspace component is not an ordinary directory: {}",
                        checked.display()
                    )));
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => fs::create_dir(&checked)?,
                Err(error) => return Err(error),
            }
            directories.push(OwnedDirectory::inspect(&checked)?);
        }
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(root.join(".lock"))?;
        regular_file(&lock, usize::MAX)?;
        // SAFETY: lock is a live owned file descriptor, released when it closes.
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(io::Error::other(
                "workshop workspace already open; close the other instance",
            ));
        }
        let source_path = root.join("main.rs");
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&source_path)
        {
            Ok(file) => file.sync_all()?,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
        read_regular(&source_path, SOURCE_LIMIT)?;
        Ok(Self {
            source_path,
            root,
            _lock: lock,
            directories,
        })
    }

    fn read_source(&self) -> io::Result<String> {
        self.verify()?;
        String::from_utf8(read_regular(&self.source_path, SOURCE_LIMIT)?)
            .map_err(|_| io::Error::other("main.rs must be UTF-8; existing file was preserved"))
    }

    fn verify(&self) -> io::Result<()> {
        for directory in &self.directories {
            directory.verify()?;
        }
        Ok(())
    }

    fn save(&self, previous: &str, text: &str) -> io::Result<()> {
        if text.len() > SOURCE_LIMIT {
            return Err(io::Error::other("source exceeds 256 KiB"));
        }
        if self.read_source()? != previous {
            return Err(io::Error::other(
                "main.rs changed outside this session; buffer preserved. Save your external copy and reopen to reconcile",
            ));
        }
        if text == previous {
            return Ok(());
        }
        let temporary = self.root.join(unique_name(".save"));
        let result = (|| {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&temporary)?;
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
            // Catch an ordinary external edit that occurred during the write.
            // Advisory locking does not constrain noncooperating writers.
            if self.read_source()? != previous {
                return Err(io::Error::other(
                    "main.rs changed during save; buffer preserved",
                ));
            }
            fs::rename(&temporary, &self.source_path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }
}

fn regular_file(file: &File, limit: usize) -> io::Result<()> {
    let meta = file.metadata()?;
    if !meta.is_file() || meta.nlink() != 1 || meta.len() > limit as u64 {
        return Err(io::Error::other(
            "expected an ordinary, unshared file within the size limit",
        ));
    }
    Ok(())
}

fn read_regular(path: &Path, limit: usize) -> io::Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    regular_file(&file, limit)?;
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(io::Error::other("file grew beyond the size limit"));
    }
    Ok(bytes)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    Compile,
    Run,
}

pub struct Record {
    pub activity: Activity,
    pub revision: u64,
    pub build_attempt: u64,
    pub command: String,
    pub capture: Capture,
    pub outcome: Option<Outcome>,
}

impl Record {
    pub fn status_text(&self) -> String {
        match (&self.activity, &self.outcome) {
            (_, None) => "Active — F8 Stop".into(),
            (Activity::Compile, Some(Outcome::Exited { code: Some(0), .. })) => {
                "Compiled successfully (exit status 0)".into()
            }
            (
                Activity::Compile,
                Some(Outcome::Exited {
                    code: Some(code), ..
                }),
            ) => format!("Compiler rejected source / build failed (exit status {code})"),
            (
                Activity::Run,
                Some(Outcome::Exited {
                    code: Some(code), ..
                }),
            ) => format!("Program exited with status {code}"),
            (_, Some(Outcome::Exited { signal, .. })) => {
                format!("Process terminated by signal {signal:?}")
            }
            (Activity::Compile, Some(Outcome::MissingTool)) => {
                "Compiler unavailable: installed rustc was not found".into()
            }
            (Activity::Run, Some(Outcome::MissingTool)) => {
                "Program executable was not found".into()
            }
            (_, Some(Outcome::SpawnFailed(reason))) => format!("Process could not start: {reason}"),
            (_, Some(Outcome::Cancelled)) => "Cancelled by Stop/Quit".into(),
            (_, Some(Outcome::TimedOut)) => "Timed out; process group stopped".into(),
            (_, Some(Outcome::OutputLimit)) => {
                "Output limit exceeded; truncated and stopped".into()
            }
            (_, Some(Outcome::IoFailed(reason))) => format!("Process I/O failed: {reason}"),
        }
    }
}

struct Artifact {
    revision: u64,
    directory: OwnedDirectory,
    bytes: Vec<u8>,
}
struct Active {
    job: Job,
    record: Record,
    directory: OwnedDirectory,
}

pub struct Session {
    workspace: Workspace,
    pub source: String,
    saved: String,
    pub revision: u64,
    pub build_attempt: u64,
    pub message: String,
    pub compiler_output: Option<Record>,
    pub program_output: Option<Record>,
    compiler: PathBuf,
    artifact: Option<Artifact>,
    active: Option<Active>,
    directories: Vec<OwnedDirectory>,
}

impl Session {
    pub fn open(root: &Path) -> io::Result<Self> {
        let workspace = Workspace::open(root)?;
        let source = workspace.read_source()?;
        Ok(Self { workspace, source: source.clone(), saved: source, revision: 1, build_attempt: 0,
            message: "Preview for Don's review. Type the first program from Guidance; F5 saves and compiles.".into(),
            compiler_output: None, program_output: None, compiler: PathBuf::from("rustc"),
            artifact: None, active: None, directories: vec![] })
    }

    pub fn source_path(&self) -> &Path {
        &self.workspace.source_path
    }
    pub fn dirty(&self) -> bool {
        self.source != self.saved
    }
    pub fn busy(&self) -> bool {
        self.active.is_some()
    }
    pub fn change_source(&mut self, text: String) {
        if text != self.source {
            self.source = text;
            self.revision += 1;
            self.artifact = None;
            self.message = "Source changed. Compile this revision before Run.".into();
        }
    }

    pub fn save(&mut self) -> bool {
        match self.workspace.save(&self.saved, &self.source) {
            Ok(()) => {
                self.saved = self.source.clone();
                self.message = "Saved main.rs.".into();
                true
            }
            Err(error) => {
                self.message = format!("Save failed: {error}. Buffer retained; Quit blocked.");
                false
            }
        }
    }

    pub fn run_block_reason(&self) -> Option<&str> {
        if self.busy() {
            Some("A job is active. Stop it or wait before Run.")
        } else if self.dirty() {
            Some("Unsaved edits. Compile first to save and build this revision.")
        } else if self
            .artifact
            .as_ref()
            .is_none_or(|artifact| artifact.revision != self.revision)
        {
            Some("No successful build for this revision. Compile first.")
        } else {
            None
        }
    }

    pub fn compile(&mut self) {
        if self.busy() {
            self.message = "A job is active. Stop it or wait before Compile.".into();
            return;
        }
        self.artifact = None;
        self.build_attempt += 1;
        if !self.save() {
            return;
        }
        let directory = self.workspace.root.join(unique_name("build"));
        let setup = (|| {
            fs::DirBuilder::new().mode(0o700).create(&directory)?;
            self.directories.push(OwnedDirectory::inspect(&directory)?);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(directory.join("source.rs"))?;
            file.write_all(self.saved.as_bytes())?;
            file.sync_all()
        })();
        if let Err(error) = setup {
            self.message = format!("Build setup failed: {error}");
            return;
        }
        let invocation = process::compiler_invocation(&self.compiler, &directory);
        self.start(Activity::Compile, invocation, process::COMPILE_TIMEOUT);
    }

    pub fn run(&mut self) {
        if let Some(reason) = self.run_block_reason() {
            self.message = reason.into();
            return;
        }
        let artifact = self.artifact.as_ref().unwrap();
        let check = self.workspace.read_source().and_then(|source| {
            if source != self.saved {
                return Err(io::Error::other("main.rs changed outside this session"));
            }
            artifact.directory.verify()?;
            if read_regular(&artifact.directory.path.join("program"), ARTIFACT_LIMIT)?
                != artifact.bytes
            {
                return Err(io::Error::other(
                    "built artifact changed outside this session",
                ));
            }
            Ok(())
        });
        if let Err(error) = check {
            self.artifact = None;
            self.message = format!("Run blocked: {error}. Reconcile source and Compile again.");
            return;
        }
        let invocation = Invocation {
            executable: artifact.directory.path.join("program"),
            args: vec![],
            directory: artifact.directory.path.clone(),
            compiler: false,
        };
        self.start(Activity::Run, invocation, process::RUN_TIMEOUT);
    }

    fn start(&mut self, activity: Activity, invocation: Invocation, timeout: std::time::Duration) {
        let mut record = Record {
            activity,
            revision: self.revision,
            build_attempt: self.build_attempt,
            command: invocation.display(),
            capture: Capture {
                stdout: vec![],
                stderr: vec![],
                truncated: false,
            },
            outcome: None,
        };
        let directory = self
            .directories
            .iter()
            .find(|directory| directory.path == invocation.directory)
            .unwrap()
            .clone();
        let started = self
            .workspace
            .verify()
            .and_then(|()| directory.verify())
            .map_err(|error| Outcome::IoFailed(error.to_string()))
            .and_then(|()| Job::start(&invocation, timeout));
        match started {
            Ok(job) => {
                self.message = format!(
                    "{activity:?} active for revision {}. F8 Stop; source remains editable.",
                    self.revision
                );
                self.active = Some(Active {
                    job,
                    record,
                    directory,
                });
            }
            Err(outcome) => {
                record.outcome = Some(outcome);
                self.message = record.status_text();
                self.store(record);
            }
        }
    }

    fn store(&mut self, record: Record) {
        match record.activity {
            Activity::Compile => self.compiler_output = Some(record),
            Activity::Run => self.program_output = Some(record),
        }
    }

    pub fn output(&self, activity: Activity) -> Option<&Record> {
        if let Some(active) = &self.active
            && active.record.activity == activity
        {
            return Some(&active.record);
        }
        match activity {
            Activity::Compile => self.compiler_output.as_ref(),
            Activity::Run => self.program_output.as_ref(),
        }
    }

    pub fn poll(&mut self) {
        let Some(active) = &mut self.active else {
            return;
        };
        let outcome = active.job.poll();
        active.record.capture = active.job.capture.clone();
        if let Some(outcome) = outcome {
            let mut active = self.active.take().unwrap();
            let mut artifact_problem = None;
            if active.record.activity == Activity::Compile
                && outcome.success()
                && active.record.revision == self.revision
            {
                match self
                    .workspace
                    .verify()
                    .and_then(|()| active.directory.verify())
                    .and_then(|()| {
                        read_regular(&active.directory.path.join("program"), ARTIFACT_LIMIT)
                    }) {
                    Ok(bytes) => {
                        self.artifact = Some(Artifact {
                            revision: self.revision,
                            directory: active.directory.clone(),
                            bytes,
                        })
                    }
                    Err(error) => {
                        artifact_problem =
                            Some(format!("Compiled, but artifact unavailable: {error}"));
                    }
                }
            }
            active.record.outcome = Some(outcome);
            self.message = format!(
                "{:?} revision {}: {}. {}",
                active.record.activity,
                active.record.revision,
                active.record.status_text(),
                artifact_problem
                    .as_deref()
                    .unwrap_or(if active.record.revision == self.revision {
                        "Inspect the actual output."
                    } else {
                        "Previous revision; Compile current source before Run."
                    })
            );
            self.store(active.record);
        }
    }

    pub fn stop(&mut self) {
        if let Some(active) = &mut self.active {
            active.job.cancel();
            self.message = "Stop requested; terminating process group.".into();
        } else {
            self.message = "No active job.".into();
        }
    }

    pub fn shutdown(&mut self) {
        self.active.take();
    }

    pub fn recover(&self) -> io::Result<PathBuf> {
        let name = format!("{}.rs", unique_name("rust-reps-recovered"));
        let mut locations = Vec::new();
        if self.workspace.verify().is_ok() {
            locations.push(self.workspace.root.join(&name));
        }
        locations.push(std::env::temp_dir().join(name));
        let mut failure = io::Error::other("no recovery location");
        for path in locations {
            let saved = (|| {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(&path)?;
                file.write_all(self.source.as_bytes())?;
                file.sync_all()
            })();
            match saved {
                Ok(()) => return Ok(path),
                Err(error) => failure = error,
            }
        }
        Err(failure)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Drop the child before deleting its owned temporary paths.
        self.active.take();
        for directory in &self.directories {
            if self.workspace.verify().is_ok() && directory.verify().is_ok() {
                let _ = fs::remove_dir_all(&directory.path);
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    pub struct Fixture {
        pub root: PathBuf,
    }
    impl Fixture {
        pub fn new() -> Self {
            let base =
                Path::new(env!("CARGO_MANIFEST_DIR")).join(".rust-reps-local/workshop-1-tests");
            fs::create_dir_all(&base).unwrap();
            let root = base.join(unique_name("rust-reps-workshop-test"));
            fs::create_dir(&root).unwrap();
            Self { root }
        }
        pub fn open(&self) -> Session {
            Session::open(&self.root).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    pub fn settle(session: &mut Session) {
        let deadline = Instant::now() + Duration::from_secs(40);
        while session.busy() {
            assert!(Instant::now() < deadline, "job hung");
            session.poll();
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn build(session: &mut Session, source: &str) {
        session.change_source(source.into());
        session.compile();
        settle(session);
        assert!(
            session
                .compiler_output
                .as_ref()
                .unwrap()
                .outcome
                .as_ref()
                .unwrap()
                .success(),
            "{}",
            session.message
        );
    }

    #[test]
    fn real_program_error_repair_change_and_stale_refusal() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        session.run();
        assert!(session.program_output.is_none());
        build(&mut session, "fn main() { println!(\"first\"); }");
        assert!(session.program_output.is_none(), "Compile must never run");
        session.run();
        settle(&mut session);
        assert_eq!(
            session.program_output.as_ref().unwrap().capture.stdout,
            b"first\n"
        );
        session.change_source("fn main() { printl!(\"broken\"); }".into());
        assert!(session.dirty());
        session.run();
        assert!(session.message.contains("Compile"));
        session.compile();
        settle(&mut session);
        assert!(matches!(
            session.compiler_output.as_ref().unwrap().outcome,
            Some(Outcome::Exited { code: Some(1), .. })
        ));
        assert!(
            process::safe_text(&session.compiler_output.as_ref().unwrap().capture.stderr)
                .contains("printl")
        );
        assert!(session.source.contains("printl"));
        assert!(session.run_block_reason().is_some());
        let previous = session.program_output.as_ref().unwrap().revision;
        session.run();
        assert_eq!(session.program_output.as_ref().unwrap().revision, previous);
        build(&mut session, "fn main() { println!(\"changed\"); }");
        session.run();
        settle(&mut session);
        assert_eq!(
            session.program_output.as_ref().unwrap().capture.stdout,
            b"changed\n"
        );
        // Even retrying a build without edits must revoke the old artifact.
        session.compiler = fixture.root.join("missing-rustc");
        session.compile();
        assert_eq!(
            session.compiler_output.as_ref().unwrap().outcome,
            Some(Outcome::MissingTool)
        );
        assert!(session.run_block_reason().is_some());
    }

    #[test]
    fn saved_source_reopens_and_external_changes_are_preserved() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        let path = session.source_path().to_path_buf();
        session.change_source("fn main() {}".into());
        assert!(session.save());
        assert!(!session.dirty());
        drop(session);
        let mut session = fixture.open();
        assert_eq!(session.source, "fn main() {}");
        assert!(
            session.run_block_reason().is_some(),
            "reopening never revives an old binary"
        );
        session.change_source("my unsaved edit".into());
        fs::write(&path, "external user's edit").unwrap();
        assert!(!session.save());
        assert!(session.dirty());
        assert_eq!(session.source, "my unsaved edit");
        assert_eq!(fs::read_to_string(&path).unwrap(), "external user's edit");
        let recovery = session.recover().unwrap();
        assert_eq!(fs::read_to_string(&recovery).unwrap(), session.source);
        fs::remove_file(recovery).unwrap();
    }

    #[test]
    fn write_failure_keeps_buffer_and_concurrent_app_is_refused() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        assert!(Session::open(&fixture.root).is_err());
        session.change_source("unsaved learner work".into());
        fs::remove_file(session.source_path()).unwrap();
        fs::create_dir(session.source_path()).unwrap();
        session.compile();
        assert!(!session.busy());
        assert!(session.message.contains("Save failed"));
        assert_eq!(session.source, "unsaved learner work");
        assert!(session.dirty());
    }

    #[test]
    fn symlinks_hardlinks_and_unrelated_files_are_protected() {
        use std::os::unix::fs::symlink;
        let fixture = Fixture::new();
        let unrelated = fixture.root.join("unrelated.rs");
        fs::write(&unrelated, "preserve me").unwrap();
        let workspace = fixture.root.join("practice");
        fs::create_dir(&workspace).unwrap();
        symlink(&unrelated, workspace.join("main.rs")).unwrap();
        assert!(Session::open(&workspace).is_err());
        fs::remove_file(workspace.join("main.rs")).unwrap();
        fs::hard_link(&unrelated, workspace.join("main.rs")).unwrap();
        assert!(Session::open(&workspace).is_err());
        symlink(&workspace, fixture.root.join("alias")).unwrap();
        assert!(Session::open(&fixture.root.join("alias")).is_err());
        assert_eq!(fs::read_to_string(&unrelated).unwrap(), "preserve me");
    }

    #[test]
    fn replaced_workspace_and_build_directories_are_not_followed_or_deleted() {
        let fixture = Fixture::new();
        let root = fixture.root.join("practice");
        let mut session = Session::open(&root).unwrap();
        build(&mut session, "fn main() {}");
        let owned = session.artifact.as_ref().unwrap().directory.path.clone();
        let parked_build = owned.with_extension("parked");
        fs::rename(&owned, &parked_build).unwrap();
        fs::create_dir(&owned).unwrap();
        fs::write(owned.join("unrelated"), "preserve").unwrap();
        session.run();
        assert!(!session.busy());
        assert!(session.message.contains("directory was replaced"));
        session.change_source("learner unsaved".into());
        fs::rename(&root, fixture.root.join("parked-workspace")).unwrap();
        fs::create_dir(&root).unwrap();
        fs::write(root.join("main.rs"), "unrelated source").unwrap();
        assert!(!session.save());
        drop(session);
        assert_eq!(
            fs::read_to_string(root.join("main.rs")).unwrap(),
            "unrelated source"
        );
        assert_eq!(
            fs::read_to_string(
                fixture
                    .root
                    .join("parked-workspace")
                    .join(owned.file_name().unwrap())
                    .join("unrelated")
            )
            .unwrap(),
            "preserve"
        );
    }

    #[test]
    fn external_source_and_artifact_changes_block_run() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        build(&mut session, "fn main() {}");
        fs::write(session.source_path(), "fn main() { panic!(); }").unwrap();
        session.run();
        assert!(!session.busy());
        assert!(session.message.contains("changed outside"));
        assert!(session.program_output.is_none());
        fs::write(session.source_path(), &session.saved).unwrap();
        session.compile();
        settle(&mut session);
        let artifact = session
            .artifact
            .as_ref()
            .unwrap()
            .directory
            .path
            .join("program");
        fs::write(artifact, "unrelated replacement").unwrap();
        session.run();
        assert!(!session.busy());
        assert!(session.program_output.is_none());
        assert!(session.run_block_reason().is_some());
    }

    #[test]
    fn edits_during_compile_and_cancelled_build_cannot_be_run() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        session.change_source("fn main() {}".into());
        session.compile();
        let old_revision = session.revision;
        session.compile();
        assert!(session.message.contains("active"));
        session.change_source("fn main() { println!(\"new\"); }".into());
        settle(&mut session);
        assert_eq!(
            session.compiler_output.as_ref().unwrap().revision,
            old_revision
        );
        assert!(session.run_block_reason().is_some());
        session.compile();
        session.stop();
        settle(&mut session);
        assert_eq!(
            session.compiler_output.as_ref().unwrap().outcome,
            Some(Outcome::Cancelled)
        );
        assert!(session.run_block_reason().is_some());
    }

    #[test]
    fn real_nonzero_stdin_environment_and_safe_output() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        build(
            &mut session,
            r#"use std::io::Read;
fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    assert!(s.is_empty());
    assert!(std::env::var("HOME").is_err());
    assert!(std::env::var("CARGO_HOME").is_err());
    assert!(std::env::var("GH_TOKEN").is_err());
    println!("actual stdout\u{1b}[2J");
    eprintln!("actual stderr");
    std::process::exit(7);
}"#,
        );
        session.run();
        settle(&mut session);
        let record = session.program_output.as_ref().unwrap();
        assert_eq!(
            record.outcome,
            Some(Outcome::Exited {
                code: Some(7),
                signal: None
            })
        );
        assert_eq!(record.capture.stderr, b"actual stderr\n");
        assert!(process::safe_text(&record.capture.stdout).contains("\\u{1b}[2J"));
    }

    #[test]
    fn real_run_timeout_stop_output_limit_and_drop_cleanup() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        build(
            &mut session,
            "fn main() { loop { std::thread::sleep(std::time::Duration::from_millis(10)); } }",
        );
        session.run();
        session.stop();
        settle(&mut session);
        assert_eq!(
            session.program_output.as_ref().unwrap().outcome,
            Some(Outcome::Cancelled)
        );
        session.run();
        settle(&mut session);
        assert_eq!(
            session.program_output.as_ref().unwrap().outcome,
            Some(Outcome::TimedOut)
        );
        build(
            &mut session,
            "fn main() { loop { println!(\"{}\", \"x\".repeat(4096)); } }",
        );
        session.run();
        settle(&mut session);
        let record = session.program_output.as_ref().unwrap();
        assert_eq!(record.outcome, Some(Outcome::OutputLimit));
        assert!(record.capture.truncated);
        assert_eq!(record.capture.stdout.len(), process::OUTPUT_LIMIT);
        // Dropping an active job has no pipe-reader thread to strand.
        let owned_directory = session.artifact.as_ref().unwrap().directory.path.clone();
        session.run();
        let start = Instant::now();
        drop(session);
        assert!(start.elapsed() < Duration::from_secs(2));
        assert!(!owned_directory.exists());
        assert!(fixture.root.join("main.rs").is_file());
    }

    #[test]
    fn inherited_pipe_descendant_cannot_hold_completion_open() {
        let fixture = Fixture::new();
        let mut session = fixture.open();
        build(
            &mut session,
            r#"fn main() {
    let child = std::process::Command::new("/bin/sleep").arg("30").spawn().unwrap();
    println!("{}", child.id());
}"#,
        );
        let start = Instant::now();
        session.run();
        settle(&mut session);
        assert!(start.elapsed() < Duration::from_secs(2));
        let record = session.program_output.as_ref().unwrap();
        assert!(record.outcome.as_ref().unwrap().success());
        let pid: u32 = String::from_utf8_lossy(&record.capture.stdout)
            .trim()
            .parse()
            .unwrap();
        #[cfg(target_os = "linux")]
        {
            thread::sleep(Duration::from_millis(50));
            let state = fs::read_to_string(format!("/proc/{pid}/stat")).unwrap_or_default();
            assert!(
                state.is_empty() || state.split_once(") ").unwrap().1.starts_with('Z'),
                "descendant remains alive: {state}"
            );
        }
        #[cfg(not(target_os = "linux"))]
        let _ = pid;
    }
}
