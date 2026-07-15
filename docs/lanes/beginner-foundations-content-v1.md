# Beginner Foundations Content V1 Lane

Context role: Active lane record.
Scope: Registration and provisioning record for one future bounded, hand-authored beginner-foundations content and guidance implementation.
Canonical for: This lane's identity, current provisioning state, future implementation boundary, authority limits, user curriculum evidence, risks, and next action.
Not canonical for: Product behavior, task answers, implementation details, candidate truth, integration authority, release authority, donor state, or private Master GPT context.
Inherits: [`../../AGENTS.md`](../../AGENTS.md) and [`lane-registry.md`](lane-registry.md).
Current-state owner: [`../context/CURRENT_STATE.md`](../context/CURRENT_STATE.md).
Related evidence: Git history, packet `RR-BEGINNER-FOUNDATIONS-20260715-012`, the verified problem-first integration result, and Don's direct curriculum-quality evidence.
Update triggers: Registration, provisioning, fresh implementation authority, candidate creation, integration, pause, block, or retirement.

- Lane ID: `beginner-foundations-content-v1`
- Current status: `PROVISIONED; AWAITING FRESH IMPLEMENTATION PACKET`
- Supervisor destination: `Master Codex`
- Supervisor expected reply type: `MASTER_CODEX_BEGINNER_CONTENT_RESULT_V1`
- Latest packet ID: `RR-BEGINNER-FOUNDATIONS-20260715-012`
- Current goal: `register-and-provision-beginner-foundations-content-v1`
- Future implementation goal: `beginner-foundations-content-and-guidance-v1`
- Worker Codex worktree: `/home/don/rust-reps-beginner-foundations-content-v1`
- Branch: `codex/beginner-foundations-content-v1`
- Accepted base: `8e789f243b24e86ec2adf7a51066f61c3f6ac7cb` (`docs: close problem-first curriculum integration`)
- Integration evidence: `/home/don/rust-reps-fleet/outbox/RR-PROBLEM-FIRST-CURRICULUM-20260714-011-RESULT.md`, SHA-256 `8ba2f56c7e8723fabbf2de085fd52c7402fed5a4b900c84edd0e31ef7afa20ad`
- Authority state: registration and provisioning only; this packet grants no implementation authority. Product implementation requires a fresh packet matching the final provisioning SHA and Context-SHA.
- Prohibited authority: product, curriculum, catalog, procgen, guidance, support, planning, orchestration, engine, validator, client, TUI, CLI, test, proof-script, README, Cargo, dependency, installation, integration, push, publication, release, deployment, donor, or queue-candidate changes.
- Unresolved risk: the exact live beginner prerequisite graph; current IDs 001-022 concept order; replacement versus retirement strategy for the current ten-task return-type loop; catalog identity consequences; plain and one-shot compatibility; default-support rendering limits; and the first meaningful artifact.
- Exact next action: after successful provisioning, Master GPT issues one fresh implementation packet using the final provisioning SHA and Context-SHA.

## User Curriculum Evidence

- `USER_DECISION`: Beginner Foundations Content V1 is the selected next product boundary.
- `USER_DECISION`: the task-first layout works, and editor and source positioning are acceptable.
- `USER_DECISION`: current curriculum formatting is ugly.
- `USER_DECISION`: current prose is extremely technical and verbose; the default explanation is not beginner-friendly and reads as glossary dumping rather than a lesson.
- `USER_DECISION`: the ten varied tasks loop over two closely related function forms and provide effectively no useful beginner progression.
- `USER_DECISION`: changing `five/i32/5` to `seven/u64/7` does not create meaningful learning.
- `USER_DECISION`: `u64` appears before the learner understands what it means or why it differs from `i32`.
- `USER_DECISION`: the issue is classified `NEEDS FIX - CONTENT QUALITY, PREREQUISITE ORDER, AND MEANINGFUL VARIATION`.
- No repair is claimed by registration or provisioning.

