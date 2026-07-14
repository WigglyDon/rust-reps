#[cfg(test)]
use super::planner::{CAPABILITIES, SupportLevel, SupportStage};

pub(crate) const RETURN_TYPE_CONCEPT_KEY: &str = "rust.function.return_literal.return_type_arrow";
pub(crate) const FUNCTION_ARTIFACT_KEY: &str = "function";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct FocusRangeSpec {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct TaskSupportSpec {
    pub(crate) rep_id: &'static str,
    pub(crate) anchor_id: &'static str,
    pub(crate) artifact_key: &'static str,
    pub(crate) concept_key: &'static str,
    pub(crate) help_key: &'static str,
    pub(crate) target_dimension: &'static str,
    pub(crate) focus_ranges: &'static [FocusRangeSpec],
}

const ROW_A_COPY_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 10, end: 16 }];
const ROW_A_FILL_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 10, end: 13 }];
const ROW_A_PRODUCE_FOCUS: &[FocusRangeSpec] = ROW_A_FILL_FOCUS;
const ROW_A_REPAIR_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 9, end: 10 }];
const ROW_A_TRANSFORM_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 7, end: 9 }];

const ROW_B_COPY_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 11, end: 17 }];
const ROW_B_FILL_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 11, end: 14 }];
const ROW_B_PRODUCE_FOCUS: &[FocusRangeSpec] = ROW_B_FILL_FOCUS;
const ROW_B_REPAIR_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 10, end: 11 }];
const ROW_B_TRANSFORM_FOCUS: &[FocusRangeSpec] = &[FocusRangeSpec { start: 8, end: 10 }];

pub(crate) const VARIED_TASK_SUPPORT: &[TaskSupportSpec] = &[
    TaskSupportSpec {
        rep_id: "013",
        anchor_id: "013",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_declaration",
        focus_ranges: ROW_A_COPY_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "014",
        anchor_id: "013",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_arrow_placeholder",
        focus_ranges: ROW_A_FILL_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "015",
        anchor_id: "013",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_arrow_placeholder",
        focus_ranges: ROW_A_PRODUCE_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "016",
        anchor_id: "013",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_syntax_defect",
        focus_ranges: ROW_A_REPAIR_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "017",
        anchor_id: "013",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_declaration_omission",
        focus_ranges: ROW_A_TRANSFORM_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "018",
        anchor_id: "018",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_declaration",
        focus_ranges: ROW_B_COPY_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "019",
        anchor_id: "018",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_arrow_placeholder",
        focus_ranges: ROW_B_FILL_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "020",
        anchor_id: "018",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_arrow_placeholder",
        focus_ranges: ROW_B_PRODUCE_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "021",
        anchor_id: "018",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_syntax_defect",
        focus_ranges: ROW_B_REPAIR_FOCUS,
    },
    TaskSupportSpec {
        rep_id: "022",
        anchor_id: "018",
        artifact_key: FUNCTION_ARTIFACT_KEY,
        concept_key: RETURN_TYPE_CONCEPT_KEY,
        help_key: RETURN_TYPE_CONCEPT_KEY,
        target_dimension: "return_type_declaration_omission",
        focus_ranges: ROW_B_TRANSFORM_FOCUS,
    },
];

pub(crate) fn task_support(rep_id: &str) -> Option<&'static TaskSupportSpec> {
    VARIED_TASK_SUPPORT
        .iter()
        .find(|support| support.rep_id == rep_id)
}

