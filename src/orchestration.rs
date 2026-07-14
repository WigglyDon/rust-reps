use std::borrow::Cow;
use std::fmt;

use crate::catalog;
use crate::curriculum::guidance::GUIDANCE_PACKS;
use crate::curriculum::guidance::resolution::{
    GuidanceAssignmentRef, GuidanceError, GuidanceProvenance, resolve_guidance,
    resolve_support_guidance,
};
use crate::curriculum::planner::planning::{
    CandidateSummary, GuidanceAvailability, PlanError, build_practice_plan,
};
use crate::curriculum::planner::{CAPABILITIES, SET_PLANS, SupportStage};
use crate::curriculum::support::task_support;
use crate::curriculum::{
    AnswerRule, DrillSpec, Formatting, GeneratedTaskProvenanceSpec, RepMetadata,
};
use crate::engine::{
    ExerciseAnswerRule, ExerciseFocusRange, ExerciseKind, ExercisePacket, ExerciseSupport,
    ExerciseSupportStage, ExerciseValidator, FormatCapability, Session,
};

pub(crate) const INITIAL_PRODUCT_GUIDANCE_LOCALE: &str = "en";
pub(crate) const INITIAL_PRODUCT_GUIDANCE_PACK_VERSION: u16 = 1;
pub(crate) const DEFAULT_INTERACTIVE_PLAN_SEED: u64 = 0;
const INTERACTIVE_ROUND_SIZE: usize = 5;
const INTERACTIVE_ROW_IDS: [[&str; INTERACTIVE_ROUND_SIZE]; 2] = [
    ["013", "014", "015", "016", "017"],
    ["018", "019", "020", "021", "022"],
];

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResolvedCoreTask {
    pub(crate) rep_id: &'static str,
    pub(crate) canonical_id: &'static str,
    pub(crate) capability_id: Option<&'static str>,
    pub(crate) generated_provenance: Option<GeneratedTaskProvenanceSpec>,
    pub(crate) instruction: String,
    pub(crate) stimulus: &'static str,
    pub(crate) expected: &'static str,
    pub(crate) formatting: Formatting,
    pub(crate) metadata: RepMetadata,
    pub(crate) guidance_provenance: GuidanceProvenance,
    pub(crate) support: Option<ResolvedTaskSupport>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResolvedTaskSupport {
    pub(crate) concept_key: &'static str,
    pub(crate) stage: SupportStage,
    pub(crate) anchor_id: &'static str,
    pub(crate) artifact_key: &'static str,
    pub(crate) target_dimension: &'static str,
    pub(crate) action_header: String,
    pub(crate) operation_label: String,
    pub(crate) artifact_label: String,
    pub(crate) concept_label: String,
    pub(crate) catalog_label: String,
    pub(crate) beginner_brief: Vec<String>,
    pub(crate) explanation: String,
    pub(crate) worked_example: Option<String>,
    pub(crate) focus_ranges: Vec<crate::curriculum::support::FocusRangeSpec>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ResolveCoreTaskError {
    Guidance(GuidanceError),
    SupportConfiguration(String),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ProductExerciseError {
    UnknownId(String),
    Guidance(ResolveCoreTaskError),
    Plan(PlanError),
}

fn validate_support_focus_ranges(
    rep_id: &str,
    stimulus: &str,
    ranges: &[crate::curriculum::support::FocusRangeSpec],
) -> Result<(), ResolveCoreTaskError> {
    for range in ranges {
        if range.start >= range.end
            || range.end > stimulus.len()
            || !stimulus.is_char_boundary(range.start)
            || !stimulus.is_char_boundary(range.end)
        {
            return Err(ResolveCoreTaskError::SupportConfiguration(format!(
                "task {rep_id} has an invalid stimulus focus range"
            )));
        }
    }
    Ok(())
}

impl fmt::Display for ProductExerciseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownId(id) => write!(formatter, "unknown drill id: {id}"),
            Self::Guidance(error) => {
                write!(
                    formatter,
                    "invalid product guidance configuration: {error:?}"
                )
            }
            Self::Plan(error) => write!(formatter, "invalid product practice plan: {error:?}"),
        }
    }
}

