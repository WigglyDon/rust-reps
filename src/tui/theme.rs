use crate::client::{CodeTokenKind, FailureFrame, RewardFrame, StatusTone};
use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub(crate) struct RustRepsDark {
    pub(crate) background: Color,
    pub(crate) surface: Color,
    pub(crate) editor_active_line: Color,
    pub(crate) editor_failure_line: Color,
    pub(crate) border_normal: Color,
    pub(crate) border_reward: Color,
    pub(crate) text_normal: Color,
    pub(crate) text_muted: Color,
    pub(crate) keyword: Color,
    pub(crate) type_name: Color,
    pub(crate) number: Color,
    pub(crate) string: Color,
    pub(crate) macro_function: Color,
    pub(crate) identifier_plain: Color,
    pub(crate) punctuation_operator: Color,
    pub(crate) comment: Color,
    pub(crate) success: Color,
    pub(crate) error: Color,
    pub(crate) warning_unknown: Color,
    #[allow(dead_code)]
    pub(crate) caret: Color,
    pub(crate) counter_faded: Color,
    pub(crate) reward_plus_one: Color,
}

pub(crate) const RUST_REPS_DARK: RustRepsDark = RustRepsDark {
    background: Color::Rgb(8, 10, 14),
    surface: Color::Rgb(12, 15, 21),
    editor_active_line: Color::Rgb(17, 22, 31),
    editor_failure_line: Color::Rgb(44, 20, 27),
    border_normal: Color::Rgb(62, 70, 84),
    border_reward: Color::Rgb(116, 238, 158),
    text_normal: Color::Rgb(232, 237, 246),
    text_muted: Color::Rgb(119, 129, 148),
    keyword: Color::Rgb(126, 169, 255),
    type_name: Color::Rgb(106, 220, 190),
    number: Color::Rgb(236, 196, 126),
    string: Color::Rgb(146, 196, 132),
    macro_function: Color::Rgb(200, 168, 255),
    identifier_plain: Color::Rgb(232, 237, 246),
    punctuation_operator: Color::Rgb(150, 159, 176),
    comment: Color::Rgb(119, 129, 148),
    success: Color::Rgb(88, 224, 135),
    error: Color::Rgb(255, 92, 105),
    warning_unknown: Color::Rgb(235, 178, 86),
    caret: Color::Rgb(116, 241, 157),
    counter_faded: Color::Rgb(45, 92, 62),
    reward_plus_one: Color::Rgb(158, 255, 187),
};

pub(crate) fn base_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_normal)
        .bg(RUST_REPS_DARK.background)
}

pub(crate) fn surface_style() -> Style {
    Style::default().bg(RUST_REPS_DARK.surface)
}

pub(crate) fn editor_canvas_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_normal)
        .bg(RUST_REPS_DARK.surface)
}

pub(crate) fn editor_active_line_style() -> Style {
    Style::default().bg(RUST_REPS_DARK.editor_active_line)
}

pub(crate) fn editor_line_style(active: bool, failure: FailureFrame) -> Style {
    let background = if active {
        active_editor_background(failure)
    } else {
        RUST_REPS_DARK.surface
    };
    let base = if active {
        editor_active_line_style()
    } else {
        editor_canvas_style()
    };

    base.fg(RUST_REPS_DARK.text_normal).bg(background)
}

#[cfg(test)]
pub(crate) fn border_style(reward: RewardFrame) -> Style {
    let intensity = if reward.active {
        reward.game_glow_intensity.min(85)
    } else {
        0
    };
    Style::default()
        .fg(blend_color(
            RUST_REPS_DARK.border_normal,
            RUST_REPS_DARK.border_reward,
            intensity,
        ))
        .bg(RUST_REPS_DARK.surface)
}

pub(crate) fn border_sweep_style(intensity: u8) -> Style {
    Style::default()
        .fg(blend_color(
            RUST_REPS_DARK.border_normal,
            RUST_REPS_DARK.border_reward,
            intensity.min(85),
        ))
        .bg(RUST_REPS_DARK.surface)
}

pub(crate) fn code_token_style(kind: CodeTokenKind) -> Style {
    match kind {
        CodeTokenKind::Keyword => Style::default()
            .fg(RUST_REPS_DARK.keyword)
            .bg(RUST_REPS_DARK.surface)
            .add_modifier(Modifier::BOLD),
        CodeTokenKind::Type => Style::default()
            .fg(RUST_REPS_DARK.type_name)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::Number => Style::default()
            .fg(RUST_REPS_DARK.number)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::StringLiteral => Style::default()
            .fg(RUST_REPS_DARK.string)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::Macro => Style::default()
            .fg(RUST_REPS_DARK.macro_function)
            .bg(RUST_REPS_DARK.surface)
            .add_modifier(Modifier::BOLD),
        CodeTokenKind::Identifier | CodeTokenKind::Plain => Style::default()
            .fg(RUST_REPS_DARK.identifier_plain)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::Punctuation => Style::default()
            .fg(RUST_REPS_DARK.punctuation_operator)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::Whitespace => Style::default()
            .fg(RUST_REPS_DARK.text_normal)
            .bg(RUST_REPS_DARK.surface),
        CodeTokenKind::Comment => Style::default()
            .fg(RUST_REPS_DARK.comment)
            .bg(RUST_REPS_DARK.surface)
            .add_modifier(Modifier::ITALIC),
    }
}

