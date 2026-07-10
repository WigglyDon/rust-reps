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

const EN_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
    key: "copy.exact_line",
    parts: EN_COPY_EXACT_LINE_PARTS,
}];

// Proof-only Ukrainian source; human linguistic/product review is required before product admission.
const UK_COPY_EXACT_LINE_PARTS: &[GuidancePart] =
    &[GuidancePart::Literal("Скопіюйте цей рядок без змін:")];

const UK_INSTRUCTIONS: &[GuidanceTemplate] = &[GuidanceTemplate {
    key: "copy.exact_line",
    parts: UK_COPY_EXACT_LINE_PARTS,
}];

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
                            assert!(!text.is_empty(), "guidance literal must not be empty")
                        }
                        GuidancePart::SlotRendered(name) | GuidancePart::SlotGloss(name) => {
                            assert!(!name.is_empty(), "guidance slot name must not be empty")
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
        }
    }

    #[test]
    fn english_copy_guidance_is_exact_and_covers_current_templates() {
        assert_eq!(GUIDANCE_PACKS.len(), 2);
        let pack = &GUIDANCE_PACKS[0];
        assert_eq!((pack.locale, pack.version), ("en", 1));
        assert_eq!(pack.instructions.len(), 1);
        assert_eq!(pack.instructions[0].key, "copy.exact_line");
        assert_eq!(
            pack.instructions[0].parts,
            &[GuidancePart::Literal("Copy this line exactly:")]
        );
        assert!(pack.glosses.is_empty());

        for part in [
            GuidancePart::SlotRendered("NAME"),
            GuidancePart::SlotGloss("VALUE"),
        ] {
            match part {
                GuidancePart::SlotRendered(name) | GuidancePart::SlotGloss(name) => {
                    assert!(!name.is_empty());
                }
                GuidancePart::Literal(_) => unreachable!(),
            }
        }

        for template in TEMPLATES {
            assert!(
                pack.instructions
                    .iter()
                    .any(|instruction| instruction.key == template.instruction_key),
                "English guidance must contain template instruction key {}",
                template.instruction_key
            );
        }
    }

    #[test]
    fn proof_packs_are_exact_ordered_and_cover_current_templates() {
        assert_eq!(GUIDANCE_PACKS.len(), 2);

        for (pack, locale, exact_text) in [
            (&GUIDANCE_PACKS[0], "en", "Copy this line exactly:"),
            (&GUIDANCE_PACKS[1], "uk", "Скопіюйте цей рядок без змін:"),
        ] {
            assert_eq!((pack.locale, pack.version), (locale, 1));
            assert_eq!(pack.instructions.len(), 1);
            assert_eq!(pack.instructions[0].key, "copy.exact_line");
            assert_eq!(
                pack.instructions[0].parts,
                &[GuidancePart::Literal(exact_text)]
            );
            assert!(!exact_text.ends_with('\n'));
            assert!(!exact_text.ends_with('\r'));
            assert!(pack.glosses.is_empty());

            for template in TEMPLATES {
                assert!(
                    pack.instructions
                        .iter()
                        .any(|instruction| instruction.key == template.instruction_key),
                    "proof pack {locale} must contain template instruction key {}",
                    template.instruction_key
                );
            }
        }
    }
}
