# Curriculum and catalog

Curriculum supplies immutable tasks, static support and guidance. Catalog owns admission, IDs, order and lookup. The schema keeps instruction keys, assignments, stimuli, expected answers, rule/format properties and provenance separate. Orchestration supplies candidate summaries to the planner and resolves authored guidance; none of those jobs belongs to the engine. Catalog never expands templates at runtime. Current counts and new curriculum design belong to their linked owners.

[Architecture](../CONTEXT_MATRIX.md) owns dependency boundaries; [current state](../CURRENT_STATE.md)
owns implementation status; [curriculum design](../../CURRICULUM.md) owns the
new teaching direction.

Validation: Catalog invariants, guidance/planner failure tests, CLI lookup/list proof and Don review for learning changes.
Use [development](../../DEVELOPMENT.md) and [evidence](../../evidence/README.md)
for task authority, required checks and claim limits.