#[cfg(test)]
pub(crate) fn support_stage_for_capability(capability_id: &str) -> Option<SupportStage> {
    CAPABILITIES
        .iter()
        .find(|capability| capability.id == capability_id)
        .map(|capability| capability.support.stage())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::{DRILLS, Operation};

    #[test]
    fn varied_support_is_static_complete_and_utf8_safe() {
        assert_eq!(VARIED_TASK_SUPPORT.len(), 10);
        for (index, support) in VARIED_TASK_SUPPORT.iter().enumerate() {
            let drill = &DRILLS[index + 12];
            assert_eq!(support.rep_id, drill.id);
            assert_eq!(support.artifact_key, FUNCTION_ARTIFACT_KEY);
            assert_eq!(support.concept_key, RETURN_TYPE_CONCEPT_KEY);
            assert_eq!(support.help_key, RETURN_TYPE_CONCEPT_KEY);
            assert!(!support.target_dimension.is_empty());
            assert!(!support.focus_ranges.is_empty());
            for range in support.focus_ranges {
                assert!(range.start < range.end);
                assert!(range.end <= drill.stimulus.len());
                assert!(drill.stimulus.is_char_boundary(range.start));
                assert!(drill.stimulus.is_char_boundary(range.end));
            }
        }
    }

    #[test]
    fn each_row_has_one_anchor_one_concept_and_teach_before_test_stages() {
        let expected_stages = [
            SupportStage::Introduce,
            SupportStage::Guided,
            SupportStage::Guided,
            SupportStage::Guided,
            SupportStage::Practice,
        ];
        let expected_operations = [
            Operation::Copy,
            Operation::Fill,
            Operation::Produce,
            Operation::Repair,
            Operation::Transform,
        ];

        for row_index in 0..2 {
            let supports = &VARIED_TASK_SUPPORT[row_index * 5..row_index * 5 + 5];
            let drills = &DRILLS[12 + row_index * 5..17 + row_index * 5];
            let anchor = supports[0].anchor_id;
            assert!(supports.iter().all(|support| support.anchor_id == anchor));
            assert!(
                supports
                    .iter()
                    .all(|support| support.concept_key == RETURN_TYPE_CONCEPT_KEY)
            );
            assert_eq!(
                drills
                    .iter()
                    .map(|drill| drill.metadata.operation)
                    .collect::<Vec<_>>(),
                expected_operations
            );
            assert_eq!(
                drills
                    .iter()
                    .map(
                        |drill| support_stage_for_capability(drill.capability_id.unwrap()).unwrap()
                    )
                    .collect::<Vec<_>>(),
                expected_stages
            );
        }
    }

    #[test]
    fn unsupported_legacy_tasks_have_no_help_contract() {
        for drill in &DRILLS[..12] {
            assert_eq!(task_support(drill.id), None);
        }
    }

    #[test]
    fn each_row_is_one_anchor_with_only_declared_operation_contrasts() {
        for row_index in 0..2 {
            let drills = &DRILLS[12 + row_index * 5..17 + row_index * 5];
            let anchor = &drills[0];
            assert_eq!(anchor.stimulus, anchor.expected);
            assert!(
                drills
                    .iter()
                    .all(|drill| drill.assignments == anchor.assignments)
            );

            assert_eq!(
                drills[1].stimulus.replacen("___", drills[1].expected, 1),
                anchor.stimulus
            );
            assert_eq!(drills[2].stimulus, drills[1].stimulus);
            assert_eq!(drills[2].expected, anchor.expected);
            assert_eq!(
                drills[3].stimulus.replacen("(): ", "() -> ", 1),
                anchor.stimulus
            );

            let type_value = anchor
                .assignments
                .iter()
                .find(|assignment| assignment.slot == "TYPE")
                .expect("anchor has TYPE")
                .rendered;
            assert_eq!(
                drills[4]
                    .stimulus
                    .replacen("() ", &format!("() -> {type_value} "), 1),
                anchor.stimulus
            );
            assert_eq!(drills[4].expected, anchor.expected);
        }
    }

    #[test]
    fn corresponding_rows_vary_name_type_and_value_without_cartesian_mixing() {
        let row_a = &DRILLS[12..17];
        let row_b = &DRILLS[17..22];
        assert_ne!(row_a[0].assignments, row_b[0].assignments);
        for operation_index in 0..5 {
            assert_eq!(
                row_a[operation_index].metadata.operation,
                row_b[operation_index].metadata.operation
            );
            assert_ne!(
                row_a[operation_index].stimulus,
                row_b[operation_index].stimulus
            );
        }
    }

    #[test]
    fn support_levels_map_to_presentation_stages_without_learner_state() {
        assert_eq!(SupportLevel::FullModel.stage(), SupportStage::Introduce);
        assert_eq!(SupportLevel::PartialModel.stage(), SupportStage::Guided);
        assert_eq!(SupportLevel::ConstrainedCue.stage(), SupportStage::Guided);
        assert_eq!(SupportLevel::MinimalCue.stage(), SupportStage::Practice);
    }
}
