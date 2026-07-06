use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

const DRILL_001_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 001\nPrompt:\n\nfn main() {}\n\n>PASS\n";
const DRILL_001_TRY_AGAIN_OUTPUT: &str = "Rust Reps\n----------------\n\nDrill 001\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n";
const DRILL_001_BLANK_LINE_OUTPUT: &str = DRILL_001_TRY_AGAIN_OUTPUT;
const DRILL_001_TRAILING_SPACE_OUTPUT: &str = DRILL_001_TRY_AGAIN_OUTPUT;
const DRILL_001_CRLF_PASS_OUTPUT: &str = DRILL_001_PASS_OUTPUT;
const DRILL_002_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 002\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>PASS\n";
const DRILL_002_TRY_AGAIN_OUTPUT: &str = "Rust Reps\n----------------\n\nDrill 002\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>TRY AGAIN\nExpected:\nprintln!(\"Rust Reps\");\n";
const DRILL_003_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 003\nPrompt:\n\nlet answer = 42;\n\n>PASS\n";
const DRILL_004_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 004\nPrompt:\n\nlet name = \"Rust Reps\";\n\n>PASS\n";
const DRILL_005_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 005\nPrompt:\n\nlet mut score = 0;\n\n>PASS\n";
const DRILL_006_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 006\nPrompt:\n\nscore += 1;\n\n>PASS\n";
const DRILL_006_TRY_AGAIN_OUTPUT: &str = "Rust Reps\n----------------\n\nDrill 006\nPrompt:\n\nscore += 1;\n\n>TRY AGAIN\nExpected:\nscore += 1;\n";
const DRILL_007_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 007\nPrompt:\n\nlet passed = true;\n\n>PASS\n";
const DRILL_008_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 008\nPrompt:\n\nlet score: i32 = 0;\n\n>PASS\n";
const DRILL_009_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 009\nPrompt:\n\nlet scores = [1, 2, 3];\n\n>PASS\n";
const DRILL_010_PASS_OUTPUT: &str = "Rust Reps\n----------------\n\nDrill 010\nPrompt:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>PASS\n";
const DRILL_011_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 011\nPrompt:\n\nif score > 0 {}\n\n>PASS\n";
const DRILL_012_PASS_OUTPUT: &str =
    "Rust Reps\n----------------\n\nDrill 012\nPrompt:\n\nwhile score < 10 {}\n\n>PASS\n";
