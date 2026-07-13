use super::{RUST_FUNCTION_CONCEPTS, RUST_FUNCTION_CONSTRAINTS};
use crate::curriculum::{AnswerRule, Formatting, Operation, StateTransition};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct MaterializedAssignmentSpec {
    pub(crate) slot: &'static str,
    pub(crate) domain_id: &'static str,
    pub(crate) domain_version: u16,
    pub(crate) value_id: &'static str,
    pub(crate) rendered: &'static str,
    pub(crate) meaning_key: Option<&'static str>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct MaterializedTaskSpec {
    pub(crate) generated_id: &'static str,
    pub(crate) capability_id: &'static str,
    pub(crate) template_id: &'static str,
    pub(crate) template_version: u16,
    pub(crate) content_seed: u64,
    pub(crate) stimulus_variant: u16,
    pub(crate) contrast_id: Option<&'static str>,
    pub(crate) assignments: &'static [MaterializedAssignmentSpec],
    pub(crate) instruction_key: &'static str,
    pub(crate) stimulus: &'static str,
    pub(crate) expected: &'static str,
    pub(crate) system: &'static str,
    pub(crate) track: &'static str,
    pub(crate) semantic_frame: &'static str,
    pub(crate) surface_frame: &'static str,
    pub(crate) operation: Operation,
    pub(crate) concepts: &'static [&'static str],
    pub(crate) constraints: &'static [&'static str],
    pub(crate) xp: u32,
    pub(crate) answer_rule: AnswerRule,
    pub(crate) formatting: Formatting,
    pub(crate) transition: StateTransition,
}

pub(crate) const ROW_A_ASSIGNMENTS: &[MaterializedAssignmentSpec] = &[
    MaterializedAssignmentSpec {
        slot: "NAME",
        domain_id: "rust.function_name",
        domain_version: 1,
        value_id: "five",
        rendered: "five",
        meaning_key: None,
    },
    MaterializedAssignmentSpec {
        slot: "TYPE",
        domain_id: "rust.integer_type",
        domain_version: 1,
        value_id: "i32",
        rendered: "i32",
        meaning_key: None,
    },
    MaterializedAssignmentSpec {
        slot: "VALUE",
        domain_id: "rust.integer_literal",
        domain_version: 1,
        value_id: "five",
        rendered: "5",
        meaning_key: None,
    },
];

pub(crate) const ROW_B_ASSIGNMENTS: &[MaterializedAssignmentSpec] = &[
    MaterializedAssignmentSpec {
        slot: "NAME",
        domain_id: "rust.function_name",
        domain_version: 1,
        value_id: "seven",
        rendered: "seven",
        meaning_key: None,
    },
    MaterializedAssignmentSpec {
        slot: "TYPE",
        domain_id: "rust.integer_type",
        domain_version: 1,
        value_id: "u64",
        rendered: "u64",
        meaning_key: None,
    },
    MaterializedAssignmentSpec {
        slot: "VALUE",
        domain_id: "rust.integer_literal",
        domain_version: 1,
        value_id: "seven",
        rendered: "7",
        meaning_key: None,
    },
];

const fn task(
    generated_id: &'static str,
    capability_id: &'static str,
    template_id: &'static str,
    content_seed: u64,
    contrast_id: Option<&'static str>,
    assignments: &'static [MaterializedAssignmentSpec],
    instruction_key: &'static str,
    stimulus: &'static str,
    expected: &'static str,
    operation: Operation,
    answer_rule: AnswerRule,
    formatting: Formatting,
) -> MaterializedTaskSpec {
    MaterializedTaskSpec {
        generated_id,
        capability_id,
        template_id,
        template_version: 2,
        content_seed,
        stimulus_variant: 0,
        contrast_id,
        assignments,
        instruction_key,
        stimulus,
        expected,
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule,
        formatting,
        transition: StateTransition::None,
    }
}

