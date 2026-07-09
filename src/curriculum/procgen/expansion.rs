use super::{SlotDomain, SlotSpec, TemplatePart, TemplateSpec};
use crate::curriculum::{AnswerRule, Operation, StateTransition};

#[derive(Debug, Eq, PartialEq)]
pub(super) struct SlotAssignment {
    pub(super) slot: String,
    pub(super) domain_id: String,
    pub(super) domain_version: u16,
    pub(super) value_id: String,
    pub(super) rendered: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct ExpandedRep {
    // Inert procgen provenance only. This is not an admitted RepMetadata canonical ID.
    pub(super) generated_id: String,

    pub(super) template_id: String,
    pub(super) template_version: u16,
    pub(super) seed: u64,
    pub(super) prompt_variant: u16,
    pub(super) contrast_id: Option<String>,

    pub(super) assignments: Vec<SlotAssignment>,
    pub(super) prompt: String,
    pub(super) expected: String,

    pub(super) system: String,
    pub(super) track: String,
    pub(super) semantic_frame: String,
    pub(super) surface_frame: String,
    pub(super) operation: Operation,
    pub(super) concepts: Vec<String>,
    pub(super) constraints: Vec<String>,
    pub(super) xp: u32,
    pub(super) answer_rule: AnswerRule,
    pub(super) transition: StateTransition,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum ExpansionError {
    UnknownDomain { id: String, version: u16 },
    DuplicateDomain { id: String, version: u16 },
    EmptyDomain { id: String, version: u16 },
    CardinalityOverflow,
    SeedOutOfRange { seed: u64, cardinality: u64 },
    DuplicateTemplateSlot(String),
    MissingSlot(String),
    DuplicateAssignment(String),
    UnknownSlot(String),
    InvalidIdentityComponent { field: &'static str, value: String },
    InvalidTemplateVersion(u16),
}

pub(super) fn template_cardinality(
    template: &TemplateSpec,
    domains: &[SlotDomain],
) -> Result<u64, ExpansionError> {
    validate_unique_template_slots(template.slots)?;

    let mut cardinality = 1_u64;
    for slot in template.slots {
        let domain_index = find_domain_index(slot, domains)?;
        let domain = &domains[domain_index];
        let radix =
            u64::try_from(domain.values.len()).map_err(|_| ExpansionError::CardinalityOverflow)?;
        cardinality = checked_cardinality_multiply(cardinality, radix)?;
    }

    Ok(cardinality)
}

pub(super) fn expand_template(
    template: &TemplateSpec,
    domains: &[SlotDomain],
    seed: u64,
) -> Result<ExpandedRep, ExpansionError> {
    validate_identity_source(template)?;
    validate_unique_template_slots(template.slots)?;

    let mut domain_indices = Vec::with_capacity(template.slots.len());
    let mut cardinality = 1_u64;
    for slot in template.slots {
        let domain_index = find_domain_index(slot, domains)?;
        let domain = &domains[domain_index];
        let radix =
            u64::try_from(domain.values.len()).map_err(|_| ExpansionError::CardinalityOverflow)?;
        cardinality = checked_cardinality_multiply(cardinality, radix)?;
        domain_indices.push(domain_index);
    }

    if seed >= cardinality {
        return Err(ExpansionError::SeedOutOfRange { seed, cardinality });
    }

    let mut remaining = seed;
    let mut assignments = Vec::with_capacity(template.slots.len());
    for (slot, domain_index) in template.slots.iter().zip(domain_indices) {
        let domain = &domains[domain_index];
        let radix =
            u64::try_from(domain.values.len()).map_err(|_| ExpansionError::CardinalityOverflow)?;
        let value_index = remaining % radix;
        remaining /= radix;
        let value_index =
            usize::try_from(value_index).map_err(|_| ExpansionError::CardinalityOverflow)?;
        let Some(&(value_id, rendered)) = domain.values.get(value_index) else {
            return Err(ExpansionError::CardinalityOverflow);
        };

        assignments.push(SlotAssignment {
            slot: slot.name.to_owned(),
            domain_id: domain.id.to_owned(),
            domain_version: domain.version,
            value_id: value_id.to_owned(),
            rendered: rendered.to_owned(),
        });
    }

    let prompt = render_parts(template.prompt, template.slots, &assignments)?;
    let expected = render_parts(template.expected, template.slots, &assignments)?;
    let generated_id = build_generated_id(template, seed);

    Ok(ExpandedRep {
        generated_id,
        template_id: template.id.to_owned(),
        template_version: template.version,
        seed,
        prompt_variant: 0,
        contrast_id: template.contrast_id.map(str::to_owned),
        assignments,
        prompt,
        expected,
        system: template.system.to_owned(),
        track: template.track.to_owned(),
        semantic_frame: template.semantic_frame.to_owned(),
        surface_frame: template.surface_frame.to_owned(),
        operation: template.operation,
        concepts: template
            .concepts
            .iter()
            .map(|concept| (*concept).to_owned())
            .collect(),
        constraints: template
            .constraints
            .iter()
            .map(|constraint| (*constraint).to_owned())
            .collect(),
        xp: template.xp,
        answer_rule: template.answer_rule,
        transition: template.transition,
    })
}

fn validate_unique_template_slots(slots: &[SlotSpec]) -> Result<(), ExpansionError> {
    for (index, slot) in slots.iter().enumerate() {
        for earlier in &slots[..index] {
            if earlier.name == slot.name {
                return Err(ExpansionError::DuplicateTemplateSlot(slot.name.to_owned()));
            }
        }
    }

    Ok(())
}

fn find_domain_index(slot: &SlotSpec, domains: &[SlotDomain]) -> Result<usize, ExpansionError> {
    let mut match_index = None;
    for (index, domain) in domains.iter().enumerate() {
        if domain.id == slot.domain_id && domain.version == slot.domain_version {
            if match_index.is_some() {
                return Err(ExpansionError::DuplicateDomain {
                    id: slot.domain_id.to_owned(),
                    version: slot.domain_version,
                });
            }
            match_index = Some(index);
        }
    }

    let Some(index) = match_index else {
        return Err(ExpansionError::UnknownDomain {
            id: slot.domain_id.to_owned(),
            version: slot.domain_version,
        });
    };

    if domains[index].values.is_empty() {
        return Err(ExpansionError::EmptyDomain {
            id: slot.domain_id.to_owned(),
            version: slot.domain_version,
        });
    }

    Ok(index)
}

fn checked_cardinality_multiply(cardinality: u64, radix: u64) -> Result<u64, ExpansionError> {
    cardinality
        .checked_mul(radix)
        .ok_or(ExpansionError::CardinalityOverflow)
}

fn render_parts(
    parts: &[TemplatePart],
    declared_slots: &[SlotSpec],
    assignments: &[SlotAssignment],
) -> Result<String, ExpansionError> {
    validate_unique_template_slots(declared_slots)?;

    for (index, assignment) in assignments.iter().enumerate() {
        for earlier in &assignments[..index] {
            if earlier.slot == assignment.slot {
                return Err(ExpansionError::DuplicateAssignment(assignment.slot.clone()));
            }
        }
        if !declared_slots
            .iter()
            .any(|slot| slot.name == assignment.slot.as_str())
        {
            return Err(ExpansionError::UnknownSlot(assignment.slot.clone()));
        }
    }

    for slot in declared_slots {
        if !assignments
            .iter()
            .any(|assignment| assignment.slot == slot.name)
        {
            return Err(ExpansionError::MissingSlot(slot.name.to_owned()));
        }
    }

    let mut output = String::new();
    for part in parts {
        match part {
            TemplatePart::Literal(text) => output.push_str(text),
            TemplatePart::Slot(name) => {
                if !declared_slots.iter().any(|slot| slot.name == *name) {
                    return Err(ExpansionError::UnknownSlot((*name).to_owned()));
                }

                let mut rendered = None;
                for assignment in assignments {
                    if assignment.slot == *name {
                        rendered = Some(assignment.rendered.as_str());
                        break;
                    }
                }
                let Some(rendered) = rendered else {
                    return Err(ExpansionError::MissingSlot((*name).to_owned()));
                };
                output.push_str(rendered);
            }
        }
    }

    Ok(output)
}

fn validate_identity_source(template: &TemplateSpec) -> Result<(), ExpansionError> {
    validate_identity_component("system", template.system)?;
    validate_identity_component("surface_frame", template.surface_frame)?;
    validate_identity_component("template_id", template.id)?;
    if template.version == 0 {
        return Err(ExpansionError::InvalidTemplateVersion(template.version));
    }
    Ok(())
}

fn validate_identity_component(field: &'static str, value: &str) -> Result<(), ExpansionError> {
    if value.is_empty() || value.contains(':') {
        return Err(ExpansionError::InvalidIdentityComponent {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn operation_identity_segment(operation: Operation) -> &'static str {
    match operation {
        Operation::Copy => "copy",
        Operation::Recognize => "recognize",
        Operation::Fill => "fill",
        Operation::Read => "read",
        Operation::Produce => "produce",
        Operation::Repair => "repair",
        Operation::Transform => "transform",
        Operation::Compose => "compose",
        Operation::Explain => "explain",
    }
}

fn build_generated_id(template: &TemplateSpec, seed: u64) -> String {
    format!(
        "gen:{}:{}:{}:v{}:{}:s{}",
        template.system,
        template.surface_frame,
        template.id,
        template.version,
        operation_identity_segment(template.operation),
        seed
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::procgen::{SLOT_DOMAINS, TEMPLATES};

    const MULTI_SLOTS: &[SlotSpec] = &[
        SlotSpec {
            name: "A",
            role: "first",
            domain_id: "test.a",
            domain_version: 1,
        },
        SlotSpec {
            name: "B",
            role: "second",
            domain_id: "test.b",
            domain_version: 1,
        },
    ];
    const MULTI_PARTS: &[TemplatePart] = &[
        TemplatePart::Slot("A"),
        TemplatePart::Literal(" "),
        TemplatePart::Slot("B"),
    ];
    const MULTI_DOMAINS: &[SlotDomain] = &[
        SlotDomain {
            id: "test.a",
            version: 1,
            values: &[("a0", "A0"), ("a1", "A1")],
        },
        SlotDomain {
            id: "test.b",
            version: 1,
            values: &[("b0", "B0"), ("b1", "B1"), ("b2", "B2")],
        },
    ];
    const MULTI_TEMPLATE: TemplateSpec = TemplateSpec {
        id: "mixed",
        version: 1,
        system: "test",
        track: "test",
        semantic_frame: "test.mixed",
        surface_frame: "mixed",
        operation: Operation::Copy,
        slots: MULTI_SLOTS,
        prompt: MULTI_PARTS,
        expected: MULTI_PARTS,
        contrast_id: Some("fixed.contrast"),
        concepts: &[],
        constraints: &[],
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
        xp: 1,
    };

    const ZERO_PARTS: &[TemplatePart] = &[TemplatePart::Literal("literal only")];
    const ZERO_TEMPLATE: TemplateSpec = TemplateSpec {
        id: "literal-only",
        version: 1,
        system: "test",
        track: "test",
        semantic_frame: "test.literal",
        surface_frame: "literal",
        operation: Operation::Copy,
        slots: &[],
        prompt: ZERO_PARTS,
        expected: ZERO_PARTS,
        contrast_id: None,
        concepts: &[],
        constraints: &[],
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
        xp: 1,
    };

    const DUPLICATE_OUTPUT_SLOTS: &[SlotSpec] = &[SlotSpec {
        name: "X",
        role: "value",
        domain_id: "test.duplicate_output",
        domain_version: 1,
    }];
    const DUPLICATE_OUTPUT_PARTS: &[TemplatePart] = &[TemplatePart::Slot("X")];
    const DUPLICATE_OUTPUT_DOMAINS: &[SlotDomain] = &[SlotDomain {
        id: "test.duplicate_output",
        version: 1,
        values: &[("first", "same"), ("second", "same")],
    }];
    const DUPLICATE_OUTPUT_TEMPLATE: TemplateSpec = TemplateSpec {
        id: "duplicate-output",
        version: 1,
        system: "test",
        track: "test",
        semantic_frame: "test.duplicate_output",
        surface_frame: "duplicate_output",
        operation: Operation::Copy,
        slots: DUPLICATE_OUTPUT_SLOTS,
        prompt: DUPLICATE_OUTPUT_PARTS,
        expected: DUPLICATE_OUTPUT_PARTS,
        contrast_id: None,
        concepts: &[],
        constraints: &[],
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
        xp: 1,
    };

    fn duplicate_output_signature(reps: &[ExpandedRep]) -> Option<(usize, usize)> {
        for (index, rep) in reps.iter().enumerate() {
            for (earlier_index, earlier) in reps[..index].iter().enumerate() {
                if rep.prompt == earlier.prompt
                    && rep.expected == earlier.expected
                    && rep.operation == earlier.operation
                {
                    return Some((earlier_index, index));
                }
            }
        }
        None
    }

    #[test]
    fn current_source_expands_exactly() {
        let expanded = expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0)
            .expect("accepted packet-1 source should expand");

        assert_eq!(
            expanded,
            ExpandedRep {
                generated_id: "gen:rust:function.return_literal:copy-line:v1:copy:s0".to_owned(),
                template_id: "copy-line".to_owned(),
                template_version: 1,
                seed: 0,
                prompt_variant: 0,
                contrast_id: None,
                assignments: vec![
                    SlotAssignment {
                        slot: "NAME".to_owned(),
                        domain_id: "rust.function_name".to_owned(),
                        domain_version: 1,
                        value_id: "five".to_owned(),
                        rendered: "five".to_owned(),
                    },
                    SlotAssignment {
                        slot: "TYPE".to_owned(),
                        domain_id: "rust.integer_type".to_owned(),
                        domain_version: 1,
                        value_id: "i32".to_owned(),
                        rendered: "i32".to_owned(),
                    },
                    SlotAssignment {
                        slot: "VALUE".to_owned(),
                        domain_id: "rust.integer_literal".to_owned(),
                        domain_version: 1,
                        value_id: "five".to_owned(),
                        rendered: "5".to_owned(),
                    },
                ],
                prompt: "fn five() -> i32 { 5 }".to_owned(),
                expected: "fn five() -> i32 { 5 }".to_owned(),
                system: "rust".to_owned(),
                track: "syntax_basics".to_owned(),
                semantic_frame: "function.no_params.return_literal".to_owned(),
                surface_frame: "function.return_literal".to_owned(),
                operation: Operation::Copy,
                concepts: vec!["rust.functions".to_owned(), "rust.return_values".to_owned()],
                constraints: vec![
                    "rust.uses_fn_keyword".to_owned(),
                    "rust.no_params_empty_parens".to_owned(),
                    "rust.return_type_uses_arrow".to_owned(),
                    "rust.tail_expression_returns_value".to_owned(),
                ],
                xp: 10,
                answer_rule: AnswerRule::ExactTrimFinalNewline,
                transition: StateTransition::None,
            }
        );
        assert!(!expanded.prompt.ends_with('\n'));
        assert!(!expanded.prompt.ends_with('\r'));
        assert!(!expanded.expected.ends_with('\n'));
        assert!(!expanded.expected.ends_with('\r'));
    }

    #[test]
    fn same_source_and_seed_are_reproducible() {
        let first = expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0)
            .expect("first accepted expansion should succeed");
        let second = expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0)
            .expect("second accepted expansion should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn current_cardinality_and_seed_range_are_exact() {
        assert_eq!(template_cardinality(&TEMPLATES[0], SLOT_DOMAINS), Ok(1));
        assert!(expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0).is_ok());
        assert_eq!(
            expand_template(&TEMPLATES[0], SLOT_DOMAINS, 1),
            Err(ExpansionError::SeedOutOfRange {
                seed: 1,
                cardinality: 1,
            })
        );
    }

    #[test]
    fn mixed_radix_varies_first_slot_fastest() {
        assert_eq!(template_cardinality(&MULTI_TEMPLATE, MULTI_DOMAINS), Ok(6));

        let expected = [
            ("A0", "B0", "A0 B0"),
            ("A1", "B0", "A1 B0"),
            ("A0", "B1", "A0 B1"),
            ("A1", "B1", "A1 B1"),
            ("A0", "B2", "A0 B2"),
            ("A1", "B2", "A1 B2"),
        ];
        for (seed, (expected_a, expected_b, expected_rendered)) in
            expected.iter().copied().enumerate()
        {
            let expanded = expand_template(&MULTI_TEMPLATE, MULTI_DOMAINS, seed as u64)
                .expect("in-range mixed-radix seed should expand");
            assert_eq!(expanded.assignments[0].rendered, expected_a);
            assert_eq!(expanded.assignments[1].rendered, expected_b);
            assert_eq!(expanded.prompt, expected_rendered);
            assert_eq!(expanded.expected, expected_rendered);
            assert_eq!(expanded.prompt_variant, 0);
            assert_eq!(expanded.contrast_id.as_deref(), Some("fixed.contrast"));
        }

        assert_eq!(
            expand_template(&MULTI_TEMPLATE, MULTI_DOMAINS, 6),
            Err(ExpansionError::SeedOutOfRange {
                seed: 6,
                cardinality: 6,
            })
        );
    }

    #[test]
    fn zero_slot_template_has_one_valid_seed() {
        assert_eq!(template_cardinality(&ZERO_TEMPLATE, &[]), Ok(1));

        let expanded =
            expand_template(&ZERO_TEMPLATE, &[], 0).expect("zero-slot seed zero should expand");
        assert!(expanded.assignments.is_empty());
        assert_eq!(expanded.prompt, "literal only");
        assert_eq!(expanded.expected, "literal only");
        assert_eq!(
            expand_template(&ZERO_TEMPLATE, &[], 1),
            Err(ExpansionError::SeedOutOfRange {
                seed: 1,
                cardinality: 1,
            })
        );
    }

    #[test]
    fn domain_failures_are_explicit() {
        assert_eq!(
            template_cardinality(&TEMPLATES[0], &[]),
            Err(ExpansionError::UnknownDomain {
                id: "rust.function_name".to_owned(),
                version: 1,
            })
        );

        const DUPLICATE_DOMAINS: &[SlotDomain] = &[
            SlotDomain {
                id: "rust.function_name",
                version: 1,
                values: &[("first", "first")],
            },
            SlotDomain {
                id: "rust.function_name",
                version: 1,
                values: &[("second", "second")],
            },
        ];
        assert_eq!(
            template_cardinality(&TEMPLATES[0], DUPLICATE_DOMAINS),
            Err(ExpansionError::DuplicateDomain {
                id: "rust.function_name".to_owned(),
                version: 1,
            })
        );

        const EMPTY_DOMAINS: &[SlotDomain] = &[SlotDomain {
            id: "rust.function_name",
            version: 1,
            values: &[],
        }];
        assert_eq!(
            template_cardinality(&TEMPLATES[0], EMPTY_DOMAINS),
            Err(ExpansionError::EmptyDomain {
                id: "rust.function_name".to_owned(),
                version: 1,
            })
        );

        assert_eq!(
            checked_cardinality_multiply(u64::MAX, 2),
            Err(ExpansionError::CardinalityOverflow)
        );
    }

    #[test]
    fn renderer_preserves_exact_bytes_and_repeated_slots() {
        const SLOTS: &[SlotSpec] = &[SlotSpec {
            name: "X",
            role: "value",
            domain_id: "test.x",
            domain_version: 1,
        }];
        const PARTS: &[TemplatePart] = &[
            TemplatePart::Literal("α\t\r\n"),
            TemplatePart::Slot("X"),
            TemplatePart::Literal("|"),
            TemplatePart::Slot("X"),
            TemplatePart::Literal("\r\nω"),
        ];
        let assignments = [SlotAssignment {
            slot: "X".to_owned(),
            domain_id: "test.x".to_owned(),
            domain_version: 1,
            value_id: "value".to_owned(),
            rendered: "값".to_owned(),
        }];

        assert_eq!(
            render_parts(PARTS, SLOTS, &assignments),
            Ok("α\t\r\n값|값\r\nω".to_owned())
        );
    }

    #[test]
    fn renderer_and_slot_failures_are_explicit() {
        const SLOTS: &[SlotSpec] = &[SlotSpec {
            name: "X",
            role: "value",
            domain_id: "test.x",
            domain_version: 1,
        }];
        const PARTS: &[TemplatePart] = &[TemplatePart::Slot("X")];

        assert_eq!(
            render_parts(PARTS, SLOTS, &[]),
            Err(ExpansionError::MissingSlot("X".to_owned()))
        );

        let duplicate_assignments = [
            SlotAssignment {
                slot: "X".to_owned(),
                domain_id: "test.x".to_owned(),
                domain_version: 1,
                value_id: "first".to_owned(),
                rendered: "first".to_owned(),
            },
            SlotAssignment {
                slot: "X".to_owned(),
                domain_id: "test.x".to_owned(),
                domain_version: 1,
                value_id: "second".to_owned(),
                rendered: "second".to_owned(),
            },
        ];
        assert_eq!(
            render_parts(PARTS, SLOTS, &duplicate_assignments),
            Err(ExpansionError::DuplicateAssignment("X".to_owned()))
        );

        let unknown_assignment = [SlotAssignment {
            slot: "Y".to_owned(),
            domain_id: "test.y".to_owned(),
            domain_version: 1,
            value_id: "value".to_owned(),
            rendered: "value".to_owned(),
        }];
        assert_eq!(
            render_parts(PARTS, SLOTS, &unknown_assignment),
            Err(ExpansionError::UnknownSlot("Y".to_owned()))
        );

        let known_assignment = [SlotAssignment {
            slot: "X".to_owned(),
            domain_id: "test.x".to_owned(),
            domain_version: 1,
            value_id: "value".to_owned(),
            rendered: "value".to_owned(),
        }];
        const UNKNOWN_PARTS: &[TemplatePart] = &[TemplatePart::Slot("Y")];
        assert_eq!(
            render_parts(UNKNOWN_PARTS, SLOTS, &known_assignment),
            Err(ExpansionError::UnknownSlot("Y".to_owned()))
        );

        const DUPLICATE_SLOTS: &[SlotSpec] = &[
            SlotSpec {
                name: "X",
                role: "first",
                domain_id: "test.x",
                domain_version: 1,
            },
            SlotSpec {
                name: "X",
                role: "second",
                domain_id: "test.x",
                domain_version: 1,
            },
        ];
        assert_eq!(
            render_parts(&[], DUPLICATE_SLOTS, &[]),
            Err(ExpansionError::DuplicateTemplateSlot("X".to_owned()))
        );

        let duplicate_slot_template = TemplateSpec {
            id: "duplicate-slot",
            version: 1,
            system: "test",
            track: "test",
            semantic_frame: "test.duplicate_slot",
            surface_frame: "duplicate_slot",
            operation: Operation::Copy,
            slots: DUPLICATE_SLOTS,
            prompt: &[],
            expected: &[],
            contrast_id: None,
            concepts: &[],
            constraints: &[],
            answer_rule: AnswerRule::ExactTrimFinalNewline,
            transition: StateTransition::None,
            xp: 1,
        };
        assert_eq!(
            expand_template(&duplicate_slot_template, &[], 0),
            Err(ExpansionError::DuplicateTemplateSlot("X".to_owned()))
        );
    }

    #[test]
    fn identity_mapping_and_failures_are_explicit() {
        let expanded = expand_template(&TEMPLATES[0], SLOT_DOMAINS, 0)
            .expect("accepted source identity should expand");
        assert_eq!(
            expanded.generated_id,
            "gen:rust:function.return_literal:copy-line:v1:copy:s0"
        );

        for (operation, expected) in [
            (Operation::Copy, "copy"),
            (Operation::Recognize, "recognize"),
            (Operation::Fill, "fill"),
            (Operation::Read, "read"),
            (Operation::Produce, "produce"),
            (Operation::Repair, "repair"),
            (Operation::Transform, "transform"),
            (Operation::Compose, "compose"),
            (Operation::Explain, "explain"),
        ] {
            assert_eq!(operation_identity_segment(operation), expected);
        }

        const LITERAL_PARTS: &[TemplatePart] = &[TemplatePart::Literal("x")];
        for (system, surface_frame, id, version, expected) in [
            (
                "",
                "surface",
                "template",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "system",
                    value: String::new(),
                },
            ),
            (
                "bad:system",
                "surface",
                "template",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "system",
                    value: "bad:system".to_owned(),
                },
            ),
            (
                "system",
                "",
                "template",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "surface_frame",
                    value: String::new(),
                },
            ),
            (
                "system",
                "bad:surface",
                "template",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "surface_frame",
                    value: "bad:surface".to_owned(),
                },
            ),
            (
                "system",
                "surface",
                "",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "template_id",
                    value: String::new(),
                },
            ),
            (
                "system",
                "surface",
                "bad:template",
                1,
                ExpansionError::InvalidIdentityComponent {
                    field: "template_id",
                    value: "bad:template".to_owned(),
                },
            ),
            (
                "system",
                "surface",
                "template",
                0,
                ExpansionError::InvalidTemplateVersion(0),
            ),
        ] {
            let template = TemplateSpec {
                id,
                version,
                system,
                track: "test",
                semantic_frame: "test.identity",
                surface_frame,
                operation: Operation::Copy,
                slots: &[],
                prompt: LITERAL_PARTS,
                expected: LITERAL_PARTS,
                contrast_id: None,
                concepts: &[],
                constraints: &[],
                answer_rule: AnswerRule::ExactTrimFinalNewline,
                transition: StateTransition::None,
                xp: 1,
            };
            assert_eq!(expand_template(&template, &[], 0), Err(expected));
        }
    }

    #[test]
    fn current_source_range_has_unique_identities_and_signatures() {
        let cardinality = template_cardinality(&TEMPLATES[0], SLOT_DOMAINS)
            .expect("current template cardinality should be valid");
        let mut expanded = Vec::new();
        for seed in 0..cardinality {
            expanded.push(
                expand_template(&TEMPLATES[0], SLOT_DOMAINS, seed)
                    .expect("every current in-range seed should expand"),
            );
        }

        for (index, rep) in expanded.iter().enumerate() {
            for earlier in &expanded[..index] {
                assert_ne!(rep.generated_id, earlier.generated_id);
            }
        }
        assert_eq!(duplicate_output_signature(&expanded), None);
    }

    #[test]
    fn duplicate_output_signature_fixture_is_detected() {
        let mut expanded = Vec::new();
        for seed in 0..2 {
            expanded.push(
                expand_template(&DUPLICATE_OUTPUT_TEMPLATE, DUPLICATE_OUTPUT_DOMAINS, seed)
                    .expect("duplicate-output fixture seed should expand"),
            );
        }

        assert_ne!(expanded[0].generated_id, expanded[1].generated_id);
        assert_ne!(
            expanded[0].assignments[0].value_id,
            expanded[1].assignments[0].value_id
        );
        assert_eq!(duplicate_output_signature(&expanded), Some((0, 1)));
    }
}
