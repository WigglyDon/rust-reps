pub(crate) mod resolution;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum GuidancePart {
    Literal(&'static str),
    SlotRendered(&'static str),
    SlotGloss(&'static str),
}

pub(crate) struct GuidanceTemplate {
    pub(crate) key: &'static str,
    pub(crate) parts: &'static [GuidancePart],
}

pub(crate) struct GlossEntry {
    pub(crate) meaning_key: &'static str,
    pub(crate) text: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct SupportGuidance {
    pub(crate) key: &'static str,
    pub(crate) concept_label: &'static str,
    pub(crate) explanation: &'static str,
    pub(crate) worked_example: Option<&'static str>,
}

pub(crate) struct GuidancePack {
    pub(crate) locale: &'static str,
    pub(crate) version: u16,
    pub(crate) instructions: &'static [GuidanceTemplate],
    pub(crate) glosses: &'static [GlossEntry],
    pub(crate) support: &'static [SupportGuidance],
}

const EN_COPY_EXACT_LINE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Copy this line exactly:")];

const EN_COPY_EXACT_FUNCTION_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Copy this line exactly: Type the complete function exactly.",
)];

const EN_FILL_RETURN_VALUE_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Complete the missing return-type arrow: Type only the code replacing ___.",
)];

const EN_PRODUCE_RETURN_LITERAL_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Use the shown parts to write the function with its return type: Submit the complete function.",
)];

const EN_REPAIR_RETURN_TYPE_ARROW_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Repair the return-type syntax in the highlighted region: Submit the complete corrected function.",
)];

const EN_TRANSFORM_TAIL_EXPRESSION_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Add the declared return type after the parameter list: Submit the complete transformed function.",
)];

const EN_INSTRUCTIONS: &[GuidanceTemplate] = &[
    GuidanceTemplate {
        key: "copy.exact_line",
        parts: EN_COPY_EXACT_LINE_PARTS,
    },
    GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: EN_COPY_EXACT_FUNCTION_PARTS,
    },
    GuidanceTemplate {
        key: "fill.return_type_arrow",
        parts: EN_FILL_RETURN_VALUE_PARTS,
    },
    GuidanceTemplate {
        key: "produce.return_type_arrow",
        parts: EN_PRODUCE_RETURN_LITERAL_PARTS,
    },
    GuidanceTemplate {
        key: "repair.return_type_arrow",
        parts: EN_REPAIR_RETURN_TYPE_ARROW_PARTS,
    },
    GuidanceTemplate {
        key: "transform.add_return_type",
        parts: EN_TRANSFORM_TAIL_EXPRESSION_PARTS,
    },
];

const EN_SUPPORT: &[SupportGuidance] = &[SupportGuidance {
    key: "rust.function.return_literal.return_type_arrow",
    concept_label: "Function return type",
    explanation: "A return type says what kind of value a function gives back. Rust puts it after () with an arrow.",
    worked_example: Some("fn sample() -> u8 { 1 }"),
}];

// Proof-only Ukrainian source; human linguistic/product review is required before product admission.
const UK_COPY_EXACT_LINE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Скопіюйте цей рядок без змін:")];

const UK_COPY_EXACT_FUNCTION_PARTS: &[GuidancePart] = UK_COPY_EXACT_LINE_PARTS;

const UK_FILL_RETURN_VALUE_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Заповніть пропущену стрілку типу повернення. Введіть лише код замість ___.",
)];

const UK_PRODUCE_RETURN_LITERAL_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "За показаними частинами напишіть повну функцію з типом повернення.",
)];

const UK_REPAIR_RETURN_TYPE_ARROW_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Виправте синтаксис типу повернення у виділеній ділянці та надішліть повну виправлену функцію.",
)];

const UK_TRANSFORM_TAIL_EXPRESSION_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Додайте оголошений тип повернення після списку параметрів і надішліть повну змінену функцію.",
)];

