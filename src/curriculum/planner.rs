pub(crate) mod planning;

use super::Operation;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum RepresentationKind {
    SemanticIntent,
    CompleteSurface,
    PartialSurface,
    BrokenSurface,
    MeaningLabel,
    RoleLabel,
    SoundLabel,
    Components,
    ArtifactFragment,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum SupportLevel {
    FullModel,
    PartialModel,
    ConstrainedCue,
    MinimalCue,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum SupportStage {
    Introduce,
    Guided,
    Practice,
}

impl SupportLevel {
    pub(crate) const fn stage(self) -> SupportStage {
        match self {
            Self::FullModel => SupportStage::Introduce,
            Self::PartialModel | Self::ConstrainedCue => SupportStage::Guided,
            Self::MinimalCue => SupportStage::Practice,
        }
    }
}

pub(crate) struct LearningTargetSpec {
    pub(crate) id: &'static str,
    pub(crate) focus_concepts: &'static [&'static str],
    pub(crate) focus_constraints: &'static [&'static str],
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CapabilitySpec {
    pub(crate) id: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) response_system: &'static str,
    pub(crate) response_surface_frame: &'static str,
    pub(crate) cue_system: Option<&'static str>,
    pub(crate) cue_surface_frame: Option<&'static str>,
    pub(crate) operation: Operation,
    pub(crate) cue_representation: RepresentationKind,
    pub(crate) response_representation: RepresentationKind,
    pub(crate) support: SupportLevel,
    pub(crate) learning_target_id: &'static str,
}

pub(crate) struct SetStep {
    pub(crate) id: &'static str,
    pub(crate) capability_id: &'static str,
}

pub(crate) struct SetPlan {
    pub(crate) id: &'static str,
    pub(crate) version: u16,
    pub(crate) system: &'static str,
    pub(crate) track: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) response_surface_frame: &'static str,
    pub(crate) steps: &'static [SetStep],
}

pub(crate) const LEARNING_TARGETS: &[LearningTargetSpec] = &[
    LearningTargetSpec {
        id: "rust.function.return_literal.whole_form",
        focus_concepts: &["rust.functions", "rust.return_values"],
        focus_constraints: &[
            "rust.uses_fn_keyword",
            "rust.no_params_empty_parens",
            "rust.return_type_uses_arrow",
            "rust.tail_expression_returns_value",
        ],
    },
    LearningTargetSpec {
        id: "rust.function.return_literal.return_value",
        focus_concepts: &["rust.return_values"],
        focus_constraints: &["rust.tail_expression_returns_value"],
    },
    LearningTargetSpec {
        id: "rust.function.return_literal.return_type_arrow",
        focus_concepts: &["rust.functions"],
        focus_constraints: &["rust.return_type_uses_arrow"],
    },
];

