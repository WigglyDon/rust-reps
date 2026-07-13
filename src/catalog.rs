use std::fmt;

use crate::curriculum::{DRILLS, DrillSpec};

pub type Drill = DrillSpec;

impl fmt::Display for Drill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = (
            self.track,
            self.skill,
            self.stage,
            self.task_type,
            self.difficulty,
            self.metadata.canonical_id,
            self.metadata.system,
            self.metadata.track,
            self.metadata.semantic_frame,
            self.metadata.surface_frame,
            self.metadata.operation,
            self.metadata.operation.as_str(),
            self.metadata.concepts,
            self.metadata.constraints,
            self.metadata.xp,
            self.metadata.answer_rule,
            self.metadata.transition,
            crate::curriculum::Operation::ALL,
            self.instruction_key,
            self.assignments,
            self.stimulus,
            self.expected,
            self.formatting,
            self.capability_id,
            self.generated_provenance,
            self.validator,
            self.prerequisites,
            self.tags,
        );
        for assignment in self.assignments {
            let _ = (
                assignment.slot,
                assignment.domain_id,
                assignment.domain_version,
                assignment.value_id,
                assignment.rendered,
                assignment.meaning_key,
            );
        }
        if let Some(provenance) = self.generated_provenance {
            let _ = (
                provenance.generated_id,
                provenance.template_id,
                provenance.template_version,
                provenance.content_seed,
                provenance.stimulus_variant,
                provenance.contrast_id,
            );
        }
        write!(f, "{}  {}", self.id, self.expected)
    }
}

pub fn drills() -> &'static [Drill] {
    DRILLS
}