const UK_INSTRUCTIONS: &[GuidanceTemplate] = &[
    GuidanceTemplate {
        key: "copy.exact_line",
        parts: UK_COPY_EXACT_LINE_PARTS,
    },
    GuidanceTemplate {
        key: "copy.return_type_intro",
        parts: UK_COPY_EXACT_FUNCTION_PARTS,
    },
    GuidanceTemplate {
        key: "fill.return_type_arrow",
        parts: UK_FILL_RETURN_VALUE_PARTS,
    },
    GuidanceTemplate {
        key: "produce.return_type_arrow",
        parts: UK_PRODUCE_RETURN_LITERAL_PARTS,
    },
    GuidanceTemplate {
        key: "repair.return_type_arrow",
        parts: UK_REPAIR_RETURN_TYPE_ARROW_PARTS,
    },
    GuidanceTemplate {
        key: "transform.add_return_type",
        parts: UK_TRANSFORM_TAIL_EXPRESSION_PARTS,
    },
];

// Proof-only Ukrainian support; human linguistic/product review is required before product admission.
const UK_SUPPORT: &[SupportGuidance] = &[SupportGuidance {
    key: "rust.function.return_literal.return_type_arrow",
    concept_label: "Тип повернення функції",
    explanation: "Функція може оголосити тип значення після списку параметрів. У Rust стрілка веде від списку параметрів до цього типу.",
    worked_example: Some("fn sample() -> u8 { 1 }"),
}];