pub(crate) fn status_tone_style(tone: StatusTone) -> Style {
    match tone {
        StatusTone::Pass => Style::default()
            .fg(RUST_REPS_DARK.success)
            .bg(RUST_REPS_DARK.background)
            .add_modifier(Modifier::BOLD),
        StatusTone::Complete => Style::default()
            .fg(RUST_REPS_DARK.type_name)
            .bg(RUST_REPS_DARK.background)
            .add_modifier(Modifier::BOLD),
        StatusTone::TryAgain => Style::default()
            .fg(RUST_REPS_DARK.error)
            .bg(RUST_REPS_DARK.background),
        StatusTone::ExpectedLabel => Style::default()
            .fg(RUST_REPS_DARK.warning_unknown)
            .bg(RUST_REPS_DARK.background),
        StatusTone::UnknownCommand | StatusTone::Warning => Style::default()
            .fg(RUST_REPS_DARK.warning_unknown)
            .bg(RUST_REPS_DARK.background),
        StatusTone::Plain => muted_style(),
    }
}

pub(crate) fn plus_one_style(reward: RewardFrame) -> Style {
    Style::default()
        .fg(plus_one_color(reward))
        .bg(RUST_REPS_DARK.surface)
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn control_hint_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_muted)
        .bg(RUST_REPS_DARK.background)
}

pub(crate) fn primary_action_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_normal)
        .bg(RUST_REPS_DARK.background)
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn footer_counter_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.reward_plus_one)
        .bg(RUST_REPS_DARK.surface)
        .add_modifier(Modifier::BOLD)
}

#[allow(dead_code)]
pub(crate) fn caret_feedback_style(failure: FailureFrame) -> Style {
    let intensity = if failure.active {
        failure.intensity.min(90)
    } else {
        0
    };

    Style::default()
        .fg(blend_color(
            RUST_REPS_DARK.caret,
            RUST_REPS_DARK.error,
            intensity,
        ))
        .bg(active_editor_background(failure))
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn muted_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_muted)
        .bg(RUST_REPS_DARK.surface)
}

