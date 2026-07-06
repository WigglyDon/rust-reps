use std::fmt;

#[derive(Copy, Clone)]
pub struct Drill {
    pub id: &'static str,
    pub expected: &'static str,
}

impl fmt::Display for Drill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}", self.id, self.expected)
    }
}

const DRILLS: &[Drill] = &[
    Drill {
        id: "001",
        expected: "fn main() {}",
    },
    Drill {
        id: "002",
        expected: "println!(\"Rust Reps\");",
    },
    Drill {
        id: "003",
        expected: "let answer = 42;",
    },
    Drill {
        id: "004",
        expected: "let name = \"Rust Reps\";",
    },
    Drill {
        id: "005",
        expected: "let mut score = 0;",
    },
    Drill {
        id: "006",
        expected: "score += 1;",
    },
    Drill {
        id: "007",
        expected: "let passed = true;",
    },
    Drill {
        id: "008",
        expected: "let score: i32 = 0;",
    },
    Drill {
        id: "009",
        expected: "let scores = [1, 2, 3];",
    },
    Drill {
        id: "010",
        expected: "fn add_one(x: i32) -> i32 { x + 1 }",
    },
    Drill {
        id: "011",
        expected: "if score > 0 {}",
    },
    Drill {
        id: "012",
        expected: "while score < 10 {}",
    },
];

pub fn drills() -> &'static [Drill] {
    DRILLS
}

pub fn find_drill(id: &str) -> Option<&'static Drill> {
    DRILLS.iter().find(|drill| drill.id == id)
}
