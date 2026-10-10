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

`cargo xtask context current --verify` derives local Git and context facts without
fetching. Its remote-main identity and divergence are cached observations.
Fetch separately for integration freshness. A remote URL is identity, not proof
that the currently authenticated account has publication authority.

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