pub(crate) fn problem_header_style() -> Style {
    Style::default()
        .fg(RUST_REPS_DARK.text_muted)
        .bg(RUST_REPS_DARK.surface)
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn editor_gutter_style(active: bool) -> Style {
    let background = if active {
        RUST_REPS_DARK.editor_active_line
    } else {
        RUST_REPS_DARK.surface
    };

    Style::default()
        .fg(RUST_REPS_DARK.text_muted)
        .bg(background)
}

pub(crate) fn editor_gutter_feedback_style(active: bool, failure: FailureFrame) -> Style {
    if !active || !failure.active {
        return editor_gutter_style(active);
    }

    let background = active_editor_background(failure);
    let foreground = blend_color(
        RUST_REPS_DARK.text_muted,
        RUST_REPS_DARK.error,
        failure.intensity.min(55),
    );

    Style::default().fg(foreground).bg(background)
}

fn active_editor_background(failure: FailureFrame) -> Color {
    if failure.active {
        blend_color(
            RUST_REPS_DARK.editor_active_line,
            RUST_REPS_DARK.editor_failure_line,
            failure.intensity.min(100),
        )
    } else {
        RUST_REPS_DARK.editor_active_line
    }
}

fn plus_one_color(reward: RewardFrame) -> Color {
    blend_color(
        RUST_REPS_DARK.counter_faded,
        RUST_REPS_DARK.reward_plus_one,
        reward.plus_one_intensity,
    )
}

fn blend_color(from: Color, to: Color, intensity: u8) -> Color {
    let intensity = intensity.min(100) as u16;
    match (from, to) {
        (Color::Rgb(from_red, from_green, from_blue), Color::Rgb(to_red, to_green, to_blue)) => {
            Color::Rgb(
                blend_channel(from_red, to_red, intensity),
                blend_channel(from_green, to_green, intensity),
                blend_channel(from_blue, to_blue, intensity),
            )
        }
        (_, _) if intensity >= 50 => to,
        _ => from,
    }
}

fn blend_channel(from: u8, to: u8, intensity: u16) -> u8 {
    let from = from as u16;
    let to = to as u16;
    (((from * (100 - intensity)) + (to * intensity)) / 100) as u8
}

#[cfg(test)]
mod tests {
    use super::{
        RUST_REPS_DARK, border_style, caret_feedback_style, code_token_style,
        editor_active_line_style, editor_canvas_style, editor_gutter_feedback_style,
        editor_gutter_style, editor_line_style, plus_one_style, status_tone_style,
    };
    use crate::client::{CodeTokenKind, FailureFrame, RewardFrame, StatusTone};
    use ratatui::style::Color;

    fn green_dominant(color: Color) -> bool {
        matches!(color, Color::Rgb(red, green, blue) if green > red && green > blue)
    }

    fn red_dominant(color: Color) -> bool {
        matches!(color, Color::Rgb(red, green, blue) if red > green && red > blue)
    }

    fn amber_like(color: Color) -> bool {
        matches!(color, Color::Rgb(red, green, blue) if red > blue && green > blue)
    }

    #[test]
    fn code_token_styles_map_to_theme_roles() {
        assert_eq!(
            code_token_style(CodeTokenKind::Keyword).fg,
            Some(RUST_REPS_DARK.keyword)
        );
        assert_eq!(
            code_token_style(CodeTokenKind::Type).fg,
            Some(RUST_REPS_DARK.type_name)
        );
        assert_eq!(
            code_token_style(CodeTokenKind::Number).fg,
            Some(RUST_REPS_DARK.number)
        );
        assert_eq!(
            code_token_style(CodeTokenKind::StringLiteral).fg,
            Some(RUST_REPS_DARK.string)
        );
    }

    #[test]
    fn status_styles_reserve_success_error_and_warning_roles() {
        let success = status_tone_style(StatusTone::Pass)
            .fg
            .expect("expected success foreground");
        let error = status_tone_style(StatusTone::TryAgain)
            .fg
            .expect("expected error foreground");
        let warning = status_tone_style(StatusTone::UnknownCommand)
            .fg
            .expect("expected warning foreground");

        assert!(green_dominant(success));
        assert!(red_dominant(error));
        assert!(amber_like(warning));
    }

    #[test]
    fn syntax_string_green_is_quieter_than_success_green() {
        let Color::Rgb(_, string_green, _) = RUST_REPS_DARK.string else {
            panic!("expected rgb string color");
        };
        let Color::Rgb(_, success_green, _) = RUST_REPS_DARK.success else {
            panic!("expected rgb success color");
        };

        assert_ne!(RUST_REPS_DARK.string, RUST_REPS_DARK.success);
        assert!(string_green < success_green);
    }

    #[test]
    fn plus_one_reward_and_border_reward_differ_from_normal_states() {
        let low_plus_one = plus_one_style(RewardFrame {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
            anchor: None,
        });
        let reward_plus_one = plus_one_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 58,
            sweep_progress: 100,
            show_plus_one: true,
            anchor: None,
        });
        let normal_border = border_style(RewardFrame {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
            anchor: None,
        });
        let reward_border = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 58,
            sweep_progress: 100,
            show_plus_one: true,
            anchor: None,
        });

        assert_ne!(low_plus_one.fg, reward_plus_one.fg);
        assert_ne!(normal_border.fg, reward_border.fg);
    }

    #[test]
    fn editor_canvas_active_line_and_gutter_styles_use_theme_roles() {
        assert_eq!(editor_canvas_style().bg, Some(RUST_REPS_DARK.surface));
        assert_eq!(
            editor_active_line_style().bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(editor_gutter_style(false).bg, Some(RUST_REPS_DARK.surface));
        assert_eq!(
            editor_gutter_style(true).bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(
            editor_gutter_style(false).fg,
            Some(RUST_REPS_DARK.text_muted)
        );
    }

    #[test]
    fn failure_feedback_styles_tint_editor_caret_and_gutter_red() {
        let idle = FailureFrame {
            active: false,
            intensity: 0,
            anchor: None,
        };
        let failure = FailureFrame {
            active: true,
            intensity: 100,
            anchor: None,
        };

        assert_eq!(
            editor_line_style(true, idle).bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(
            editor_line_style(true, failure).bg,
            Some(RUST_REPS_DARK.editor_failure_line)
        );
        assert_ne!(
            editor_gutter_feedback_style(true, failure).fg,
            editor_gutter_style(true).fg
        );
        assert!(
            red_dominant(
                caret_feedback_style(failure)
                    .fg
                    .expect("expected failure caret foreground")
            ),
            "failure caret should read as soft red"
        );
        assert_eq!(caret_feedback_style(idle).fg, Some(RUST_REPS_DARK.caret));
    }
}
