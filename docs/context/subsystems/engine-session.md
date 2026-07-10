# Engine And Session Truth

Context role: Subsystem authority and navigation.
Scope: `src/engine/mod.rs` and its explicit catalog/lab input seams.
Canonical for: Exercise packets, attempts, pass/skip/completion transitions, and snapshots.
Not canonical for: Curriculum generation, terminal rendering, or editor state.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../CONTEXT_MATRIX.md`](../CONTEXT_MATRIX.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Engine tests and plain/TUI adapter tests.
Update triggers: Packet fields, transition semantics, source selection, or session counters change.

## Mission And Authority

Engine is pure session truth. It owns `ExercisePacket`, session index, passed,
skipped, attempts, completion, submit results, skip results, and snapshots.
Catalog supplies ordinary reps; lab supplies explicit synthetic packets.

Engine may call validator dispatch for a submission. It must not generate
curriculum, assign catalog IDs, print terminal output, own editor state, award
displayed rewards, or import terminal UI types. Client and practice consume
engine results; dependency must not reverse.

Attempts increment for submitted correct and wrong answers. Skip increments
skip and advances without becoming an attempt. Completion is navigation
structure, not mastery. Wrong answers are normal results, not errors.

## Integration And Proof

Current catalog packets carry prompt, expected answer, fixed Copy instruction,
task kind, validator, and answer rule. `LIVE_REPO_FACT`:
`catalog_exercise_packet_at` hard-codes `Copy this line exactly:` and therefore
cannot safely expose non-Copy curriculum yet.

Accepted proof: pure engine unit tests, adapter tests that use the same session,
and full CLI proof. These tests do not prove learning quality, manual TUI feel,
or future prompt metadata propagation.

Rollback is restoration of the previous pure packet/session mapping. There is
no persistence or telemetry; snapshots and tests are the observable state.
Resource behavior is bounded by the static catalog except explicit endless lab
indexing.

## Gate And Unknowns

The next production gate is a narrowly tested prompt/instruction propagation
decision that also resolves one-shot presentation. Until then, non-Copy output
stays inert. Update this page for packet, transition, source, or counter changes.
