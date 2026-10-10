//! Discussion-preview guidance. This is not catalog admission or a syllabus.
pub const STEPS: &[(&str, &str)] = &[
    (
        "1/3 Make something visible",
        r#"Purpose: make your own small program write a message. Type this complete program in the source pane; the editor supplies paired brackets and quotes. It starts empty and never fills in a solution for you.

fn main() {
    println!("I made this");
}

fn introduces a function: a named set of instructions. main is the starting function. () says it takes no inputs. { and } enclose its body. println! writes a line of text; ! marks a macro (a helper that expands into Rust code). Its (...) contains what to print. Double quotes enclose text; ; ends this instruction. Spaces indent the body for readability.

F5 Compile saves main.rs and asks installed rustc to turn that source into a program. Read Compiler output; an error is something to investigate. F6 Run separately starts your successful build. Read Program output. Authored example: I made this followed by a newline. Your observed output appears only after you Run.

Change the message yourself, Compile, then Run again. What changed? Compilation checks Rust's rules; it cannot tell whether you understand the program."#,
    ),
    (
        "2/3 Keep a value, then change it",
        r#"Purpose: keep a number and make a change visible in the same program. Keep fn main and its braces. Inside them, replace your message instruction by typing these additions yourself:

let mut count = 3;
println!("count: {}", count);
count = count + 1;
println!("count: {}", count);

let creates a named value. count is a name you choose, not a hardware register. mut allows its value to change. = gives it the value on the right. 3 and 1 are whole numbers. + adds; count + 1 reads the old value and computes a new one, then = replaces count.

In the quoted text, {} is a place to display a value. The comma separates the text from count, the value to display. Each ; ends an instruction. These lines go inside main, not after its closing brace.

Compile and Run. Authored example: count: 3, then count: 4 on separate lines. Actual output is in Program. Change the starting number or the amount added, predict what you will see, then Compile and Run. This is Rust state; where a compiler stores it is a separate question.

F3/F4 browse steps without changing your source. No completion score is awarded."#,
    ),
    (
        "3/3 Investigate and repair",
        r#"Purpose: find out how the compiler helps you fix a mistake, using your existing program.

In let mut count = 3;, remove mut. Compile. The later count = count + 1 tries to change a value whose declaration now forbids change. Read the compiler's genuine message and source location in Compiler output. A rejected build cannot Run, even if an older program succeeded.

Restore mut yourself. Compile and Run again. Try misspelling println as printl next, read the diagnostic, and repair it. Keep the source pane visible while Tab focuses the information pane; arrows/PageUp/PageDown scroll it. Tab returns to editing.

Help never submits, fixes code or replaces your work. Compiler diagnostics describe Rust rules, not your ability. A program exiting successfully does not establish its correctness or your understanding.

F2 cycles Guidance / Compiler / Program. F3/F4 change the guidance step. Ctrl-S saves; F5 saves and Compiles; F6 Runs; F8 Stops; Esc/Ctrl-C saves and quits. If saving fails, the buffer stays open for recovery.

This tiny progression is a preview for Don's review. Tell Don which explanation, control or error message was confusing before expanding it into a course."#,
    ),
];

pub const TRUST: &str = "Trusted local code only: Compile/Run use your permissions, not a security sandbox. No interactive stdin. Compile limit 30s; Run 5s; capture 64 KiB per stream. Stop kills the process group. Esc saves and quits; save failures retain the buffer.";
