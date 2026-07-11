use crate::curriculum::guidance::GUIDANCE_PACKS;
use crate::curriculum::guidance::resolution::{
    GuidanceAssignmentRef, GuidanceError, GuidanceProvenance, resolve_guidance,
};
use crate::curriculum::{DrillSpec, GeneratedTaskProvenanceSpec, RepMetadata};

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
}