const DRILL_012_TRY_AGAIN_OUTPUT: &str = "Rust Reps\n----------------\n\nDrill 012\nPrompt:\n\nwhile score < 10 {}\n\n>TRY AGAIN\nExpected:\nwhile score < 10 {}\n";
const LIST_OUTPUT: &str = "Rust Reps\n----------------\n\nAvailable drills:\n001  fn main() {}\n002  println!(\"Rust Reps\");\n003  let answer = 42;\n004  let name = \"Rust Reps\";\n005  let mut score = 0;\n006  score += 1;\n007  let passed = true;\n008  let score: i32 = 0;\n009  let scores = [1, 2, 3];\n010  fn add_one(x: i32) -> i32 { x + 1 }\n011  if score > 0 {}\n012  while score < 10 {}\n";
const HELP_OUTPUT: &str = "Rust Reps\n----------------\n\nUsage:\n  rust-reps\n  rust-reps [DRILL_ID]\n  rust-reps play\n  rust-reps --list\n  rust-reps tui\n  rust-reps --help\n  rust-reps --version\n\nDefault:\n  Starts a practice session.\n\nCurrent drills:\n001  fn main() {}\n002  println!(\"Rust Reps\");\n003  let answer = 42;\n004  let name = \"Rust Reps\";\n005  let mut score = 0;\n006  score += 1;\n007  let passed = true;\n008  let score: i32 = 0;\n009  let scores = [1, 2, 3];\n010  fn add_one(x: i32) -> i32 { x + 1 }\n011  if score > 0 {}\n012  while score < 10 {}\n";
const VERSION_OUTPUT: &str = "rust-reps 0.1.0\n";
const PRACTICE_ALL_CORRECT_INPUT: &str = concat!(
    "fn main() {}\n",
    "println!(\"Rust Reps\");\n",
    "let answer = 42;\n",
    "let name = \"Rust Reps\";\n",
    "let mut score = 0;\n",
    "score += 1;\n",
    "let passed = true;\n",
    "let score: i32 = 0;\n",
    "let scores = [1, 2, 3];\n",
    "fn add_one(x: i32) -> i32 { x + 1 }\n",
    "if score > 0 {}\n",
    "while score < 10 {}\n",
);
const PRACTICE_ALL_CORRECT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>PASS\n\n",
    "Drill 003/012\nPrompt:\n\nlet answer = 42;\n\n>PASS\n\n",
    "Drill 004/012\nPrompt:\n\nlet name = \"Rust Reps\";\n\n>PASS\n\n",
    "Drill 005/012\nPrompt:\n\nlet mut score = 0;\n\n>PASS\n\n",
    "Drill 006/012\nPrompt:\n\nscore += 1;\n\n>PASS\n\n",
    "Drill 007/012\nPrompt:\n\nlet passed = true;\n\n>PASS\n\n",
    "Drill 008/012\nPrompt:\n\nlet score: i32 = 0;\n\n>PASS\n\n",
    "Drill 009/012\nPrompt:\n\nlet scores = [1, 2, 3];\n\n>PASS\n\n",
    "Drill 010/012\nPrompt:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>PASS\n\n",
    "Drill 011/012\nPrompt:\n\nif score > 0 {}\n\n>PASS\n\n",
    "Drill 012/012\nPrompt:\n\nwhile score < 10 {}\n\n>PASS\n\n",
    "COMPLETE\n",
    "Summary:\n",
    "Passed: 12/12\n",
    "Skipped: 0\n",
    "Attempts: 12\n",
);
const PRACTICE_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);
const PRACTICE_RETRY_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\n",
    "Expected:\n",
    "fn main() {}\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Skipped: 0\n",
    "Attempts: 2\n",
);
const PRACTICE_WRONG_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n\nDrill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\nSummary:\nPassed: 0/12\nSkipped: 0\nAttempts: 1\n",
);
const PRACTICE_HELP_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Commands:\n",
    ":help    show commands\n",
    ":status  show session status\n",
    ":list    show drills\n",
    ":skip    skip current drill\n",
    ":q       quit\n",
    ":quit    quit\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);
const PRACTICE_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Status:\n",
    "Current: 001/012\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);
const PRACTICE_LIST_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Available drills:\n",
    "001  fn main() {}\n",
    "002  println!(\"Rust Reps\");\n",
    "003  let answer = 42;\n",
    "004  let name = \"Rust Reps\";\n",
    "005  let mut score = 0;\n",
    "006  score += 1;\n",
    "007  let passed = true;\n",
    "008  let score: i32 = 0;\n",
    "009  let scores = [1, 2, 3];\n",
    "010  fn add_one(x: i32) -> i32 { x + 1 }\n",
    "011  if score > 0 {}\n",
    "012  while score < 10 {}\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);
const PRACTICE_SKIP_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>SKIP\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 1\n",
    "Attempts: 0\n",
);

const PRACTICE_HELP_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Commands:\n",
    ":help    show commands\n",
    ":status  show session status\n",
    ":list    show drills\n",
    ":skip    skip current drill\n",
    ":q       quit\n",
    ":quit    quit\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Status:\n",
    "Current: 001/012\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);

const PRACTICE_WRONG_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\n",
    "Expected:\n",
    "fn main() {}\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Status:\n",
    "Current: 001/012\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n",
);

const PRACTICE_CORRECT_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>Status:\n",
    "Current: 002/012\n",
    "Passed: 1/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n",
);

const PRACTICE_SKIP_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>SKIP\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>Status:\n",
    "Current: 002/012\n",
    "Passed: 0/12\n",
    "Skipped: 1\n",
    "Attempts: 0\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 1\n",
    "Attempts: 0\n",
);

const PRACTICE_UNKNOWN_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>UNKNOWN COMMAND\nType :help for commands.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);

const PRACTICE_UNKNOWN_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>UNKNOWN COMMAND\nType :help for commands.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Status:\n",
    "Current: 001/012\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 0\n",
);

