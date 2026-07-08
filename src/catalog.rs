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
            self.prompt,
            self.expected,
            self.validator,
            self.prerequisites,
            self.tags,
        );
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
        AnswerRule, DRILL_SHARDS, Operation, StateTransition, TaskType, Validator,
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
    fn metadata_rules_match_current_exact_rep_contract() {
        for drill in drills() {
            assert_eq!(drill.metadata.operation, Operation::Copy);
            assert_eq!(
                drill.metadata.answer_rule,
                AnswerRule::ExactTrimFinalNewline
            );
            assert_eq!(drill.metadata.transition, StateTransition::None);
        }
    }

    #[test]
    fn canonical_id_shape_matches_static_metadata_v1() {
        for drill in drills() {
            let canonical_id = drill.metadata.canonical_id;
            let operation = drill.metadata.operation.as_str();
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
                canonical_id.contains(&format!(".v1.{}.", operation)),
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
    fn prompts_are_not_empty() {
        for drill in drills() {
            assert!(
                !drill.prompt.is_empty(),
                "expected drill {} to have a non-empty prompt",
                drill.id
            );
        }
    }

    #[test]
    fn prompts_for_type_exact_are_one_line() {
        for drill in drills() {
            assert_eq!(drill.task_type, TaskType::TypeExact);
            assert!(
                !drill.prompt.contains('\n'),
                "expected drill {} prompt to be one line",
                drill.id
            );
            assert!(
                !drill.prompt.contains('\r'),
                "expected drill {} prompt to be one line",
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
    fn validator_is_supported_for_task_type() {
        for drill in drills() {
            match drill.task_type {
                TaskType::TypeExact => assert_eq!(drill.validator, Validator::Exact),
            }
        }
    }

    #[test]
    fn catalog_has_twelve_drills() {
        assert_eq!(drills().len(), 12);
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

        let last = find_drill("012").unwrap();
        assert_eq!(last.id, "012");
        assert_eq!(
            last.metadata.canonical_id,
            "rust.control.while_empty.v1.copy.0012"
        );
        assert_eq!(last.expected, "while score < 10 {}");
    }

    #[test]
    fn every_short_one_shot_id_still_resolves() {
        for id in [
            "001", "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012",
        ] {
            let drill = find_drill(id).expect("expected short id to resolve");
            assert_eq!(drill.id, id);
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