pub fn find_drill(id: &str) -> Option<&'static Drill> {
    DRILLS.iter().find(|drill| drill.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::{
        AnswerRule, DRILL_SHARDS, Formatting, Operation, Stage, StateTransition, TaskType,
        Validator,
    };
    use crate::lab;
    use std::collections::HashSet;

    #[test]
    fn curriculum_has_registered_shards() {
        assert!(!DRILL_SHARDS.is_empty());
    }

    #[test]
    fn no_registered_shard_is_empty() {
        for (index, shard) in DRILL_SHARDS.iter().enumerate() {
            assert!(
                !shard.is_empty(),
                "expected registered shard {} to be non-empty",
                index
            );
        }
    }

    #[test]
    fn drills_include_every_registered_shard_drill() {
        let catalog_drill_ids: Vec<&str> = drills().iter().map(|drill| drill.id).collect();
        let flat_shard_ids: Vec<&str> = DRILL_SHARDS
            .iter()
            .flat_map(|shard| shard.iter())
            .map(|drill| drill.id)
            .collect();
        assert_eq!(catalog_drill_ids, flat_shard_ids);
    }

    #[test]
    fn drill_ids_are_unique() {
        let mut ids = HashSet::new();
        for drill in drills() {
            assert!(!drill.id.is_empty(), "drill id must not be empty");
            assert!(
                ids.insert(drill.id),
                "expected drill ids to be unique, found duplicate {}",
                drill.id
            );
        }
    }

    #[test]
    fn canonical_ids_are_unique() {
        let mut canonical_ids = HashSet::new();
        for drill in drills() {
            let canonical_id = drill.metadata.canonical_id;
            assert!(
                !canonical_id.is_empty(),
                "drill {} canonical id must not be empty",
                drill.id
            );
            assert!(
                canonical_ids.insert(canonical_id),
                "expected canonical ids to be unique, found duplicate {}",
                canonical_id
            );
        }
    }

    #[test]
    fn drill_ids_are_three_ascii_digits() {
        for drill in drills() {
            assert_eq!(drill.id.len(), 3, "expected drill id to be 3 characters");
            assert!(
                drill.id.bytes().all(|byte| byte.is_ascii_digit()),
                "expected drill id {} to be digits",
                drill.id
            );
        }
    }

    #[test]
    fn drills_are_sorted_in_ascending_id_order() {
        for i in 1..drills().len() {
            let previous = drills()[i - 1].id;
            let current = drills()[i].id;
            assert!(previous < current, "expected drills sorted ascending by id");
        }
    }

    #[test]
    fn metadata_required_fields_are_present() {
        for drill in drills() {
            let metadata = drill.metadata;
            assert!(
                !metadata.system.is_empty(),
                "drill {} system must not be empty",
                drill.id
            );
            assert!(
                !metadata.track.is_empty(),
                "drill {} track must not be empty",
                drill.id
            );
            assert!(
                !metadata.semantic_frame.is_empty(),
                "drill {} semantic frame must not be empty",
                drill.id
            );
            assert!(
                !metadata.surface_frame.is_empty(),
                "drill {} surface frame must not be empty",
                drill.id
            );
            assert!(
                !metadata.concepts.is_empty(),
                "drill {} must have at least one concept",
                drill.id
            );
            assert!(
                !metadata.constraints.is_empty(),
                "drill {} must have at least one constraint",
                drill.id
            );
            assert!(metadata.xp > 0, "drill {} xp must be positive", drill.id);
        }
    }

    #[test]
    fn metadata_rules_match_explicit_operation_contract() {
        for (index, drill) in drills().iter().enumerate() {
            assert_eq!(drill.metadata.transition, StateTransition::None);
            if index < 12 {
                assert_eq!(drill.metadata.operation, Operation::Copy);
                assert_eq!(
                    drill.metadata.answer_rule,
                    AnswerRule::ExactTrimFinalNewline
                );
                assert_eq!(drill.formatting, Formatting::Disabled);
            } else {
                match drill.metadata.operation {
                    Operation::Copy | Operation::Fill => {
                        assert_eq!(
                            drill.metadata.answer_rule,
                            AnswerRule::ExactTrimFinalNewline
                        );
                        assert_eq!(drill.formatting, Formatting::Disabled);
                    }
                    Operation::Produce | Operation::Repair | Operation::Transform => {
                        assert_eq!(drill.metadata.answer_rule, AnswerRule::RustfmtExact);
                        assert_eq!(drill.formatting, Formatting::Rustfmt);
                    }
                    operation => panic!("unexpected admitted operation {operation:?}"),
                }
            }
        }
    }

    #[test]
    fn canonical_id_shape_matches_static_metadata_version() {
        for drill in drills() {
            let canonical_id = drill.metadata.canonical_id;
            let operation = drill.metadata.operation.as_str();
            let template_version = if drill.id <= "012" { 1 } else { 2 };
            assert!(
                canonical_id.starts_with(&format!(
                    "{}.{}.",
                    drill.metadata.system, drill.metadata.surface_frame
                )),
                "drill {} canonical id {} should start with system.surface_frame",
                drill.id,
                canonical_id
            );
            assert!(
                canonical_id.contains(&format!(".v{template_version}.{}.", operation)),
                "drill {} canonical id {} should include template version and operation",
                drill.id,
                canonical_id
            );
            assert!(
                canonical_id.ends_with(&format!(".0{}", drill.id)),
                "drill {} canonical id {} should end with a stable seed number",
                drill.id,
                canonical_id
            );
        }
    }

    #[test]
    fn operation_catalog_keeps_declared_v1_operations_inspectable() {
        let operation_names: Vec<&str> = Operation::ALL
            .iter()
            .map(|operation| operation.as_str())
            .collect();

        assert_eq!(
            operation_names,
            vec![
                "copy",
                "recognize",
                "fill",
                "read",
                "produce",
                "repair",
                "transform",
                "compose",
                "explain",
            ]
        );
    }

    #[test]
    fn expected_answers_are_not_empty() {
        for drill in drills() {
            assert!(
                !drill.expected.is_empty(),
                "expected drill {} to have a non-empty expected answer",
                drill.id
            );
        }
    }

    #[test]
    fn expected_answers_are_one_line() {
        for drill in drills() {
            assert!(
                !drill.expected.contains('\n'),
                "expected drill {} expected answer to be one line",
                drill.id
            );
            assert!(
                !drill.expected.contains('\r'),
                "expected drill {} expected answer to be one line",
                drill.id
            );
        }
    }

    #[test]
    fn every_admitted_task_has_an_inspectable_stimulus() {
        for drill in drills() {
            assert!(
                !drill.stimulus.is_empty(),
                "expected drill {} to have a non-empty stimulus",
                drill.id
            );
        }
    }

    #[test]
    fn admitted_stimuli_are_one_line() {
        for drill in drills() {
            assert!(
                !drill.stimulus.contains('\n'),
                "expected drill {} stimulus to be one line",
                drill.id
            );
            assert!(
                !drill.stimulus.contains('\r'),
                "expected drill {} stimulus to be one line",
                drill.id
            );
        }
    }

    #[test]
    fn difficulty_is_in_approved_range() {
        for drill in drills() {
            assert!(
                (1..=10).contains(&drill.difficulty),
                "expected drill {} difficulty to be 1..=10",
                drill.id
            );
        }
    }

    #[test]
    fn prerequisites_reference_existing_drills() {
        let existing_ids: HashSet<&'static str> = drills().iter().map(|drill| drill.id).collect();
        for drill in drills() {
            for prereq in drill.prerequisites {
                assert!(
                    existing_ids.contains(prereq),
                    "expected drill {} prerequisite {} to exist",
                    drill.id,
                    prereq
                );
            }
        }
    }

    #[test]
    fn validator_is_supported_for_task_type_and_operation() {
        for drill in drills() {
            match drill.task_type {
                TaskType::TypeExact => {
                    assert_eq!(drill.validator, Validator::Exact);
                }
                TaskType::RustCode => assert_eq!(drill.validator, Validator::RustfmtExact),
            }
        }
    }

    #[test]
    fn operation_order_and_validation_policy_are_explicit_for_both_rows() {
        let operations = [
            Operation::Copy,
            Operation::Fill,
            Operation::Produce,
            Operation::Repair,
            Operation::Transform,
        ];
        for row in [&drills()[12..17], &drills()[17..22]] {
            assert_eq!(
                row.iter()
                    .map(|drill| drill.metadata.operation)
                    .collect::<Vec<_>>(),
                operations
            );
            assert_eq!(row[0].stage, Stage::Copy);
            assert_eq!(row[1].stage, Stage::Fill);
            assert_eq!(row[2].stage, Stage::Produce);
            assert_eq!(row[3].stage, Stage::Repair);
            assert_eq!(row[4].stage, Stage::Transform);
        }
    }

    #[test]
    fn catalog_has_exactly_twenty_two_drills() {
        assert_eq!(drills().len(), 22);
    }

    #[test]
    fn current_core_task_migration_is_exact_and_inert() {
        let expected_ids = [
            "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
        ];
        let expected_canonical_ids = [
            "rust.function.main.v1.copy.0001",
            "rust.macro.println.v1.copy.0002",
            "rust.binding.integer.v1.copy.0003",
            "rust.binding.string.v1.copy.0004",
            "rust.binding.mutable.v1.copy.0005",
            "rust.assignment.add_assign.v1.copy.0006",
            "rust.binding.boolean.v1.copy.0007",
            "rust.binding.type_annotation.v1.copy.0008",
            "rust.binding.array.v1.copy.0009",
            "rust.function.return_expression.v1.copy.0010",
            "rust.control.if_empty.v1.copy.0011",
            "rust.control.while_empty.v1.copy.0012",
        ];
        let expected_task_bytes = [
            "fn main() {}",
            "println!(\"Rust Reps\");",
            "let answer = 42;",
            "let name = \"Rust Reps\";",
            "let mut score = 0;",
            "score += 1;",
            "let passed = true;",
            "let score: i32 = 0;",
            "let scores = [1, 2, 3];",
            "fn add_one(x: i32) -> i32 { x + 1 }",
            "if score > 0 {}",
            "while score < 10 {}",
        ];

        assert_eq!(&drills()[..12].len(), &expected_ids.len());
        for (index, drill) in drills()[..12].iter().enumerate() {
            assert_eq!(drill.id, expected_ids[index]);
            assert_eq!(drill.metadata.canonical_id, expected_canonical_ids[index]);
            assert_eq!(drill.instruction_key, "copy.exact_line");
            assert!(drill.assignments.is_empty());
            assert_eq!(drill.capability_id, None);
            assert_eq!(drill.generated_provenance, None);
            assert_eq!(drill.stimulus, expected_task_bytes[index]);
            assert_eq!(drill.expected, expected_task_bytes[index]);
            assert_eq!(drill.stimulus, drill.expected);
            assert_eq!(drill.formatting, Formatting::Disabled);
            assert_eq!(drill.validator, Validator::Exact);
            assert!(!drill.metadata.canonical_id.starts_with("gen:"));
        }
    }

    #[test]
    fn lookup_first_middle_and_last_drill_ids() {
        let first = find_drill("001").unwrap();
        assert_eq!(first.id, "001");
        assert_eq!(
            first.metadata.canonical_id,
            "rust.function.main.v1.copy.0001"
        );
        assert_eq!(first.expected, "fn main() {}");

        let middle = find_drill("006").unwrap();
        assert_eq!(middle.id, "006");
        assert_eq!(
            middle.metadata.canonical_id,
            "rust.assignment.add_assign.v1.copy.0006"
        );
        assert_eq!(middle.expected, "score += 1;");

        let last_legacy = find_drill("012").unwrap();
        assert_eq!(last_legacy.id, "012");
        assert_eq!(
            last_legacy.metadata.canonical_id,
            "rust.control.while_empty.v1.copy.0012"
        );
        assert_eq!(last_legacy.expected, "while score < 10 {}");

        let last = find_drill("022").unwrap();
        assert_eq!(last.id, "022");
        assert_eq!(
            last.metadata.canonical_id,
            "rust.function.return_literal.v2.transform.0022"
        );
        assert_eq!(last.expected, "fn seven() -> u64 { 7 }");
    }

    #[test]
    fn every_short_one_shot_id_still_resolves() {
        for id in [
            "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
            "013", "014", "015", "016", "017", "018", "019", "020", "021", "022",
        ] {
            let drill = find_drill(id).expect("expected short id to resolve");
            assert_eq!(drill.id, id);
        }
    }

    #[test]
    fn admitted_short_ids_are_exactly_001_through_022_without_gaps() {
        let expected = (1..=22)
            .map(|number| format!("{number:03}"))
            .collect::<Vec<_>>();
        assert_eq!(
            drills().iter().map(|drill| drill.id).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn admitted_varied_tasks_match_checked_materialization_field_for_field() {
        use crate::curriculum::procgen::materialized::MATERIALIZED_TASKS;

        assert_eq!(MATERIALIZED_TASKS.len(), 10);
        for (drill, source) in drills()[12..].iter().zip(MATERIALIZED_TASKS) {
            let provenance = drill
                .generated_provenance
                .expect("varied admitted task must carry generated provenance");
            assert_eq!(provenance.generated_id, source.generated_id);
            assert_eq!(provenance.template_id, source.template_id);
            assert_eq!(provenance.template_version, source.template_version);
            assert_eq!(provenance.content_seed, source.content_seed);
            assert_eq!(provenance.stimulus_variant, source.stimulus_variant);
            assert_eq!(provenance.contrast_id, source.contrast_id);
            assert_eq!(drill.capability_id, Some(source.capability_id));
            assert_eq!(drill.instruction_key, source.instruction_key);
            assert_eq!(drill.stimulus, source.stimulus);
            assert_eq!(drill.expected, source.expected);
            assert_eq!(drill.metadata.system, source.system);
            assert_eq!(drill.metadata.track, source.track);
            assert_eq!(drill.metadata.semantic_frame, source.semantic_frame);
            assert_eq!(drill.metadata.surface_frame, source.surface_frame);
            assert_eq!(drill.metadata.operation, source.operation);
            assert_eq!(drill.metadata.concepts, source.concepts);
            assert_eq!(drill.metadata.constraints, source.constraints);
            assert_eq!(drill.metadata.xp, source.xp);
            assert_eq!(drill.metadata.answer_rule, source.answer_rule);
            assert_eq!(drill.formatting, source.formatting);
            assert_eq!(drill.metadata.transition, source.transition);
            assert_eq!(drill.assignments.len(), source.assignments.len());
            for (assignment, source_assignment) in drill.assignments.iter().zip(source.assignments)
            {
                assert_eq!(assignment.slot, source_assignment.slot);
                assert_eq!(assignment.domain_id, source_assignment.domain_id);
                assert_eq!(assignment.domain_version, source_assignment.domain_version);
                assert_eq!(assignment.value_id, source_assignment.value_id);
                assert_eq!(assignment.rendered, source_assignment.rendered);
                assert_eq!(assignment.meaning_key, source_assignment.meaning_key);
            }
        }
    }

    #[test]
    fn trusted_rustfmt_exact_answers_format_successfully_and_match_themselves() {
        for drill in drills()
            .iter()
            .filter(|drill| drill.metadata.answer_rule == AnswerRule::RustfmtExact)
        {
            assert_eq!(
                crate::validators::validate(
                    crate::validators::ValidationInput::new(drill.expected, drill.expected),
                    crate::validators::ValidatorSpec::rustfmt_exact(),
                ),
                crate::validators::ValidationOutcome::Correct,
                "trusted expected bytes for {} must be accepted by real rustfmt",
                drill.id
            );
        }
    }

    #[test]
    fn lookup_missing_drill_returns_none() {
        assert!(find_drill("999").is_none());
    }

    #[test]
    fn visible_drill_order_is_stable_for_list_help_rendering() {
        let ids_from_catalog: Vec<&str> = drills().iter().map(|drill| drill.id).collect();
        let sorted_ids: Vec<&str> = {
            let mut ids = ids_from_catalog.clone();
            ids.sort();
            ids
        };
        assert_eq!(ids_from_catalog, sorted_ids);
    }

    #[test]
    fn lab_packets_stay_outside_normal_catalog() {
        let lab_packet = lab::exercise_packet_at(0);

        assert!(lab_packet.id.starts_with("LAB-"));
        assert!(find_drill(lab_packet.id.as_ref()).is_none());
        assert!(drills().iter().all(|drill| !drill.id.starts_with("LAB-")));
    }
}
