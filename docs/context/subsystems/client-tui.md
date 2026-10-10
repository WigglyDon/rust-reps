# Portable client and terminal adapter

Client owns portable editor/cursor/viewports, visual feedback and task/help projection. TUI owns terminal lifecycle, input mapping, scheduling, drawing and restoration. F2 exposes authored support without submission or answer leakage. F1 requests capability-gated editor formatting independently. Both consume engine truth. Do not infer real-terminal feel from synthetic frames.

[Architecture](../CONTEXT_MATRIX.md) owns dependency boundaries; [current state](../CURRENT_STATE.md)
owns implementation status; [curriculum design](../../CURRICULUM.md) owns the
new teaching direction.

Validation: Editor/input/render/lifecycle/privacy tests, product proof and real-terminal review for interaction changes.
Use [development](../../DEVELOPMENT.md) and [evidence](../../evidence/README.md)
for task authority, required checks and claim limits.
