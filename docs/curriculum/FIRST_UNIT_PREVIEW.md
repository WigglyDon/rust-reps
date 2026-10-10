# First unit preview: a program whose state we can see

This is a discussion draft, not active app content or an accepted syllabus.
The proposed project grows toward a small processor model inspired by FN64.
The first unit starts before processor terminology: write a Rust program that
holds a number, changes it, and shows the change. Later units can turn those
changes into explicit machine operations and encoded commands.

A program is a set of instructions we write for the computer to carry out.
Rust source is text in the Rust language. A compiler translates a complete
valid program into executable instructions for the host computer. Rust Reps
currently checks typing; it does not run the learner's submission. The results
below are explanations of these authored programs.

## Rep 1: make a program say something

Goal: establish where execution starts and how an observable result is requested.

```rust
fn main() {
    println!("Machine ready");
}
```

`fn` introduces a function: a named piece of work. `main` is the starting
function for this executable program. Its empty parentheses mean this function
has no declared inputs. The braces surround the work in its body. Indentation
helps us see what belongs inside; the braces define that boundary.

`println!` is Rust's instruction here to produce a line of text. More precisely,
it is a macro: the compiler expands this request into code. The `!` marks that
macro form. Its parentheses surround the request's input. Quotes mark literal
text; the quote marks themselves are not printed. The semicolon ends the
statement. The program's displayed line is `Machine ready`.

Typing task: copy the complete program. Every required symbol has been introduced.

## Rep 2: store a value and read it

Goal: the displayed result should come from a value in the program.

```rust
fn main() {
    let state = 0;
    println!("State: {state}");
}
```

`let` introduces the name `state` and binds it to the starting value `0`.
This `=` supplies a value; it is not a question asking whether two things are
equal. `state` is a programmer-chosen name, not a physical register address.
Inside the quoted printing text, `{state}` asks Rust to insert the current value
named `state`. These braces have a different role from the braces around a
function body. The displayed line is `State: 0`.

Typing task: replace the first program with this complete version. The prior
program boundary and printing form now support storing and observing a value.

## Rep 3: change the stored value

Goal: show both the starting state and a result produced from it.

```rust
fn main() {
    let mut state = 0;
    println!("Before: {state}");
    state = state + 1;
    println!("After: {state}");
}
```

`mut` says the value bound to this name may be changed. Read the right side of
`state = state + 1;` first: take the current value of `state`, then add `1`.
The left side says where to put that result. After this statement, the name
`state` refers to `1`. The two displayed lines are `Before: 0` and `After: 1`.
Statements in this function run in their written order, so the first printing
request observes the old value and the second observes the changed value.

Typing task: enter the complete version, using the explained additions. This
is the same small program growing, not a separate arbitrary syntax question.

## What this unit earns

The learner has met a program boundary, a display request, a name and initial
value, reading that value, an update and written execution order. The program
has observable changing state. It is not yet a processor or an N64 core.
Next units can give the program explicit state storage and named operations,
then introduce instructions that choose those operations. The appropriate
step size and amount of repetition need learner feedback.

Admission would require actual task/support data, answer-rule and default-flow
decisions and product verification under [curriculum design](../CURRICULUM.md).
An authored example is not a result from executing a submitted answer. No
arbitrary learner-code execution is proposed by this preview.