pub(crate) const CAPABILITIES: &[CapabilitySpec] = &[
    CapabilitySpec {
        id: "rust.function.return_literal.copy.return_type.introduce",
        semantic_frame: "function.no_params.return_literal",
        response_system: "rust",
        response_surface_frame: "function.return_literal",
        cue_system: None,
        cue_surface_frame: None,
        operation: Operation::Copy,
        cue_representation: RepresentationKind::CompleteSurface,
        response_representation: RepresentationKind::CompleteSurface,
        support: SupportLevel::FullModel,
        learning_target_id: "rust.function.return_literal.return_type_arrow",
    },
    CapabilitySpec {
        id: "rust.function.return_literal.fill.return_type_arrow",
        semantic_frame: "function.no_params.return_literal",
        response_system: "rust",
        response_surface_frame: "function.return_literal",
        cue_system: None,
        cue_surface_frame: None,
        operation: Operation::Fill,
        cue_representation: RepresentationKind::PartialSurface,
        response_representation: RepresentationKind::ArtifactFragment,
        support: SupportLevel::PartialModel,
        learning_target_id: "rust.function.return_literal.return_type_arrow",
    },
    CapabilitySpec {
        id: "rust.function.return_literal.produce.return_type.guided",
        semantic_frame: "function.no_params.return_literal",
        response_system: "rust",
        response_surface_frame: "function.return_literal",
        cue_system: None,
        cue_surface_frame: None,
        operation: Operation::Produce,
        cue_representation: RepresentationKind::SemanticIntent,
        response_representation: RepresentationKind::CompleteSurface,
        support: SupportLevel::ConstrainedCue,
        learning_target_id: "rust.function.return_literal.return_type_arrow",
    },
    CapabilitySpec {
        id: "rust.function.return_literal.repair.return_type_arrow",
        semantic_frame: "function.no_params.return_literal",
        response_system: "rust",
        response_surface_frame: "function.return_literal",
        cue_system: None,
        cue_surface_frame: None,
        operation: Operation::Repair,
        cue_representation: RepresentationKind::BrokenSurface,
        response_representation: RepresentationKind::CompleteSurface,
        support: SupportLevel::ConstrainedCue,
        learning_target_id: "rust.function.return_literal.return_type_arrow",
    },
    CapabilitySpec {
        id: "rust.function.return_literal.transform.return_type.practice",
        semantic_frame: "function.no_params.return_literal",
        response_system: "rust",
        response_surface_frame: "function.return_literal",
        cue_system: None,
        cue_surface_frame: None,
        operation: Operation::Transform,
        cue_representation: RepresentationKind::CompleteSurface,
        response_representation: RepresentationKind::CompleteSurface,
        support: SupportLevel::MinimalCue,
        learning_target_id: "rust.function.return_literal.return_type_arrow",
    },
];

