use std::borrow::Cow;
use std::fmt;

use crate::catalog;
use crate::curriculum::guidance::GUIDANCE_PACKS;
use crate::curriculum::guidance::resolution::{
    GuidanceAssignmentRef, GuidanceError, GuidanceProvenance, resolve_guidance,
};
use crate::curriculum::{DrillSpec, GeneratedTaskProvenanceSpec, RepMetadata};
use crate::engine::{
    ExerciseAnswerRule, ExerciseKind, ExercisePacket, ExerciseValidator, FormatCapability, Session,
};

pub(crate) const INITIAL_PRODUCT_GUIDANCE_LOCALE: &str = "en";
pub(crate) const INITIAL_PRODUCT_GUIDANCE_PACK_VERSION: u16 = 1;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResolvedCoreTask {
    pub(crate) rep_id: &'static str,
    pub(crate) canonical_id: &'static str,
    pub(crate) capability_id: Option<&'static str>,
    pub(crate) generated_provenance: Option<GeneratedTaskProvenanceSpec>,
    pub(crate) instruction: String,
    pub(crate) stimulus: &'static str,
    pub(crate) expected: &'static str,
    pub(crate) metadata: RepMetadata,
    pub(crate) guidance_provenance: GuidanceProvenance,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ResolveCoreTaskError {
    Guidance(GuidanceError),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ProductExerciseError {
    UnknownId(String),
    Guidance(ResolveCoreTaskError),
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
        }
    }
}

impl From<ResolveCoreTaskError> for ProductExerciseError {
    fn from(error: ResolveCoreTaskError) -> Self {
        Self::Guidance(error)
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

    Ok(ResolvedCoreTask {
        rep_id: drill.id,
        canonical_id: drill.metadata.canonical_id,
        capability_id: drill.capability_id,
        generated_provenance: drill.generated_provenance,
        instruction: resolved_guidance.instruction,
        stimulus: drill.stimulus,
        expected: drill.expected,
        metadata: drill.metadata,
        guidance_provenance: resolved_guidance.provenance,
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
        metadata,
        guidance_provenance,
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

    Ok(ExercisePacket::new(
        Cow::Borrowed(rep_id),
        position,
        total,
        Cow::Owned(instruction),
        Cow::Borrowed(stimulus),
        Cow::Borrowed(expected),
        FormatCapability::Disabled,
        ExerciseKind::from_task_type(drill.task_type),
        ExerciseValidator::from_validator(drill.validator),
        ExerciseAnswerRule::from_answer_rule(metadata.answer_rule),
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::DRILLS;

    static MISSING_INSTRUCTION_DRILL: DrillSpec = DrillSpec {
        instruction_key: "missing.instruction",
        ..DRILLS[0]
    };

    #[test]
    fn every_current_rep_resolves_exact_english_task_truth() {
        assert_eq!(DRILLS.len(), 12);

        for drill in DRILLS {
            let resolved = resolve_core_task(
                drill,
                INITIAL_PRODUCT_GUIDANCE_LOCALE,
                INITIAL_PRODUCT_GUIDANCE_PACK_VERSION,
            )
            .expect("every current Copy rep must resolve exact English guidance");

            assert_eq!(resolved.rep_id, drill.id);
            assert_eq!(resolved.canonical_id, drill.metadata.canonical_id);
            assert_eq!(resolved.capability_id, None);
            assert_eq!(resolved.generated_provenance, None);
            assert_eq!(resolved.instruction, "Copy this line exactly:");
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
                "copy.exact_line"
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
            metadata,
            guidance_provenance,
        } = resolved;

        assert_eq!(rep_id, drill.id);
        assert_eq!(canonical_id, drill.metadata.canonical_id);
        assert_eq!(capability_id, None);
        assert_eq!(generated_provenance, None);
        assert_eq!(instruction, "Copy this line exactly:");
        assert_eq!(stimulus, drill.stimulus);
        assert_eq!(expected, drill.expected);
        assert_eq!(metadata, drill.metadata);
        assert_eq!(guidance_provenance.core_task_id, canonical_id);
    }

    #[test]
    fn every_current_product_packet_preserves_resolved_catalog_truth() {
        assert_eq!(DRILLS.len(), 12);

        for (index, drill) in DRILLS.iter().enumerate() {
            let packet = current_product_exercise_packet_at(index)
                .expect("current product packet must resolve")
                .expect("current product index must exist");

            assert_eq!(packet.id, drill.id);
            assert_eq!(packet.position, index + 1);
            assert_eq!(packet.total, DRILLS.len());
            assert_eq!(packet.instruction, "Copy this line exactly:");
            assert_eq!(packet.stimulus, drill.stimulus);
            assert_eq!(packet.expected_answer(), drill.expected);
            assert_eq!(packet.format_capability, FormatCapability::Disabled);
            assert_eq!(
                packet.answer_rule,
                ExerciseAnswerRule::from_answer_rule(drill.metadata.answer_rule)
            );
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

        assert_eq!(packets.len(), 12);
        assert_eq!(
            ids,
            [
                "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
            ]
        );
        assert!(
            ["013", "014", "015", "016", "017"]
                .iter()
                .all(|id| !ids.contains(id))
        );
        assert_eq!(current_product_exercise_packet_at(12), Ok(None));
        assert_eq!(
            current_product_exercise_packet_by_id("999"),
            Err(ProductExerciseError::UnknownId("999".to_owned()))
        );
    }

    #[test]
    fn product_guidance_failure_is_configuration_not_learner_wrong() {
        let error = product_exercise_packet_from_drill(&MISSING_INSTRUCTION_DRILL, 1, 12)
            .expect_err("missing guidance must fail packet construction");

        assert!(matches!(error, ProductExerciseError::Guidance(_)));
        assert_ne!(error.to_string(), "TRY AGAIN");
        assert_eq!(MISSING_INSTRUCTION_DRILL.stimulus, DRILLS[0].stimulus);
        assert_eq!(MISSING_INSTRUCTION_DRILL.expected, DRILLS[0].expected);
    }
}