impl From<ResolveCoreTaskError> for ProductExerciseError {
    fn from(error: ResolveCoreTaskError) -> Self {
        Self::Guidance(error)
    }
}

impl From<PlanError> for ProductExerciseError {
    fn from(error: PlanError) -> Self {
        Self::Plan(error)
    }
}

pub(crate) fn resolve_core_task(
    drill: &'static DrillSpec,
    locale: &str,
    pack_version: u16,
) -> Result<ResolvedCoreTask, ResolveCoreTaskError> {
    let assignments = drill
        .assignments
        .iter()
        .map(|assignment| GuidanceAssignmentRef {
            slot: assignment.slot,
            rendered: assignment.rendered,
            meaning_key: assignment.meaning_key,
        })
        .collect::<Vec<_>>();
    let resolved_guidance = resolve_guidance(
        drill.metadata.canonical_id,
        drill.instruction_key,
        &assignments,
        GUIDANCE_PACKS,
        locale,
        pack_version,
    )
    .map_err(ResolveCoreTaskError::Guidance)?;
    let support = if let Some(source) = task_support(drill.id) {
        validate_support_focus_ranges(drill.id, drill.stimulus, source.focus_ranges)?;
        let capability_id = drill.capability_id.ok_or_else(|| {
            ResolveCoreTaskError::SupportConfiguration(format!(
                "task {} has support without a capability",
                drill.id
            ))
        })?;
        let capability = CAPABILITIES
            .iter()
            .find(|capability| capability.id == capability_id)
            .ok_or_else(|| {
                ResolveCoreTaskError::SupportConfiguration(format!(
                    "task {} names unknown capability {capability_id}",
                    drill.id
                ))
            })?;
        if capability.learning_target_id != source.concept_key {
            return Err(ResolveCoreTaskError::SupportConfiguration(format!(
                "task {} support concept does not match its capability",
                drill.id
            )));
        }
        let resolved = resolve_support_guidance(
            drill.metadata.operation,
            source.artifact_key,
            source.help_key,
            GUIDANCE_PACKS,
            locale,
            pack_version,
        )
        .map_err(ResolveCoreTaskError::Guidance)?;
        Some(ResolvedTaskSupport {
            concept_key: source.concept_key,
            stage: capability.support.stage(),
            anchor_id: source.anchor_id,
            artifact_key: source.artifact_key,
            target_dimension: source.target_dimension,
            action_header: resolved.action_header,
            operation_label: resolved.operation_label,
            artifact_label: resolved.artifact_label,
            concept_label: resolved.concept_label,
            catalog_label: resolved.catalog_label,
            beginner_brief: resolved.beginner_brief,
            explanation: resolved.explanation,
            worked_example: resolved.worked_example,
            focus_ranges: source.focus_ranges.to_vec(),
        })
    } else {
        None
    };

    Ok(ResolvedCoreTask {
        rep_id: drill.id,
        canonical_id: drill.metadata.canonical_id,
        capability_id: drill.capability_id,
        generated_provenance: drill.generated_provenance,
        instruction: resolved_guidance.instruction,
        stimulus: drill.stimulus,
        expected: drill.expected,
        formatting: drill.formatting,
        metadata: drill.metadata,
        guidance_provenance: resolved_guidance.provenance,
        support,
    })
}