pub(crate) const SET_PLANS: &[SetPlan] = &[SetPlan {
    id: "rust.function.return_literal.five_step",
    version: 2,
    system: "rust",
    track: "syntax_basics",
    semantic_frame: "function.no_params.return_literal",
    response_surface_frame: "function.return_literal",
    steps: &[
        SetStep {
            id: "copy-introduce-return-type",
            capability_id: "rust.function.return_literal.copy.return_type.introduce",
        },
        SetStep {
            id: "fill-return-type-arrow",
            capability_id: "rust.function.return_literal.fill.return_type_arrow",
        },
        SetStep {
            id: "produce-return-type",
            capability_id: "rust.function.return_literal.produce.return_type.guided",
        },
        SetStep {
            id: "repair-return-type-arrow",
            capability_id: "rust.function.return_literal.repair.return_type_arrow",
        },
        SetStep {
            id: "transform-add-return-type",
            capability_id: "rust.function.return_literal.transform.return_type.practice",
        },
    ],
}];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::procgen::{SEMANTIC_FRAMES, SURFACE_FRAMES};

    fn assert_nonempty_unique(values: &[&str], label: &str) {
        for (index, value) in values.iter().enumerate() {
            assert!(!value.is_empty(), "{label} value {index} must not be empty");
            for earlier in &values[..index] {
                assert_ne!(earlier, value, "{label} values must be unique");
            }
        }
    }

    #[test]
    fn representation_and_support_kinds_are_exact() {
        let representation_rank = |kind| match kind {
            RepresentationKind::SemanticIntent => 0,
            RepresentationKind::CompleteSurface => 1,
            RepresentationKind::PartialSurface => 2,
            RepresentationKind::BrokenSurface => 3,
            RepresentationKind::MeaningLabel => 4,
            RepresentationKind::RoleLabel => 5,
            RepresentationKind::SoundLabel => 6,
            RepresentationKind::Components => 7,
            RepresentationKind::ArtifactFragment => 8,
        };
        assert_eq!(
            [
                RepresentationKind::SemanticIntent,
                RepresentationKind::CompleteSurface,
                RepresentationKind::PartialSurface,
                RepresentationKind::BrokenSurface,
                RepresentationKind::MeaningLabel,
                RepresentationKind::RoleLabel,
                RepresentationKind::SoundLabel,
                RepresentationKind::Components,
                RepresentationKind::ArtifactFragment,
            ]
            .map(representation_rank),
            [0, 1, 2, 3, 4, 5, 6, 7, 8]
        );

        let support_rank = |level| match level {
            SupportLevel::FullModel => 0,
            SupportLevel::PartialModel => 1,
            SupportLevel::ConstrainedCue => 2,
            SupportLevel::MinimalCue => 3,
        };
        assert_eq!(
            [
                SupportLevel::FullModel,
                SupportLevel::PartialModel,
                SupportLevel::ConstrainedCue,
                SupportLevel::MinimalCue,
            ]
            .map(support_rank),
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn learning_target_source_invariants() {
        let rust_surface = SURFACE_FRAMES
            .iter()
            .find(|frame| frame.system == "rust" && frame.local_id == "function.return_literal")
            .expect("Rust function-return-literal surface must exist");

        for (index, target) in LEARNING_TARGETS.iter().enumerate() {
            assert!(
                !target.id.is_empty(),
                "learning target ID must not be empty"
            );
            for earlier in &LEARNING_TARGETS[..index] {
                assert_ne!(earlier.id, target.id, "learning target IDs must be unique");
            }
            assert!(
                !target.focus_concepts.is_empty() || !target.focus_constraints.is_empty(),
                "learning target must declare focus"
            );
            assert_nonempty_unique(target.focus_concepts, "focus concept");
            assert_nonempty_unique(target.focus_constraints, "focus constraint");
            for concept in target.focus_concepts {
                assert!(
                    rust_surface.concepts.contains(concept),
                    "focus concept {concept} must exist on the Rust surface"
                );
            }
            for constraint in target.focus_constraints {
                assert!(
                    rust_surface.constraints.contains(constraint),
                    "focus constraint {constraint} must exist on the Rust surface"
                );
            }
        }

        assert_eq!(
            LEARNING_TARGETS
                .iter()
                .map(|target| target.id)
                .collect::<Vec<_>>(),
            vec![
                "rust.function.return_literal.whole_form",
                "rust.function.return_literal.return_value",
                "rust.function.return_literal.return_type_arrow",
            ]
        );
        assert_eq!(
            LEARNING_TARGETS[0].focus_concepts,
            &["rust.functions", "rust.return_values"]
        );
        assert_eq!(
            LEARNING_TARGETS[0].focus_constraints,
            &[
                "rust.uses_fn_keyword",
                "rust.no_params_empty_parens",
                "rust.return_type_uses_arrow",
                "rust.tail_expression_returns_value",
            ]
        );
        assert_eq!(LEARNING_TARGETS[1].focus_concepts, &["rust.return_values"]);
        assert_eq!(
            LEARNING_TARGETS[1].focus_constraints,
            &["rust.tail_expression_returns_value"]
        );
        assert_eq!(LEARNING_TARGETS[2].focus_concepts, &["rust.functions"]);
        assert_eq!(
            LEARNING_TARGETS[2].focus_constraints,
            &["rust.return_type_uses_arrow"]
        );
    }

    #[test]
    fn capability_source_invariants() {
        for (index, capability) in CAPABILITIES.iter().enumerate() {
            assert!(!capability.id.is_empty(), "capability ID must not be empty");
            for earlier in &CAPABILITIES[..index] {
                assert_ne!(earlier.id, capability.id, "capability IDs must be unique");
            }
            assert!(
                SEMANTIC_FRAMES
                    .iter()
                    .any(|frame| frame.id == capability.semantic_frame),
                "capability semantic frame must be known"
            );
            assert!(
                !capability.response_system.is_empty(),
                "response system must not be empty"
            );
            assert!(
                !capability.response_surface_frame.is_empty(),
                "response surface frame must not be empty"
            );
            let surface = SURFACE_FRAMES
                .iter()
                .find(|frame| {
                    frame.system == capability.response_system
                        && frame.local_id == capability.response_surface_frame
                })
                .expect("capability response surface must be known");
            assert_eq!(surface.semantic_frame, capability.semantic_frame);
            assert_eq!(
                capability.cue_system.is_some(),
                capability.cue_surface_frame.is_some(),
                "cue system and surface must be paired"
            );
            assert!(surface.supported_operations.contains(&capability.operation));
            assert!(
                LEARNING_TARGETS
                    .iter()
                    .any(|target| target.id == capability.learning_target_id),
                "capability learning target must be known"
            );
            if capability.operation == Operation::Fill {
                assert_eq!(
                    capability.response_representation,
                    RepresentationKind::ArtifactFragment
                );
            } else {
                assert_eq!(
                    capability.response_representation,
                    RepresentationKind::CompleteSurface
                );
            }
        }

        let accepted = [
            (
                "rust.function.return_literal.copy.return_type.introduce",
                Operation::Copy,
                RepresentationKind::CompleteSurface,
                SupportLevel::FullModel,
                "rust.function.return_literal.return_type_arrow",
            ),
            (
                "rust.function.return_literal.fill.return_type_arrow",
                Operation::Fill,
                RepresentationKind::PartialSurface,
                SupportLevel::PartialModel,
                "rust.function.return_literal.return_type_arrow",
            ),
            (
                "rust.function.return_literal.produce.return_type.guided",
                Operation::Produce,
                RepresentationKind::SemanticIntent,
                SupportLevel::ConstrainedCue,
                "rust.function.return_literal.return_type_arrow",
            ),
            (
                "rust.function.return_literal.repair.return_type_arrow",
                Operation::Repair,
                RepresentationKind::BrokenSurface,
                SupportLevel::ConstrainedCue,
                "rust.function.return_literal.return_type_arrow",
            ),
            (
                "rust.function.return_literal.transform.return_type.practice",
                Operation::Transform,
                RepresentationKind::CompleteSurface,
                SupportLevel::MinimalCue,
                "rust.function.return_literal.return_type_arrow",
            ),
        ];
        for (capability, (id, operation, cue, support, target)) in CAPABILITIES.iter().zip(accepted)
        {
            assert_eq!(capability.id, id);
            assert_eq!(
                capability.semantic_frame,
                "function.no_params.return_literal"
            );
            assert_eq!(capability.response_system, "rust");
            assert_eq!(capability.response_surface_frame, "function.return_literal");
            assert_eq!(capability.cue_system, None);
            assert_eq!(capability.cue_surface_frame, None);
            assert_eq!(capability.operation, operation);
            assert_eq!(capability.cue_representation, cue);
            assert_eq!(
                capability.response_representation,
                if operation == Operation::Fill {
                    RepresentationKind::ArtifactFragment
                } else {
                    RepresentationKind::CompleteSurface
                }
            );
            assert_eq!(capability.support, support);
            assert_eq!(capability.learning_target_id, target);
        }

        assert_ne!(CAPABILITIES[0], CAPABILITIES[2]);
        assert_ne!(CAPABILITIES[1], CAPABILITIES[3]);

        let production = include_str!("planner.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("production planner source must exist");
        assert!(!production.contains("locale"));
    }

    #[test]
    fn set_plan_source_invariants() {
        for (set_index, set) in SET_PLANS.iter().enumerate() {
            assert!(!set.id.is_empty(), "set ID must not be empty");
            for earlier in &SET_PLANS[..set_index] {
                assert_ne!(earlier.id, set.id, "set IDs must be unique");
            }
            assert!(set.version > 0, "set version must be positive");
            for value in [
                set.system,
                set.track,
                set.semantic_frame,
                set.response_surface_frame,
            ] {
                assert!(!value.is_empty(), "set metadata must not be empty");
            }
            assert!(
                SEMANTIC_FRAMES
                    .iter()
                    .any(|frame| frame.id == set.semantic_frame),
                "set semantic frame must be known"
            );
            assert!(
                SURFACE_FRAMES.iter().any(|surface| {
                    surface.system == set.system
                        && surface.local_id == set.response_surface_frame
                        && surface.semantic_frame == set.semantic_frame
                }),
                "set response surface must be known"
            );
            assert!(!set.steps.is_empty(), "set must contain steps");
            for (step_index, step) in set.steps.iter().enumerate() {
                assert!(!step.id.is_empty(), "set step ID must not be empty");
                for earlier in &set.steps[..step_index] {
                    assert_ne!(earlier.id, step.id, "step IDs must be unique within a set");
                }
                let capability = CAPABILITIES
                    .iter()
                    .find(|capability| capability.id == step.capability_id)
                    .expect("set step capability must exist");
                assert_eq!(capability.response_system, set.system);
                assert_eq!(capability.semantic_frame, set.semantic_frame);
                assert_eq!(
                    capability.response_surface_frame,
                    set.response_surface_frame
                );
            }
        }

        let set = &SET_PLANS[0];
        assert_eq!(set.id, "rust.function.return_literal.five_step");
        assert_eq!(set.version, 2);
        assert_eq!(set.system, "rust");
        assert_eq!(set.track, "syntax_basics");
        assert_eq!(set.semantic_frame, "function.no_params.return_literal");
        assert_eq!(set.response_surface_frame, "function.return_literal");
        assert_eq!(
            set.steps.iter().map(|step| step.id).collect::<Vec<_>>(),
            vec![
                "copy-introduce-return-type",
                "fill-return-type-arrow",
                "produce-return-type",
                "repair-return-type-arrow",
                "transform-add-return-type",
            ]
        );
        assert_eq!(
            set.steps
                .iter()
                .map(|step| step.capability_id)
                .collect::<Vec<_>>(),
            CAPABILITIES
                .iter()
                .map(|capability| capability.id)
                .collect::<Vec<_>>()
        );

        let operations = set
            .steps
            .iter()
            .map(|step| {
                CAPABILITIES
                    .iter()
                    .find(|capability| capability.id == step.capability_id)
                    .expect("set step capability must exist")
                    .operation
            })
            .collect::<Vec<_>>();
        assert_eq!(
            operations,
            vec![
                Operation::Copy,
                Operation::Fill,
                Operation::Produce,
                Operation::Repair,
                Operation::Transform,
            ]
        );
        assert_ne!(operations.as_slice(), &Operation::ALL[..5]);
    }

    #[test]
    fn accepted_static_source_is_exact() {
        assert_eq!(LEARNING_TARGETS.len(), 3);
        assert_eq!(CAPABILITIES.len(), 5);
        assert_eq!(SET_PLANS.len(), 1);
        assert_eq!(SET_PLANS[0].steps.len(), 5);
        assert_eq!(
            CAPABILITIES
                .iter()
                .map(|capability| capability.id)
                .collect::<Vec<_>>(),
            vec![
                "rust.function.return_literal.copy.return_type.introduce",
                "rust.function.return_literal.fill.return_type_arrow",
                "rust.function.return_literal.produce.return_type.guided",
                "rust.function.return_literal.repair.return_type_arrow",
                "rust.function.return_literal.transform.return_type.practice",
            ]
        );
        assert_eq!(
            SET_PLANS[0]
                .steps
                .iter()
                .map(|step| step.id)
                .collect::<Vec<_>>(),
            vec![
                "copy-introduce-return-type",
                "fill-return-type-arrow",
                "produce-return-type",
                "repair-return-type-arrow",
                "transform-add-return-type",
            ]
        );
        assert_eq!(
            CAPABILITIES
                .iter()
                .map(|capability| capability.support)
                .collect::<Vec<_>>(),
            vec![
                SupportLevel::FullModel,
                SupportLevel::PartialModel,
                SupportLevel::ConstrainedCue,
                SupportLevel::ConstrainedCue,
                SupportLevel::MinimalCue,
            ]
        );
        assert_eq!(
            CAPABILITIES
                .iter()
                .map(|capability| capability.cue_representation)
                .collect::<Vec<_>>(),
            vec![
                RepresentationKind::CompleteSurface,
                RepresentationKind::PartialSurface,
                RepresentationKind::SemanticIntent,
                RepresentationKind::BrokenSurface,
                RepresentationKind::CompleteSurface,
            ]
        );
        assert_eq!(
            CAPABILITIES
                .iter()
                .map(|capability| capability.response_representation)
                .collect::<Vec<_>>(),
            vec![
                RepresentationKind::CompleteSurface,
                RepresentationKind::ArtifactFragment,
                RepresentationKind::CompleteSurface,
                RepresentationKind::CompleteSurface,
                RepresentationKind::CompleteSurface,
            ]
        );
    }
}
