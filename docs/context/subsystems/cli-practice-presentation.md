# CLI, Practice, Presentation, And Main

Context role: Subsystem authority and navigation.
Scope: `src/cli.rs`, `src/practice.rs`, `src/presentation.rs`, and `src/main.rs`.
Canonical for: Launch selection, plain stdin/stdout adaptation, and orchestration boundaries.
Not canonical for: Session truth, curriculum ownership, or answer correctness.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../CONTEXT_MATRIX.md`](../CONTEXT_MATRIX.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: `tests/cli.rs` and `scripts/proof.sh` transcripts.
Update triggers: CLI grammar, launch mode, prompt output, command handling, or orchestration changes.

## Mission And Authority

CLI selects help/list/version/plain/play/lab/TUI/one-shot forms. Main wires the
selected adapter and interactive default. Practice reads stdin, delegates to
engine, and prints deterministic plain results. Presentation owns shared text
formatting but not behavior truth.

Allowed direction is orchestration reading catalog/guidance and adapters
reading resolved engine APIs. Forbidden is
duplicating session transitions, normalizing answers, generating curriculum,
or allowing docs/TUI text to silently redefine plain output.

One-shot treats every input as an answer. Practice recognizes only its declared
colon commands; unknown colon input is a command error without an attempt.
Wrong answers exit successfully as product results. CLI parsing/tool failures
use stderr and nonzero status.

## Proof And Limits

Accepted evidence is exact unit/CLI output, stdin, stderr, and exit status at an
identified executable/source SHA. `scripts/proof.sh` also checks an installed
temporary command. This does not prove interactive fullscreen feel.

`LIVE_REPO_FACT`: current product launch paths obtain resolved packets through
orchestration. One shared task-block writer receives instruction and stimulus
only, preserves exact bytes and omits empty stimuli, and is used by plain and
one-shot projection. Expected-answer truth does not enter task formatting.

Rollback is restoring exact prior transcripts. Observability is stdout/stderr
and exit status; there is no telemetry. Resource use is synchronous terminal
I/O over a small static catalog.

## Gate And Unknowns

Selector and multilingual presentation work are paused. Plain and one-shot
remain finite exact proof oracles; future multiline input framing requires a
separate contract before multiline tasks enter those clients. Varied
Deterministic Rust Practice Flow V1 is planned but inactive. Update on any
argument, output, task block, command, or launch-mode change.
