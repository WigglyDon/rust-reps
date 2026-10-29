# Current Rust Reps state

## Implementation

The recovered repository contains one Rust terminal game. Catalog IDs 001–012
are literal Copy tasks; 013–022 form two Copy/Fill/Produce/Repair/Transform rows
around a function's return type. Plain and one-shot modes are finite. The
restored interactive course cycles the two five-task rows indefinitely. The lab
is separate synthetic UI stress data. The restored course has no persistence or
adaptive mastery.

`cargo run --locked --bin rust-reps -- workshop` opts into a Unix workshop preview.
The existing editor displays learner-owned `.rust-reps-local/workshop/main.rs`
under the launch directory, alongside Guidance/Compiler/Program tabs. Ctrl-S
saves; F5 saves and invokes real installed rustc; F6 runs a successful build for
the unchanged displayed revision; F8 stops; Esc/Ctrl-C saves and quits. Save
failures retain the buffer and block normal quit. Reopening preserves source
and requires a fresh build. Revision/build-attempt labels identify previous
output. One job at a time, process-group cleanup, bounded nonblocking capture
and time limits preserve responsiveness. This is trusted local execution with
user permissions, not a sandbox. [The workshop contract](subsystems/workshop.md)
owns its workspace, execution and recovery details.

Exact comparisons remove at most one final LF or CRLF. Full-code Produce,
Repair and Transform tasks use RustfmtExact through fixed rustfmt requests.
F1 formatting is an independent editor operation; F2 exposes authored help.
Orchestration resolves English guidance and constructs engine packets. The
planner selects supplied admitted tasks; it does not generate curriculum at
runtime. The Ukrainian guidance pack is proof-only.

The root/scoped instructions and current context have been reconciled with
source. `cargo xtask` provides workspace verification and local context
observation. Historical fleet packet/queue/respawn diagnostics remain optional;
they do not launch agents or grant task authority.

## New direction and unresolved design

[Product direction](../PRODUCT.md) and [curriculum design](../CURRICULUM.md) own
Don's clarified goal: make something he understands through manually typed Rust,
real compilation, output inspection, changes and repair. Underlying mechanisms
motivate that work. Assume no prior programming experience or CPU terminology.
The course should build a connected program one Rust rep at a time, inspired by FN64 and potentially
other projects. The original subtraction example is not the chosen starter. The
first unit is being designed through concrete examples and conversation. Its exact
content, order and scaffolding are unresolved; the workshop's small connected
program and repair guidance are explicitly subject to Don's review. The restored
catalog has not been replaced or claimed to satisfy the new course.

The [WORKSHOP-1 review](https://github.com/WigglyDon/rust-reps/issues/1#issuecomment-6098946445)
accepted proceeding to Don's small first-use review: type, compile, run, change
the message and confirm save/reopen. That remains the next product checkpoint;
further lessons and curriculum admission have not been dispatched. The current
workflow review changes operational documentation only, with no new runtime or
curriculum acceptance claim.

## Source and workflow

The public source destination is [WigglyDon/rust-reps](https://github.com/WigglyDon/rust-reps).
`main` is the integration branch. The restored `master` and three linked donor
worktrees are preserved. Local Git facts and context hashes are derived by
`cargo xtask context current --verify`; fetch separately for remote freshness.

The old supervisor/worker lane loop and mandatory packet relay are retired.
Active tasks and handbacks live in [GitHub issues](https://github.com/WigglyDon/rust-reps/issues).
[Development](../DEVELOPMENT.md) owns the workflow. Raw local sessions, reference
copies and evidence are excluded from source publication.

## Evidence limits

The restored baseline passed 576 product unit tests and 71 CLI integration
tests before modernization. Current validation is reported on the active issue;
this document is not a pass-by-pass task log. Runtime correctness, terminal feel,
learning value and physical-machine correspondence are separate claims. The
new curriculum requires Don's review of the first unit.
