# Curriculum design

## Agreed purpose

Make something understandable through manual Rust typing, real compilation,
observed output, deliberate changes and repair. Don has programmed in other
languages but wants the course to
work for someone completely new to programming. Assume neither programming
knowledge nor CPU terminology. Explain what each addition means, why it belongs
in the program, and what its Rust symbols do before asking the learner to use it.

The course should feel like building something one rep at a time. Isolated
syntax copying, arbitrary renaming and unexplained integer-width changes do
not supply that progression. A small completed unit should leave a visible
capability or an understandable state change that later units build upon.
Typing remains voluntary practice for pleasure, not proof of mastery.

## Project progression

Don proposed “build the FN64 core” one rep at a time in a teaching order, while
also welcoming other projects. FN64 is a motivating history with many useful
examples, not a requirement to copy the subtraction analogy from the original
excerpt. That analogy was evidence of interest, not an accepted starter lesson.

A possible main thread is a small processor model that grows from observable
values and changes into operations, encoded instructions, memory and execution.
Each Rust addition should solve a problem already explained in plain language.
Later connections to FN64 must identify the actual source rule and its limits.
The learning model is not an N64 core, and completing its exercises is not an
emulator accuracy claim.

Other project threads can reinforce the same foundations through different
results: an inspectable byte format, a tiny image made from stored pixel values,
or a deterministic event/state simulation. These are possible directions, not
an admitted syllabus. Choose their order and depth after a concrete first unit
has been tried. Don should be asked about clarity, interest and pacing rather
than expected to choose among unfamiliar architecture terms.

## Teaching a rep

Show the program's current purpose and the next small thing it needs to do.
Explain that change and introduce each required Rust token. Provide a short
worked example, then a bounded manual typing task. Show how the addition fits
into the growing program and what it changes. Early tasks should use complete
examples and ample support; variation and recall follow familiarity.

A typed snippet must state its surrounding context. A complete program must
explain its required scaffolding. The learner should not have to guess what
`fn`, punctuation, a name or a type means. Explain reading, initial values,
updates and later operations as different actions when they become relevant.
Teach programmer-written names as names rather than pretending they are
physical processor registers.

The restored course compares submitted text. The separately requested opt-in
workshop compiles a saved single-file Rust program and runs its successful build
only on deliberate learner action. Its [runner contract](context/subsystems/workshop.md)
is for trusted local practice code and does not provide a security sandbox.
Guidance distinguishes authored example output from observed compiler/program
streams. Compile success is feedback on Rust rules; neither it nor a successful
exit proves program correctness or understanding.

## Physical-to-software connection

Use understandable physical information, the convention that represents states
as bits, storage and logic, and encoded instructions to explain the relationships
that interest Don. Introduce them as needed by the growing project rather than
requiring a preliminary electronics course. Explain separately how documentation
and controlled observations inform an emulator's Rust implementation.

Hardware behavior, instruction-set rules, Rust semantics and actual compiler
output are related levels, not interchangeable objects. A Rust array is Rust
data; its compiler controls the host representation. An emulator uses such data
to model a guest machine. Chosen model state is not measured silicon evidence;
code matching its own tests does not establish a physical hardware match.

## Admission and next decisions

Review the [first unit preview](curriculum/FIRST_UNIT_PREVIEW.md) with Don before
replacing the active curriculum. It is a concrete discussion draft, not admitted
app content.
The first project, exact starter content, pacing, optional depth and relationships
between project threads remain open. The accepted baseline is now a course for
someone new to programming, built through accumulating Rust practice.

Keep task instructions, visible stimulus, private expected answer, answer rule
and format capability distinct. Introduce every required token before testing
it. Define catalog IDs/order, prerequisite and default-flow consequences before
admission. Validate actual plain, one-shot and TUI behavior, wrong answers, skip,
formatting failure and answer privacy. Preserve earlier evidence in Git.

The restored 22-task catalog and two-row interactive loop remain available.
`workshop` adds three short preview guidance steps: type a complete printing
program, grow it with an observable changing value, and investigate/repair a
compiler error. Steps never overwrite the source or insert a solution. This is
one reviewable experience slice, not catalog admission or acceptance of a syllabus.
