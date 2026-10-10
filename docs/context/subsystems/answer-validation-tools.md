# Answer validation and formatting

`answer` implements literal equality after one final line-ending trim. `validators` dispatches explicit Exact or RustfmtExact. Full-code tasks currently use a fixed rustfmt runner; syntax rejection is Wrong, infrastructure failure is unavailable. F1 formatting is independent, capability-gated and cannot read expected answers. These tools do not compile or run learner programs. The separate
[workshop adapter](workshop.md) implements the explicitly authorized compiler/program
loop without changing validation or formatter contracts.

[Architecture](../CONTEXT_MATRIX.md) owns dependency boundaries; [current state](../CURRENT_STATE.md)
owns implementation status; [curriculum design](../../CURRICULUM.md) owns the
new teaching direction.

Validation: Answer/rule/tool failure tests, CLI proof and formatting capability tests.
Use [development](../../DEVELOPMENT.md) and [evidence](../../evidence/README.md)
for task authority, required checks and claim limits.
