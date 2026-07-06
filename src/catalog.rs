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
    use crate::curriculum::{TaskType, Validator};
    use std::collections::HashSet;

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
        assert_eq!(first.expected, "fn main() {}");

        let middle = find_drill("006").unwrap();
        assert_eq!(middle.id, "006");
        assert_eq!(middle.expected, "score += 1;");

        let last = find_drill("012").unwrap();
        assert_eq!(last.id, "012");
        assert_eq!(last.expected, "while score < 10 {}");
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
}