const PRACTICE_BLANK_LINE_THEN_STATUS_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\nExpected:\nfn main() {}\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>Status:\n",
    "Current: 001/012\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n",
);

const PRACTICE_TRAILING_SPACE_THEN_STATUS_THEN_QUIT_OUTPUT: &str =
    PRACTICE_BLANK_LINE_THEN_STATUS_THEN_QUIT_OUTPUT;

const PRACTICE_CRLF_CORRECT_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n",
);

const PRACTICE_UNKNOWN_THEN_CORRECT_THEN_QUIT_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>UNKNOWN COMMAND\nType :help for commands.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>QUIT\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Skipped: 0\n",
    "Attempts: 1\n",
);

const PRACTICE_SKIP_FINAL_INPUT: &str = concat!(
    "fn main() {}\n",
    "println!(\"Rust Reps\");\n",
    "let answer = 42;\n",
    "let name = \"Rust Reps\";\n",
    "let mut score = 0;\n",
    "score += 1;\n",
    "let passed = true;\n",
    "let score: i32 = 0;\n",
    "let scores = [1, 2, 3];\n",
    "fn add_one(x: i32) -> i32 { x + 1 }\n",
    "if score > 0 {}\n",
    ":skip\n",
);
const PRACTICE_SKIP_FINAL_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>PASS\n\n",
    "Drill 003/012\nPrompt:\n\nlet answer = 42;\n\n>PASS\n\n",
    "Drill 004/012\nPrompt:\n\nlet name = \"Rust Reps\";\n\n>PASS\n\n",
    "Drill 005/012\nPrompt:\n\nlet mut score = 0;\n\n>PASS\n\n",
    "Drill 006/012\nPrompt:\n\nscore += 1;\n\n>PASS\n\n",
    "Drill 007/012\nPrompt:\n\nlet passed = true;\n\n>PASS\n\n",
    "Drill 008/012\nPrompt:\n\nlet score: i32 = 0;\n\n>PASS\n\n",
    "Drill 009/012\nPrompt:\n\nlet scores = [1, 2, 3];\n\n>PASS\n\n",
    "Drill 010/012\nPrompt:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>PASS\n\n",
    "Drill 011/012\nPrompt:\n\nif score > 0 {}\n\n>PASS\n\n",
    "Drill 012/012\nPrompt:\n\nwhile score < 10 {}\n\n>SKIP\n",
    "COMPLETE\n",
    "Summary:\n",
    "Passed: 11/12\n",
    "Skipped: 1\n",
    "Attempts: 11\n",
);

const PRACTICE_ALL_SKIP_INPUT: &str = concat!(
    ":skip\n", ":skip\n", ":skip\n", ":skip\n", ":skip\n", ":skip\n", ":skip\n", ":skip\n",
    ":skip\n", ":skip\n", ":skip\n", ":skip\n",
);

const PRACTICE_ALL_SKIP_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>SKIP\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>SKIP\n\n",
    "Drill 003/012\nPrompt:\n\nlet answer = 42;\n\n>SKIP\n\n",
    "Drill 004/012\nPrompt:\n\nlet name = \"Rust Reps\";\n\n>SKIP\n\n",
    "Drill 005/012\nPrompt:\n\nlet mut score = 0;\n\n>SKIP\n\n",
    "Drill 006/012\nPrompt:\n\nscore += 1;\n\n>SKIP\n\n",
    "Drill 007/012\nPrompt:\n\nlet passed = true;\n\n>SKIP\n\n",
    "Drill 008/012\nPrompt:\n\nlet score: i32 = 0;\n\n>SKIP\n\n",
    "Drill 009/012\nPrompt:\n\nlet scores = [1, 2, 3];\n\n>SKIP\n\n",
    "Drill 010/012\nPrompt:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>SKIP\n\n",
    "Drill 011/012\nPrompt:\n\nif score > 0 {}\n\n>SKIP\n\n",
    "Drill 012/012\nPrompt:\n\nwhile score < 10 {}\n\n>SKIP\n",
    "COMPLETE\n",
    "Summary:\n",
    "Passed: 0/12\n",
    "Skipped: 12\n",
    "Attempts: 0\n",
);

