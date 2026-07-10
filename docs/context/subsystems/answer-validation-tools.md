# Answer, Validators, Formatter, And Tool Runner

Context role: Subsystem authority and navigation.
Scope: `src/answer.rs`, `src/validators.rs`, `src/formatter.rs`, and `src/tool_runner.rs`.
Canonical for: Exact law and the separate non-exact/tool/format capability boundaries.
Not canonical for: Curriculum selection, terminal UI, or product admission.
Inherits: [`../../../AGENTS.md`](../../../AGENTS.md) and [`../CONTEXT_MATRIX.md`](../CONTEXT_MATRIX.md).
Current-state owner: [`../CURRENT_STATE.md`](../CURRENT_STATE.md).
Related evidence: Answer/validator/tool/formatter tests and CLI proof.
Update triggers: Normalization, validator dispatch, tool request, formatter, or product caller changes.

## Mission And Separation

`answer` owns exact truth: remove at most one final LF or one final CRLF and
compare every other byte/character shape literally. Validators classify
validation capability and failures; they may not weaken Exact. Tool runner owns
fixed local execution requests; formatter owns explicit F1 editor formatting.

Allowed dependencies: validators call answer and the fixed runner boundary;
explicit TUI format action calls formatter. Forbidden: answer invoking tools,
formatter mutating submitted/expected answers, arbitrary product commands,
catalog admission based on formatting, and any classifier turning mismatch into
pass.

The real runner accepts only fixed `rustfmt --emit stdout` formatting requests,
with timeout/output bounds and no custom working directory. Compiler/linter/test
kinds are representable but rejected by real execution. RustfmtExact evaluation
exists for explicit tests; normal dispatch returns unsupported.

## Proof And Limits

Accepted proof covers exact whitespace/Unicode/line endings, failure classes,
fake request capture, real-runner allow-list behavior, timeout/output bounds,
and explicit formatter buffer preservation. Full proof seals product transcripts.

It does not authorize product RustfmtExact, prove every host rustfmt version,
or make formatter output answer truth. Runtime integration is Exact only plus
the separate user-invoked F1 helper. Rollback is fail-closed removal of an
unaccepted capability path. There is no persistence; stderr/status captures are
test evidence. Runtime resource limits are explicit time and byte bounds.

## Gate And Unknowns

Validator integration, new tools, dependencies, and broader commands are
paused. A future gate needs product failure semantics, transcript proof, and
Don judgment. Update for any normalization, caller, executable, limit, or
failure-class change.