fn product_exercise_packet_from_drill(
    drill: &'static DrillSpec,
    position: usize,
    total: usize,
) -> Result<ExercisePacket, ProductExerciseError> {
    let ResolvedCoreTask {
        rep_id,
        canonical_id,
        capability_id,
        generated_provenance,
        instruction,
        stimulus,
        expected,
        formatting,
        metadata,
        guidance_provenance,
        support,
    } = resolve_core_task(
        drill,
        INITIAL_PRODUCT_GUIDANCE_LOCALE,
        INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
    )?;
    let _ = (
        canonical_id,
        capability_id,
        generated_provenance,
        guidance_provenance,
    );

    let packet = ExercisePacket::new(
        Cow::Borrowed(rep_id),
        position,
        total,
        Cow::Owned(instruction),
        Cow::Borrowed(stimulus),
        Cow::Borrowed(expected),
        match formatting {
            Formatting::Disabled => FormatCapability::Disabled,
            Formatting::Rustfmt => FormatCapability::Rustfmt,
        },
        ExerciseKind::from_task_type(drill.task_type),
        ExerciseValidator::from_validator(drill.validator),
        ExerciseAnswerRule::from_answer_rule(metadata.answer_rule),
    );
    Ok(if let Some(support) = support {
        packet.with_support(ExerciseSupport {
            stage: match support.stage {
                SupportStage::Introduce => ExerciseSupportStage::Introduce,
                SupportStage::Guided => ExerciseSupportStage::Guided,
                SupportStage::Practice => ExerciseSupportStage::Practice,
            },
            action_header: Cow::Owned(support.action_header),
            catalog_label: Cow::Owned(support.catalog_label),
            beginner_brief: support.beginner_brief.into_iter().map(Cow::Owned).collect(),
            explanation: Cow::Owned(support.explanation),
            worked_example: support.worked_example.map(Cow::Owned),
            focus_ranges: support
                .focus_ranges
                .into_iter()
                .map(|range| ExerciseFocusRange {
                    start: range.start,
                    end: range.end,
                })
                .collect(),
        })
    } else {
        packet
    })
}

pub(crate) fn current_product_exercise_packet_at(
    index: usize,
) -> Result<Option<ExercisePacket>, ProductExerciseError> {
    let drills = catalog::drills();
    drills
        .get(index)
        .map(|drill| product_exercise_packet_from_drill(drill, index + 1, drills.len()))
        .transpose()
}

pub(crate) fn current_product_exercise_packet_by_id(
    id: &str,
) -> Result<ExercisePacket, ProductExerciseError> {
    let drill =
        catalog::find_drill(id).ok_or_else(|| ProductExerciseError::UnknownId(id.to_owned()))?;
    let index = catalog::drills()
        .iter()
        .position(|candidate| candidate.id == drill.id)
        .ok_or_else(|| ProductExerciseError::UnknownId(id.to_owned()))?;
    current_product_exercise_packet_at(index)?
        .ok_or_else(|| ProductExerciseError::UnknownId(id.to_owned()))
}

pub(crate) fn current_product_exercise_packets() -> Result<Vec<ExercisePacket>, ProductExerciseError>
{
    (0..catalog::drills().len())
        .map(|index| {
            current_product_exercise_packet_at(index).and_then(|packet| {
                packet.ok_or_else(|| ProductExerciseError::UnknownId(index.to_string()))
            })
        })
        .collect()
}