const PRACTICE_MIXED_COMPLETE_INPUT: &str = concat!(
    "wrong\n",
    "fn main() {}\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
    ":skip\n",
);

const PRACTICE_MIXED_COMPLETE_OUTPUT: &str = concat!(
    "Rust Reps\n----------------\n\n",
    "Practice\n",
    "Exact typing practice.\n",
    "Answers must match exactly.\n",
    "Type :help for commands and :q to quit.\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>TRY AGAIN\n",
    "Expected:\n",
    "fn main() {}\n\n",
    "Drill 001/012\nPrompt:\n\nfn main() {}\n\n>PASS\n\n",
    "Drill 002/012\nPrompt:\n\nprintln!(\"Rust Reps\");\n\n>SKIP\n\n",
    "Drill 003/012\nPrompt:\n\nlet answer = 42;\n\n>SKIP\n\n",
    "Drill 004/012\nPrompt:\n\nlet name = \"Rust Reps\";\n\n>SKIP\n\n",
    "Drill 005/012\nPrompt:\n\nlet mut score = 0;\n\n>SKIP\n\n",
    "Drill 006/012\nPrompt:\n\nscore += 1;\n\n>SKIP\n\n",
    "Drill 007/012\nPrompt:\n\nlet passed = true;\n\n>SKIP\n\n",
    "Drill 008/012\nPrompt:\n\nlet score: i32 = 0;\n\n>SKIP\n\n",
    "Drill 009/012\nPrompt:\n\nlet scores = [1, 2, 3];\n\n>SKIP\n\n",
    "Drill 010/012\nPrompt:\n\nfn add_one(x: i32) -> i32 { x + 1 }\n\n>SKIP\n\n",
    "Drill 011/012\nPrompt:\n\nif score > 0 {}\n\n>SKIP\n\n",
    "Drill 012/012\nPrompt:\n\nwhile score < 10 {}\n\n>SKIP\n",
    "COMPLETE\n",
    "Summary:\n",
    "Passed: 1/12\n",
    "Skipped: 11\n",
    "Attempts: 2\n",
);

fn run_rust_reps(args: &[&str], input: &str) -> Output {
    let binary = rust_reps_binary();
    let mut child = Command::new(&binary)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("failed to start rust-reps at {}: {err}", binary.display()));

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    stdin
        .write_all(input.as_bytes())
        .expect("failed to write stdin");
    drop(stdin);

    child.wait_with_output().expect("failed to wait for output")
}

fn rust_reps_binary() -> PathBuf {
    let mut path = std::env::current_exe().expect("failed to locate current test executable");
    path.pop();

    if path.file_name().and_then(|name| name.to_str()) == Some("deps") {
        path.pop();
    }

    path.push(format!("rust-reps{}", std::env::consts::EXE_SUFFIX));
    path
}

