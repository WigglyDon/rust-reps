# Static Guidance Schema V1 Lane

Context role: Active lane coordination record.
Scope: Inert core-task field migration and static English guidance source schema.
Canonical for: This lane's purpose, worktree, branch, scope, proof, and stop conditions.
Not canonical for: Product behavior, accepted-main implementation, catalog admission, or private Master GPT context.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`lane-registry.md`](lane-registry.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: Live Git/worktree state, candidate proof, and packet `RR-GUIDANCE-SCHEMA-20260710-001`.
Update triggers: Base, worktree, branch, scope, status, proof, integration, or retirement changes.

- Lane ID: `static-guidance-schema-v1`
- Purpose: implement the inert core-task/guidance schema split without product use.
- Supervisor destination: `Master GPT`
- Supervisor expected reply type: `MASTER_GPT_IMPLEMENTATION_REVIEW`
- Latest packet ID: `RR-GUIDANCE-SCHEMA-20260710-001`
- Current goal: `Static Core Task / Guidance Schema Split V1`
- Worker Codex worktree: `/home/don/rust-reps-guidance-schema-v1`
- Branch: `lane/static-guidance-schema-v1`
- Accepted base source: local canonical `master` at `6a5aae5f0228fe82bf1eecb401f4cf940dc397b5`
- Artifact references: `/tmp/rust-reps-static-core-guidance-schema-split-v1-proof.txt`
- Current status: active; context activated, implementation not yet accepted or integrated.
- Allowed scope: inert curriculum procgen source migration, inert static guidance source, module-local tests, proof, and one local candidate commit.
- Prohibited authority: catalog registration, engine integration, guidance resolution, product settings/localization/output, dependencies, push, deploy, release, and donor-worktree changes.
- Required proof: targeted procgen/guidance tests, source-isolation audits, cargo formatting/tests, `scripts/proof.sh`, diff checks, and clean final candidate.
- Subsystem links: [`../context/subsystems/curriculum-catalog.md`](../context/subsystems/curriculum-catalog.md), [`../context/subsystems/procgen.md`](../context/subsystems/procgen.md).
- Dependencies: accepted Cross-Lingual Guidance and Presentation Architecture V1 and canonical product base `6a5aae5f0228fe82bf1eecb401f4cf940dc397b5`.
- Overlap risks: `src/curriculum/mod.rs` and inert procgen files; no other active lane exists.
- Integration condition: Master GPT accepts complete candidate proof and patch; a later Master Codex packet alone may integrate.
- Stop conditions: product behavior or Exact changes, legacy/replacement fields coexist, guidance gains expected-answer authority, unapproved paths/dependencies are required, or unknown work is at risk.
- Retirement condition: candidate is integrated or explicitly rejected and preserved as donor evidence.
- Next milestone: Pure Guidance Resolution V1 only after this candidate is accepted and integrated.
- Unresolved risk: static schema tests do not prove linguistic quality or future product integration.
- Exact next action: create the isolated worktree, implement only the authorized inert schema, prove it, and return it for Master GPT review.
- Authority state: one local candidate commit only; no integration, push, deploy, product authority, or behavior change.
- Expected proof: targeted and full automated gates, source isolation, exact patches, clean canonical and lane worktrees.
- Stale session identity: none; this is the first session for the lane.
- Deactivation instruction: after integration or explicit rejection, stop assigning work while preserving the branch, worktree, packets, reports, and proof until retirement is authorized.
- Donor-worktree authority: none; parked curriculum donor worktrees remain preserved and unchanged.