## Future Implementation Intent

The accepted task-first shell remains the base. Future implementation must
improve curriculum content rather than add another presentation abstraction or
more mechanical slot combinations.

### Preserve Accepted Product Behavior

- Preserve task-first TUI layout, editor/stimulus alignment, F2 lifecycle, and expected-answer privacy.
- Preserve F1, F5, F7, Esc, editor, mouse, smart-pair, smart-Enter, Delete, wrapping, scrolling, gutter, counter, `+1`, Wrong, and Skip behavior.
- Preserve Exact and RustfmtExact laws, deterministic planning, endless normal interactive practice, and finite plain, fallback, and one-shot behavior.
- Preserve stdout, stderr, exit status, and newline contracts.
- Add no runtime LLM generation, persistence, or dependency.

### Progressive Explanation

1. The default view teaches one immediate idea.
2. Default support uses short conversational sentences.
3. Default support is not a glossary or specification.
4. A definition appears when its term becomes relevant, not all at once.
5. F2 contains optional depth and a different example.
6. F2 does not restate the whole default block.
7. Labels remain visually secondary.
8. Category metadata does not wrap into a dominant pseudo-heading.
9. Uppercase is reserved for short action controls, not paragraphs.
10. No line exists merely because metadata can produce it.

Exact tested wording and rendering limits require fresh authority and live
implementation inspection.

### Prerequisite Order

- Every player-visible token must already be taught, be explained in the current introduction, be deliberately highlighted with immediate support, or be removed from the task.
- Do not introduce `u64` in the first beginner unit merely to vary a slot.
- Do not assume knowledge of bit widths or require inference among `i32`, `u64`, whole numbers, signedness, and size.
- Keep one reviewed numeric type stable inside a set unless numeric-type contrast is the declared concept.
- Teach a numeric-type contrast only in its own reviewed set.

### Meaningful Variation

- Variation must change what the learner understands or does.
- Changing only a function name, integer literal, or integer type is not a new beginner lesson unless that change is the declared, taught target.
- A good set has one learner-facing concept, one clear prerequisite state, one coherent symbolic anchor, operation changes that train the concept, no unrelated syntax, a reason for each example, and a small visible sense of progress.

### Tiny First Implementation Boundary

- A later packet must inspect live source and implement one bounded hand-authored beginner-foundations unit.
- The unit must be large enough to feel like learning and small enough for Don to inspect completely; it must not add mass curriculum.
- Live inspection must determine the smallest coherent prerequisite chain before return-type variation using current IDs 001-022 and their established concepts.
- Candidate concepts include code as instructions, a value, a whole-number value, a function as named runnable code, a function giving a value back, one reviewed Rust number type, and return-type notation.
- Do not retain the current ten-task loop as the beginner default merely because it is integrated.
- Do not delete or rewrite accepted content without exact identity, catalog, planning, CLI, and rollback proof.

### Language And Symbolic-System Boundary

- Human guidance locale and symbolic practice system remain independent axes.
- Generic concept meaning remains separable from Rust surface syntax; a function giving a value back may be explained independently from Rust's arrow notation.
- Client and TUI must not branch on Rust tokens.
- Do not product-admit Ukrainian, Korean, TypeScript, or another system merely to prove this boundary.

## Provisioning Boundary

- Current product behavior remains unchanged.
- The declared worktree and branch exist and initially pointed to registration commit `030e9d83d1b2cd2f7863681981aea7e9464a575c`.
- The worker was clean at creation, and lane-doctor returned `READY` for the declared worktree, branch, lane ID, lane page, and accepted base.
- No implementation has started and no candidate exists.
- No integration-queue row exists.
- No installation occurs under this packet.
- Don playtest is not applicable to provisioning.
- Both parked donors remain outside this lane's authority.
- Fresh implementation authority requires a new packet matching the final provisioning commit and Context-SHA recorded by the verified provisioning result.
