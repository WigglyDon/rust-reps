pub(crate) mod expansion;
pub(crate) mod materialized;

use super::{AnswerRule, Operation, StateTransition};

pub(crate) struct SemanticFrameSpec {
    pub(crate) id: &'static str,
    pub(crate) roles: &'static [&'static str],
    pub(crate) concepts: &'static [&'static str],
}

pub(crate) struct SurfaceFrameSpec {
    pub(crate) system: &'static str,
    pub(crate) local_id: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) slots: &'static [(&'static str, &'static str)],
    pub(crate) concepts: &'static [&'static str],
    pub(crate) constraints: &'static [&'static str],
    pub(crate) supported_contrasts: &'static [&'static str],
    pub(crate) supported_operations: &'static [Operation],
}

pub(crate) struct SlotSpec {
    pub(crate) name: &'static str,
    pub(crate) role: &'static str,
    pub(crate) domain_id: &'static str,
    pub(crate) domain_version: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DomainValueSpec {
    pub(crate) id: &'static str,
    pub(crate) rendered: &'static str,
    pub(crate) meaning_key: Option<&'static str>,
}

pub(crate) struct SlotDomain {
    pub(crate) id: &'static str,
    pub(crate) version: u16,
    pub(crate) values: &'static [DomainValueSpec],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompatibilityAssignmentSpec {
    pub(crate) slot: &'static str,
    pub(crate) domain_id: &'static str,
    pub(crate) domain_version: u16,
    pub(crate) value_id: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompatibilityRowSpec {
    pub(crate) id: &'static str,
    pub(crate) content_seed: u64,
    pub(crate) assignments: &'static [CompatibilityAssignmentSpec],
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TemplatePart {
    Literal(&'static str),
    Slot(&'static str),
}

pub(crate) struct TemplateSpec {
    pub(crate) id: &'static str,
    pub(crate) version: u16,
    pub(crate) system: &'static str,
    pub(crate) track: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) surface_frame: &'static str,
    pub(crate) operation: Operation,
    pub(crate) instruction_key: &'static str,
    pub(crate) slots: &'static [SlotSpec],
    pub(crate) stimulus: &'static [TemplatePart],
    pub(crate) expected: &'static [TemplatePart],
    pub(crate) contrast_id: Option<&'static str>,
    pub(crate) concepts: &'static [&'static str],
    pub(crate) constraints: &'static [&'static str],
    pub(crate) answer_rule: AnswerRule,
    pub(crate) transition: StateTransition,
    pub(crate) xp: u32,
}

pub(crate) const SEMANTIC_FRAMES: &[SemanticFrameSpec] = &[SemanticFrameSpec {
    id: "function.no_params.return_literal",
    roles: &["callable_name", "result_kind", "returned_literal"],
    concepts: &["functions", "return_values"],
}];

const RUST_FUNCTION_CONCEPTS: &[&str] = &["rust.functions", "rust.return_values"];
const RUST_FUNCTION_CONSTRAINTS: &[&str] = &[
    "rust.uses_fn_keyword",
    "rust.no_params_empty_parens",
    "rust.return_type_uses_arrow",
    "rust.tail_expression_returns_value",
];

pub(crate) const SURFACE_FRAMES: &[SurfaceFrameSpec] = &[SurfaceFrameSpec {
    system: "rust",
    local_id: "function.return_literal",
    semantic_frame: "function.no_params.return_literal",
    slots: &[
        ("NAME", "callable_name"),
        ("TYPE", "result_kind"),
        ("VALUE", "returned_literal"),
    ],
    concepts: RUST_FUNCTION_CONCEPTS,
    constraints: RUST_FUNCTION_CONSTRAINTS,
    supported_contrasts: &["typescript.return_type_colon", "rust.return_type_omitted"],
    // Conceptual support does not imply templates, materialization, catalog admission, or product support.
    supported_operations: &[
        Operation::Copy,
        Operation::Fill,
        Operation::Produce,
        Operation::Repair,
        Operation::Transform,
    ],
}];

pub(crate) const SLOT_DOMAINS: &[SlotDomain] = &[
    SlotDomain {
        id: "rust.function_name",
        version: 1,
        values: &[
            DomainValueSpec {
                id: "five",
                rendered: "five",
                meaning_key: None,
            },
            DomainValueSpec {
                id: "seven",
                rendered: "seven",
                meaning_key: None,
            },
        ],
    },
    SlotDomain {
        id: "rust.integer_type",
        version: 1,
        values: &[
            DomainValueSpec {
                id: "i32",
                rendered: "i32",
                meaning_key: None,
            },
            DomainValueSpec {
                id: "u64",
                rendered: "u64",
                meaning_key: None,
            },
        ],
    },
    SlotDomain {
        id: "rust.integer_literal",
        version: 1,
        values: &[
            DomainValueSpec {
                id: "five",
                rendered: "5",
                meaning_key: None,
            },
            DomainValueSpec {
                id: "seven",
                rendered: "7",
                meaning_key: None,
            },
        ],
    },
];

const ROW_A_ASSIGNMENTS: &[CompatibilityAssignmentSpec] = &[
    CompatibilityAssignmentSpec {
        slot: "NAME",
        domain_id: "rust.function_name",
        domain_version: 1,
        value_id: "five",
    },
    CompatibilityAssignmentSpec {
        slot: "TYPE",
        domain_id: "rust.integer_type",
        domain_version: 1,
        value_id: "i32",
    },
    CompatibilityAssignmentSpec {
        slot: "VALUE",
        domain_id: "rust.integer_literal",
        domain_version: 1,
        value_id: "five",
    },
];

const ROW_B_ASSIGNMENTS: &[CompatibilityAssignmentSpec] = &[
    CompatibilityAssignmentSpec {
        slot: "NAME",
        domain_id: "rust.function_name",
        domain_version: 1,
        value_id: "seven",
    },
    CompatibilityAssignmentSpec {
        slot: "TYPE",
        domain_id: "rust.integer_type",
        domain_version: 1,
        value_id: "u64",
    },
    CompatibilityAssignmentSpec {
        slot: "VALUE",
        domain_id: "rust.integer_literal",
        domain_version: 1,
        value_id: "seven",
    },
];

pub(crate) const COMPATIBILITY_ROWS: &[CompatibilityRowSpec] = &[
    CompatibilityRowSpec {
        id: "row-a-five-i32",
        content_seed: 0,
        assignments: ROW_A_ASSIGNMENTS,
    },
    CompatibilityRowSpec {
        id: "row-b-seven-u64",
        content_seed: 1,
        assignments: ROW_B_ASSIGNMENTS,
    },
];

const RETURN_LITERAL_SLOTS: &[SlotSpec] = &[
    SlotSpec {
        name: "NAME",
        role: "callable_name",
        domain_id: "rust.function_name",
        domain_version: 1,
    },
    SlotSpec {
        name: "TYPE",
        role: "result_kind",
        domain_id: "rust.integer_type",
        domain_version: 1,
    },
    SlotSpec {
        name: "VALUE",
        role: "returned_literal",
        domain_id: "rust.integer_literal",
        domain_version: 1,
    },
];

const COPY_LINE_STIMULUS: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() -> "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

const RETURN_LITERAL_EXPECTED: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() -> "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

const RETURN_TYPE_PLACEHOLDER_STIMULUS: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() ___ "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

const FILL_RETURN_TYPE_EXPECTED: &[TemplatePart] = &[TemplatePart::Literal("->")];

const PRODUCE_RETURN_TYPE_STIMULUS: &[TemplatePart] = RETURN_TYPE_PLACEHOLDER_STIMULUS;

const REPAIR_RETURN_TYPE_ARROW_STIMULUS: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("(): "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

const TRANSFORM_ADD_RETURN_TYPE_STIMULUS: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

pub(crate) const TEMPLATES: &[TemplateSpec] = &[
    TemplateSpec {
        id: "copy-return-type",
        version: 2,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Copy,
        instruction_key: "copy.return_type_intro",
        slots: RETURN_LITERAL_SLOTS,
        stimulus: COPY_LINE_STIMULUS,
        expected: RETURN_LITERAL_EXPECTED,
        contrast_id: None,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
        xp: 10,
    },
    TemplateSpec {
        id: "fill-return-type-arrow",
        version: 2,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Fill,
        instruction_key: "fill.return_type_arrow",
        slots: RETURN_LITERAL_SLOTS,
        stimulus: RETURN_TYPE_PLACEHOLDER_STIMULUS,
        expected: FILL_RETURN_TYPE_EXPECTED,
        contrast_id: None,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
        xp: 10,
    },
    TemplateSpec {
        id: "produce-return-type",
        version: 2,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Produce,
        instruction_key: "produce.return_type_arrow",
        slots: RETURN_LITERAL_SLOTS,
        stimulus: PRODUCE_RETURN_TYPE_STIMULUS,
        expected: RETURN_LITERAL_EXPECTED,
        contrast_id: None,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        answer_rule: AnswerRule::RustfmtExact,
        transition: StateTransition::None,
        xp: 10,
    },
    TemplateSpec {
        id: "repair-return-type-arrow",
        version: 2,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Repair,
        instruction_key: "repair.return_type_arrow",
        slots: RETURN_LITERAL_SLOTS,
        stimulus: REPAIR_RETURN_TYPE_ARROW_STIMULUS,
        expected: RETURN_LITERAL_EXPECTED,
        contrast_id: Some("typescript.return_type_colon"),
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        answer_rule: AnswerRule::RustfmtExact,
        transition: StateTransition::None,
        xp: 10,
    },
    TemplateSpec {
        id: "transform-add-return-type",
        version: 2,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Transform,
        instruction_key: "transform.add_return_type",
        slots: RETURN_LITERAL_SLOTS,
        stimulus: TRANSFORM_ADD_RETURN_TYPE_STIMULUS,
        expected: RETURN_LITERAL_EXPECTED,
        contrast_id: Some("rust.return_type_omitted"),
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        answer_rule: AnswerRule::RustfmtExact,
        transition: StateTransition::None,
        xp: 10,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    const HAND_EXPANDED_SEED: u64 = 0;
    const HAND_EXPANDED_STIMULUS: &str = "fn five() -> i32 { 5 }";
    const HAND_EXPANDED_EXPECTED: &str = "fn five() -> i32 { 5 }";

    fn assert_nonempty_unique(values: &[&str], context: &str) {
        for (index, value) in values.iter().enumerate() {
            assert!(
                !value.is_empty(),
                "{context} value {index} must not be empty"
            );
            for earlier in &values[..index] {
                assert_ne!(
                    earlier, value,
                    "{context} values must be unique; found {value} twice"
                );
            }
        }
    }

    #[test]
    fn semantic_frame_source_invariants() {
        for (index, frame) in SEMANTIC_FRAMES.iter().enumerate() {
            assert!(!frame.id.is_empty(), "semantic frame ID must not be empty");
            for earlier in &SEMANTIC_FRAMES[..index] {
                assert_ne!(earlier.id, frame.id, "semantic frame IDs must be unique");
            }

            assert!(
                !frame.roles.is_empty(),
                "semantic frame must declare at least one role"
            );
            assert!(
                !frame.concepts.is_empty(),
                "semantic frame must declare at least one concept"
            );
            assert_nonempty_unique(frame.roles, "semantic role");
            assert_nonempty_unique(frame.concepts, "semantic concept");

            for field in std::iter::once(frame.id)
                .chain(frame.roles.iter().copied())
                .chain(frame.concepts.iter().copied())
            {
                assert!(
                    !field.starts_with("rust."),
                    "semantic source must not contain Rust-qualified field {field}"
                );
                assert_ne!(
                    field, "fn",
                    "semantic source must not contain Rust keyword fn"
                );
            }
        }

        let frame = &SEMANTIC_FRAMES[0];
        assert_eq!(
            frame.roles,
            &["callable_name", "result_kind", "returned_literal"]
        );
        assert_eq!(frame.concepts, &["functions", "return_values"]);
    }

    #[test]
    fn surface_frame_source_invariants() {
        for (index, frame) in SURFACE_FRAMES.iter().enumerate() {
            assert!(!frame.system.is_empty(), "surface system must not be empty");
            assert!(
                !frame.local_id.is_empty(),
                "surface local ID must not be empty"
            );
            for earlier in &SURFACE_FRAMES[..index] {
                assert_ne!(
                    (earlier.system, earlier.local_id),
                    (frame.system, frame.local_id),
                    "surface (system, local ID) keys must be unique"
                );
            }

            let semantic = SEMANTIC_FRAMES
                .iter()
                .find(|semantic| semantic.id == frame.semantic_frame)
                .expect("surface frame must reference a known semantic frame");

            for (slot_index, (name, role)) in frame.slots.iter().enumerate() {
                assert!(!name.is_empty(), "surface slot name must not be empty");
                assert!(!role.is_empty(), "surface slot role must not be empty");
                for (earlier_name, _) in &frame.slots[..slot_index] {
                    assert_ne!(
                        earlier_name, name,
                        "surface slot names must be unique within a frame"
                    );
                }
                assert!(
                    semantic.roles.contains(role),
                    "surface slot role {role} must exist in its semantic frame"
                );
            }

            assert_nonempty_unique(frame.concepts, "surface concept");
            assert_nonempty_unique(frame.constraints, "surface constraint");
            assert_nonempty_unique(frame.supported_contrasts, "surface contrast");
            for (operation_index, operation) in frame.supported_operations.iter().enumerate() {
                for earlier in &frame.supported_operations[..operation_index] {
                    assert_ne!(
                        earlier, operation,
                        "surface operations must not contain duplicates"
                    );
                }
            }
        }

        let frame = &SURFACE_FRAMES[0];
        assert_eq!(
            frame.supported_operations,
            &[
                Operation::Copy,
                Operation::Fill,
                Operation::Produce,
                Operation::Repair,
                Operation::Transform,
            ]
        );
        assert_eq!(
            frame.supported_contrasts,
            &["typescript.return_type_colon", "rust.return_type_omitted"]
        );
        assert_eq!(
            frame.slots,
            &[
                ("NAME", "callable_name"),
                ("TYPE", "result_kind"),
                ("VALUE", "returned_literal"),
            ]
        );
    }

    #[test]
    fn slot_domain_source_invariants() {
        for (index, domain) in SLOT_DOMAINS.iter().enumerate() {
            assert!(!domain.id.is_empty(), "slot domain ID must not be empty");
            assert!(domain.version > 0, "slot domain version must be positive");
            assert!(!domain.values.is_empty(), "slot domain must contain values");
            for earlier in &SLOT_DOMAINS[..index] {
                assert_ne!(
                    (earlier.id, earlier.version),
                    (domain.id, domain.version),
                    "slot domain (ID, version) keys must be unique"
                );
            }

            for (value_index, value) in domain.values.iter().enumerate() {
                assert!(!value.id.is_empty(), "domain value ID must not be empty");
                assert!(
                    !value.rendered.is_empty(),
                    "rendered domain value must not be empty"
                );
                if let Some(meaning_key) = value.meaning_key {
                    assert!(
                        !meaning_key.is_empty(),
                        "domain value meaning key must not be empty when present"
                    );
                }
                for earlier in &domain.values[..value_index] {
                    assert_ne!(
                        earlier.id, value.id,
                        "value IDs must be unique within a domain"
                    );
                }
            }
        }

        assert_eq!(SLOT_DOMAINS.len(), 3);
        assert_eq!(
            (SLOT_DOMAINS[0].id, SLOT_DOMAINS[0].version),
            ("rust.function_name", 1)
        );
        assert_eq!(
            SLOT_DOMAINS[0].values,
            &[
                DomainValueSpec {
                    id: "five",
                    rendered: "five",
                    meaning_key: None,
                },
                DomainValueSpec {
                    id: "seven",
                    rendered: "seven",
                    meaning_key: None,
                },
            ]
        );
        assert_eq!(
            (SLOT_DOMAINS[1].id, SLOT_DOMAINS[1].version),
            ("rust.integer_type", 1)
        );
        assert_eq!(
            SLOT_DOMAINS[1].values,
            &[
                DomainValueSpec {
                    id: "i32",
                    rendered: "i32",
                    meaning_key: None,
                },
                DomainValueSpec {
                    id: "u64",
                    rendered: "u64",
                    meaning_key: None,
                },
            ]
        );
        assert_eq!(
            (SLOT_DOMAINS[2].id, SLOT_DOMAINS[2].version),
            ("rust.integer_literal", 1)
        );
        assert_eq!(
            SLOT_DOMAINS[2].values,
            &[
                DomainValueSpec {
                    id: "five",
                    rendered: "5",
                    meaning_key: None,
                },
                DomainValueSpec {
                    id: "seven",
                    rendered: "7",
                    meaning_key: None,
                },
            ]
        );

        assert_eq!(COMPATIBILITY_ROWS.len(), 2);
        assert_eq!(COMPATIBILITY_ROWS[0].id, "row-a-five-i32");
        assert_eq!(COMPATIBILITY_ROWS[1].id, "row-b-seven-u64");
        assert_ne!(
            COMPATIBILITY_ROWS[0].assignments,
            COMPATIBILITY_ROWS[1].assignments
        );
    }

    #[test]
    fn template_source_invariants() {
        for (index, template) in TEMPLATES.iter().enumerate() {
            assert!(!template.id.is_empty(), "template ID must not be empty");
            assert!(template.version > 0, "template version must be positive");
            assert!(
                !template.system.is_empty(),
                "template system must not be empty"
            );
            assert!(
                !template.track.is_empty(),
                "template track must not be empty"
            );
            assert!(
                !template.semantic_frame.is_empty(),
                "template semantic frame must not be empty"
            );
            assert!(
                !template.surface_frame.is_empty(),
                "template surface frame must not be empty"
            );
            assert!(
                !template.instruction_key.is_empty(),
                "template instruction key must not be empty"
            );
            for earlier in &TEMPLATES[..index] {
                assert_ne!(
                    (
                        earlier.system,
                        earlier.surface_frame,
                        earlier.id,
                        earlier.version,
                    ),
                    (
                        template.system,
                        template.surface_frame,
                        template.id,
                        template.version,
                    ),
                    "template source keys must be unique"
                );
                assert_ne!(
                    earlier.instruction_key, template.instruction_key,
                    "template instruction keys must be unique"
                );
            }

            assert!(
                SEMANTIC_FRAMES
                    .iter()
                    .any(|frame| frame.id == template.semantic_frame),
                "template must reference a known semantic frame"
            );
            let surface = SURFACE_FRAMES
                .iter()
                .find(|frame| {
                    frame.system == template.system && frame.local_id == template.surface_frame
                })
                .expect("template must reference a known surface frame");
            assert_eq!(template.system, surface.system);
            assert_eq!(template.semantic_frame, surface.semantic_frame);
            assert!(surface.supported_operations.contains(&template.operation));

            for (slot_index, slot) in template.slots.iter().enumerate() {
                assert!(
                    !slot.name.is_empty(),
                    "template slot name must not be empty"
                );
                for earlier in &template.slots[..slot_index] {
                    assert_ne!(
                        earlier.name, slot.name,
                        "template slot names must be unique"
                    );
                }
                assert!(
                    surface
                        .slots
                        .iter()
                        .any(|(name, role)| *name == slot.name && *role == slot.role),
                    "template slot must exactly match a surface slot-role pair"
                );
                assert!(
                    SLOT_DOMAINS.iter().any(|domain| {
                        domain.id == slot.domain_id && domain.version == slot.domain_version
                    }),
                    "template slot must reference a known domain version"
                );
            }

            assert!(
                !template.expected.is_empty(),
                "template expected form must not be empty"
            );
            for part in template.stimulus.iter().chain(template.expected.iter()) {
                if let TemplatePart::Slot(name) = part {
                    assert!(
                        template.slots.iter().any(|slot| slot.name == *name),
                        "template part slot {name} must be declared"
                    );
                }
            }
            for slot in template.slots {
                assert!(
                    template
                        .stimulus
                        .iter()
                        .chain(template.expected.iter())
                        .any(|part| matches!(part, TemplatePart::Slot(name) if *name == slot.name)),
                    "declared slot {} must be used by stimulus or expected",
                    slot.name
                );
            }

            assert_eq!(template.concepts, surface.concepts);
            assert_eq!(template.constraints, surface.constraints);
            assert_eq!(
                template.answer_rule,
                match template.operation {
                    Operation::Copy | Operation::Fill => AnswerRule::ExactTrimFinalNewline,
                    Operation::Produce | Operation::Repair | Operation::Transform => {
                        AnswerRule::RustfmtExact
                    }
                    operation => panic!("unexpected template operation {operation:?}"),
                }
            );
            assert_eq!(template.transition, StateTransition::None);
            if let Some(contrast_id) = template.contrast_id {
                assert!(
                    !contrast_id.is_empty(),
                    "template contrast must not be empty"
                );
                assert!(
                    surface.supported_contrasts.contains(&contrast_id),
                    "template contrast must be supported by its surface"
                );
            }
            assert_eq!(template.xp, 10);
        }
    }

    #[test]
    fn accepted_five_template_source_is_exact() {
        assert_eq!(SEMANTIC_FRAMES.len(), 1);
        assert_eq!(SEMANTIC_FRAMES[0].id, "function.no_params.return_literal");

        assert_eq!(SURFACE_FRAMES.len(), 1);
        assert_eq!(SURFACE_FRAMES[0].system, "rust");
        assert_eq!(SURFACE_FRAMES[0].local_id, "function.return_literal");

        assert_eq!(TEMPLATES.len(), 5);
        assert_eq!(
            TEMPLATES
                .iter()
                .map(|template| template.id)
                .collect::<Vec<_>>(),
            vec![
                "copy-return-type",
                "fill-return-type-arrow",
                "produce-return-type",
                "repair-return-type-arrow",
                "transform-add-return-type",
            ]
        );
        assert_eq!(
            TEMPLATES
                .iter()
                .map(|template| template.instruction_key)
                .collect::<Vec<_>>(),
            vec![
                "copy.return_type_intro",
                "fill.return_type_arrow",
                "produce.return_type_arrow",
                "repair.return_type_arrow",
                "transform.add_return_type",
            ]
        );
        assert_eq!(
            TEMPLATES
                .iter()
                .map(|template| template.operation)
                .collect::<Vec<_>>(),
            vec![
                Operation::Copy,
                Operation::Fill,
                Operation::Produce,
                Operation::Repair,
                Operation::Transform,
            ]
        );

        for template in TEMPLATES {
            assert_eq!(template.version, 2);
            assert_eq!(template.system, "rust");
            assert_eq!(template.track, "syntax_basics");
            assert_eq!(
                template
                    .slots
                    .iter()
                    .map(|slot| slot.name)
                    .collect::<Vec<_>>(),
                vec!["NAME", "TYPE", "VALUE"]
            );
        }

        let complete_parts = [
            TemplatePart::Literal("fn "),
            TemplatePart::Slot("NAME"),
            TemplatePart::Literal("() -> "),
            TemplatePart::Slot("TYPE"),
            TemplatePart::Literal(" { "),
            TemplatePart::Slot("VALUE"),
            TemplatePart::Literal(" }"),
        ];
        let fill_parts = [
            TemplatePart::Literal("fn "),
            TemplatePart::Slot("NAME"),
            TemplatePart::Literal("() ___ "),
            TemplatePart::Slot("TYPE"),
            TemplatePart::Literal(" { "),
            TemplatePart::Slot("VALUE"),
            TemplatePart::Literal(" }"),
        ];
        let repair_parts = [
            TemplatePart::Literal("fn "),
            TemplatePart::Slot("NAME"),
            TemplatePart::Literal("(): "),
            TemplatePart::Slot("TYPE"),
            TemplatePart::Literal(" { "),
            TemplatePart::Slot("VALUE"),
            TemplatePart::Literal(" }"),
        ];
        let transform_parts = [
            TemplatePart::Literal("fn "),
            TemplatePart::Slot("NAME"),
            TemplatePart::Literal("() { "),
            TemplatePart::Slot("VALUE"),
            TemplatePart::Literal(" }"),
        ];

        assert_eq!(TEMPLATES[0].stimulus, complete_parts.as_slice());
        assert_eq!(TEMPLATES[1].stimulus, fill_parts.as_slice());
        assert_eq!(TEMPLATES[2].stimulus, fill_parts.as_slice());
        assert_eq!(TEMPLATES[3].stimulus, repair_parts.as_slice());
        assert_eq!(TEMPLATES[4].stimulus, transform_parts.as_slice());
        assert_eq!(TEMPLATES[0].expected, complete_parts.as_slice());
        assert_eq!(TEMPLATES[1].expected, &[TemplatePart::Literal("->")]);
        for template in &TEMPLATES[2..] {
            assert_eq!(template.expected, complete_parts.as_slice());
        }
        assert_eq!(
            TEMPLATES
                .iter()
                .map(|template| template.answer_rule)
                .collect::<Vec<_>>(),
            vec![
                AnswerRule::ExactTrimFinalNewline,
                AnswerRule::ExactTrimFinalNewline,
                AnswerRule::RustfmtExact,
                AnswerRule::RustfmtExact,
                AnswerRule::RustfmtExact,
            ]
        );
        assert_eq!(
            TEMPLATES
                .iter()
                .map(|template| template.contrast_id)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                None,
                Some("typescript.return_type_colon"),
                Some("rust.return_type_omitted"),
            ]
        );

        assert_eq!(HAND_EXPANDED_SEED, 0);
        assert_eq!(HAND_EXPANDED_STIMULUS, "fn five() -> i32 { 5 }");
        assert_eq!(HAND_EXPANDED_EXPECTED, "fn five() -> i32 { 5 }");
        for fixture in [HAND_EXPANDED_STIMULUS, HAND_EXPANDED_EXPECTED] {
            assert!(!fixture.ends_with('\n'));
            assert!(!fixture.ends_with('\r'));
        }
    }
}
