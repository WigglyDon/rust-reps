# Rust Reps product direction

Rust Reps exists for Don's enjoyable, voluntary practice of Rust syntax. Manual
typing can make unfamiliar code familiar while exploring ideas he finds
fundamentally interesting. Completion and accuracy counters describe practice;
they do not establish mastery or predict whether coding will be necessary in
the future.

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
example. Existing source is the restored terminal game described in [current
state](context/CURRENT_STATE.md). No claim that the existing curriculum already
meets the new direction is made.

The product remains Rust, terminal-first, deterministic and inspectable.
[Architecture](context/CONTEXT_MATRIX.md) owns implementation boundaries.
Dependencies and mechanisms are replaceable; learning purpose and honest
feedback are stable. Device mutation, emulator engineering and teaching a
second programming language are outside this curriculum assignment.