pub(crate) const MATERIALIZED_TASKS: &[MaterializedTaskSpec] = &[
    task(
        "gen:rust:function.return_literal:copy-return-type:v2:copy:s0",
        "rust.function.return_literal.copy.return_type.introduce",
        "copy-return-type",
        0,
        None,
        ROW_A_ASSIGNMENTS,
        "copy.return_type_intro",
        "fn five() -> i32 { 5 }",
        "fn five() -> i32 { 5 }",
        Operation::Copy,
        AnswerRule::ExactTrimFinalNewline,
        Formatting::Disabled,
    ),
    task(
        "gen:rust:function.return_literal:fill-return-type-arrow:v2:fill:s0",
        "rust.function.return_literal.fill.return_type_arrow",
        "fill-return-type-arrow",
        0,
        None,
        ROW_A_ASSIGNMENTS,
        "fill.return_type_arrow",
        "fn five() ___ i32 { 5 }",
        "->",
        Operation::Fill,
        AnswerRule::ExactTrimFinalNewline,
        Formatting::Disabled,
    ),
    task(
        "gen:rust:function.return_literal:produce-return-type:v2:produce:s0",
        "rust.function.return_literal.produce.return_type.guided",
        "produce-return-type",
        0,
        None,
        ROW_A_ASSIGNMENTS,
        "produce.return_type_arrow",
        "fn five() ___ i32 { 5 }",
        "fn five() -> i32 { 5 }",
        Operation::Produce,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
    task(
        "gen:rust:function.return_literal:repair-return-type-arrow:v2:repair:s0",
        "rust.function.return_literal.repair.return_type_arrow",
        "repair-return-type-arrow",
        0,
        Some("typescript.return_type_colon"),
        ROW_A_ASSIGNMENTS,
        "repair.return_type_arrow",
        "fn five(): i32 { 5 }",
        "fn five() -> i32 { 5 }",
        Operation::Repair,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
    task(
        "gen:rust:function.return_literal:transform-add-return-type:v2:transform:s0",
        "rust.function.return_literal.transform.return_type.practice",
        "transform-add-return-type",
        0,
        Some("rust.return_type_omitted"),
        ROW_A_ASSIGNMENTS,
        "transform.add_return_type",
        "fn five() { 5 }",
        "fn five() -> i32 { 5 }",
        Operation::Transform,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
    task(
        "gen:rust:function.return_literal:copy-return-type:v2:copy:s1",
        "rust.function.return_literal.copy.return_type.introduce",
        "copy-return-type",
        1,
        None,
        ROW_B_ASSIGNMENTS,
        "copy.return_type_intro",
        "fn seven() -> u64 { 7 }",
        "fn seven() -> u64 { 7 }",
        Operation::Copy,
        AnswerRule::ExactTrimFinalNewline,
        Formatting::Disabled,
    ),
    task(
        "gen:rust:function.return_literal:fill-return-type-arrow:v2:fill:s1",
        "rust.function.return_literal.fill.return_type_arrow",
        "fill-return-type-arrow",
        1,
        None,
        ROW_B_ASSIGNMENTS,
        "fill.return_type_arrow",
        "fn seven() ___ u64 { 7 }",
        "->",
        Operation::Fill,
        AnswerRule::ExactTrimFinalNewline,
        Formatting::Disabled,
    ),
    task(
        "gen:rust:function.return_literal:produce-return-type:v2:produce:s1",
        "rust.function.return_literal.produce.return_type.guided",
        "produce-return-type",
        1,
        None,
        ROW_B_ASSIGNMENTS,
        "produce.return_type_arrow",
        "fn seven() ___ u64 { 7 }",
        "fn seven() -> u64 { 7 }",
        Operation::Produce,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
    task(
        "gen:rust:function.return_literal:repair-return-type-arrow:v2:repair:s1",
        "rust.function.return_literal.repair.return_type_arrow",
        "repair-return-type-arrow",
        1,
        Some("typescript.return_type_colon"),
        ROW_B_ASSIGNMENTS,
        "repair.return_type_arrow",
        "fn seven(): u64 { 7 }",
        "fn seven() -> u64 { 7 }",
        Operation::Repair,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
    task(
        "gen:rust:function.return_literal:transform-add-return-type:v2:transform:s1",
        "rust.function.return_literal.transform.return_type.practice",
        "transform-add-return-type",
        1,
        Some("rust.return_type_omitted"),
        ROW_B_ASSIGNMENTS,
        "transform.add_return_type",
        "fn seven() { 7 }",
        "fn seven() -> u64 { 7 }",
        Operation::Transform,
        AnswerRule::RustfmtExact,
        Formatting::Rustfmt,
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::guidance::GUIDANCE_PACKS;
    use crate::curriculum::guidance::resolution::{GuidanceAssignmentRef, resolve_guidance};
    use crate::curriculum::planner::planning::{
        CandidateSummary, GuidanceAvailability, build_practice_plan, plan_cardinality,
    };
    use crate::curriculum::planner::{CAPABILITIES, SET_PLANS};
    use crate::curriculum::procgen::expansion::{
        ExpandedTask, expand_template_from_compatibility_row, template_cardinality,
    };
    use crate::curriculum::procgen::{COMPATIBILITY_ROWS, SLOT_DOMAINS, TEMPLATES};

    fn assert_materialized_matches_expansion(
        source: &MaterializedTaskSpec,
        expanded: &ExpandedTask,
    ) {
        assert_eq!(expanded.generated_id, source.generated_id);
        assert_eq!(expanded.template_id, source.template_id);
        assert_eq!(expanded.template_version, source.template_version);
        assert_eq!(expanded.content_seed, source.content_seed);
        assert_eq!(expanded.stimulus_variant, source.stimulus_variant);
        assert_eq!(expanded.contrast_id.as_deref(), source.contrast_id);
        assert_eq!(expanded.assignments.len(), source.assignments.len());
        for (expanded_assignment, source_assignment) in
            expanded.assignments.iter().zip(source.assignments)
        {
            assert_eq!(expanded_assignment.slot, source_assignment.slot);
            assert_eq!(expanded_assignment.domain_id, source_assignment.domain_id);
            assert_eq!(
                expanded_assignment.domain_version,
                source_assignment.domain_version
            );
            assert_eq!(expanded_assignment.value_id, source_assignment.value_id);
            assert_eq!(expanded_assignment.rendered, source_assignment.rendered);
            assert_eq!(
                expanded_assignment.meaning_key.as_deref(),
                source_assignment.meaning_key
            );
        }
        assert_eq!(expanded.instruction_key, source.instruction_key);
        assert_eq!(expanded.stimulus, source.stimulus);
        assert_eq!(expanded.expected, source.expected);
        assert_eq!(expanded.system, source.system);
        assert_eq!(expanded.track, source.track);
        assert_eq!(expanded.semantic_frame, source.semantic_frame);
        assert_eq!(expanded.surface_frame, source.surface_frame);
        assert_eq!(expanded.operation, source.operation);
        assert_eq!(
            expanded
                .concepts
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            source.concepts
        );
        assert_eq!(
            expanded
                .constraints
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            source.constraints
        );
        assert_eq!(expanded.xp, source.xp);
        assert_eq!(expanded.answer_rule, source.answer_rule);
        assert_eq!(expanded.transition, source.transition);
    }

    fn candidates_for_row(row_index: usize) -> Vec<CandidateSummary> {
        MATERIALIZED_TASKS[row_index * 5..row_index * 5 + 5]
            .iter()
            .enumerate()
            .map(|(index, task)| CandidateSummary {
                rep_id: task.generated_id,
                catalog_order: u32::try_from(index + 1).expect("five positions fit in u32"),
                capability_id: task.capability_id,
                system: task.system,
                semantic_frame: task.semantic_frame,
                surface_frame: task.surface_frame,
                operation: task.operation,
                instruction_key: task.instruction_key,
                answer_rule: task.answer_rule,
                content_seed: Some(task.content_seed),
            })
            .collect()
    }

    fn guidance_availability(pack_index: usize) -> Vec<GuidanceAvailability> {
        let pack = &GUIDANCE_PACKS[pack_index];
        pack.instructions
            .iter()
            .map(|instruction| GuidanceAvailability {
                locale: pack.locale,
                pack_version: pack.version,
                instruction_key: instruction.key,
            })
            .collect()
    }

    #[test]
    fn source_is_exactly_two_reviewed_rows_and_ten_tasks() {
        assert_eq!(COMPATIBILITY_ROWS.len(), 2);
        assert_eq!(TEMPLATES.len(), 5);
        assert_eq!(MATERIALIZED_TASKS.len(), 10);
        assert_eq!(COMPATIBILITY_ROWS[0].id, "row-a-five-i32");
        assert_eq!(COMPATIBILITY_ROWS[1].id, "row-b-seven-u64");
        assert_eq!(COMPATIBILITY_ROWS[0].content_seed, 0);
        assert_eq!(COMPATIBILITY_ROWS[1].content_seed, 1);
        assert_eq!(ROW_A_ASSIGNMENTS.len(), 3);
        assert_eq!(ROW_B_ASSIGNMENTS.len(), 3);
        assert_ne!(ROW_A_ASSIGNMENTS, ROW_B_ASSIGNMENTS);

        let operations = [
            Operation::Copy,
            Operation::Fill,
            Operation::Produce,
            Operation::Repair,
            Operation::Transform,
        ];
        for (row_index, row) in MATERIALIZED_TASKS.chunks_exact(5).enumerate() {
            assert_eq!(
                row.iter().map(|task| task.operation).collect::<Vec<_>>(),
                operations
            );
            assert!(row.iter().all(|task| task.content_seed == row_index as u64));
            assert!(row.iter().all(|task| task.assignments
                == if row_index == 0 {
                    ROW_A_ASSIGNMENTS
                } else {
                    ROW_B_ASSIGNMENTS
                }));
        }
        for index in 0..5 {
            assert_ne!(
                MATERIALIZED_TASKS[index].stimulus,
                MATERIALIZED_TASKS[index + 5].stimulus
            );
            if index == 1 {
                assert_eq!(
                    MATERIALIZED_TASKS[index].expected,
                    MATERIALIZED_TASKS[index + 5].expected
                );
            } else {
                assert_ne!(
                    MATERIALIZED_TASKS[index].expected,
                    MATERIALIZED_TASKS[index + 5].expected
                );
            }
        }
    }

    #[test]
    fn checked_materialization_uses_only_explicit_compatibility_rows() {
        assert_eq!(template_cardinality(&TEMPLATES[0], SLOT_DOMAINS), Ok(8));
        for (row_index, row) in COMPATIBILITY_ROWS.iter().enumerate() {
            for (template_index, template) in TEMPLATES.iter().enumerate() {
                let expanded = expand_template_from_compatibility_row(template, SLOT_DOMAINS, row)
                    .expect("each reviewed compatibility row must expand");
                let source = &MATERIALIZED_TASKS[row_index * 5 + template_index];
                assert_materialized_matches_expansion(source, &expanded);

                let repeated = expand_template_from_compatibility_row(template, SLOT_DOMAINS, row)
                    .expect("the same row must reproduce exactly");
                assert_eq!(expanded, repeated);
            }
        }
    }

    #[test]
    fn materialized_metadata_validation_and_format_policy_are_exact() {
        for (index, task) in MATERIALIZED_TASKS.iter().enumerate() {
            assert!(!task.generated_id.is_empty());
            assert!(!task.capability_id.is_empty());
            assert!(!task.expected.is_empty());
            assert_eq!(task.template_version, 2);
            assert_eq!(task.stimulus_variant, 0);
            assert_eq!(task.system, "rust");
            assert_eq!(task.track, "syntax_basics");
            assert_eq!(task.semantic_frame, "function.no_params.return_literal");
            assert_eq!(task.surface_frame, "function.return_literal");
            assert_eq!(task.concepts, RUST_FUNCTION_CONCEPTS);
            assert_eq!(task.constraints, RUST_FUNCTION_CONSTRAINTS);
            assert_eq!(task.xp, 10);
            assert_eq!(task.transition, StateTransition::None);
            for earlier in &MATERIALIZED_TASKS[..index] {
                assert_ne!(earlier.generated_id, task.generated_id);
                assert_ne!(
                    (earlier.template_id, earlier.content_seed),
                    (task.template_id, task.content_seed)
                );
            }
            match task.operation {
                Operation::Copy | Operation::Fill => {
                    assert_eq!(task.answer_rule, AnswerRule::ExactTrimFinalNewline);
                    assert_eq!(task.formatting, Formatting::Disabled);
                }
                Operation::Produce | Operation::Repair | Operation::Transform => {
                    assert_eq!(task.answer_rule, AnswerRule::RustfmtExact);
                    assert_eq!(task.formatting, Formatting::Rustfmt);
                }
                other => panic!("unexpected operation {other:?}"),
            }
        }
        assert_eq!(MATERIALIZED_TASKS[1].expected, "->");
        assert_eq!(MATERIALIZED_TASKS[6].expected, "->");
        assert!(!MATERIALIZED_TASKS[2].stimulus.is_empty());
        assert!(!MATERIALIZED_TASKS[7].stimulus.is_empty());
    }

    #[test]
    fn guidance_resolves_for_both_rows_without_seed_selecting_locale() {
        for (row_index, row) in MATERIALIZED_TASKS.chunks_exact(5).enumerate() {
            for (task_index, task) in row.iter().enumerate() {
                let assignments = task
                    .assignments
                    .iter()
                    .map(|assignment| GuidanceAssignmentRef {
                        slot: assignment.slot,
                        rendered: assignment.rendered,
                        meaning_key: assignment.meaning_key,
                    })
                    .collect::<Vec<_>>();
                for pack in GUIDANCE_PACKS {
                    let resolved = resolve_guidance(
                        task.generated_id,
                        task.instruction_key,
                        &assignments,
                        GUIDANCE_PACKS,
                        pack.locale,
                        pack.version,
                    )
                    .expect("both reviewed rows must resolve in each accepted proof pack");
                    assert_eq!(resolved.provenance.locale, pack.locale);
                    assert_eq!(resolved.provenance.pack_version, 1);
                    assert!(!resolved.instruction.is_empty());
                    assert!(
                        !resolved.instruction.is_empty(),
                        "row {row_index} task {task_index}"
                    );
                }
            }
        }
    }

    #[test]
    fn each_correlated_row_builds_one_five_step_plan() {
        let availability = guidance_availability(0);
        let supported_rules = [AnswerRule::ExactTrimFinalNewline, AnswerRule::RustfmtExact];
        for row_index in 0..2 {
            let candidates = candidates_for_row(row_index);
            assert_eq!(
                plan_cardinality(
                    &SET_PLANS[0],
                    CAPABILITIES,
                    &candidates,
                    &availability,
                    "en",
                    1,
                    &supported_rules,
                ),
                Ok(1)
            );
            let plan = build_practice_plan(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                &availability,
                "en",
                1,
                &supported_rules,
                0,
            )
            .expect("one filtered compatibility row must build one coherent plan");
            assert_eq!(plan.reps.len(), 5);
            assert_eq!(plan.provenance.plan_cardinality, 1);
            assert_eq!(plan.provenance.guidance_locale, "en");
            assert_eq!(
                plan.reps
                    .iter()
                    .map(|rep| rep.rep_id.as_str())
                    .collect::<Vec<_>>(),
                MATERIALIZED_TASKS[row_index * 5..row_index * 5 + 5]
                    .iter()
                    .map(|task| task.generated_id)
                    .collect::<Vec<_>>()
            );
        }
    }
}
