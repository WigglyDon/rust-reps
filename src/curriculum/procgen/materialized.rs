use super::{RUST_FUNCTION_CONCEPTS, RUST_FUNCTION_CONSTRAINTS};
use crate::curriculum::{AnswerRule, Operation, StateTransition};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct MaterializedAssignmentSpec {
    slot: &'static str,
    domain_id: &'static str,
    domain_version: u16,
    value_id: &'static str,
    rendered: &'static str,
    meaning_key: Option<&'static str>,
}

struct MaterializedTaskSpec {
    generated_id: &'static str,
    capability_id: &'static str,
    template_id: &'static str,
    template_version: u16,
    content_seed: u64,
    stimulus_variant: u16,
    contrast_id: Option<&'static str>,
    assignments: &'static [MaterializedAssignmentSpec],
    instruction_key: &'static str,
    stimulus: &'static str,
    expected: &'static str,
    system: &'static str,
    track: &'static str,
    semantic_frame: &'static str,
    surface_frame: &'static str,
    operation: Operation,
    concepts: &'static [&'static str],
    constraints: &'static [&'static str],
    xp: u32,
    answer_rule: AnswerRule,
    transition: StateTransition,
}

const MATERIALIZED_ASSIGNMENTS: &[MaterializedAssignmentSpec] = &[
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

const MATERIALIZED_TASKS: &[MaterializedTaskSpec] = &[
    MaterializedTaskSpec {
        generated_id: "gen:rust:function.return_literal:copy-line:v1:copy:s0",
        capability_id: "rust.function.return_literal.copy.full_model",
        template_id: "copy-line",
        template_version: 1,
        content_seed: 0,
        stimulus_variant: 0,
        contrast_id: None,
        assignments: MATERIALIZED_ASSIGNMENTS,
        instruction_key: "copy.exact_line",
        stimulus: "fn five() -> i32 { 5 }",
        expected: "fn five() -> i32 { 5 }",
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Copy,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
    },
    MaterializedTaskSpec {
        generated_id: "gen:rust:function.return_literal:fill-return-value:v1:fill:s0",
        capability_id: "rust.function.return_literal.fill.return_value",
        template_id: "fill-return-value",
        template_version: 1,
        content_seed: 0,
        stimulus_variant: 0,
        contrast_id: None,
        assignments: MATERIALIZED_ASSIGNMENTS,
        instruction_key: "fill.return_value",
        stimulus: "fn five() -> i32 { ___ }",
        expected: "fn five() -> i32 { 5 }",
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Fill,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
    },
    MaterializedTaskSpec {
        generated_id: "gen:rust:function.return_literal:produce-return-literal:v1:produce:s0",
        capability_id: "rust.function.return_literal.produce.constrained",
        template_id: "produce-return-literal",
        template_version: 1,
        content_seed: 0,
        stimulus_variant: 0,
        contrast_id: None,
        assignments: MATERIALIZED_ASSIGNMENTS,
        instruction_key: "produce.return_literal",
        stimulus: "",
        expected: "fn five() -> i32 { 5 }",
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Produce,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
    },
    MaterializedTaskSpec {
        generated_id: "gen:rust:function.return_literal:repair-return-type-arrow:v1:repair:s0",
        capability_id: "rust.function.return_literal.repair.return_type_arrow",
        template_id: "repair-return-type-arrow",
        template_version: 1,
        content_seed: 0,
        stimulus_variant: 0,
        contrast_id: Some("typescript.return_type_colon"),
        assignments: MATERIALIZED_ASSIGNMENTS,
        instruction_key: "repair.return_type_arrow",
        stimulus: "fn five(): i32 { 5 }",
        expected: "fn five() -> i32 { 5 }",
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Repair,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
    },
    MaterializedTaskSpec {
        generated_id: "gen:rust:function.return_literal:transform-tail-expression:v1:transform:s0",
        capability_id: "rust.function.return_literal.transform.tail_expression",
        template_id: "transform-tail-expression",
        template_version: 1,
        content_seed: 0,
        stimulus_variant: 0,
        contrast_id: None,
        assignments: MATERIALIZED_ASSIGNMENTS,
        instruction_key: "transform.tail_expression",
        stimulus: "fn five() -> i32 { return 5; }",
        expected: "fn five() -> i32 { 5 }",
        system: "rust",
        track: "syntax_basics",
        semantic_frame: "function.no_params.return_literal",
        surface_frame: "function.return_literal",
        operation: Operation::Transform,
        concepts: RUST_FUNCTION_CONCEPTS,
        constraints: RUST_FUNCTION_CONSTRAINTS,
        xp: 10,
        answer_rule: AnswerRule::ExactTrimFinalNewline,
        transition: StateTransition::None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::guidance::GUIDANCE_PACKS;
    use crate::curriculum::guidance::resolution::resolve_guidance;
    use crate::curriculum::planner::planning::{
        CandidateSummary, GuidanceAvailability, build_practice_plan, plan_cardinality,
    };
    use crate::curriculum::planner::{CAPABILITIES, SET_PLANS};
    use crate::curriculum::procgen::expansion::{
        ExpandedTask, ExpansionError, expand_template, template_cardinality,
    };
    use crate::curriculum::procgen::{SLOT_DOMAINS, TEMPLATES};

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

    fn candidate_fixtures() -> Vec<CandidateSummary> {
        MATERIALIZED_TASKS
            .iter()
            .enumerate()
            .map(|(index, task)| CandidateSummary {
                rep_id: task.generated_id,
                catalog_order: u32::try_from(index + 1)
                    .expect("five test-only fixture positions fit in u32"),
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
    fn materialized_source_is_exact_unique_and_tiny() {
        assert_eq!(TEMPLATES.len(), 5);
        assert_eq!(MATERIALIZED_TASKS.len(), 5);
        assert_eq!(SET_PLANS.len(), 1);
        assert_eq!(SET_PLANS[0].steps.len(), 5);
        assert_eq!(
            MATERIALIZED_ASSIGNMENTS,
            &[
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
            ]
        );

        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.generated_id)
                .collect::<Vec<_>>(),
            vec![
                "gen:rust:function.return_literal:copy-line:v1:copy:s0",
                "gen:rust:function.return_literal:fill-return-value:v1:fill:s0",
                "gen:rust:function.return_literal:produce-return-literal:v1:produce:s0",
                "gen:rust:function.return_literal:repair-return-type-arrow:v1:repair:s0",
                "gen:rust:function.return_literal:transform-tail-expression:v1:transform:s0",
            ]
        );
        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.capability_id)
                .collect::<Vec<_>>(),
            vec![
                "rust.function.return_literal.copy.full_model",
                "rust.function.return_literal.fill.return_value",
                "rust.function.return_literal.produce.constrained",
                "rust.function.return_literal.repair.return_type_arrow",
                "rust.function.return_literal.transform.tail_expression",
            ]
        );
        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.template_id)
                .collect::<Vec<_>>(),
            vec![
                "copy-line",
                "fill-return-value",
                "produce-return-literal",
                "repair-return-type-arrow",
                "transform-tail-expression",
            ]
        );
        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.operation)
                .collect::<Vec<_>>(),
            vec![
                Operation::Copy,
                Operation::Fill,
                Operation::Produce,
                Operation::Repair,
                Operation::Transform,
            ]
        );
        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.instruction_key)
                .collect::<Vec<_>>(),
            vec![
                "copy.exact_line",
                "fill.return_value",
                "produce.return_literal",
                "repair.return_type_arrow",
                "transform.tail_expression",
            ]
        );
        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.stimulus)
                .collect::<Vec<_>>(),
            vec![
                "fn five() -> i32 { 5 }",
                "fn five() -> i32 { ___ }",
                "",
                "fn five(): i32 { 5 }",
                "fn five() -> i32 { return 5; }",
            ]
        );

        for (index, task) in MATERIALIZED_TASKS.iter().enumerate() {
            assert!(!task.generated_id.is_empty());
            assert!(!task.capability_id.is_empty());
            assert!(!task.expected.is_empty());
            assert_eq!(task.expected, "fn five() -> i32 { 5 }");
            assert_eq!(task.template_version, 1);
            assert_eq!(task.content_seed, 0);
            assert_eq!(task.stimulus_variant, 0);
            assert_eq!(task.assignments, MATERIALIZED_ASSIGNMENTS);
            assert_eq!(
                task.assignments
                    .iter()
                    .map(|assignment| assignment.slot)
                    .collect::<Vec<_>>(),
                vec!["NAME", "TYPE", "VALUE"]
            );
            for (assignment_index, assignment) in task.assignments.iter().enumerate() {
                assert!(!assignment.slot.is_empty());
                assert!(!assignment.domain_id.is_empty());
                assert!(assignment.domain_version > 0);
                assert!(!assignment.value_id.is_empty());
                assert!(!assignment.rendered.is_empty());
                assert!(assignment.meaning_key.is_none());
                for earlier in &task.assignments[..assignment_index] {
                    assert_ne!(earlier.slot, assignment.slot);
                }
            }
            assert_eq!(task.system, "rust");
            assert_eq!(task.track, "syntax_basics");
            assert_eq!(task.semantic_frame, "function.no_params.return_literal");
            assert_eq!(task.surface_frame, "function.return_literal");
            assert_eq!(task.concepts, RUST_FUNCTION_CONCEPTS);
            assert_eq!(task.constraints, RUST_FUNCTION_CONSTRAINTS);
            assert_eq!(task.xp, 10);
            assert_eq!(task.answer_rule, AnswerRule::ExactTrimFinalNewline);
            assert_eq!(task.transition, StateTransition::None);

            let template = TEMPLATES
                .iter()
                .find(|template| {
                    template.id == task.template_id && template.version == task.template_version
                })
                .expect("materialized task must reference a known template");
            let capability = CAPABILITIES
                .iter()
                .find(|capability| capability.id == task.capability_id)
                .expect("materialized task must reference a known capability");
            assert_eq!(template.system, capability.response_system);
            assert_eq!(template.semantic_frame, capability.semantic_frame);
            assert_eq!(template.surface_frame, capability.response_surface_frame);
            assert_eq!(template.operation, capability.operation);
            assert_eq!(task.operation, template.operation);
            assert_eq!(task.operation, capability.operation);
            assert_eq!(task.instruction_key, template.instruction_key);

            for earlier in &MATERIALIZED_TASKS[..index] {
                assert_ne!(earlier.generated_id, task.generated_id);
                assert_ne!(
                    (
                        earlier.template_id,
                        earlier.template_version,
                        earlier.content_seed,
                    ),
                    (task.template_id, task.template_version, task.content_seed)
                );
                assert_ne!(earlier.capability_id, task.capability_id);
                assert_ne!(
                    (earlier.stimulus, earlier.expected, earlier.operation),
                    (task.stimulus, task.expected, task.operation)
                );
                assert_ne!(
                    (earlier.instruction_key, earlier.operation),
                    (task.instruction_key, task.operation)
                );
            }
        }

        assert_eq!(
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.contrast_id)
                .collect::<Vec<_>>(),
            vec![None, None, None, Some("typescript.return_type_colon"), None,]
        );
        assert!(!MATERIALIZED_TASKS[0].stimulus.is_empty());
        assert!(!MATERIALIZED_TASKS[1].stimulus.is_empty());
        assert!(MATERIALIZED_TASKS[2].stimulus.is_empty());
        assert!(!MATERIALIZED_TASKS[3].stimulus.is_empty());
        assert!(!MATERIALIZED_TASKS[4].stimulus.is_empty());
    }

    #[test]
    fn every_materialized_field_matches_pure_expansion() {
        let mut total_valid_seeds = 0_u64;
        for (template, source) in TEMPLATES.iter().zip(MATERIALIZED_TASKS) {
            assert_eq!(template_cardinality(template, SLOT_DOMAINS), Ok(1));
            total_valid_seeds += 1;
            let expanded = expand_template(template, SLOT_DOMAINS, 0)
                .expect("each accepted seed-zero template must expand");
            assert_materialized_matches_expansion(source, &expanded);
            assert_eq!(
                expand_template(template, SLOT_DOMAINS, 1),
                Err(ExpansionError::SeedOutOfRange {
                    seed: 1,
                    cardinality: 1,
                })
            );
        }
        assert_eq!(total_valid_seeds, 5);
    }

    #[test]
    fn exact_guidance_resolves_for_every_materialized_task_in_both_packs() {
        let english = [
            "Copy this line exactly:",
            "Complete the missing return value:",
            "Write a Rust function named five with no parameters that returns the i32 value 5 using a tail expression:",
            "Repair the return-type syntax:",
            "Rewrite the function to use a tail expression:",
        ];
        let ukrainian = [
            "Скопіюйте цей рядок без змін:",
            "Заповніть пропущене значення, що повертається:",
            "Напишіть функцію Rust з назвою five без параметрів, яка повертає значення 5 типу i32 за допомогою хвостового виразу:",
            "Виправте синтаксис типу повернення:",
            "Перепишіть функцію, використовуючи хвостовий вираз:",
        ];

        for (index, template) in TEMPLATES.iter().enumerate() {
            let task = expand_template(template, SLOT_DOMAINS, 0)
                .expect("each accepted seed-zero template must expand");
            let generated_id = task.generated_id.clone();
            let instruction_key = task.instruction_key.clone();
            let stimulus = task.stimulus.clone();
            let expected = task.expected.clone();

            for (locale, pack_index, exact_instruction) in
                [("en", 0, english[index]), ("uk", 1, ukrainian[index])]
            {
                let resolved = resolve_guidance(&task, GUIDANCE_PACKS, locale, 1)
                    .expect("each exact proof instruction must resolve");
                assert_eq!(resolved.instruction, exact_instruction);
                assert!(!resolved.instruction.ends_with('\n'));
                assert!(!resolved.instruction.ends_with('\r'));
                assert_eq!(resolved.provenance.core_generated_id, generated_id);
                assert_eq!(resolved.provenance.locale, locale);
                assert_eq!(resolved.provenance.pack_version, 1);
                assert_eq!(resolved.provenance.instruction_key, instruction_key);
                assert_eq!(GUIDANCE_PACKS[pack_index].locale, locale);
            }

            assert_eq!(task.generated_id, generated_id);
            assert_eq!(task.instruction_key, instruction_key);
            assert_eq!(task.stimulus, stimulus);
            assert_eq!(task.expected, expected);
        }
    }

    #[test]
    fn materialized_candidates_build_the_exact_inert_five_step_plan() {
        let candidates = candidate_fixtures();
        let english_availability = guidance_availability(0);
        let ukrainian_availability = guidance_availability(1);
        let supported_rules = [AnswerRule::ExactTrimFinalNewline];

        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                &english_availability,
                "en",
                1,
                &supported_rules,
            ),
            Ok(1)
        );
        assert_eq!(
            plan_cardinality(
                &SET_PLANS[0],
                CAPABILITIES,
                &candidates,
                &ukrainian_availability,
                "uk",
                1,
                &supported_rules,
            ),
            Ok(1)
        );

        let english_plan = build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            &candidates,
            &english_availability,
            "en",
            1,
            &supported_rules,
            0,
        )
        .expect("English proof availability must build one exact plan");
        let ukrainian_plan = build_practice_plan(
            &SET_PLANS[0],
            CAPABILITIES,
            &candidates,
            &ukrainian_availability,
            "uk",
            1,
            &supported_rules,
            0,
        )
        .expect("Ukrainian proof availability must build one exact plan");

        assert_eq!(english_plan.provenance.plan_seed, 0);
        assert_eq!(english_plan.provenance.plan_cardinality, 1);
        assert_eq!(english_plan.provenance.guidance_locale, "en");
        assert_eq!(english_plan.provenance.guidance_pack_version, 1);
        assert_eq!(ukrainian_plan.provenance.plan_seed, 0);
        assert_eq!(ukrainian_plan.provenance.plan_cardinality, 1);
        assert_eq!(ukrainian_plan.provenance.guidance_locale, "uk");
        assert_eq!(ukrainian_plan.provenance.guidance_pack_version, 1);
        assert_eq!(
            english_plan.provenance.set_id,
            ukrainian_plan.provenance.set_id
        );
        assert_eq!(
            english_plan.provenance.set_version,
            ukrainian_plan.provenance.set_version
        );
        assert_eq!(
            english_plan.provenance.supported_answer_rules,
            ukrainian_plan.provenance.supported_answer_rules
        );
        assert_eq!(
            english_plan.provenance.supported_answer_rules,
            supported_rules
        );
        assert_eq!(english_plan.reps, ukrainian_plan.reps);
        assert_eq!(
            english_plan
                .reps
                .iter()
                .map(|rep| rep.rep_id.as_str())
                .collect::<Vec<_>>(),
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.generated_id)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            english_plan
                .reps
                .iter()
                .map(|rep| rep.step_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "copy-full-form",
                "fill-return-value",
                "produce-from-intent",
                "repair-return-type-arrow",
                "transform-tail-expression",
            ]
        );
        assert_eq!(
            english_plan
                .reps
                .iter()
                .map(|rep| rep.capability_id.as_str())
                .collect::<Vec<_>>(),
            MATERIALIZED_TASKS
                .iter()
                .map(|task| task.capability_id)
                .collect::<Vec<_>>()
        );
    }
}