pub(crate) const GUIDANCE_PACKS: &[GuidancePack] = &[
    GuidancePack {
        locale: "en",
        version: 1,
        instructions: EN_INSTRUCTIONS,
        glosses: &[],
        support: EN_SUPPORT,
    },
    GuidancePack {
        locale: "uk",
        version: 1,
        instructions: UK_INSTRUCTIONS,
        glosses: &[],
        support: UK_SUPPORT,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::procgen::TEMPLATES;

    #[test]
    fn guidance_source_invariants() {
        for (pack_index, pack) in GUIDANCE_PACKS.iter().enumerate() {
            assert!(!pack.locale.is_empty(), "guidance locale must not be empty");
            assert!(pack.version > 0, "guidance pack version must be positive");
            for earlier in &GUIDANCE_PACKS[..pack_index] {
                assert_ne!(
                    (earlier.locale, earlier.version),
                    (pack.locale, pack.version),
                    "guidance (locale, version) keys must be unique"
                );
            }

            for (instruction_index, instruction) in pack.instructions.iter().enumerate() {
                assert!(
                    !instruction.key.is_empty(),
                    "guidance instruction key must not be empty"
                );
                assert!(
                    !instruction.parts.is_empty(),
                    "guidance instruction parts must not be empty"
                );
                for earlier in &pack.instructions[..instruction_index] {
                    assert_ne!(
                        earlier.key, instruction.key,
                        "instruction keys must be unique within a guidance pack"
                    );
                }

                for part in instruction.parts {
                    match part {
                        GuidancePart::Literal(text) => {
                            assert!(!text.is_empty(), "guidance literal must not be empty");
                            assert!(
                                !text.ends_with('\n') && !text.ends_with('\r'),
                                "guidance literal must not add a final line ending"
                            );
                        }
                        GuidancePart::SlotRendered(name) | GuidancePart::SlotGloss(name) => {
                            assert!(!name.is_empty(), "guidance slot name must not be empty");
                            let matching_templates = TEMPLATES
                                .iter()
                                .filter(|template| template.instruction_key == instruction.key)
                                .collect::<Vec<_>>();
                            assert_eq!(
                                matching_templates.len(),
                                1,
                                "each guidance key must name one current template"
                            );
                            assert!(
                                matching_templates[0]
                                    .slots
                                    .iter()
                                    .any(|slot| slot.name == *name),
                                "guidance slot {name} must be declared by its template"
                            );
                        }
                    }
                }
            }

            for (gloss_index, gloss) in pack.glosses.iter().enumerate() {
                assert!(
                    !gloss.meaning_key.is_empty(),
                    "guidance gloss meaning key must not be empty"
                );
                assert!(
                    !gloss.text.is_empty(),
                    "guidance gloss text must not be empty"
                );
                for earlier in &pack.glosses[..gloss_index] {
                    assert_ne!(
                        earlier.meaning_key, gloss.meaning_key,
                        "gloss keys must be unique within a guidance pack"
                    );
                }
            }

            for (support_index, support) in pack.support.iter().enumerate() {
                assert!(!support.key.is_empty(), "support key must not be empty");
                assert!(
                    !support.concept_label.is_empty(),
                    "support concept label must not be empty"
                );
                assert!(
                    !support.explanation.is_empty(),
                    "support explanation must not be empty"
                );
                for earlier in &pack.support[..support_index] {
                    assert_ne!(
                        earlier.key, support.key,
                        "support keys must be unique within a guidance pack"
                    );
                }
            }

            assert!(pack.glosses.is_empty());
            assert_eq!(pack.support.len(), 1);

            for template in TEMPLATES {
                assert_eq!(
                    pack.instructions
                        .iter()
                        .filter(|instruction| instruction.key == template.instruction_key)
                        .count(),
                    1,
                    "pack {} must contain template key {} exactly once",
                    pack.locale,
                    template.instruction_key
                );
            }
        }
    }

    #[test]
    fn proof_packs_are_exact_ordered_and_cover_current_templates() {
        assert_eq!(GUIDANCE_PACKS.len(), 2);
        assert_eq!(
            GUIDANCE_PACKS
                .iter()
                .map(|pack| (pack.locale, pack.version))
                .collect::<Vec<_>>(),
            vec![("en", 1), ("uk", 1)]
        );

        let keys = [
            "copy.exact_line",
            "copy.return_type_intro",
            "fill.return_type_arrow",
            "produce.return_type_arrow",
            "repair.return_type_arrow",
            "transform.add_return_type",
        ];
        for pack in GUIDANCE_PACKS {
            assert_eq!(pack.instructions.len(), 6);
            assert_eq!(
                pack.instructions
                    .iter()
                    .map(|instruction| instruction.key)
                    .collect::<Vec<_>>(),
                keys
            );
            assert!(pack.glosses.is_empty());
        }

        let english = &GUIDANCE_PACKS[0];
        assert_eq!(
            english.instructions[0].parts,
            &[GuidancePart::Literal("Copy this line exactly:")]
        );
        assert_eq!(english.instructions[1].parts, EN_COPY_EXACT_FUNCTION_PARTS);
        assert_eq!(english.instructions[2].parts, EN_FILL_RETURN_VALUE_PARTS);
        assert_eq!(
            english.instructions[3].parts,
            EN_PRODUCE_RETURN_LITERAL_PARTS
        );
        assert_eq!(
            english.instructions[4].parts,
            EN_REPAIR_RETURN_TYPE_ARROW_PARTS
        );
        assert_eq!(
            english.instructions[5].parts,
            EN_TRANSFORM_TAIL_EXPRESSION_PARTS
        );
        assert_eq!(english.support, EN_SUPPORT);

        let ukrainian = &GUIDANCE_PACKS[1];
        assert_eq!(
            ukrainian.instructions[0].parts,
            &[GuidancePart::Literal("Скопіюйте цей рядок без змін:")]
        );
        assert_eq!(
            ukrainian.instructions[1].parts,
            UK_COPY_EXACT_FUNCTION_PARTS
        );
        assert_eq!(ukrainian.instructions[2].parts, UK_FILL_RETURN_VALUE_PARTS);
        assert_eq!(
            ukrainian.instructions[3].parts,
            UK_PRODUCE_RETURN_LITERAL_PARTS
        );
        assert_eq!(
            ukrainian.instructions[4].parts,
            UK_REPAIR_RETURN_TYPE_ARROW_PARTS
        );
        assert_eq!(
            ukrainian.instructions[5].parts,
            UK_TRANSFORM_TAIL_EXPRESSION_PARTS
        );
        assert_eq!(ukrainian.support, UK_SUPPORT);
    }
}
