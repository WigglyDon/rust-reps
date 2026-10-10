# Rust Reps product direction

Rust Reps exists so Don can make something he understands: type Rust himself,
compile it for real, inspect its output, change it and fix mistakes. Getting
closer to the underlying mechanism is the reward. Enjoyable, voluntary syntax
practice supports that curiosity. Compilation, program behavior and human
understanding are distinct; counters and successful exits do not establish mastery.

The requested experience is a simple terminal TUI with source on one side and
teaching information, compiler diagnostics and observed program output on the
other. An opt-in workshop preview now supplies one persistent edit–compile–run
loop. Its teaching content and interface still need Don's first-use review;
the restored course remains available through ordinary launch and practice modes.

The new curriculum should connect computing fundamentals to Rust, inspired by
FN64. Start without assumed knowledge of binary, registers, memory, processors
or functions. Use concrete examples and introduce terminology when it becomes
useful. Build a connected program one small Rust rep at a time. Explain each addition
and its result; start from no assumed programming experience.
Small components, explicit relationships and clear ownership are the relevant
Unix-inspired design values.

Two journeys must remain distinguishable. Hardware implements observable state
and operations; instruction encodings select those operations. Rust source can
be compiled into instructions executed by a host processor. Rust can also model
another processor's state and rules inside an emulator. These connections do
not mean that each Rust variable is a hardware register or each Rust expression
is one instruction.

The starter course is being designed through conversation with Don. The
[curriculum owner](CURRICULUM.md) records the current questions and first proposed
example. The restored course and opt-in workshop are described in [current
state](context/CURRENT_STATE.md). No claim that the existing curriculum already
meets the new direction is made.

The product remains Rust, terminal-first and inspectable. The restored practice
flow is deterministic; workshop results come from the actual learner program.
[Architecture](context/CONTEXT_MATRIX.md) owns implementation boundaries.
Dependencies and mechanisms are replaceable; learning purpose and honest
feedback are stable. Device mutation, emulator engineering and teaching a
second programming language are outside this curriculum assignment.
