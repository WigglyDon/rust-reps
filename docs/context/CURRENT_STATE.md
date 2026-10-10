# Current Rust Reps state

## Implementation

The recovered repository contains one Rust terminal game. Catalog IDs 001–012
are literal Copy tasks; 013–022 form two Copy/Fill/Produce/Repair/Transform rows
around a function's return type. Plain and one-shot modes are finite. The
interactive product cycles the two five-task rows indefinitely. The lab is
separate synthetic UI stress data. There is no persistence or adaptive mastery.

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
Don's new goal: enjoyable manual Rust typing while learning the state and
operations beneath processor behavior. Assume no prior programming experience or CPU terminology. The course should
build a connected program one Rust rep at a time, inspired by FN64 and potentially
other projects. The original subtraction example is not the chosen starter. The
first unit is being designed through concrete examples and conversation. Its exact
content, order and scaffolding are unresolved; the restored catalog has not yet
been replaced or claimed to satisfy the new course.

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
