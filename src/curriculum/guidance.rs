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

pub(crate) struct GuidancePack {
    pub(crate) locale: &'static str,
    pub(crate) version: u16,
    pub(crate) instructions: &'static [GuidanceTemplate],
    pub(crate) glosses: &'static [GlossEntry],
}

const EN_COPY_EXACT_LINE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Copy this line exactly:")];

const EN_FILL_RETURN_VALUE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Complete the missing return value:")];

const EN_PRODUCE_RETURN_LITERAL_PARTS: &[GuidancePart] = &[
    GuidancePart::Literal("Write a Rust function named "),
    GuidancePart::SlotRendered("NAME"),
    GuidancePart::Literal(" with no parameters that returns the "),
    GuidancePart::SlotRendered("TYPE"),
    GuidancePart::Literal(" value "),
    GuidancePart::SlotRendered("VALUE"),
    GuidancePart::Literal(" using a tail expression:"),
];

const EN_REPAIR_RETURN_TYPE_ARROW_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Repair the return-type syntax:")];

const EN_TRANSFORM_TAIL_EXPRESSION_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Rewrite the function to use a tail expression:",
)];

const EN_INSTRUCTIONS: &[GuidanceTemplate] = &[
    GuidanceTemplate {
        key: "copy.exact_line",
        parts: EN_COPY_EXACT_LINE_PARTS,
    },
    GuidanceTemplate {
        key: "fill.return_value",
        parts: EN_FILL_RETURN_VALUE_PARTS,
    },
    GuidanceTemplate {
        key: "produce.return_literal",
        parts: EN_PRODUCE_RETURN_LITERAL_PARTS,
    },
    GuidanceTemplate {
        key: "repair.return_type_arrow",
        parts: EN_REPAIR_RETURN_TYPE_ARROW_PARTS,
    },
    GuidanceTemplate {
        key: "transform.tail_expression",
        parts: EN_TRANSFORM_TAIL_EXPRESSION_PARTS,
    },
];

// Proof-only Ukrainian source; human linguistic/product review is required before product admission.
const UK_COPY_EXACT_LINE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Скопіюйте цей рядок без змін:")];

const UK_FILL_RETURN_VALUE_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Заповніть пропущене значення, що повертається:",
)];

const UK_PRODUCE_RETURN_LITERAL_PARTS: &[GuidancePart] = &[
    GuidancePart::Literal("Напишіть функцію Rust з назвою "),
    GuidancePart::SlotRendered("NAME"),
    GuidancePart::Literal(" без параметрів, яка повертає значення "),
    GuidancePart::SlotRendered("VALUE"),
    GuidancePart::Literal(" типу "),
    GuidancePart::SlotRendered("TYPE"),
    GuidancePart::Literal(" за допомогою хвостового виразу:"),
];

const UK_REPAIR_RETURN_TYPE_ARROW_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Виправте синтаксис типу повернення:")];

const UK_TRANSFORM_TAIL_EXPRESSION_PARTS: &[GuidancePart] = &[GuidancePart::Literal(
    "Перепишіть функцію, використовуючи хвостовий вираз:",
)];

const UK_INSTRUCTIONS: &[GuidanceTemplate] = &[
    GuidanceTemplate {
        key: "copy.exact_line",
        parts: UK_COPY_EXACT_LINE_PARTS,
    },
    GuidanceTemplate {
        key: "fill.return_value",
        parts: UK_FILL_RETURN_VALUE_PARTS,
    },
    GuidanceTemplate {
        key: "produce.return_literal",
        parts: UK_PRODUCE_RETURN_LITERAL_PARTS,
    },
    GuidanceTemplate {
        key: "repair.return_type_arrow",
        parts: UK_REPAIR_RETURN_TYPE_ARROW_PARTS,
    },
    GuidanceTemplate {
        key: "transform.tail_expression",
        parts: UK_TRANSFORM_TAIL_EXPRESSION_PARTS,
    },
];

pub(crate) const GUIDANCE_PACKS: &[GuidancePack] = &[
    GuidancePack {
        locale: "en",
        version: 1,
        instructions: EN_INSTRUCTIONS,
        glosses: &[],
    },
    GuidancePack {
        locale: "uk",
        version: 1,
        instructions: UK_INSTRUCTIONS,
        glosses: &[],
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

            assert!(pack.glosses.is_empty());

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
            "fill.return_value",
            "produce.return_literal",
            "repair.return_type_arrow",
            "transform.tail_expression",
        ];
        for pack in GUIDANCE_PACKS {
            assert_eq!(pack.instructions.len(), 5);
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
        assert_eq!(
            english.instructions[1].parts,
            &[GuidancePart::Literal("Complete the missing return value:")]
        );
        assert_eq!(
            english.instructions[2].parts,
            &[
                GuidancePart::Literal("Write a Rust function named "),
                GuidancePart::SlotRendered("NAME"),
                GuidancePart::Literal(" with no parameters that returns the "),
                GuidancePart::SlotRendered("TYPE"),
                GuidancePart::Literal(" value "),
                GuidancePart::SlotRendered("VALUE"),
                GuidancePart::Literal(" using a tail expression:"),
            ]
        );
        assert_eq!(
            english.instructions[3].parts,
            &[GuidancePart::Literal("Repair the return-type syntax:")]
        );
        assert_eq!(
            english.instructions[4].parts,
            &[GuidancePart::Literal(
                "Rewrite the function to use a tail expression:"
            )]
        );

        let ukrainian = &GUIDANCE_PACKS[1];
        assert_eq!(
            ukrainian.instructions[0].parts,
            &[GuidancePart::Literal("Скопіюйте цей рядок без змін:")]
        );
        assert_eq!(
            ukrainian.instructions[1].parts,
            &[GuidancePart::Literal(
                "Заповніть пропущене значення, що повертається:"
            )]
        );
        assert_eq!(
            ukrainian.instructions[2].parts,
            &[
                GuidancePart::Literal("Напишіть функцію Rust з назвою "),
                GuidancePart::SlotRendered("NAME"),
                GuidancePart::Literal(" без параметрів, яка повертає значення "),
                GuidancePart::SlotRendered("VALUE"),
                GuidancePart::Literal(" типу "),
                GuidancePart::SlotRendered("TYPE"),
                GuidancePart::Literal(" за допомогою хвостового виразу:"),
            ]
        );
        assert_eq!(
            ukrainian.instructions[3].parts,
            &[GuidancePart::Literal("Виправте синтаксис типу повернення:")]
        );
        assert_eq!(
            ukrainian.instructions[4].parts,
            &[GuidancePart::Literal(
                "Перепишіть функцію, використовуючи хвостовий вираз:"
            )]
        );
    }
}