pub(crate) fn current_product_session() -> Result<Session, ProductExerciseError> {
    current_product_exercise_packets().map(Session::from_exercises)
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct InteractiveRoundPlan {
    pub(crate) plan_seed: u64,
    pub(crate) round_index: u64,
    pub(crate) row_index: usize,
    pub(crate) content_seed: u64,
    pub(crate) guidance_locale: &'static str,
    pub(crate) ids: Vec<&'static str>,
}

pub(crate) fn interactive_round_plan(
    plan_seed: u64,
    round_index: u64,
) -> Result<InteractiveRoundPlan, ProductExerciseError> {
    let row_index = usize::try_from(
        plan_seed.wrapping_add(round_index) % u64::try_from(INTERACTIVE_ROW_IDS.len()).unwrap(),
    )
    .expect("two row indices fit in usize");
    let selected_ids = &INTERACTIVE_ROW_IDS[row_index];
    let candidates = selected_ids
        .iter()
        .map(|id| {
            let drill = catalog::find_drill(id)
                .ok_or_else(|| ProductExerciseError::UnknownId((*id).to_owned()))?;
            let catalog_order = catalog::drills()
                .iter()
                .position(|candidate| candidate.id == *id)
                .ok_or_else(|| ProductExerciseError::UnknownId((*id).to_owned()))?;
            Ok(CandidateSummary {
                rep_id: drill.id,
                catalog_order: u32::try_from(catalog_order + 1)
                    .expect("current catalog order fits in u32"),
                capability_id: drill
                    .capability_id
                    .ok_or_else(|| ProductExerciseError::UnknownId((*id).to_owned()))?,
                system: drill.metadata.system,
                semantic_frame: drill.metadata.semantic_frame,
                surface_frame: drill.metadata.surface_frame,
                operation: drill.metadata.operation,
                instruction_key: drill.instruction_key,
                answer_rule: drill.metadata.answer_rule,
                content_seed: drill
                    .generated_provenance
                    .map(|provenance| provenance.content_seed),
            })
        })
        .collect::<Result<Vec<_>, ProductExerciseError>>()?;
    let pack = crate::curriculum::guidance::GUIDANCE_PACKS
        .iter()
        .find(|pack| {
            pack.locale == INITIAL_PRODUCT_GUIDANCE_LOCALE
                && pack.version == INITIAL_PRODUCT_GUIDANCE_PACK_VERSION
        })
        .expect("accepted English guidance pack exists");
    let guidance = pack
        .instructions
        .iter()
        .map(|instruction| GuidanceAvailability {
            locale: pack.locale,
            pack_version: pack.version,
            instruction_key: instruction.key,
        })
        .collect::<Vec<_>>();
    let supported_rules = [AnswerRule::ExactTrimFinalNewline, AnswerRule::RustfmtExact];
    let plan = build_practice_plan(
        &SET_PLANS[0],
        CAPABILITIES,
        &candidates,
        &guidance,
        INITIAL_PRODUCT_GUIDANCE_LOCALE,
        INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
        &supported_rules,
        0,
    )?;
    let ids = plan
        .reps
        .iter()
        .map(|planned| {
            catalog::find_drill(&planned.rep_id)
                .map(|drill| drill.id)
                .ok_or_else(|| ProductExerciseError::UnknownId(planned.rep_id.clone()))
        })
        .collect::<Result<Vec<_>, ProductExerciseError>>()?;
    let content_seed = candidates[0]
        .content_seed
        .expect("interactive admitted row has content provenance");
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate.content_seed == Some(content_seed)),
        "interactive round candidates must share one correlated content row"
    );

    Ok(InteractiveRoundPlan {
        plan_seed,
        round_index,
        row_index,
        content_seed,
        guidance_locale: INITIAL_PRODUCT_GUIDANCE_LOCALE,
        ids,
    })
}

