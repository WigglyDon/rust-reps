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

pub(crate) struct SlotDomain {
    pub(crate) id: &'static str,
    pub(crate) version: u16,
    pub(crate) values: &'static [(&'static str, &'static str)],
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
    pub(crate) slots: &'static [SlotSpec],
    pub(crate) prompt: &'static [TemplatePart],
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
    supported_contrasts: &[],
    supported_operations: &[Operation::Copy],
}];

pub(crate) const SLOT_DOMAINS: &[SlotDomain] = &[
    SlotDomain {
        id: "rust.function_name",
        version: 1,
        values: &[("five", "five")],
    },
    SlotDomain {
        id: "rust.integer_type",
        version: 1,
        values: &[("i32", "i32")],
    },
    SlotDomain {
        id: "rust.integer_literal",
        version: 1,
        values: &[("five", "5")],
    },
];

const COPY_LINE_SLOTS: &[SlotSpec] = &[
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

const COPY_LINE_PROMPT: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() -> "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

const COPY_LINE_EXPECTED: &[TemplatePart] = &[
    TemplatePart::Literal("fn "),
    TemplatePart::Slot("NAME"),
    TemplatePart::Literal("() -> "),
    TemplatePart::Slot("TYPE"),
    TemplatePart::Literal(" { "),
    TemplatePart::Slot("VALUE"),
    TemplatePart::Literal(" }"),
];

pub(crate) const TEMPLATES: &[TemplateSpec] = &[TemplateSpec {
    id: "copy-line",
    version: 1,
    system: "rust",
    track: "syntax_basics",
    semantic_frame: "function.no_params.return_literal",
    surface_frame: "function.return_literal",
    operation: Operation::Copy,
    slots: COPY_LINE_SLOTS,
    prompt: COPY_LINE_PROMPT,
    expected: COPY_LINE_EXPECTED,
    contrast_id: None,
    concepts: RUST_FUNCTION_CONCEPTS,
    constraints: RUST_FUNCTION_CONSTRAINTS,
    answer_rule: AnswerRule::ExactTrimFinalNewline,
    transition: StateTransition::None,
    xp: 10,
}];

#[cfg(test)]
mod tests {
    use super::*;

    const HAND_EXPANDED_SEED: u64 = 0;
    const HAND_EXPANDED_PROMPT: &str = "fn five() -> i32 { 5 }";
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
        assert_eq!(frame.supported_operations, &[Operation::Copy]);
        assert!(frame.supported_contrasts.is_empty());
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

            for (value_index, (value_id, rendered)) in domain.values.iter().enumerate() {
                assert!(!value_id.is_empty(), "domain value ID must not be empty");
                assert!(
                    !rendered.is_empty(),
                    "rendered domain value must not be empty"
                );
                for (earlier_id, _) in &domain.values[..value_index] {
                    assert_ne!(
                        earlier_id, value_id,
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
        assert_eq!(SLOT_DOMAINS[0].values, &[("five", "five")]);
        assert_eq!(
            (SLOT_DOMAINS[1].id, SLOT_DOMAINS[1].version),
            ("rust.integer_type", 1)
        );
        assert_eq!(SLOT_DOMAINS[1].values, &[("i32", "i32")]);
        assert_eq!(
            (SLOT_DOMAINS[2].id, SLOT_DOMAINS[2].version),
            ("rust.integer_literal", 1)
        );
        assert_eq!(SLOT_DOMAINS[2].values, &[("five", "5")]);
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
                !template.prompt.is_empty(),
                "template prompt must not be empty"
            );
            assert!(
                !template.expected.is_empty(),
                "template expected form must not be empty"
            );
            for part in template.prompt.iter().chain(template.expected.iter()) {
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
                        .prompt
                        .iter()
                        .chain(template.expected.iter())
                        .any(|part| matches!(part, TemplatePart::Slot(name) if *name == slot.name)),
                    "declared slot {} must be used by prompt or expected",
                    slot.name
                );
            }

            assert_eq!(template.concepts, surface.concepts);
            assert_eq!(template.constraints, surface.constraints);
            assert_eq!(template.answer_rule, AnswerRule::ExactTrimFinalNewline);
            assert_eq!(template.transition, StateTransition::None);
            assert_eq!(template.contrast_id, None);
            assert_eq!(template.xp, 10);
        }
    }

    #[test]
    fn accepted_packet_one_source_is_exact() {
        assert_eq!(SEMANTIC_FRAMES.len(), 1);
        assert_eq!(SEMANTIC_FRAMES[0].id, "function.no_params.return_literal");

        assert_eq!(SURFACE_FRAMES.len(), 1);
        assert_eq!(SURFACE_FRAMES[0].system, "rust");
        assert_eq!(SURFACE_FRAMES[0].local_id, "function.return_literal");

        assert_eq!(TEMPLATES.len(), 1);
        let template = &TEMPLATES[0];
        assert_eq!(template.id, "copy-line");
        assert_eq!(template.version, 1);
        assert_eq!(template.track, "syntax_basics");
        assert_eq!(
            template
                .slots
                .iter()
                .map(|slot| slot.name)
                .collect::<Vec<_>>(),
            vec!["NAME", "TYPE", "VALUE"]
        );

        let exact_parts = [
            TemplatePart::Literal("fn "),
            TemplatePart::Slot("NAME"),
            TemplatePart::Literal("() -> "),
            TemplatePart::Slot("TYPE"),
            TemplatePart::Literal(" { "),
            TemplatePart::Slot("VALUE"),
            TemplatePart::Literal(" }"),
        ];
        assert_eq!(template.prompt, exact_parts.as_slice());
        assert_eq!(template.expected, exact_parts.as_slice());
        assert_eq!(template.prompt, template.expected);

        assert_eq!(HAND_EXPANDED_SEED, 0);
        assert_eq!(HAND_EXPANDED_PROMPT, "fn five() -> i32 { 5 }");
        assert_eq!(HAND_EXPANDED_EXPECTED, "fn five() -> i32 { 5 }");
        for fixture in [HAND_EXPANDED_PROMPT, HAND_EXPANDED_EXPECTED] {
            assert!(!fixture.ends_with('\n'));
            assert!(!fixture.ends_with('\r'));
        }
    }
}
