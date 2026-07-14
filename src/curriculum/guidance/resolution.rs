use super::{GuidancePack, GuidancePart};
use crate::curriculum::Operation;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct GuidanceAssignmentRef<'a> {
    pub(crate) slot: &'a str,
    pub(crate) rendered: &'a str,
    pub(crate) meaning_key: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct GuidanceProvenance {
    pub(crate) core_task_id: String,
    pub(crate) locale: String,
    pub(crate) pack_version: u16,
    pub(crate) instruction_key: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResolvedGuidance {
    pub(crate) provenance: GuidanceProvenance,
    pub(crate) instruction: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ResolvedSupportGuidance {
    pub(crate) action_header: String,
    pub(crate) operation_label: String,
    pub(crate) artifact_label: String,
    pub(crate) concept_label: String,
    pub(crate) catalog_label: String,
    pub(crate) beginner_brief: Vec<String>,
    pub(crate) explanation: String,
    pub(crate) worked_example: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum GuidanceError {
    UnknownPack {
        locale: String,
        version: u16,
    },
    DuplicatePack {
        locale: String,
        version: u16,
    },
    UnknownInstruction {
        locale: String,
        version: u16,
        key: String,
    },
    DuplicateInstruction {
        locale: String,
        version: u16,
        key: String,
    },
    UnknownSupport {
        locale: String,
        version: u16,
        key: String,
    },
    DuplicateSupport {
        locale: String,
        version: u16,
        key: String,
    },
    UnknownOperationGuidance {
        locale: String,
        version: u16,
        operation: Operation,
    },
    DuplicateOperationGuidance {
        locale: String,
        version: u16,
        operation: Operation,
    },
    SupportArtifactMismatch {
        key: String,
        expected: String,
        actual: String,
    },
    DuplicateAssignment(String),
    UnknownSlot(String),
    MissingMeaningKey(String),
    UnknownGloss {
        locale: String,
        version: u16,
        meaning_key: String,
    },
    DuplicateGloss {
        locale: String,
        version: u16,
        meaning_key: String,
    },
}

pub(crate) fn resolve_support_guidance(
    operation: Operation,
    artifact_key: &str,
    support_key: &str,
    packs: &[GuidancePack],
    locale: &str,
    pack_version: u16,
) -> Result<ResolvedSupportGuidance, GuidanceError> {
    let pack_index = find_pack_index(packs, locale, pack_version)?;
    let pack = &packs[pack_index];
    let mut operation_index = None;
    for (index, operation_guidance) in pack.operations.iter().enumerate() {
        if operation_guidance.operation == operation {
            if operation_index.is_some() {
                return Err(GuidanceError::DuplicateOperationGuidance {
                    locale: pack.locale.to_owned(),
                    version: pack.version,
                    operation,
                });
            }
            operation_index = Some(index);
        }
    }
    let operation_guidance = operation_index
        .map(|index| &pack.operations[index])
        .ok_or_else(|| GuidanceError::UnknownOperationGuidance {
            locale: pack.locale.to_owned(),
            version: pack.version,
            operation,
        })?;
    let mut support_index = None;
    for (index, support) in pack.support.iter().enumerate() {
        if support.key == support_key {
            if support_index.is_some() {
                return Err(GuidanceError::DuplicateSupport {
                    locale: pack.locale.to_owned(),
                    version: pack.version,
                    key: support_key.to_owned(),
                });
            }
            support_index = Some(index);
        }
    }
    let support = support_index
        .map(|index| &pack.support[index])
        .ok_or_else(|| GuidanceError::UnknownSupport {
            locale: pack.locale.to_owned(),
            version: pack.version,
            key: support_key.to_owned(),
        })?;
    if support.artifact_key != artifact_key {
        return Err(GuidanceError::SupportArtifactMismatch {
            key: support_key.to_owned(),
            expected: support.artifact_key.to_owned(),
            actual: artifact_key.to_owned(),
        });
    }

    let catalog_label = format!(
        "{} · {} · {}",
        operation_guidance.operation_label, support.artifact_label, support.concept_label
    );

    Ok(ResolvedSupportGuidance {
        action_header: operation_guidance.action_header.to_owned(),
        operation_label: operation_guidance.operation_label.to_owned(),
        artifact_label: support.artifact_label.to_owned(),
        concept_label: support.concept_label.to_owned(),
        catalog_label,
        beginner_brief: support
            .beginner_brief
            .iter()
            .map(|line| (*line).to_owned())
            .collect(),
        explanation: support.explanation.to_owned(),
        worked_example: support.worked_example.map(str::to_owned),
    })
}

pub(crate) fn resolve_guidance(
    core_task_id: &str,
    instruction_key: &str,
    assignments: &[GuidanceAssignmentRef<'_>],
    packs: &[GuidancePack],
    locale: &str,
    pack_version: u16,
) -> Result<ResolvedGuidance, GuidanceError> {
    let pack_index = find_pack_index(packs, locale, pack_version)?;
    let pack = &packs[pack_index];
    let instruction_index = find_instruction_index(pack, instruction_key)?;
    validate_assignments(assignments)?;
    let instruction = render_guidance_parts(
        pack,
        pack.instructions[instruction_index].parts,
        assignments,
    )?;

    Ok(ResolvedGuidance {
        provenance: GuidanceProvenance {
            core_task_id: core_task_id.to_owned(),
            locale: pack.locale.to_owned(),
            pack_version: pack.version,
            instruction_key: instruction_key.to_owned(),
        },
        instruction,
    })
}

fn find_pack_index(
    packs: &[GuidancePack],
    locale: &str,
    version: u16,
) -> Result<usize, GuidanceError> {
    let mut match_index = None;
    for (index, pack) in packs.iter().enumerate() {
        if pack.locale == locale && pack.version == version {
            if match_index.is_some() {
                return Err(GuidanceError::DuplicatePack {
                    locale: locale.to_owned(),
                    version,
                });
            }
            match_index = Some(index);
        }
    }

    match_index.ok_or_else(|| GuidanceError::UnknownPack {
        locale: locale.to_owned(),
        version,
    })
}

fn find_instruction_index(pack: &GuidancePack, key: &str) -> Result<usize, GuidanceError> {
    let mut match_index = None;
    for (index, instruction) in pack.instructions.iter().enumerate() {
        if instruction.key == key {
            if match_index.is_some() {
                return Err(GuidanceError::DuplicateInstruction {
                    locale: pack.locale.to_owned(),
                    version: pack.version,
                    key: key.to_owned(),
                });
            }
            match_index = Some(index);
        }
    }

    match_index.ok_or_else(|| GuidanceError::UnknownInstruction {
        locale: pack.locale.to_owned(),
        version: pack.version,
        key: key.to_owned(),
    })
}

fn validate_assignments(assignments: &[GuidanceAssignmentRef<'_>]) -> Result<(), GuidanceError> {
    for (index, assignment) in assignments.iter().enumerate() {
        for earlier in &assignments[..index] {
            if earlier.slot == assignment.slot {
                return Err(GuidanceError::DuplicateAssignment(
                    assignment.slot.to_owned(),
                ));
            }
        }
    }
    Ok(())
}

fn find_assignment_index(
    assignments: &[GuidanceAssignmentRef<'_>],
    slot: &str,
) -> Result<usize, GuidanceError> {
    assignments
        .iter()
        .position(|assignment| assignment.slot == slot)
        .ok_or_else(|| GuidanceError::UnknownSlot(slot.to_owned()))
}

fn find_gloss_index(pack: &GuidancePack, meaning_key: &str) -> Result<usize, GuidanceError> {
    let mut match_index = None;
    for (index, gloss) in pack.glosses.iter().enumerate() {
        if gloss.meaning_key == meaning_key {
            if match_index.is_some() {
                return Err(GuidanceError::DuplicateGloss {
                    locale: pack.locale.to_owned(),
                    version: pack.version,
                    meaning_key: meaning_key.to_owned(),
                });
            }
            match_index = Some(index);
        }
    }

    match_index.ok_or_else(|| GuidanceError::UnknownGloss {
        locale: pack.locale.to_owned(),
        version: pack.version,
        meaning_key: meaning_key.to_owned(),
    })
}

fn render_guidance_parts(
    pack: &GuidancePack,
    parts: &[GuidancePart],
    assignments: &[GuidanceAssignmentRef<'_>],
) -> Result<String, GuidanceError> {
    let mut output = String::new();
    for part in parts {
        match part {
            GuidancePart::Literal(text) => output.push_str(text),
            GuidancePart::SlotRendered(slot) => {
                let assignment_index = find_assignment_index(assignments, slot)?;
                output.push_str(&assignments[assignment_index].rendered);
            }
            GuidancePart::SlotGloss(slot) => {
                let assignment_index = find_assignment_index(assignments, slot)?;
                let assignment = &assignments[assignment_index];
                let Some(meaning_key) = assignment.meaning_key else {
                    return Err(GuidanceError::MissingMeaningKey((*slot).to_owned()));
                };
                let gloss_index = find_gloss_index(pack, meaning_key)?;
                output.push_str(pack.glosses[gloss_index].text);
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::guidance::{
        GUIDANCE_PACKS, GlossEntry, GuidanceTemplate, OperationGuidance, SupportGuidance,
    };
    use crate::curriculum::procgen::expansion::{ExpandedTask, expand_template};
    use crate::curriculum::procgen::{SLOT_DOMAINS, TEMPLATES};

    const BASIC_PARTS: &[GuidancePart] = &[GuidancePart::Literal("test")];
    const BASIC_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: BASIC_PARTS,
    }];
    const BASIC_OPERATIONS: &[OperationGuidance] = &[OperationGuidance {
        operation: Operation::Copy,
        action_header: "DO IT:",
        operation_label: "DO",
    }];
    const BASIC_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: BASIC_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const DUPLICATE_SUPPORT: &[SupportGuidance] = &[
        SupportGuidance {
            key: "test.concept",
            artifact_key: "function",
            artifact_label: "FUNCTION",
            concept_label: "first",
            beginner_brief: &["first brief"],
            explanation: "first explanation",
            worked_example: None,
        },
        SupportGuidance {
            key: "test.concept",
            artifact_key: "function",
            artifact_label: "FUNCTION",
            concept_label: "second",
            beginner_brief: &["second brief"],
            explanation: "second explanation",
            worked_example: None,
        },
    ];

    const DUPLICATE_SUPPORT_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: BASIC_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: DUPLICATE_SUPPORT,
    };

    const DUPLICATE_PACKS: &[GuidancePack] = &[
        GuidancePack {
            locale: "test",
            version: 1,
            instructions: BASIC_INSTRUCTIONS,
            glosses: &[],
            operations: BASIC_OPERATIONS,
            support: &[],
        },
        GuidancePack {
            locale: "test",
            version: 1,
            instructions: BASIC_INSTRUCTIONS,
            glosses: &[],
            operations: BASIC_OPERATIONS,
            support: &[],
        },
    ];

    const DUPLICATE_INSTRUCTIONS: &[GuidanceTemplate] = &[
        GuidanceTemplate {
            key: "copy.return_type_intro",
            parts: BASIC_PARTS,
        },
        GuidanceTemplate {
            key: "copy.return_type_intro",
            parts: BASIC_PARTS,
        },
    ];
    const DUPLICATE_INSTRUCTION_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: DUPLICATE_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const EXACT_PARTS: &[GuidancePart] = &[
        GuidancePart::Literal("α\t\r\n"),
        GuidancePart::SlotRendered("VALUE"),
        GuidancePart::Literal("|"),
        GuidancePart::SlotRendered("VALUE"),
        GuidancePart::Literal("|"),
        GuidancePart::SlotGloss("VALUE"),
    ];
    const EXACT_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "test.exact",
        parts: EXACT_PARTS,
    }];
    const SINGLE_GLOSS: &[GlossEntry] = &[GlossEntry {
        meaning_key: "number.five",
        text: "five",
    }];
    const EXACT_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: EXACT_INSTRUCTIONS,
        glosses: SINGLE_GLOSS,
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const UNKNOWN_RENDERED_PARTS: &[GuidancePart] = &[GuidancePart::SlotRendered("MISSING")];
    const UNKNOWN_RENDERED_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: UNKNOWN_RENDERED_PARTS,
    }];
    const UNKNOWN_RENDERED_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: UNKNOWN_RENDERED_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const UNKNOWN_GLOSS_SLOT_PARTS: &[GuidancePart] = &[GuidancePart::SlotGloss("MISSING")];
    const UNKNOWN_GLOSS_SLOT_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: UNKNOWN_GLOSS_SLOT_PARTS,
    }];
    const UNKNOWN_GLOSS_SLOT_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: UNKNOWN_GLOSS_SLOT_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const VALUE_GLOSS_PARTS: &[GuidancePart] = &[GuidancePart::SlotGloss("VALUE")];
    const VALUE_GLOSS_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: VALUE_GLOSS_PARTS,
    }];
    const VALUE_GLOSS_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: VALUE_GLOSS_INSTRUCTIONS,
        glosses: &[],
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const DUPLICATE_GLOSSES: &[GlossEntry] = &[
        GlossEntry {
            meaning_key: "number.five",
            text: "five",
        },
        GlossEntry {
            meaning_key: "number.five",
            text: "five again",
        },
    ];
    const DUPLICATE_GLOSS_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: VALUE_GLOSS_INSTRUCTIONS,
        glosses: DUPLICATE_GLOSSES,
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    const REPEATED_PARTS: &[GuidancePart] = &[
        GuidancePart::SlotRendered("VALUE"),
        GuidancePart::Literal("/"),
        GuidancePart::SlotRendered("VALUE"),
        GuidancePart::Literal("/"),
        GuidancePart::SlotGloss("VALUE"),
        GuidancePart::Literal("/"),
        GuidancePart::SlotGloss("VALUE"),
    ];
    const REPEATED_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: REPEATED_PARTS,
    }];
    const REPEATED_PACK: GuidancePack = GuidancePack {
        locale: "test",
        version: 1,
        instructions: REPEATED_INSTRUCTIONS,
        glosses: SINGLE_GLOSS,
        operations: BASIC_OPERATIONS,
        support: &[],
    };

    fn current_task() -> ExpandedTask {
        expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0).expect("accepted core task should expand")
    }

    fn assignment_refs(task: &ExpandedTask) -> Vec<GuidanceAssignmentRef<'_>> {
        task.assignments
            .iter()
            .map(|assignment| GuidanceAssignmentRef {
                slot: &assignment.slot,
                rendered: &assignment.rendered,
                meaning_key: assignment.meaning_key.as_deref(),
            })
            .collect()
    }

    fn resolve_task_guidance(
        task: &ExpandedTask,
        packs: &[GuidancePack],
        locale: &str,
        pack_version: u16,
    ) -> Result<ResolvedGuidance, GuidanceError> {
        let assignments = assignment_refs(task);
        resolve_guidance(
            &task.generated_id,
            &task.instruction_key,
            &assignments,
            packs,
            locale,
            pack_version,
        )
    }

    #[test]
    fn current_task_resolves_in_english_and_ukrainian() {
        let task = current_task();
        let before = format!("{task:?}");

        let english = resolve_task_guidance(&task, GUIDANCE_PACKS, "en", 1)
            .expect("English proof pack should resolve");
        let ukrainian = resolve_task_guidance(&task, GUIDANCE_PACKS, "uk", 1)
            .expect("Ukrainian proof pack should resolve");

        assert_eq!(
            english.instruction,
            "Copy this line exactly: Type the complete function exactly."
        );
        assert_eq!(ukrainian.instruction, "Скопіюйте цей рядок без змін:");
        assert_eq!(
            english.provenance.core_task_id,
            "gen:rust:function.return_literal:copy-return-type:v2:copy:s0"
        );
        assert_eq!(
            english.provenance.core_task_id,
            ukrainian.provenance.core_task_id
        );
        assert_eq!(english.provenance.instruction_key, "copy.return_type_intro");
        assert_eq!(
            english.provenance.instruction_key,
            ukrainian.provenance.instruction_key
        );
        assert_eq!(english.provenance.pack_version, 1);
        assert_eq!(ukrainian.provenance.pack_version, 1);
        assert_eq!(english.provenance.locale, "en");
        assert_eq!(ukrainian.provenance.locale, "uk");
        assert_ne!(english.instruction, ukrainian.instruction);
        assert!(before.contains("content_seed: 0"));
        assert_eq!(before, format!("{task:?}"));
    }

    #[test]
    fn support_guidance_resolves_by_locale_without_touching_symbolic_content() {
        let key = "rust.function.return_literal.return_type_arrow";
        let task = current_task();
        let before = format!("{task:?}");
        let english =
            resolve_support_guidance(Operation::Copy, "function", key, GUIDANCE_PACKS, "en", 1)
                .expect("English support must resolve");
        let ukrainian =
            resolve_support_guidance(Operation::Copy, "function", key, GUIDANCE_PACKS, "uk", 1)
                .expect("Ukrainian proof support must resolve");

        assert_eq!(english.action_header, "COPY EXACTLY:");
        assert_eq!(english.catalog_label, "COPY · FUNCTION · RETURN TYPE");
        assert_eq!(english.beginner_brief.len(), 7);
        assert_ne!(english.action_header, ukrainian.action_header);
        assert_ne!(english.catalog_label, ukrainian.catalog_label);
        assert_ne!(english.concept_label, ukrainian.concept_label);
        assert_ne!(english.explanation, ukrainian.explanation);
        assert_eq!(english.worked_example, ukrainian.worked_example);
        assert_eq!(before, format!("{task:?}"));
    }

    #[test]
    fn support_guidance_failures_are_explicit() {
        assert_eq!(
            resolve_support_guidance(
                Operation::Copy,
                "function",
                "missing",
                GUIDANCE_PACKS,
                "en",
                1,
            ),
            Err(GuidanceError::UnknownSupport {
                locale: "en".to_owned(),
                version: 1,
                key: "missing".to_owned(),
            })
        );
        assert_eq!(
            resolve_support_guidance(
                Operation::Copy,
                "function",
                "test.concept",
                &[DUPLICATE_SUPPORT_PACK],
                "test",
                1,
            ),
            Err(GuidanceError::DuplicateSupport {
                locale: "test".to_owned(),
                version: 1,
                key: "test.concept".to_owned(),
            })
        );
    }

    #[test]
    fn resolution_is_reproducible() {
        let task = current_task();
        assert_eq!(
            resolve_task_guidance(&task, GUIDANCE_PACKS, "en", 1),
            resolve_task_guidance(&task, GUIDANCE_PACKS, "en", 1)
        );
    }

    #[test]
    fn rendering_preserves_exact_bytes_and_repeated_references() {
        let mut task = current_task();
        task.instruction_key = "test.exact".to_owned();
        let value = task
            .assignments
            .iter_mut()
            .find(|assignment| assignment.slot == "VALUE")
            .expect("current task has VALUE");
        value.rendered = "5".to_owned();
        value.meaning_key = Some("number.five".to_owned());

        let resolved = resolve_task_guidance(&task, &[EXACT_PACK], "test", 1)
            .expect("exact rendering fixture should resolve");
        assert_eq!(
            resolved.instruction.as_bytes(),
            "α\t\r\n5|5|five".as_bytes()
        );
        assert!(!resolved.instruction.ends_with('\n'));
    }

    #[test]
    fn pack_failures_are_explicit() {
        let task = current_task();
        assert_eq!(
            resolve_task_guidance(&task, GUIDANCE_PACKS, "missing", 1),
            Err(GuidanceError::UnknownPack {
                locale: "missing".to_owned(),
                version: 1,
            })
        );
        assert_eq!(
            resolve_task_guidance(&task, GUIDANCE_PACKS, "en", 2),
            Err(GuidanceError::UnknownPack {
                locale: "en".to_owned(),
                version: 2,
            })
        );
        assert_eq!(
            resolve_task_guidance(&task, DUPLICATE_PACKS, "test", 1),
            Err(GuidanceError::DuplicatePack {
                locale: "test".to_owned(),
                version: 1,
            })
        );
    }

    #[test]
    fn instruction_failures_are_explicit() {
        let mut task = current_task();
        task.instruction_key = "missing".to_owned();
        assert_eq!(
            resolve_task_guidance(&task, &[BASIC_PACK], "test", 1),
            Err(GuidanceError::UnknownInstruction {
                locale: "test".to_owned(),
                version: 1,
                key: "missing".to_owned(),
            })
        );

        task.instruction_key = "copy.return_type_intro".to_owned();
        assert_eq!(
            resolve_task_guidance(&task, &[DUPLICATE_INSTRUCTION_PACK], "test", 1),
            Err(GuidanceError::DuplicateInstruction {
                locale: "test".to_owned(),
                version: 1,
                key: "copy.return_type_intro".to_owned(),
            })
        );
    }

    #[test]
    fn assignment_failures_are_explicit() {
        let mut duplicate = current_task();
        duplicate.assignments[1].slot = duplicate.assignments[0].slot.clone();
        assert_eq!(
            resolve_task_guidance(&duplicate, &[BASIC_PACK], "test", 1),
            Err(GuidanceError::DuplicateAssignment("NAME".to_owned()))
        );

        assert_eq!(
            resolve_task_guidance(&current_task(), &[UNKNOWN_RENDERED_PACK], "test", 1),
            Err(GuidanceError::UnknownSlot("MISSING".to_owned()))
        );
        assert_eq!(
            resolve_task_guidance(&current_task(), &[UNKNOWN_GLOSS_SLOT_PACK], "test", 1),
            Err(GuidanceError::UnknownSlot("MISSING".to_owned()))
        );
        assert_eq!(
            resolve_task_guidance(&current_task(), &[VALUE_GLOSS_PACK], "test", 1),
            Err(GuidanceError::MissingMeaningKey("VALUE".to_owned()))
        );
    }

    #[test]
    fn gloss_failures_are_explicit() {
        let mut task = current_task();
        let value = task
            .assignments
            .iter_mut()
            .find(|assignment| assignment.slot == "VALUE")
            .expect("current task has VALUE");
        value.meaning_key = Some("number.five".to_owned());

        assert_eq!(
            resolve_task_guidance(&task, &[VALUE_GLOSS_PACK], "test", 1),
            Err(GuidanceError::UnknownGloss {
                locale: "test".to_owned(),
                version: 1,
                meaning_key: "number.five".to_owned(),
            })
        );
        assert_eq!(
            resolve_task_guidance(&task, &[DUPLICATE_GLOSS_PACK], "test", 1),
            Err(GuidanceError::DuplicateGloss {
                locale: "test".to_owned(),
                version: 1,
                meaning_key: "number.five".to_owned(),
            })
        );
    }

    #[test]
    fn repeated_rendered_and_gloss_references_are_allowed() {
        let mut task = current_task();
        let value = task
            .assignments
            .iter_mut()
            .find(|assignment| assignment.slot == "VALUE")
            .expect("current task has VALUE");
        value.meaning_key = Some("number.five".to_owned());

        assert_eq!(
            resolve_task_guidance(&task, &[REPEATED_PACK], "test", 1)
                .expect("repeated references should resolve")
                .instruction,
            "5/5/five/five"
        );
    }

    #[test]
    fn resolved_guidance_has_only_provenance_and_instruction() {
        let resolved = resolve_task_guidance(&current_task(), GUIDANCE_PACKS, "en", 1)
            .expect("English proof pack should resolve");
        let ResolvedGuidance {
            provenance,
            instruction,
        } = resolved;
        let GuidanceProvenance {
            core_task_id,
            locale,
            pack_version,
            instruction_key,
        } = provenance;

        assert_eq!(
            core_task_id,
            "gen:rust:function.return_literal:copy-return-type:v2:copy:s0"
        );
        assert_eq!(locale, "en");
        assert_eq!(pack_version, 1);
        assert_eq!(instruction_key, "copy.return_type_intro");
        assert_eq!(
            instruction,
            "Copy this line exactly: Type the complete function exactly."
        );
    }
}
