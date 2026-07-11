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
| truthful-task-projection-v1 | [truthful-task-projection-v1](truthful-task-projection-v1.md) | /home/don/rust-reps-task-projection-v1 | lane/truthful-task-projection-v1 | active |

Active lane records must use this five-column table because `lane-doctor`
consumes it. Bullet-form active records are invalid coordination memory.

## Planned but not active

- `USER_DECISION`: Tiny Generated Catalog Admission V1 remains later and
  blocked; proposed IDs 013-017 are unassigned.
- `USER_DECISION`: Five-Step Product Flow V1 remains later and is needed only
  if generated admission does not already define the accepted flow.
- `USER_DECISION`: Guidance-Locale Settings Decision remains design-only and
  Ukrainian product guidance still requires human review.

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
| `pure-practiceplan-builder-v1` | [`pure-practiceplan-builder-v1.md`](pure-practiceplan-builder-v1.md) | `0521e448dfa204993c6a65edc51cb9018ef9b185` | Activated at `64ec59c185ca22387dd583b145fcb9c4ed8f15ed`, fast-forwarded to local canonical master; worktree removed and branch deleted; product behavior unchanged; no push or deploy |
| `tiny-multi-operation-materialization-v1` | [`tiny-multi-operation-materialization-v1.md`](tiny-multi-operation-materialization-v1.md) | `67e0e55ee7ebc7970c81c41123c1c994eccbf12c` | Activated at `bbffd795733b1f23f99c121ed94951cdfff7581a`, fast-forwarded to local canonical master; worktree removed and branch deleted; pre-integration lane metadata was unavailable because the active entry used noncanonical bullet form; registry restored to canonical table form; product behavior unchanged; no push or deploy |
| `resolved-core-task-orchestration-v1` | [`resolved-core-task-orchestration-v1.md`](resolved-core-task-orchestration-v1.md) | `e4ab24cf9447601abc79f8a2bcead2e588252483` | Activated at `7c1c45d32fa030e85d277983c125d73c2e9bca69`, fast-forwarded to local canonical master; worktree removed and branch deleted; IDs 013-017 remain unassigned; product behavior unchanged; no push or deploy |

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
- Pure deterministic PracticePlan construction over supplied candidate IDs,
  selected guidance availability, supported answer rules, and plan seed.
- Five exact inert Rust templates and checked materialized tasks with exact
  English/Ukrainian proof guidance and authored PracticePlan agreement.
- Unified admitted core-task records, source-neutral guidance input, and one
  inert pure resolved-core-task orchestration seam for current reps 001-012.

See [`../context/PROJECT_HISTORY.md`](../context/PROJECT_HISTORY.md) for evidence and transitions.
