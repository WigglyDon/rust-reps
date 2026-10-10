# Engine and session

Engine owns resolved ExercisePacket, attempts, correct/wrong, skip, completion and snapshots. Orchestration supplies packets; engine does not look up catalog or guidance. Submission uses the explicit validator; wrong is a normal result. Skip advances without an attempt. Finite catalog practice, supplied looping rounds and synthetic lab sources are distinct. Expected answers remain private submission truth.

[Architecture](../CONTEXT_MATRIX.md) owns dependency boundaries; [current state](../CURRENT_STATE.md)
owns implementation status; [curriculum design](../../CURRICULUM.md) owns the
new teaching direction.

Validation: Engine transition/failure tests and actual plain/TUI adapter proof.
Use [development](../../DEVELOPMENT.md) and [evidence](../../evidence/README.md)
for task authority, required checks and claim limits.
