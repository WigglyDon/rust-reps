use super::{CapabilitySpec, SetPlan};
use crate::curriculum::{AnswerRule, Operation};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CandidateSummary {
    pub(crate) rep_id: &'static str,
    pub(crate) catalog_order: u32,
    pub(crate) capability_id: &'static str,
    pub(crate) system: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) surface_frame: &'static str,
    pub(crate) operation: Operation,
    pub(crate) instruction_key: &'static str,
    pub(crate) answer_rule: AnswerRule,
    pub(crate) content_seed: Option<u64>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct GuidanceAvailability {
    pub(crate) locale: &'static str,
    pub(crate) pack_version: u16,
    pub(crate) instruction_key: &'static str,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PlannedRep {
    pub(crate) step_id: String,
    pub(crate) capability_id: String,
    pub(crate) rep_id: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PlanProvenance {
    pub(crate) set_id: String,
    pub(crate) set_version: u16,
    pub(crate) plan_seed: u64,
    pub(crate) plan_cardinality: u64,
    pub(crate) guidance_locale: String,
    pub(crate) guidance_pack_version: u16,
    pub(crate) supported_answer_rules: Vec<AnswerRule>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PracticePlan {
    pub(crate) provenance: PlanProvenance,
    pub(crate) reps: Vec<PlannedRep>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum PlanError {
    InvalidGuidanceSelection {
        locale: String,
        version: u16,
    },
    DuplicateCapability(String),
    UnknownCapability {
        step_id: String,
        capability_id: String,
    },
    CapabilitySetMismatch {
        step_id: String,
        capability_id: String,
    },
    DuplicateCandidateId(String),
    CandidateOrderViolation {
        earlier_order: u32,
        current_order: u32,
    },
    UnknownCandidateCapability {
        rep_id: String,
        capability_id: String,
    },
    CandidateCapabilityMismatch {
        rep_id: String,
        capability_id: String,
    },
    DuplicateGuidanceAvailability {
        locale: String,
        version: u16,
        instruction_key: String,
    },
    NoCandidate {
        step_id: String,
        capability_id: String,
    },
    UnsupportedAnswerRule {
        step_id: String,
        capability_id: String,
    },
    GuidanceUnavailable {
        step_id: String,
        locale: String,
        version: u16,
    },
    CardinalityOverflow,
    PlanSeedOutOfRange {
        seed: u64,
        cardinality: u64,
    },
}

pub(crate) fn plan_cardinality(
    set: &SetPlan,
    capabilities: &[CapabilitySpec],
    candidates: &[CandidateSummary],
    guidance: &[GuidanceAvailability],
    guidance_locale: &str,
    guidance_pack_version: u16,
    supported_answer_rules: &[AnswerRule],
) -> Result<u64, PlanError> {
    let eligible = eligible_candidate_indices(
        set,
        capabilities,
        candidates,
        guidance,
        guidance_locale,
        guidance_pack_version,
        supported_answer_rules,
    )?;

    let mut cardinality = 1_u64;
    for indices in &eligible {
        let radix = u64::try_from(indices.len()).map_err(|_| PlanError::CardinalityOverflow)?;
        cardinality = checked_cardinality_multiply(cardinality, radix)?;
    }
    Ok(cardinality)
}

pub(crate) fn build_practice_plan(
    set: &SetPlan,
    capabilities: &[CapabilitySpec],
    candidates: &[CandidateSummary],
    guidance: &[GuidanceAvailability],
    guidance_locale: &str,
    guidance_pack_version: u16,
    supported_answer_rules: &[AnswerRule],
    plan_seed: u64,
) -> Result<PracticePlan, PlanError> {
    let eligible = eligible_candidate_indices(
        set,
        capabilities,
        candidates,
        guidance,
        guidance_locale,
        guidance_pack_version,
        supported_answer_rules,
    )?;

    let mut plan_cardinality = 1_u64;
    for indices in &eligible {
        let radix = u64::try_from(indices.len()).map_err(|_| PlanError::CardinalityOverflow)?;
        plan_cardinality = checked_cardinality_multiply(plan_cardinality, radix)?;
    }
    if plan_seed >= plan_cardinality {
        return Err(PlanError::PlanSeedOutOfRange {
            seed: plan_seed,
            cardinality: plan_cardinality,
        });
    }

    let mut remaining = plan_seed;
    let mut reps = Vec::with_capacity(set.steps.len());
    for (step, indices) in set.steps.iter().zip(&eligible) {
        let radix = u64::try_from(indices.len()).map_err(|_| PlanError::CardinalityOverflow)?;
        let selected_index = remaining % radix;
        remaining /= radix;
        let selected_index =
            usize::try_from(selected_index).map_err(|_| PlanError::CardinalityOverflow)?;
        let candidate = &candidates[indices[selected_index]];
        reps.push(PlannedRep {
            step_id: step.id.to_owned(),
            capability_id: step.capability_id.to_owned(),
            rep_id: candidate.rep_id.to_owned(),
        });
    }

    Ok(PracticePlan {
        provenance: PlanProvenance {
            set_id: set.id.to_owned(),
            set_version: set.version,
            plan_seed,
            plan_cardinality,
            guidance_locale: guidance_locale.to_owned(),
            guidance_pack_version,
            supported_answer_rules: supported_answer_rules.iter().copied().collect(),
        },
        reps,
    })
}

fn find_capability<'a>(capabilities: &'a [CapabilitySpec], id: &str) -> Option<&'a CapabilitySpec> {
    capabilities.iter().find(|capability| capability.id == id)
}

fn validate_capability_source(
    set: &SetPlan,
    capabilities: &[CapabilitySpec],
) -> Result<(), PlanError> {
    for (index, capability) in capabilities.iter().enumerate() {
        for earlier in &capabilities[..index] {
            if earlier.id == capability.id {
                return Err(PlanError::DuplicateCapability(capability.id.to_owned()));
            }
        }
    }

    for step in set.steps {
        let Some(capability) = find_capability(capabilities, step.capability_id) else {
            return Err(PlanError::UnknownCapability {
                step_id: step.id.to_owned(),
                capability_id: step.capability_id.to_owned(),
            });
        };
        if capability.response_system != set.system
            || capability.semantic_frame != set.semantic_frame
            || capability.response_surface_frame != set.response_surface_frame
        {
            return Err(PlanError::CapabilitySetMismatch {
                step_id: step.id.to_owned(),
                capability_id: step.capability_id.to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_candidate_source(
    capabilities: &[CapabilitySpec],
    candidates: &[CandidateSummary],
) -> Result<(), PlanError> {
    for (index, candidate) in candidates.iter().enumerate() {
        for earlier in &candidates[..index] {
            if earlier.rep_id == candidate.rep_id {
                return Err(PlanError::DuplicateCandidateId(candidate.rep_id.to_owned()));
            }
        }

        if index > 0 {
            let earlier_order = candidates[index - 1].catalog_order;
            if earlier_order >= candidate.catalog_order {
                return Err(PlanError::CandidateOrderViolation {
                    earlier_order,
                    current_order: candidate.catalog_order,
                });
            }
        }

        let Some(capability) = find_capability(capabilities, candidate.capability_id) else {
            return Err(PlanError::UnknownCandidateCapability {
                rep_id: candidate.rep_id.to_owned(),
                capability_id: candidate.capability_id.to_owned(),
            });
        };
        if candidate.system != capability.response_system
            || candidate.semantic_frame != capability.semantic_frame
            || candidate.surface_frame != capability.response_surface_frame
            || candidate.operation != capability.operation
        {
            return Err(PlanError::CandidateCapabilityMismatch {
                rep_id: candidate.rep_id.to_owned(),
                capability_id: candidate.capability_id.to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_guidance_availability(guidance: &[GuidanceAvailability]) -> Result<(), PlanError> {
    for (index, available) in guidance.iter().enumerate() {
        for earlier in &guidance[..index] {
            if earlier.locale == available.locale
                && earlier.pack_version == available.pack_version
                && earlier.instruction_key == available.instruction_key
            {
                return Err(PlanError::DuplicateGuidanceAvailability {
                    locale: available.locale.to_owned(),
                    version: available.pack_version,
                    instruction_key: available.instruction_key.to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn eligible_candidate_indices(
    set: &SetPlan,
    capabilities: &[CapabilitySpec],
    candidates: &[CandidateSummary],
    guidance: &[GuidanceAvailability],
    guidance_locale: &str,
    guidance_pack_version: u16,
    supported_answer_rules: &[AnswerRule],
) -> Result<Vec<Vec<usize>>, PlanError> {
    if guidance_locale.is_empty() || guidance_pack_version == 0 {
        return Err(PlanError::InvalidGuidanceSelection {
            locale: guidance_locale.to_owned(),
            version: guidance_pack_version,
        });
    }

    validate_capability_source(set, capabilities)?;
    validate_candidate_source(capabilities, candidates)?;
    validate_guidance_availability(guidance)?;

    let mut eligible_by_step = Vec::with_capacity(set.steps.len());
    for step in set.steps {
        let mut has_candidate = false;
        let mut has_supported_candidate = false;
        let mut eligible = Vec::new();

        for (candidate_index, candidate) in candidates.iter().enumerate() {
            if candidate.capability_id != step.capability_id {
                continue;
            }
            has_candidate = true;
            if !supported_answer_rules.contains(&candidate.answer_rule) {
                continue;
            }
            has_supported_candidate = true;
            if guidance.iter().any(|available| {
                available.locale == guidance_locale
                    && available.pack_version == guidance_pack_version
                    && available.instruction_key == candidate.instruction_key
            }) {
                eligible.push(candidate_index);
            }
        }

        if !has_candidate {
            return Err(PlanError::NoCandidate {
                step_id: step.id.to_owned(),
                capability_id: step.capability_id.to_owned(),
            });
        }
        if !has_supported_candidate {
            return Err(PlanError::UnsupportedAnswerRule {
                step_id: step.id.to_owned(),
                capability_id: step.capability_id.to_owned(),
            });
        }
        if eligible.is_empty() {
            return Err(PlanError::GuidanceUnavailable {
                step_id: step.id.to_owned(),
                locale: guidance_locale.to_owned(),
                version: guidance_pack_version,
            });
        }
        eligible_by_step.push(eligible);
    }
    Ok(eligible_by_step)
}

fn checked_cardinality_multiply(cardinality: u64, radix: u64) -> Result<u64, PlanError> {
    cardinality
        .checked_mul(radix)
        .ok_or(PlanError::CardinalityOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::planner::{
        CAPABILITIES, RepresentationKind, SET_PLANS, SetStep, SupportLevel,
    };

    // These are inert planner fixtures, not catalog records, accepted rep IDs,
    // or product guidance declarations.
    const FIVE_CANDIDATES: &[CandidateSummary] = &[
        CandidateSummary {
            rep_id: "test-copy",
            catalog_order: 1,
            capability_id: "rust.function.return_literal.copy.full_model",
            system: "rust",
            semantic_frame: "function.no_params.return_literal",
            surface_frame: "function.return_literal",
            operation: Operation::Copy,
            instruction_key: "copy.exact_line",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(0),
        },
        CandidateSummary {
            rep_id: "test-fill",
            catalog_order: 2,
            capability_id: "rust.function.return_literal.fill.return_value",
            system: "rust",
            semantic_frame: "function.no_params.return_literal",
            surface_frame: "function.return_literal",
            operation: Operation::Fill,
            instruction_key: "fill.return_value",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(0),
        },
        CandidateSummary {
            rep_id: "test-produce",
            catalog_order: 3,
            capability_id: "rust.function.return_literal.produce.constrained",
            system: "rust",
            semantic_frame: "function.no_params.return_literal",
            surface_frame: "function.return_literal",
            operation: Operation::Produce,
            instruction_key: "produce.return_literal",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(0),
        },
        CandidateSummary {
            rep_id: "test-repair",
            catalog_order: 4,
            capability_id: "rust.function.return_literal.repair.return_type_arrow",
            system: "rust",
            semantic_frame: "function.no_params.return_literal",
            surface_frame: "function.return_literal",
            operation: Operation::Repair,
            instruction_key: "repair.return_type_arrow",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(0),
        },
        CandidateSummary {
            rep_id: "test-transform",
            catalog_order: 5,
            capability_id: "rust.function.return_literal.transform.tail_expression",
            system: "rust",
            semantic_frame: "function.no_params.return_literal",
            surface_frame: "function.return_literal",
            operation: Operation::Transform,
            instruction_key: "transform.tail_expression",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(0),
        },
    ];

    const EN_GUIDANCE: &[GuidanceAvailability] = &[
        GuidanceAvailability {
            locale: "en",
            pack_version: 1,
            instruction_key: "copy.exact_line",
        },
        GuidanceAvailability {
            locale: "en",
            pack_version: 1,
            instruction_key: "fill.return_value",
        },
        GuidanceAvailability {
            locale: "en",
            pack_version: 1,
            instruction_key: "produce.return_literal",
        },
        GuidanceAvailability {
            locale: "en",
            pack_version: 1,
            instruction_key: "repair.return_type_arrow",
        },
        GuidanceAvailability {
            locale: "en",
            pack_version: 1,
            instruction_key: "transform.tail_expression",
        },
    ];
    const UK_GUIDANCE: &[GuidanceAvailability] = &[
        GuidanceAvailability {
            locale: "uk",
            pack_version: 1,
            instruction_key: "copy.exact_line",
        },
        GuidanceAvailability {
            locale: "uk",
            pack_version: 1,
            instruction_key: "fill.return_value",
        },
        GuidanceAvailability {
            locale: "uk",
            pack_version: 1,
            instruction_key: "produce.return_literal",
        },
        GuidanceAvailability {
            locale: "uk",
            pack_version: 1,
            instruction_key: "repair.return_type_arrow",
        },
        GuidanceAvailability {
            locale: "uk",
            pack_version: 1,
            instruction_key: "transform.tail_expression",
        },
    ];
    const EXACT_RULES: &[AnswerRule] = &[AnswerRule::ExactTrimFinalNewline];

    const MIXED_CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec {
            id: "test.capability.a",
            semantic_frame: "test.frame",
            response_system: "test",
            response_surface_frame: "test.surface",
            cue_system: None,
            cue_surface_frame: None,
            operation: Operation::Copy,
            cue_representation: RepresentationKind::CompleteSurface,
            response_representation: RepresentationKind::CompleteSurface,
            support: SupportLevel::FullModel,
            learning_target_id: "test.target",
        },
        CapabilitySpec {
            id: "test.capability.b",
            semantic_frame: "test.frame",
            response_system: "test",
            response_surface_frame: "test.surface",
            cue_system: None,
            cue_surface_frame: None,
            operation: Operation::Fill,
            cue_representation: RepresentationKind::PartialSurface,
            response_representation: RepresentationKind::CompleteSurface,
            support: SupportLevel::PartialModel,
            learning_target_id: "test.target",
        },
    ];
    const MIXED_STEPS: &[SetStep] = &[
        SetStep {
            id: "step-a",
            capability_id: "test.capability.a",
        },
        SetStep {
            id: "step-b",
            capability_id: "test.capability.b",
        },
    ];
    const MIXED_SET: SetPlan = SetPlan {
        id: "test.mixed",
        version: 1,
        system: "test",
        track: "test",
        semantic_frame: "test.frame",
        response_surface_frame: "test.surface",
        steps: MIXED_STEPS,
    };
    const MIXED_CANDIDATES: &[CandidateSummary] = &[
        CandidateSummary {
            rep_id: "A0",
            catalog_order: 1,
            capability_id: "test.capability.a",
            system: "test",
            semantic_frame: "test.frame",
            surface_frame: "test.surface",
            operation: Operation::Copy,
            instruction_key: "test.key",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: None,
        },
        CandidateSummary {
            rep_id: "A1",
            catalog_order: 2,
            capability_id: "test.capability.a",
            system: "test",
            semantic_frame: "test.frame",
            surface_frame: "test.surface",
            operation: Operation::Copy,
            instruction_key: "test.key",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(4),
        },
        CandidateSummary {
            rep_id: "B0",
            catalog_order: 3,
            capability_id: "test.capability.b",
            system: "test",
            semantic_frame: "test.frame",
            surface_frame: "test.surface",
            operation: Operation::Fill,
            instruction_key: "test.key",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: None,
        },
        CandidateSummary {
            rep_id: "B1",
            catalog_order: 4,
            capability_id: "test.capability.b",
            system: "test",
            semantic_frame: "test.frame",
            surface_frame: "test.surface",
            operation: Operation::Fill,
            instruction_key: "test.key",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(8),
        },
        CandidateSummary {
            rep_id: "B2",
            catalog_order: 5,
            capability_id: "test.capability.b",
            system: "test",
            semantic_frame: "test.frame",
            surface_frame: "test.surface",
            operation: Operation::Fill,
            instruction_key: "test.key",
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            content_seed: Some(9),
        },
    ];
    const MIXED_GUIDANCE: &[GuidanceAvailability] = &[GuidanceAvailability {
        locale: "en",
        pack_version: 1,
        instruction_key: "test.key",
    }];

    fn build_five(
        guidance: &[GuidanceAvailability],
        locale: &str,
        seed: u64,
    ) -> Result<PracticePlan, PlanError> {
        build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            FIVE_CANDIDATES,
            guidance,
            locale,
            1,
            EXACT_RULES,
            seed,
        )
    }

    #[test]
    fn accepted_five_step_source_builds_exact_seed_zero_plan() {
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Ok(1)
        );
        let plan = build_five(EN_GUIDANCE, "en", 0).expect("seed-zero plan must build");
        assert_eq!(
            plan.provenance.set_id,
            "rust.function.return_literal.five_step"
        );
        assert_eq!(plan.provenance.set_version, 1);
        assert_eq!(plan.provenance.plan_seed, 0);
        assert_eq!(plan.provenance.plan_cardinality, 1);
        assert_eq!(plan.provenance.guidance_locale, "en");
        assert_eq!(plan.provenance.guidance_pack_version, 1);
        assert_eq!(plan.provenance.supported_answer_rules, EXACT_RULES);
        assert_eq!(
            plan.reps
                .iter()
                .map(|rep| rep.rep_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "test-copy",
                "test-fill",
                "test-produce",
                "test-repair",
                "test-transform"
            ]
        );
        assert_eq!(
            plan.reps
                .iter()
                .map(|rep| rep.step_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "copy-full-form",
                "fill-return-value",
                "produce-from-intent",
                "repair-return-type-arrow",
                "transform-tail-expression",
            ]
        );
        assert_eq!(
            plan.reps
                .iter()
                .map(|rep| rep.capability_id.as_str())
                .collect::<Vec<_>>(),
            CAPABILITIES
                .iter()
                .map(|capability| capability.id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn identical_inputs_are_reproducible() {
        assert_eq!(
            build_five(EN_GUIDANCE, "en", 0).expect("first plan must build"),
            build_five(EN_GUIDANCE, "en", 0).expect("second plan must build")
        );
    }

    #[test]
    fn guidance_locale_changes_provenance_not_selected_reps() {
        let en = build_five(EN_GUIDANCE, "en", 0).expect("English plan must build");
        let uk = build_five(UK_GUIDANCE, "uk", 0).expect("Ukrainian plan must build");
        assert_eq!(en.reps, uk.reps);
        assert_eq!(en.provenance.set_id, uk.provenance.set_id);
        assert_eq!(en.provenance.set_version, uk.provenance.set_version);
        assert_eq!(en.provenance.plan_seed, uk.provenance.plan_seed);
        assert_eq!(
            en.provenance.plan_cardinality,
            uk.provenance.plan_cardinality
        );
        assert_ne!(en.provenance.guidance_locale, uk.provenance.guidance_locale);
    }

    #[test]
    fn mixed_radix_varies_first_step_fastest() {
        assert_eq!(
            plan_cardinality(
                &MIXED_SET,
                MIXED_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Ok(6)
        );
        let cases = [
            (0, ["A0", "B0"]),
            (1, ["A1", "B0"]),
            (2, ["A0", "B1"]),
            (3, ["A1", "B1"]),
            (4, ["A0", "B2"]),
            (5, ["A1", "B2"]),
        ];
        for (seed, ids) in cases {
            let plan = build_practice_plan(
                &MIXED_SET,
                MIXED_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
                seed,
            )
            .expect("in-range mixed plan must build");
            assert_eq!(
                plan.reps
                    .iter()
                    .map(|rep| rep.rep_id.as_str())
                    .collect::<Vec<_>>(),
                ids
            );
        }
        assert_eq!(
            build_practice_plan(
                &MIXED_SET,
                MIXED_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
                6,
            ),
            Err(PlanError::PlanSeedOutOfRange {
                seed: 6,
                cardinality: 6
            })
        );
    }

    #[test]
    fn zero_step_contract_is_cardinality_one() {
        const EMPTY_STEPS: &[SetStep] = &[];
        const EMPTY_SET: SetPlan = SetPlan {
            id: "test.empty",
            version: 1,
            system: "test",
            track: "test",
            semantic_frame: "test.frame",
            response_surface_frame: "test.surface",
            steps: EMPTY_STEPS,
        };
        assert_eq!(
            plan_cardinality(&EMPTY_SET, &[], &[], &[], "en", 1, EXACT_RULES),
            Ok(1)
        );
        let plan = build_practice_plan(&EMPTY_SET, &[], &[], &[], "en", 1, EXACT_RULES, 0)
            .expect("seed zero must build an empty plan");
        assert!(plan.reps.is_empty());
        assert_eq!(plan.provenance.set_id, "test.empty");
        assert_eq!(plan.provenance.set_version, 1);
        assert_eq!(plan.provenance.plan_seed, 0);
        assert_eq!(plan.provenance.plan_cardinality, 1);
        assert_eq!(plan.provenance.guidance_locale, "en");
        assert_eq!(plan.provenance.guidance_pack_version, 1);
        assert_eq!(plan.provenance.supported_answer_rules, EXACT_RULES);
        assert_eq!(
            build_practice_plan(&EMPTY_SET, &[], &[], &[], "en", 1, EXACT_RULES, 1),
            Err(PlanError::PlanSeedOutOfRange {
                seed: 1,
                cardinality: 1
            })
        );
    }

    #[test]
    fn plan_seed_range_and_checked_overflow_are_explicit() {
        for seed in 0..6 {
            assert!(
                build_practice_plan(
                    &MIXED_SET,
                    MIXED_CAPABILITIES,
                    MIXED_CANDIDATES,
                    MIXED_GUIDANCE,
                    "en",
                    1,
                    EXACT_RULES,
                    seed,
                )
                .is_ok()
            );
        }
        assert_eq!(
            checked_cardinality_multiply(u64::MAX, 2),
            Err(PlanError::CardinalityOverflow)
        );
        assert_eq!(checked_cardinality_multiply(3, 2), Ok(6));
    }

    #[test]
    fn capability_source_failures_are_explicit() {
        const DUPLICATE_CAPABILITIES: &[CapabilitySpec] = &[
            CapabilitySpec {
                id: "test.capability.a",
                semantic_frame: "test.frame",
                response_system: "test",
                response_surface_frame: "test.surface",
                cue_system: None,
                cue_surface_frame: None,
                operation: Operation::Copy,
                cue_representation: RepresentationKind::CompleteSurface,
                response_representation: RepresentationKind::CompleteSurface,
                support: SupportLevel::FullModel,
                learning_target_id: "test.target",
            },
            CapabilitySpec {
                id: "test.capability.a",
                semantic_frame: "test.frame",
                response_system: "test",
                response_surface_frame: "test.surface",
                cue_system: None,
                cue_surface_frame: None,
                operation: Operation::Copy,
                cue_representation: RepresentationKind::CompleteSurface,
                response_representation: RepresentationKind::CompleteSurface,
                support: SupportLevel::FullModel,
                learning_target_id: "test.target",
            },
        ];
        assert_eq!(
            plan_cardinality(
                &MIXED_SET,
                DUPLICATE_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::DuplicateCapability(
                "test.capability.a".to_owned()
            ))
        );

        const UNKNOWN_STEPS: &[SetStep] = &[SetStep {
            id: "unknown-step",
            capability_id: "missing.capability",
        }];
        const UNKNOWN_SET: SetPlan = SetPlan {
            id: "test.unknown",
            version: 1,
            system: "test",
            track: "test",
            semantic_frame: "test.frame",
            response_surface_frame: "test.surface",
            steps: UNKNOWN_STEPS,
        };
        assert_eq!(
            plan_cardinality(
                &UNKNOWN_SET,
                MIXED_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::UnknownCapability {
                step_id: "unknown-step".to_owned(),
                capability_id: "missing.capability".to_owned(),
            })
        );

        const MISMATCH_STEPS: &[SetStep] = &[SetStep {
            id: "mismatch-step",
            capability_id: "test.capability.a",
        }];
        const MISMATCH_SET: SetPlan = SetPlan {
            id: "test.mismatch",
            version: 1,
            system: "other",
            track: "test",
            semantic_frame: "test.frame",
            response_surface_frame: "test.surface",
            steps: MISMATCH_STEPS,
        };
        assert_eq!(
            plan_cardinality(
                &MISMATCH_SET,
                MIXED_CAPABILITIES,
                MIXED_CANDIDATES,
                MIXED_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::CapabilitySetMismatch {
                step_id: "mismatch-step".to_owned(),
                capability_id: "test.capability.a".to_owned(),
            })
        );
    }

    #[test]
    fn candidate_source_failures_are_explicit() {
        let mut candidates = FIVE_CANDIDATES.to_vec();
        candidates[1].rep_id = candidates[0].rep_id;
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::DuplicateCandidateId("test-copy".to_owned()))
        );

        let mut candidates = FIVE_CANDIDATES.to_vec();
        candidates[1].catalog_order = candidates[0].catalog_order;
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::CandidateOrderViolation {
                earlier_order: 1,
                current_order: 1
            })
        );

        let mut candidates = FIVE_CANDIDATES.to_vec();
        candidates[1].catalog_order = 0;
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::CandidateOrderViolation {
                earlier_order: 1,
                current_order: 0
            })
        );

        let mut candidates = FIVE_CANDIDATES.to_vec();
        candidates[0].capability_id = "missing.capability";
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::UnknownCandidateCapability {
                rep_id: "test-copy".to_owned(),
                capability_id: "missing.capability".to_owned(),
            })
        );

        for field in ["system", "semantic", "surface", "operation"] {
            let mut candidates = FIVE_CANDIDATES.to_vec();
            match field {
                "system" => candidates[0].system = "other",
                "semantic" => candidates[0].semantic_frame = "other.frame",
                "surface" => candidates[0].surface_frame = "other.surface",
                "operation" => candidates[0].operation = Operation::Recognize,
                _ => unreachable!(),
            }
            assert_eq!(
                plan_cardinality(
                    &SET_PLANS[0],
                    CAPABILITIES,
                    &candidates,
                    EN_GUIDANCE,
                    "en",
                    1,
                    EXACT_RULES,
                ),
                Err(PlanError::CandidateCapabilityMismatch {
                    rep_id: "test-copy".to_owned(),
                    capability_id: "rust.function.return_literal.copy.full_model".to_owned(),
                })
            );
        }
    }

    #[test]
    fn eligibility_and_guidance_failures_are_explicit() {
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &FIVE_CANDIDATES[1..],
                EN_GUIDANCE,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::NoCandidate {
                step_id: "copy-full-form".to_owned(),
                capability_id: "rust.function.return_literal.copy.full_model".to_owned(),
            })
        );
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                EN_GUIDANCE,
                "en",
                1,
                &[AnswerRule::RustfmtExact],
            ),
            Err(PlanError::UnsupportedAnswerRule {
                step_id: "copy-full-form".to_owned(),
                capability_id: "rust.function.return_literal.copy.full_model".to_owned(),
            })
        );
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                &[],
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::GuidanceUnavailable {
                step_id: "copy-full-form".to_owned(),
                locale: "en".to_owned(),
                version: 1,
            })
        );
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                EN_GUIDANCE,
                "",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::InvalidGuidanceSelection {
                locale: String::new(),
                version: 1,
            })
        );
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                EN_GUIDANCE,
                "en",
                0,
                EXACT_RULES,
            ),
            Err(PlanError::InvalidGuidanceSelection {
                locale: "en".to_owned(),
                version: 0,
            })
        );

        let duplicate_guidance = [EN_GUIDANCE[0], EN_GUIDANCE[0]];
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                FIVE_CANDIDATES,
                &duplicate_guidance,
                "en",
                1,
                EXACT_RULES,
            ),
            Err(PlanError::DuplicateGuidanceAvailability {
                locale: "en".to_owned(),
                version: 1,
                instruction_key: "copy.exact_line".to_owned(),
            })
        );
    }

    #[test]
    fn content_seed_is_inert_planning_provenance() {
        let first = FIVE_CANDIDATES.to_vec();
        let mut second = FIVE_CANDIDATES.to_vec();
        for candidate in &mut second {
            candidate.content_seed = candidate.content_seed.map(|seed| seed + 100);
        }
        let first_plan = build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            &first,
            EN_GUIDANCE,
            "en",
            1,
            EXACT_RULES,
            0,
        )
        .expect("first plan must build");
        let second_plan = build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            &second,
            EN_GUIDANCE,
            "en",
            1,
            EXACT_RULES,
            0,
        )
        .expect("second plan must build");
        assert_eq!(first_plan, second_plan);
    }

    #[test]
    fn repeated_capability_steps_may_repeat_one_rep() {
        const REPEATED_STEPS: &[SetStep] = &[
            SetStep {
                id: "copy-once",
                capability_id: "rust.function.return_literal.copy.full_model",
            },
            SetStep {
                id: "copy-again",
                capability_id: "rust.function.return_literal.copy.full_model",
            },
        ];
        const REPEATED_SET: SetPlan = SetPlan {
            id: "test.repeated",
            version: 1,
            system: "rust",
            track: "test",
            semantic_frame: "function.no_params.return_literal",
            response_surface_frame: "function.return_literal",
            steps: REPEATED_STEPS,
        };
        let plan = build_practice_plan(
            &REPEATED_SET,
            CAPABILITIES,
            &FIVE_CANDIDATES[..1],
            EN_GUIDANCE,
            "en",
            1,
            EXACT_RULES,
            0,
        )
        .expect("explicit repeated steps must build");
        assert_eq!(plan.reps.len(), 2);
        assert_eq!(plan.reps[0].rep_id, "test-copy");
        assert_eq!(plan.reps[1].rep_id, "test-copy");
        assert_ne!(plan.reps[0].step_id, plan.reps[1].step_id);
    }

    #[test]
    fn supported_answer_rule_provenance_preserves_caller_order() {
        let rules = [
            AnswerRule::RustfmtExact,
            AnswerRule::ExactTrimFinalNewline,
            AnswerRule::RustfmtExact,
        ];
        let plan = build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            FIVE_CANDIDATES,
            EN_GUIDANCE,
            "en",
            1,
            &rules,
            0,
        )
        .expect("supported rule order must build");
        assert_eq!(plan.provenance.supported_answer_rules, rules);
    }

    #[test]
    fn owned_output_has_only_planning_identity_and_provenance() {
        let plan = build_five(EN_GUIDANCE, "en", 0).expect("plan must build");
        let PracticePlan { provenance, reps } = plan;
        let PlanProvenance {
            set_id,
            set_version,
            plan_seed,
            plan_cardinality,
            guidance_locale,
            guidance_pack_version,
            supported_answer_rules,
        } = provenance;
        assert_eq!(set_id, "rust.function.return_literal.five_step");
        assert_eq!(set_version, 1);
        assert_eq!(plan_seed, 0);
        assert_eq!(plan_cardinality, 1);
        assert_eq!(guidance_locale, "en");
        assert_eq!(guidance_pack_version, 1);
        assert_eq!(supported_answer_rules, EXACT_RULES);
        for rep in reps {
            let PlannedRep {
                step_id,
                capability_id,
                rep_id,
            } = rep;
            assert!(!step_id.is_empty());
            assert!(!capability_id.is_empty());
            assert!(!rep_id.is_empty());
        }
    }
}
