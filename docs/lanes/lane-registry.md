# Rust Reps Lane Registry

Context role: Canonical lane-status registry.
Scope: Active, planned, paused, blocked, parked, retired, and historical lanes.
Canonical for: Whether a repository-evidenced lane is active and where its page lives.
Not canonical for: Product behavior, private supervisor state, or branch tips.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`../context/README.md`](../context/README.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: Live worktree inventory and lane pages.
Update triggers: Lane creation, pause, block, integration, retirement, respawn, or worktree change.

## Active lanes

None.

## Planned but not active

- `USER_DECISION`: Cross-Lingual Guidance and Presentation Architecture V1 is
  the next design-only boundary. No supervisor, worker, branch, or worktree is active.
- `UNKNOWN`: selector implementation and new curriculum have product intent but
  no active worktree/branch evidence.

## Paused lanes

- `USER_DECISION`: Tiny Checked Procgen Materialization V1.
- `USER_DECISION`: validator integration, lab cleanup, product settings, and localization.

No paused item is represented as a live worker without repository/worktree evidence.

## Blocked lanes

None currently established by live lane evidence.

## Parked donor lanes

| Branch | Worktree | HEAD at discovery | Classification |
| --- | --- | --- | --- |
| `curriculum-data-seam-v0` | `/home/don/rust-reps-curriculum-data` | `d8d8207303d98c953e6c656ab680ed4ccabd3d8a` | Clean historical divergence; donor/reference, 62 master commits vs 2 branch commits |
| `curriculum-sharding-v0` | `/home/don/rust-reps-curriculum-sharding` | `c3ac7f748e88ff93f12d252babb219560c325641` | Clean ancestor already absorbed by master |

These worktrees remain preserved and receive no context update because they are
not active lanes.

## Retired lanes

| Lane ID | Page | Accepted candidate | Retirement state |
| --- | --- | --- | --- |
| `master-infrastructure` | [`master-infrastructure.md`](master-infrastructure.md) | `34874d51a6ec54a5ec219f9f2ca99d1449eadf6e` | Fast-forwarded to local canonical master; worktree removed and branch deleted |

## Historical milestones

- Exact CLI/practice foundation.
- TUI/client boundary and sealed editor shell.
- Curriculum schema, sharding, and catalog metadata.
- Validator/tool-runner capability seams.
- Static procgen source and pure owned expansion.

See [`../context/PROJECT_HISTORY.md`](../context/PROJECT_HISTORY.md) for evidence and transitions.