#[test]
fn default_practice_all_correct_exits_successfully_and_prints_complete() {
    let output = run_rust_reps(&[], PRACTICE_ALL_CORRECT_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_ALL_CORRECT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn play_practice_all_correct_exits_successfully_and_prints_complete() {
    let output = run_rust_reps(&["play"], PRACTICE_ALL_CORRECT_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_ALL_CORRECT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_wrong_answer_repeats_then_accepts_correct_answer() {
    let output = run_rust_reps(&[], "fn main(){}\nfn main() {}\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_RETRY_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_wrong_answer_then_quit() {
    let output = run_rust_reps(&[], "wrong\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_WRONG_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_help_repeats_same_drill_then_quits() {
    let output = run_rust_reps(&[], ":help\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_HELP_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_help_then_status_then_quits() {
    let output = run_rust_reps(&[], ":help\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_HELP_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_unknown_then_status_then_quits() {
    let output = run_rust_reps(&[], ":unknown\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_UNKNOWN_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_unknown_then_correct_answer_then_quits() {
    let output = run_rust_reps(&[], ":unknown\nfn main() {}\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_UNKNOWN_THEN_CORRECT_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_blank_line_then_status_then_quits() {
    let output = run_rust_reps(&[], "\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_BLANK_LINE_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_trailing_space_then_status_then_quits() {
    let output = run_rust_reps(&[], "fn main() {} \n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_TRAILING_SPACE_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_crlf_correct_then_quit() {
    let output = run_rust_reps(&[], "fn main() {}\r\n:q\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_CRLF_CORRECT_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_stats_typo_treated_as_unknown_command() {
    let output = run_rust_reps(&[], ":stats\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_UNKNOWN_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_status_at_start_repeats_same_drill_then_quits() {
    let output = run_rust_reps(&[], ":status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_wrong_then_status_then_quits() {
    let output = run_rust_reps(&[], "wrong\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_WRONG_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_correct_then_status_then_quits() {
    let output = run_rust_reps(&[], "fn main() {}\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_CORRECT_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_skip_then_status_then_quits() {
    let output = run_rust_reps(&[], ":skip\n:status\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_SKIP_THEN_STATUS_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_list_repeats_same_drill_then_quits() {
    let output = run_rust_reps(&[], ":list\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_LIST_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_skip_advances_then_quits_with_skipped_summary() {
    let output = run_rust_reps(&[], ":skip\n:q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_SKIP_THEN_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_skip_final_drill_completes_with_skipped_summary() {
    let output = run_rust_reps(&[], PRACTICE_SKIP_FINAL_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_SKIP_FINAL_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_all_skips_complete_session() {
    let output = run_rust_reps(&[], PRACTICE_ALL_SKIP_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_ALL_SKIP_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_mixed_completion_includes_wrong_correct_and_skip() {
    let output = run_rust_reps(&[], PRACTICE_MIXED_COMPLETE_INPUT);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_MIXED_COMPLETE_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_q_exits_successfully_and_prints_quit() {
    let output = run_rust_reps(&[], ":q\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_quit_exits_successfully_and_prints_quit() {
    let output = run_rust_reps(&[], ":quit\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn practice_eof_before_completion_exits_successfully_and_prints_quit() {
    let output = run_rust_reps(&[], "");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        PRACTICE_QUIT_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn explicit_drill_001_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["001"], "fn main() {}\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn missing_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["001"], "fn main(){}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn double_internal_space_input_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["001"], "fn  main() {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn explicit_drill_001_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["001"], "fn main() {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_PASS_OUTPUT
    );
}

#[test]
fn one_shot_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["001"], "wrong\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn one_shot_blank_line_is_wrong_answer() {
    let output = run_rust_reps(&["001"], "\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_BLANK_LINE_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn one_shot_trailing_space_is_wrong_answer() {
    let output = run_rust_reps(&["001"], "fn main() {} \n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRAILING_SPACE_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn one_shot_crlf_answer_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["001"], "fn main() {}\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_CRLF_PASS_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn one_shot_help_is_wrong_answer() {
    let output = run_rust_reps(&["001"], ":help\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_001_TRY_AGAIN_OUTPUT
    );
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn one_shot_selected_drill_outputs_do_not_include_summary() {
    let drill_001_output = run_rust_reps(&["001"], "fn main() {}\n");
    let drill_012_output = run_rust_reps(&["012"], "while score < 10 {}\n");
    let command_like_output = run_rust_reps(&["001"], ":help\n");

    assert!(drill_001_output.status.success());
    assert!(drill_012_output.status.success());
    assert!(command_like_output.status.success());
    assert!(
        !String::from_utf8(drill_001_output.stdout)
            .unwrap()
            .contains("Summary:")
    );
    assert!(
        !String::from_utf8(drill_012_output.stdout)
            .unwrap()
            .contains("Summary:")
    );
    let command_like_stdout = String::from_utf8(command_like_output.stdout).unwrap();
    assert_eq!(command_like_stdout, DRILL_001_TRY_AGAIN_OUTPUT);
    assert!(!command_like_stdout.contains("Commands:"));
}

#[test]
fn drill_002_lf_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["002"], "println!(\"Rust Reps\");\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_PASS_OUTPUT
    );
}

#[test]
fn drill_002_crlf_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["002"], "println!(\"Rust Reps\");\r\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_PASS_OUTPUT
    );
}

#[test]
fn drill_002_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["002"], "println!(\"Rust Reps\")\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_002_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn list_command_exits_successfully_and_prints_catalog() {
    let output = run_rust_reps(&["--list"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), LIST_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn help_command_exits_successfully_and_prints_help() {
    let output = run_rust_reps(&["--help"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), HELP_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn short_help_command_exits_successfully_and_prints_help() {
    let output = run_rust_reps(&["-h"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), HELP_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn version_command_exits_successfully_and_prints_version() {
    let output = run_rust_reps(&["--version"], "");

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), VERSION_OUTPUT);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

#[test]
fn drill_003_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["003"], "let answer = 42;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_003_PASS_OUTPUT
    );
}

#[test]
fn drill_004_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["004"], "let name = \"Rust Reps\";\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_004_PASS_OUTPUT
    );
}

#[test]
fn drill_005_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["005"], "let mut score = 0;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_005_PASS_OUTPUT
    );
}

#[test]
fn drill_006_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["006"], "score += 1;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_006_PASS_OUTPUT
    );
}

#[test]
fn drill_006_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["006"], "score += 1\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_006_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn drill_007_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["007"], "let passed = true;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_007_PASS_OUTPUT
    );
}

#[test]
fn drill_008_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["008"], "let score: i32 = 0;\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_008_PASS_OUTPUT
    );
}

#[test]
fn drill_009_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["009"], "let scores = [1, 2, 3];\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_009_PASS_OUTPUT
    );
}

#[test]
fn drill_010_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["010"], "fn add_one(x: i32) -> i32 { x + 1 }\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_010_PASS_OUTPUT
    );
}

#[test]
fn drill_011_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["011"], "if score > 0 {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_011_PASS_OUTPUT
    );
}

#[test]
fn drill_012_input_exits_successfully_and_prints_pass() {
    let output = run_rust_reps(&["012"], "while score < 10 {}\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_012_PASS_OUTPUT
    );
}

#[test]
fn drill_012_wrong_answer_exits_successfully_and_prints_try_again() {
    let output = run_rust_reps(&["012"], "while score < 10\n");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        DRILL_012_TRY_AGAIN_OUTPUT
    );
}

#[test]
fn unknown_drill_id_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["999"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown drill id: 999\n"
    );
}

#[test]
fn too_many_args_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["001", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn list_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["--list", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn unknown_long_option_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["--wat"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown argument: --wat\n"
    );
}

#[test]
fn unknown_short_option_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["-x"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown argument: -x\n"
    );
}

#[test]
fn help_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["--help", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn version_extra_arg_exits_nonzero_and_prints_error() {
    let output = run_rust_reps(&["--version", "extra"], "");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: expected at most one argument\n"
    );
}

#[test]
fn stderr_is_empty_for_normal_product_results() {
    let practice_output = run_rust_reps(&[], PRACTICE_ALL_CORRECT_INPUT);
    let pass_output = run_rust_reps(&["001"], "fn main() {}\n");
    let try_again_output = run_rust_reps(&["012"], "while score < 10\n");
    let quit_output = run_rust_reps(&[], ":q\n");
    let practice_help_output = run_rust_reps(&[], ":help\n:q\n");
    let practice_status_output = run_rust_reps(&[], ":status\n:q\n");
    let practice_list_output = run_rust_reps(&[], ":list\n:q\n");
    let practice_skip_output = run_rust_reps(&[], ":skip\n:q\n");
    let list_output = run_rust_reps(&["--list"], "");
    let help_output = run_rust_reps(&["--help"], "");
    let version_output = run_rust_reps(&["--version"], "");

    assert!(practice_output.status.success());
    assert!(pass_output.status.success());
    assert!(try_again_output.status.success());
    assert!(quit_output.status.success());
    assert!(practice_help_output.status.success());
    assert!(practice_status_output.status.success());
    assert!(practice_list_output.status.success());
    assert!(practice_skip_output.status.success());
    assert!(list_output.status.success());
    assert!(help_output.status.success());
    assert!(version_output.status.success());
    assert_eq!(String::from_utf8(practice_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(pass_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(try_again_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(quit_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(practice_help_output.stderr).unwrap(), "");
    assert_eq!(
        String::from_utf8(practice_status_output.stderr).unwrap(),
        ""
    );
    assert_eq!(String::from_utf8(practice_list_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(practice_skip_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(list_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(help_output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(version_output.stderr).unwrap(), "");
}
