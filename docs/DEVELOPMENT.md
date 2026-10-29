# Development workflow

## Complete assigned outcomes

The active Don instruction or authorized GitHub issue supplies purpose, scope
and external authority. Issues carry amendments, progress, questions, dependencies
and handback; source and repository documents carry lasting meaning. Direct
instructions remain valid while an issue is being established. Reuse the existing
assignment rather than creating duplicate issues or empty PRs.

Agents own inspection, implementation, focused proof, repair, reconciliation,
commits and authorized delivery. Checkpoints are progress, not automatic stops.
An unresolved product decision blocks its dependent changes while independent
work continues. Explain real choices with concrete examples. Do not ask Don to
mediate ordinary Git mechanics or to choose among terminology he has not learned.

## Entry and ownership

Start with [root law](../AGENTS.md), [the index](INDEX.md), the current task and
nearest scoped instructions. Inspect checkout/branch/HEAD, dirty/index/untracked
state, remotes, linked worktrees, interrupted operations and affected resources.
Read source and observe runtime before promoting old documentation into fact.

Use an owned task branch. Existing parked branches/worktrees are donor evidence;
their names or historical ACTIVE labels do not authorize a worker or reserve a
resource indefinitely. Linked worktrees share refs. Own build targets, output
paths, ports and mutable test storage; serialize conflicting operations and
preserve unique evidence. A restricted process view does not prove host-wide
inactivity. Coordinate actual conflicting shared semantics or resources.

Existing local tooling is useful for review, but a Cargo entry can rebuild that
tooling. Inspect source/lockfile identity and the existing artifact before reuse;
a shared version string does not establish which source produced an executable.
For a documentation-only or preservation review, invoke an existing matching
developer helper directly for `context current --verify` and `context verify`.
If none is available, first use Git/diff and read-only file/hash observations;
report the missing helper rather than starting an unplanned toolchain build.

`cargo xtask context current --verify` derives local Git and context facts without
fetching. Its remote-main identity and divergence are cached observations.
Fetch separately for integration freshness. A remote URL is identity, not proof
that the currently authenticated account has publication authority.

Before a growing build/test/install phase, check the actual filesystem and RAM,
the active task's bounds, source/profile/target ownership and competing writers.
Use one admitted target directory for that source and profile; route temporary
files, proof output and install roots to owned writable locations. Retain existing
host margins and serialize conflicting heavy phases through their actual owner.
A previous free-space reading is not a reservation. Missing space, cache inputs,
toolchain or destination blocks that phase; do not lower guards or silently
redirect to another filesystem. Prefer focused current checks to rebuilding old
matrices. `--locked` preserves dependency resolution; it does not make a command
offline or prevent compilation. Use `--offline` only with the required inputs
already present, and report a cache miss explicitly.

The terminal workshop needs an interactive terminal and installed Rust compiler,
not a phone, server, provider account or signing identity. Optional facility
discovery follows the existing owner's private route or a locally installed
`resource-discovery` entry. Discovery grants no operation. Personal devices keep
their current owner window and task-specific custody; ordinary authorized local
review needs no unrelated device approval. Keep fleet paths, identities, capacity
receipts and recovery material out of public source and issues.

## Local checks

`cargo xtask verify` runs workspace formatting, warnings-denied Clippy, workspace
tests, context verification, actual product transcript/temporary-install proof,
and working/staged diff checks. Product proof remains usable through
`./scripts/proof.sh`; historical fleet diagnostics remain accessible through
`cargo xtask` and `./scripts/fleet.sh`.

Choose focused checks by the affected input and boundary; repeat the full gate
when changes or failures justify it. Documentation checks establish paths and
hashes, not semantic truth. Synthetic UI tests do not establish real-terminal
feel, and game counters do not establish learning. Record unrun or failed checks
honestly. Keep raw logs in the ignored `.rust-reps-local/` directory.

Update the nearest canonical owner when accepted understanding changes. After
review, regenerate context with `cargo xtask context manifest` and check it with
`cargo xtask context verify`. Verification itself does not rewrite context.

Workshop proof uses owned temporary workspaces and actual installed rustc,
without global installation or dependency downloads. The workshop PTY check
covers the explicit terminal route and restoration; its scope is software
interaction, not Don's first-use acceptance. [The runner contract](context/subsystems/workshop.md)
owns process limits, trust and source recovery behavior.

## Persistence and recovery

Keep the learner's source separate from compiler caches. Changing a target or
install prefix does not relocate the workshop's working-directory-based save.
Preserve that file and any reported recovery copy before replacing a checkout,
changing storage or retiring output. Recheck the actual input/destination and
current ownership; an old storage receipt, symlink or backup path does not prove
that its bytes survive. Retired destinations require a fresh owner route and
qualification, with no implicit fallback.

Ignored directories can contain unique reports, unfinished source or nested Git
repositories as well as reconstructible compiler output. Inspect the exact
subtree before cleanup; do not treat the whole `target` directory as disposable.
Preserve dirty/staged/untracked bytes, local-only refs and any object-alternate
dependencies together. A clean outer checkout or remote main cannot restore
all of those. Rebuildable binaries are not substitutes for original diagnostic
evidence, and a local hash list alone is not an independent backup/readback.

## Integration and publication

`main` is the public integration branch. Restored `master` and parked donor
branches remain references. Preserve unexplained dirt and unpublished commits;
use additive commits and merges, never resets, hidden stashes, rebase/amend/squash
of shared history, force-push or broad cleanup. Stage only owned scope and review
the complete staged diff.

Re-read material task amendments and fetch fresh `origin/main`. Reconcile a
competing advance by inspecting and merging it, then revalidate affected combined
behavior. With active publication authority, use a normal fast-forward push to
main or an authorized PR merge. Observe the remote result; do not forge or bypass
repository restrictions. PRs are optional review vehicles, not authority.

Don's current instruction authorizes creating this public repository and
publishing this modernization. Future publication follows the applicable active
assignment; new visibility, licensing, spending, hosted automation, packages,
installation or devices require their own authority. No GitHub Actions is used.
Source publication is distinct from installation, release and Don's acceptance.

Hand back the actual outcome, relevant identities/checks, limits and remaining
product questions on the issue, with a short chat result. An agent report is
reported evidence, not an independent rerun. Routine continuation needs no chat
archive or manual packet transport.