pub(crate) fn current_interactive_product_session() -> Result<Session, ProductExerciseError> {
    let catalog_exercises = current_product_exercise_packets()?;
    let mut looping_exercises =
        Vec::with_capacity(INTERACTIVE_ROW_IDS.len() * INTERACTIVE_ROUND_SIZE);
    for round_index in 0..u64::try_from(INTERACTIVE_ROW_IDS.len()).unwrap() {
        let plan = interactive_round_plan(DEFAULT_INTERACTIVE_PLAN_SEED, round_index)?;
        for (position, id) in plan.ids.iter().enumerate() {
            let drill = catalog::find_drill(id)
                .ok_or_else(|| ProductExerciseError::UnknownId((*id).to_owned()))?;
            looping_exercises.push(product_exercise_packet_from_drill(
                drill,
                position + 1,
                INTERACTIVE_ROUND_SIZE,
            )?);
        }
    }

    Ok(Session::looping_from_exercises(
        looping_exercises,
        catalog_exercises,
        INTERACTIVE_ROUND_SIZE,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::DRILLS;

    static MISSING_INSTRUCTION_DRILL: DrillSpec = DrillSpec {
        instruction_key: "missing.instruction",
        ..DRILLS[0]
    };

    #[test]
    fn support_focus_validation_rejects_out_of_bounds_and_non_utf8_boundaries() {
        use crate::curriculum::support::FocusRangeSpec;

        assert!(
            validate_support_focus_ranges(
                "test",
                "é: number",
                &[FocusRangeSpec { start: 2, end: 3 }],
            )
            .is_ok()
        );
        assert!(matches!(
            validate_support_focus_ranges(
                "test",
                "é: number",
                &[FocusRangeSpec { start: 1, end: 3 }],
            ),
            Err(ResolveCoreTaskError::SupportConfiguration(_))
        ));
        assert!(matches!(
            validate_support_focus_ranges("test", "short", &[FocusRangeSpec { start: 1, end: 8 }],),
            Err(ResolveCoreTaskError::SupportConfiguration(_))
        ));
    }

    #[test]
    fn every_current_rep_resolves_exact_english_task_truth() {
        assert_eq!(DRILLS.len(), 22);

        for drill in DRILLS {
            let resolved = resolve_core_task(
                drill,
                INITIAL_PRODUCT_GUIDANCE_LOCALE,
                INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
            )
            .expect("every current admitted rep must resolve English guidance");

            assert_eq!(resolved.rep_id, drill.id);
            assert_eq!(resolved.canonical_id, drill.metadata.canonical_id);
            if drill.id <= "012" {
                assert_eq!(resolved.capability_id, None);
                assert_eq!(resolved.generated_provenance, None);
                assert_eq!(resolved.instruction, "Copy this line exactly:");
            } else {
                assert!(resolved.capability_id.is_some());
                assert!(resolved.generated_provenance.is_some());
                assert!(!resolved.instruction.is_empty());
            }
            assert_eq!(resolved.stimulus, drill.stimulus);
            assert_eq!(resolved.expected, drill.expected);
            assert_eq!(resolved.metadata, drill.metadata);
            assert_eq!(
                resolved.guidance_provenance.core_task_id,
                drill.metadata.canonical_id
            );
            assert_eq!(resolved.guidance_provenance.locale, "en");
            assert_eq!(resolved.guidance_provenance.pack_version, 1);
            assert_eq!(
                resolved.guidance_provenance.instruction_key,
                drill.instruction_key
            );
        }
    }

    #[test]
    fn missing_instruction_is_a_guidance_error_without_task_mutation() {
        assert_eq!(
            resolve_core_task(
                &MISSING_INSTRUCTION_DRILL,
                INITIAL_PRODUCT_GUIDANCE_LOCALE,
                INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
            ),
            Err(ResolveCoreTaskError::Guidance(
                GuidanceError::UnknownInstruction {
                    locale: "en".to_owned(),
                    version: 1,
                    key: "missing.instruction".to_owned(),
                }
            ))
        );
        assert_eq!(MISSING_INSTRUCTION_DRILL.stimulus, DRILLS[0].stimulus);
        assert_eq!(MISSING_INSTRUCTION_DRILL.expected, DRILLS[0].expected);
    }

    #[test]
    fn resolved_task_keeps_guidance_and_core_answer_authority_separate() {
        let drill = &DRILLS[0];
        let resolved = resolve_core_task(
            drill,
            INITIAL_PRODUCT_GUIDANCE_LOCALE,
            INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
        )
        .expect("current core task must resolve");
        let ResolvedCoreTask {
            rep_id,
            canonical_id,
            capability_id,
            generated_provenance,
            instruction,
            stimulus,
            expected,
            formatting,
            metadata,
            guidance_provenance,
            support,
        } = resolved;

        assert_eq!(rep_id, drill.id);
        assert_eq!(canonical_id, drill.metadata.canonical_id);
        assert_eq!(capability_id, None);
        assert_eq!(generated_provenance, None);
        assert_eq!(instruction, "Copy this line exactly:");
        assert_eq!(stimulus, drill.stimulus);
        assert_eq!(expected, drill.expected);
        assert_eq!(formatting, drill.formatting);
        assert_eq!(metadata, drill.metadata);
        assert_eq!(guidance_provenance.core_task_id, canonical_id);
        assert_eq!(support, None);
    }

    #[test]
    fn every_current_product_packet_preserves_resolved_catalog_truth() {
        assert_eq!(DRILLS.len(), 22);

        for (index, drill) in DRILLS.iter().enumerate() {
            let packet = current_product_exercise_packet_at(index)
                .expect("current product packet must resolve")
                .expect("current product index must exist");

            assert_eq!(packet.id, drill.id);
            assert_eq!(packet.position, index + 1);
            assert_eq!(packet.total, DRILLS.len());
            assert_eq!(
                packet.instruction,
                resolve_core_task(drill, "en", 1)
                    .expect("admitted task guidance must resolve")
                    .instruction
            );
            assert_eq!(packet.stimulus, drill.stimulus);
            assert_eq!(packet.expected_answer(), drill.expected);
            assert_eq!(
                packet.format_capability,
                match drill.formatting {
                    Formatting::Disabled => FormatCapability::Disabled,
                    Formatting::Rustfmt => FormatCapability::Rustfmt,
                }
            );
            assert_eq!(
                packet.answer_rule,
                ExerciseAnswerRule::from_answer_rule(drill.metadata.answer_rule)
            );
        }
    }

    #[test]
    fn varied_tasks_resolve_explicit_submission_artifacts_without_answer_changes() {
        let expectations = [
            (
                "013",
                "copy.return_type_intro",
                "Copy this line exactly: Type the complete function exactly.",
            ),
            (
                "014",
                "fill.return_type_arrow",
                "Complete the missing return-type arrow: Type only the code replacing ___.",
            ),
            (
                "015",
                "produce.return_type_arrow",
                "Use the shown parts to write the function with its return type: Submit the complete function.",
            ),
            (
                "016",
                "repair.return_type_arrow",
                "Repair the return-type syntax in the highlighted region: Submit the complete corrected function.",
            ),
            (
                "017",
                "transform.add_return_type",
                "Add the declared return type after the parameter list: Submit the complete transformed function.",
            ),
            (
                "018",
                "copy.return_type_intro",
                "Copy this line exactly: Type the complete function exactly.",
            ),
            (
                "019",
                "fill.return_type_arrow",
                "Complete the missing return-type arrow: Type only the code replacing ___.",
            ),
            (
                "020",
                "produce.return_type_arrow",
                "Use the shown parts to write the function with its return type: Submit the complete function.",
            ),
            (
                "021",
                "repair.return_type_arrow",
                "Repair the return-type syntax in the highlighted region: Submit the complete corrected function.",
            ),
            (
                "022",
                "transform.add_return_type",
                "Add the declared return type after the parameter list: Submit the complete transformed function.",
            ),
        ];

        for (id, instruction_key, instruction) in expectations {
            let drill = DRILLS
                .iter()
                .find(|drill| drill.id == id)
                .expect("varied task must remain admitted");
            let resolved = resolve_core_task(
                drill,
                INITIAL_PRODUCT_GUIDANCE_LOCALE,
                INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
            )
            .expect("varied task guidance must resolve");

            assert_eq!(drill.instruction_key, instruction_key);
            assert_eq!(resolved.instruction, instruction);
            assert_eq!(resolved.stimulus, drill.stimulus);
            assert_eq!(resolved.expected, drill.expected);
            assert_eq!(resolved.metadata, drill.metadata);
        }
    }

    #[test]
    fn varied_support_resolves_before_expected_free_client_projection() {
        let stages = [
            SupportStage::Introduce,
            SupportStage::Guided,
            SupportStage::Guided,
            SupportStage::Guided,
            SupportStage::Practice,
        ];

        for (index, drill) in DRILLS[12..].iter().enumerate() {
            let resolved = resolve_core_task(drill, "en", 1).expect("varied support must resolve");
            let support = resolved.support.expect("varied task must carry support");
            assert_eq!(support.stage, stages[index % 5]);
            assert_eq!(
                support.concept_key,
                crate::curriculum::support::RETURN_TYPE_CONCEPT_KEY
            );
            assert_eq!(support.anchor_id, if index < 5 { "013" } else { "018" });
            assert_eq!(support.artifact_key, "function");
            assert!(!support.target_dimension.is_empty());
            assert!(!support.action_header.is_empty());
            assert!(!support.operation_label.is_empty());
            assert_eq!(support.artifact_label, "FUNCTION");
            assert!(!support.concept_label.is_empty());
            assert!(support.catalog_label.contains(" · FUNCTION · "));
            assert_eq!(support.beginner_brief.len(), 7);
            assert!(!support.explanation.is_empty());
            assert_ne!(support.explanation, resolved.expected);
            assert_ne!(support.worked_example.as_deref(), Some(resolved.expected));
            assert!(support.focus_ranges.iter().all(|range| {
                range.start < range.end
                    && range.end <= resolved.stimulus.len()
                    && resolved.stimulus.is_char_boundary(range.start)
                    && resolved.stimulus.is_char_boundary(range.end)
            }));

            let packet = current_product_exercise_packet_by_id(drill.id)
                .expect("varied packet must resolve");
            let ExerciseSupport {
                stage: _,
                action_header: _,
                catalog_label: _,
                beginner_brief: _,
                explanation: _,
                worked_example: _,
                focus_ranges: _,
            } = packet.support.expect("packet must carry resolved support");
        }
    }

    #[test]
    fn problem_first_headers_labels_and_beginner_brief_resolve_separately_from_cli_instructions() {
        let expected_headers = [
            "COPY EXACTLY:",
            "FILL ___ ONLY:",
            "WRITE THE COMPLETE FUNCTION:",
            "REPAIR THE HIGHLIGHTED PART:",
            "ADD THE RETURN TYPE:",
        ];
        let expected_catalog_labels = [
            "COPY · FUNCTION · RETURN TYPE",
            "FILL · FUNCTION · RETURN TYPE",
            "PRODUCE · FUNCTION · RETURN TYPE",
            "REPAIR · FUNCTION · RETURN TYPE",
            "TRANSFORM · FUNCTION · RETURN TYPE",
        ];
        let expected_brief = [
            "FUNCTION — a named block of instructions that does a job.",
            "CALL — tell the computer to run a function.",
            "RETURN — give a value back to the code that called the function.",
            "TYPE — the kind of value something is.",
            "RETURN TYPE — the kind of value a function promises to give back.",
            "WHY — other code and tools can know what kind of value to expect.",
            "RUST FORM — after the function's (), write -> and then the type.",
        ];

        for drill in &DRILLS[12..] {
            let operation_index = (drill.id.parse::<usize>().unwrap() - 13) % 5;
            let resolved = resolve_core_task(drill, "en", 1).unwrap();
            let support = resolved.support.as_ref().unwrap();

            assert_eq!(support.action_header, expected_headers[operation_index]);
            assert_eq!(
                support.catalog_label,
                expected_catalog_labels[operation_index]
            );
            assert_eq!(support.beginner_brief, expected_brief);
            assert_ne!(support.action_header, resolved.instruction);
            assert!(!support.catalog_label.contains(resolved.expected));
            assert!(
                support
                    .beginner_brief
                    .iter()
                    .all(|line| line != resolved.expected)
            );
            assert_ne!(support.explanation, resolved.expected);
            assert_ne!(support.worked_example.as_deref(), Some(resolved.expected));
        }
    }

    #[test]
    fn locale_changes_only_resolved_prose_not_symbolic_task_or_support_focus() {
        for drill in &DRILLS[12..] {
            let english = resolve_core_task(drill, "en", 1).expect("English must resolve");
            let ukrainian =
                resolve_core_task(drill, "uk", 1).expect("Ukrainian proof must resolve");
            let english_support = english.support.as_ref().unwrap();
            let ukrainian_support = ukrainian.support.as_ref().unwrap();

            assert_eq!(english.rep_id, ukrainian.rep_id);
            assert_eq!(english.canonical_id, ukrainian.canonical_id);
            assert_eq!(english.stimulus, ukrainian.stimulus);
            assert_eq!(english.expected, ukrainian.expected);
            assert_eq!(english.metadata, ukrainian.metadata);
            assert_eq!(english_support.concept_key, ukrainian_support.concept_key);
            assert_eq!(english_support.stage, ukrainian_support.stage);
            assert_eq!(english_support.anchor_id, ukrainian_support.anchor_id);
            assert_eq!(english_support.artifact_key, ukrainian_support.artifact_key);
            assert_eq!(
                english_support.target_dimension,
                ukrainian_support.target_dimension
            );
            assert_eq!(english_support.focus_ranges, ukrainian_support.focus_ranges);
            assert_ne!(
                english_support.action_header,
                ukrainian_support.action_header
            );
            assert_ne!(
                english_support.catalog_label,
                ukrainian_support.catalog_label
            );
            assert_ne!(
                english_support.beginner_brief,
                ukrainian_support.beginner_brief
            );
            assert_ne!(
                english_support.concept_label,
                ukrainian_support.concept_label
            );
            assert_ne!(english_support.explanation, ukrainian_support.explanation);
        }
    }

    #[test]
    fn current_product_collection_and_lookup_are_exact_and_bounded() {
        let packets =
            current_product_exercise_packets().expect("current product collection must resolve");
        let ids = packets
            .iter()
            .map(|packet| packet.id.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(packets.len(), 22);
        assert_eq!(
            ids,
            [
                "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
                "013", "014", "015", "016", "017", "018", "019", "020", "021", "022",
            ]
        );
        assert_eq!(current_product_exercise_packet_at(22), Ok(None));
        assert_eq!(
            current_product_exercise_packet_by_id("999"),
            Err(ProductExerciseError::UnknownId("999".to_owned()))
        );
    }

    #[test]
    fn product_guidance_failure_is_configuration_not_learner_wrong() {
        let error = product_exercise_packet_from_drill(&MISSING_INSTRUCTION_DRILL, 1, 22)
            .expect_err("missing guidance must fail packet construction");

        assert!(matches!(error, ProductExerciseError::Guidance(_)));
        assert_ne!(error.to_string(), "TRY AGAIN");
        assert_eq!(MISSING_INSTRUCTION_DRILL.stimulus, DRILLS[0].stimulus);
        assert_eq!(MISSING_INSTRUCTION_DRILL.expected, DRILLS[0].expected);
    }

    #[test]
    fn interactive_round_plans_select_whole_rows_deterministically() {
        let round_zero = interactive_round_plan(DEFAULT_INTERACTIVE_PLAN_SEED, 0).unwrap();
        let round_one = interactive_round_plan(DEFAULT_INTERACTIVE_PLAN_SEED, 1).unwrap();
        let round_two = interactive_round_plan(DEFAULT_INTERACTIVE_PLAN_SEED, 2).unwrap();
        let alternate = interactive_round_plan(1, 0).unwrap();

        assert_eq!(round_zero.ids, ["013", "014", "015", "016", "017"]);
        assert_eq!(round_one.ids, ["018", "019", "020", "021", "022"]);
        assert_eq!(round_two.ids, round_zero.ids);
        assert_eq!(alternate.ids, round_one.ids);
        assert_eq!(round_zero.row_index, 0);
        assert_eq!(round_one.row_index, 1);
        assert_eq!(round_zero.content_seed, 0);
        assert_eq!(round_one.content_seed, 1);
        assert_eq!(round_zero.guidance_locale, "en");
        assert_eq!(round_one.guidance_locale, "en");
        assert_eq!(
            interactive_round_plan(DEFAULT_INTERACTIVE_PLAN_SEED, 0).unwrap(),
            round_zero
        );
    }

    #[test]
    fn interactive_session_contains_two_resolved_rounds_and_lists_full_catalog() {
        let session = current_interactive_product_session().unwrap();
        assert_eq!(session.total_drills(), 5);
        assert_eq!(session.round_position(), Some((1, 5)));
        assert_eq!(session.current_exercise().unwrap().id, "013");
        assert_eq!(session.product_exercises().len(), 22);
        assert!(!session.is_complete());
    }
}
