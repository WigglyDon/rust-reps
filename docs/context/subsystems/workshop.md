# Workshop contract

WORKSHOP-1 authorizes an opt-in, trusted local edit–compile–run preview, separate
from the restored course and its validators. The contract was recorded before
implementation; source and focused proof establish its behavior.

`workshop` owns saved source, revision identity, successful artifact eligibility
and compiler/program records. `workshop_process` owns controlled process launch,
capture and termination. `client::workshop` reuses the portable editor and owns
focus, guidance selection and scrolling. `tui::workshop` owns terminal events and
rendering. `curriculum::workshop` owns reviewable teaching prose, without catalog
admission or expected-answer matching.

The workspace is `.rust-reps-local/workshop` under the launch directory. Its
ordinary UTF-8 `main.rs` starts empty so the learner types the first complete
program. Save is explicit; Compile saves first; normal Quit saves before leaving
and refuses to leave after a save failure. Guidance changes never change source.
On an unexpected terminal-loop error, attempt a source save after child cleanup
and terminal restoration. If saving fails, write a private uniquely named recovery
`.rs` file in the verified workspace, falling back to the system temporary
directory, and report its path. If both writes fail, report the escaped buffer on
stderr after restoration. Abrupt process termination/power loss can lose edits
made since the last Save/Compile. Atomic
save protects ordinary concurrent edits; it is not exclusion against hostile or
noncooperating external writers racing the final rename.

A single advisory workspace lock excludes another app instance. Reject symlink
workspace components and nonregular source files. Refuse to overwrite source
changed by another writer; use a new temporary file and atomic rename for saves.
Recheck directory device/inode identities before use or cleanup; preserve replaced
directories rather than treating a new path occupant as owned work.
Learner source and build paths are ignored locally and excluded from publication.
The executable location, install prefix and Cargo target directory do not select
this workspace. Return to the same working directory to reopen the same source.
Preserve source/recovery copies when changing storage; revalidate actual bytes
instead of assuming that an old path or backup receipt still resolves. Compiler
output can be rebuilt after recovery; it cannot replace the learner's source.

Compile snapshots one saved revision in a uniquely owned build directory and
invokes installed `rustc` directly with fixed single-file, edition-2024 arguments.
No Cargo, dependencies, build scripts, shell interpolation or interactive stdin.
Artifacts are temporary and removed on normal closure; reopening needs a build.
Run explicitly invokes only the successful artifact for the unchanged displayed
revision. Any edit or new build attempt revokes eligibility. Verify source and
artifact identity again before Run. Retained output labels its revision and
whether it belongs to a previous revision or build attempt. Compilation and exit
status award no learning progress.

The first adapter supports Unix process groups. One job at a time, nonblocking
stdout/stderr capture, 30-second compile and 5-second run limits, and 64 KiB per
stream keep input responsive. Stop, timeout, output overflow, quit and errors
terminate the group and reap the direct child. On normal completion terminate
remaining group members too; held pipes cannot block capture or exit. These
limits and group cleanup do not contain deliberately escaping hostile code.
Stdin is closed. Child environments are cleared; compiler receives only toolchain
lookup/home variables and locale/temp settings, with rustup auto-install disabled;
programs receive a fixed system PATH, locale and owned temporary directory.
No credential variables are forwarded.
Commands, separate streams, exit/signal, missing tool, spawn/I/O failures,
compiler rejection, cancellation, timeout and truncation remain visible.
Source is limited to 256 KiB and eligible artifacts to 32 MiB. Build directories
are private (mode 0700); source/recovery files use mode 0600.
These are job limits, not host storage/RAM admission. A growing compile phase
still follows [development](../../DEVELOPMENT.md) and the current resource owner.

Output is rendered as escaped text, including control and directional formatting
characters. Working directories, environment filtering and limits are **not a
security sandbox**. Locally authored Rust runs deliberately with the user's
permissions and can access files, processes and the network. This preview is not
for hostile code or downloaded projects. No global installation is involved.

Validation must exercise real rustc and child processes, persistence/conflicts,
stale execution refusal, editor/navigation/resize and a real PTY route. Human
first-use feedback and curriculum acceptance remain Don's decisions.

[Architecture](../CONTEXT_MATRIX.md) owns boundaries;
[development](../../DEVELOPMENT.md) owns verification and delivery.
