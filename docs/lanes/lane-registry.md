> Historical record. Its former lane/status/packet requirements confer no current authority. Follow [current development](../DEVELOPMENT.md).

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

None. Live assignments are tracked in GitHub issues. Preserved branches and worktrees are historical donor state.

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
| `truthful-task-projection-v1` | [`truthful-task-projection-v1.md`](truthful-task-projection-v1.md) | `ad56f9725eab16203f44e8518e31e230c2d3987b` | Activated at `8b72ddb0b47261d8c34dcda18ea98cd65d96f664`, fast-forwarded to local canonical master after Don's accepted playtest; worktree removed and branch deleted; no push or deploy |
| `varied-deterministic-flow-v1` | [`varied-deterministic-flow-v1.md`](varied-deterministic-flow-v1.md) | `a0bdbdc00751b08d49b36956fb82600001348217` | Fast-forwarded to local canonical master after Don's `PASS_WITH_OBSERVATIONS` playtest; worktree removed and branch deleted without force; no push or deploy |
| `problem-first-curriculum-v1` | [`problem-first-curriculum-v1.md`](problem-first-curriculum-v1.md) | `39c1ba0d606e6bc7dc1a7c857c12dbb856f6e645` | Fast-forwarded to local canonical master after Don's `PASS` playtest; worktree removed and branch deleted without force; no push or deploy |

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
- Truthful resolved instruction/stimulus projection, aligned editor/task source
  geometry, marked soft continuations, and explicit task-format capability.
- Two deterministic admitted five-operation rows, endless interactive round
  cycling, finite proof surfaces, beginner concept support, static focus ranges,
  and F2 help without expected-answer exposure.

See [`../context/PROJECT_HISTORY.md`](../context/PROJECT_HISTORY.md) for evidence and transitions.
