#[derive(Copy, Clone)]
pub struct DrillSpec {
    pub id: &'static str,
    pub expected: &'static str,
}

pub const DRILLS: &[DrillSpec] = &[
    DrillSpec {
        id: "001",
        expected: "fn main() {}",
    },
    DrillSpec {
        id: "002",
        expected: "println!(\"Rust Reps\");",
    },
    DrillSpec {
        id: "003",
        expected: "let answer = 42;",
    },
    DrillSpec {
        id: "004",
        expected: "let name = \"Rust Reps\";",
    },
    DrillSpec {
        id: "005",
        expected: "let mut score = 0;",
    },
    DrillSpec {
        id: "006",
        expected: "score += 1;",
    },
    DrillSpec {
        id: "007",
        expected: "let passed = true;",
    },
    DrillSpec {
        id: "008",
        expected: "let score: i32 = 0;",
    },
    DrillSpec {
        id: "009",
        expected: "let scores = [1, 2, 3];",
    },
    DrillSpec {
        id: "010",
        expected: "fn add_one(x: i32) -> i32 { x + 1 }",
    },
    DrillSpec {
        id: "011",
        expected: "if score > 0 {}",
    },
    DrillSpec {
        id: "012",
        expected: "while score < 10 {}",
    },
];
