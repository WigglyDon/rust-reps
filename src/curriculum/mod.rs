mod syntax_basics;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Track {
    RustSyntax,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Stage {
    Copy,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaskType {
    TypeExact,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Validator {
    Exact,
}

#[derive(Copy, Clone)]
pub struct DrillSpec {
    pub id: &'static str,
    pub track: Track,
    pub skill: &'static str,
    pub stage: Stage,
    pub task_type: TaskType,
    pub difficulty: u8,
    pub prompt: &'static str,
    pub expected: &'static str,
    pub validator: Validator,
    pub prerequisites: &'static [&'static str],
    pub tags: &'static [&'static str],
}

/// Backward-compatible alias for shard module naming in future packets.
pub type RepSpec = DrillSpec;

/// Ordered catalog shards.
pub const DRILL_SHARDS: &[&[RepSpec]] = &[syntax_basics::REPS];

/// Backward-compatible flat view into the current curriculum.
pub const DRILLS: &[RepSpec] = DRILL_SHARDS[0];
