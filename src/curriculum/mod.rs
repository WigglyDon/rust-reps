#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "guidance source and pure resolution are intentionally inert until a later presentation-integration boundary"
    )
)]
pub(crate) mod guidance;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "capability and set-plan source are intentionally inert until a later pure planning boundary"
    )
)]
pub(crate) mod planner;
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "procgen source and expansion are intentionally inert until a later catalog-admission boundary"
    )
)]
pub(crate) mod procgen;
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
    #[allow(dead_code)]
    RustfmtExact,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Copy,
    Recognize,
    Fill,
    Read,
    Produce,
    Repair,
    Transform,
    Compose,
    Explain,
}

impl Operation {
    pub const ALL: [Self; 9] = [
        Self::Copy,
        Self::Recognize,
        Self::Fill,
        Self::Read,
        Self::Produce,
        Self::Repair,
        Self::Transform,
        Self::Compose,
        Self::Explain,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Recognize => "recognize",
            Self::Fill => "fill",
            Self::Read => "read",
            Self::Produce => "produce",
            Self::Repair => "repair",
            Self::Transform => "transform",
            Self::Compose => "compose",
            Self::Explain => "explain",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnswerRule {
    ExactTrimFinalNewline,
    RustfmtExact,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StateTransition {
    None,
}

#[derive(Copy, Clone)]
pub struct RepMetadata {
    pub canonical_id: &'static str,
    pub system: &'static str,
    pub track: &'static str,
    pub semantic_frame: &'static str,
    pub surface_frame: &'static str,
    pub operation: Operation,
    pub concepts: &'static [&'static str],
    pub constraints: &'static [&'static str],
    pub xp: u32,
    pub answer_rule: AnswerRule,
    pub transition: StateTransition,
}

#[derive(Copy, Clone)]
pub struct DrillSpec {
    pub id: &'static str,
    pub metadata: RepMetadata,
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
