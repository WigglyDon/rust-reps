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

| Lane ID | Page | Worktree | Branch | Status |
| --- | --- | --- | --- | --- |
| `pure-practiceplan-builder-v1` | [`pure-practiceplan-builder-v1.md`](pure-practiceplan-builder-v1.md) | `/home/don/rust-reps-practiceplan-v1` | `lane/pure-practiceplan-builder-v1` | active; implementation not yet accepted or integrated |

## Planned but not active

- `USER_DECISION`: Tiny Multi-Operation Rust Set Materialization V1.
- `USER_DECISION`: Product Integration Decision V1.

## Paused lanes

- `USER_DECISION`: new Ukrainian/French curriculum, validator integration, lab
  cleanup, product settings, interface localization, persistence, and scheduling.

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
| `static-guidance-schema-v1` | [`static-guidance-schema-v1.md`](static-guidance-schema-v1.md) | `1ae5da321a7b1aeb0265a6856be7f8db7d1325f1` | Activated at `9de0227345f7589cd24322fc9861b7d61ac4d506`, fast-forwarded to local canonical master; worktree removed and branch deleted |
| `pure-guidance-resolution-v1` | [`pure-guidance-resolution-v1.md`](pure-guidance-resolution-v1.md) | `b61d509e19c7627d63d062fe31c95b1ce579b928` | Activated at `6ddc22dfc717a9da0dfc8bf9f1fa4a79fc784b13`, fast-forwarded to local canonical master; worktree removed and branch deleted; product behavior unchanged; no push or deploy |
| `static-capability-setplan-v1` | [`static-capability-setplan-v1.md`](static-capability-setplan-v1.md) | `bff7107d6485c06773af25987b321a5494b4e47e` | Activated at `86fd5a13c416a50373ca8a8ac72354c812161233`, fast-forwarded to local canonical master; worktree removed and branch deleted; product behavior unchanged; no push or deploy |

## Historical milestones

- Exact CLI/practice foundation.
- TUI/client boundary and sealed editor shell.
- Curriculum schema, sharding, and catalog metadata.
- Validator/tool-runner capability seams.
- Static procgen source and pure owned expansion.
- Static core-task/guidance schema split.
- Pure deterministic guidance resolution with inert English and Ukrainian proof
  packs.
- Static representation, support, learning-focus, capability, and SetPlan
  source for one Rust five-step ladder.

See [`../context/PROJECT_HISTORY.md`](../context/PROJECT_HISTORY.md) for evidence and transitions.
