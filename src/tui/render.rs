use crate::{
    client::{ClientFrame, CodeToken, CounterRewardFrame, StatusLine, StatusTone},
    tui::{
        app::TuiApp,
        theme::{
            base_style, border_style, caret_style, code_token_style, counter_number_style,
            counter_symbol_style, editor_gutter_style, problem_header_style, status_tone_style,
            surface_style,
        },
    },
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

const FEEDBACK_CHROME_HEIGHT: u16 = 3;
const BOTTOM_CHROME_HEIGHT: u16 = 1;

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp, now: std::time::Instant) {
    let client_frame = app.frame_at(now);
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let feedback_lines = feedback_chrome_lines(client_frame);
    let chunks = layout_chunks(frame.area(), feedback_height(&feedback_lines));

    frame.render_widget(Block::default().style(base_style()), frame.area());

    let game_block = Block::default()
        .borders(Borders::ALL)
        .style(surface_style())
        .border_style(border_style(client_frame.counter_reward));
    let game_area = chunks[0];
    let game_body = game_area.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });
    let game_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(game_body);

    frame.render_widget(game_block, game_area);

    frame.render_widget(
        Paragraph::new(Text::from(problem_lines(client_frame))).style(base_style()),
        game_chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(workbox_lines(client_frame))).style(base_style()),
        game_chunks[1],
    );

    if !feedback_lines.is_empty() {
        frame.render_widget(
            Paragraph::new(Text::from(feedback_lines)).style(base_style()),
            chunks[1],
        );
    }

    let bottom_area = chunks[2].inner(Margin {
        vertical: 0,
        horizontal: 1,
    });
    frame.render_widget(
        Paragraph::new(Text::from(bottom_chrome_lines(client_frame)))
            .alignment(bottom_chrome_alignment())
            .style(base_style()),
        bottom_area,
    );
}

fn layout_chunks(area: Rect, feedback_height: u16) -> std::rc::Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),
            Constraint::Length(feedback_height),
            Constraint::Length(BOTTOM_CHROME_HEIGHT),
        ])
        .split(area)
}

fn problem_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            client_frame.game_window.problem.instruction.clone(),
            problem_header_style(),
        )),
        Line::from(""),
        Line::from(code_spans(&client_frame.game_window.problem.prompt_tokens)),
    ]
}

fn workbox_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(input_spans(client_frame)),
        Line::from(""),
        Line::from(""),
        Line::from(""),
    ]
}

fn feedback_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    let mut lines = client_frame.status_chrome.lines.clone();
    lines.extend(
        client_frame
            .help_chrome
            .lines
            .iter()
            .map(|line| StatusLine {
                tone: StatusTone::Plain,
                text: line.to_string(),
                code_tokens: Vec::new(),
            }),
    );
    lines.into_iter().filter_map(feedback_line).collect()
}

fn feedback_height(lines: &[Line<'static>]) -> u16 {
    let _ = lines;
    FEEDBACK_CHROME_HEIGHT
}

fn input_spans(client_frame: &ClientFrame) -> Vec<Span<'static>> {
    let cursor = if client_frame.game_window.workbox.cursor_visible {
        "|"
    } else {
        " "
    };
    let mut spans = vec![Span::styled("1 │ ", editor_gutter_style())];
    spans.extend(code_spans(&client_frame.game_window.workbox.input_tokens));
    spans.push(Span::styled(cursor.to_string(), caret_style()));
    spans
}

fn feedback_line(status_line: StatusLine) -> Option<Line<'static>> {
    if status_line.tone == StatusTone::Pass {
        return None;
    }

    if !status_line.code_tokens.is_empty() {
        return Some(Line::from(code_spans(&status_line.code_tokens)));
    }

    Some(Line::from(Span::styled(
        status_line.text,
        status_tone_style(status_line.tone),
    )))
}

fn bottom_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    let symbol = counter_symbol(client_frame.counter_reward);
    vec![Line::from(vec![
        Span::styled(symbol, counter_symbol_style(client_frame.counter_reward)),
        Span::styled(
            format!(" {}", client_frame.completed_reps_this_run),
            counter_number_style(client_frame.counter_reward),
        ),
    ])]
}

fn counter_symbol(reward: CounterRewardFrame) -> &'static str {
    if reward.active { "✓" } else { "●" }
}

fn code_spans(tokens: &[CodeToken]) -> Vec<Span<'static>> {
    tokens
        .iter()
        .map(|token| Span::styled(token.text.clone(), code_token_style(token.kind)))
        .collect()
}

fn bottom_chrome_alignment() -> Alignment {
    Alignment::Center
}

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    feedback: Vec<String>,
    bottom_chrome: Vec<String>,
}

#[cfg(test)]
fn render_snapshot(client_frame: &ClientFrame) -> RenderSnapshot {
    RenderSnapshot {
        game: [problem_lines(client_frame), workbox_lines(client_frame)]
            .concat()
            .iter()
            .map(line_text)
            .collect(),
        feedback: feedback_chrome_lines(client_frame)
            .iter()
            .map(line_text)
            .collect(),
        bottom_chrome: bottom_chrome_lines(client_frame)
            .iter()
            .map(line_text)
            .collect(),
    }
}

#[cfg(test)]
fn line_text(line: &Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{
        FEEDBACK_CHROME_HEIGHT, bottom_chrome_alignment, bottom_chrome_lines,
        feedback_chrome_lines, feedback_height, layout_chunks, line_text, problem_lines,
        render_snapshot,
    };
    use crate::{
        client::{
            COUNTER_REWARD_ATTACK_DURATION, COUNTER_REWARD_DURATION, COUNTER_REWARD_HOLD_DURATION,
            ClientAction, CounterRewardFrame, StatusTone,
        },
        tui::{
            TuiApp,
            theme::{RUST_REPS_DARK, base_style, border_style, status_tone_style},
        },
    };
    use ratatui::layout::{Alignment, Rect};
    use ratatui::style::Modifier;

    fn test_now() -> Instant {
        Instant::now()
    }

    #[test]
    fn render_snapshot_keeps_only_problem_and_workbox_in_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let instruction_position = snapshot
            .game
            .iter()
            .position(|line| line == "do this:")
            .expect("expected concise problem instruction in game snapshot");
        let prompt_value_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt in game snapshot");
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == "1 │ |")
            .expect("expected editor gutter and caret in game snapshot");

        assert!(instruction_position < prompt_value_position);
        assert!(prompt_value_position < input_position);
        assert!(!snapshot.game.iter().any(|line| line == "Problem"));
        assert!(!snapshot.game.iter().any(|line| line == "Workbox"));
        assert!(!snapshot.game.iter().any(|line| line.contains("Drill")));
        assert!(!snapshot.game.iter().any(|line| line == "Feedback"));
        assert!(!snapshot.game.iter().any(|line| line.contains("PASS")));
        assert!(!snapshot.game.iter().any(|line| line.contains("TRY AGAIN")));
    }

    #[test]
    fn editor_surface_snapshot_includes_gutter_spacing_and_reserved_space() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == "1 │ |")
            .expect("expected editor input line");

        assert!(input_position > 0);
        assert_eq!(snapshot.game[input_position - 1], "");
        assert!(!snapshot.game.iter().any(|line| line == "  │"));
        assert!(
            !snapshot.game.iter().any(|line| line.contains('─')),
            "editor surface should not use a partial horizontal divider"
        );
    }

    #[test]
    fn render_snapshot_keeps_status_outside_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let visible_lines: Vec<&String> = snapshot
            .game
            .iter()
            .chain(snapshot.feedback.iter())
            .chain(snapshot.bottom_chrome.iter())
            .collect();
        for forbidden in [
            "Passed",
            "Skipped",
            "Attempts",
            "READY",
            "Enter submit",
            "Esc quit",
            "? help",
        ] {
            assert!(
                !visible_lines.iter().any(|line| line.contains(forbidden)),
                "default render should not contain {forbidden}"
            );
        }
        assert!(snapshot.feedback.is_empty());
    }

    #[test]
    fn render_snapshot_problem_instruction_is_concise() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let descriptor_position = snapshot
            .game
            .iter()
            .position(|line| line == "do this:")
            .expect("expected problem descriptor");
        let prompt_position = snapshot
            .game
            .iter()
            .position(|line| line == "fn main() {}")
            .expect("expected prompt line");

        assert!(descriptor_position < prompt_position);
        assert_eq!(snapshot.game[descriptor_position - 1], "");
        assert_eq!(snapshot.game[descriptor_position + 1], "");
    }

    #[test]
    fn render_snapshot_contains_centered_counter_text() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert_eq!(snapshot.bottom_chrome, vec!["● 0".to_string()]);
        assert_eq!(bottom_chrome_alignment(), Alignment::Center);
        assert!(!snapshot.game.iter().any(|line| line.contains("Passed")));
        assert!(!snapshot.feedback.iter().any(|line| line.contains("Passed")));
    }

    #[test]
    fn counter_dot_and_number_are_green_and_bold() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");

        assert_eq!(line.spans[0].content.as_ref(), "●");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert!(line.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[1].content.as_ref(), " 0");
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert!(line.spans[1].style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn reward_peak_counter_symbol_and_number_are_checkmark_bright_and_bold() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + COUNTER_REWARD_ATTACK_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");

        assert_eq!(line.spans[0].content.as_ref(), "✓");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_reward));
        assert!(line.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[1].content.as_ref(), " 1");
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.counter_reward));
        assert!(line.spans[1].style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn reward_counter_sequence_hits_fast_holds_then_releases_slowly() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");
        assert_eq!(line.spans[0].content.as_ref(), "✓");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));

        let frame = app.frame_at(now + COUNTER_REWARD_ATTACK_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");
        assert_eq!(line.spans[0].content.as_ref(), "✓");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_reward));
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.counter_reward));

        let frame =
            app.frame_at(now + COUNTER_REWARD_ATTACK_DURATION + (COUNTER_REWARD_HOLD_DURATION / 2));
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");
        assert_eq!(line.spans[0].content.as_ref(), "✓");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_reward));
        assert_eq!(line.spans[1].content.as_ref(), " 1");

        let frame = app.frame_at(now + COUNTER_REWARD_DURATION - Duration::from_millis(200));
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");
        assert_eq!(line.spans[0].content.as_ref(), "✓");
        assert_ne!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_reward));
        assert_eq!(line.spans[1].content.as_ref(), " 1");

        let frame = app.frame_at(now + COUNTER_REWARD_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");
        assert_eq!(line.spans[0].content.as_ref(), "●");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
    }

    #[test]
    fn reward_counter_symbol_returns_to_normal_dot_after_ticks() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + COUNTER_REWARD_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.first().expect("expected bottom counter line");

        assert_eq!(line.spans[0].content.as_ref(), "●");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert_eq!(line.spans[1].content.as_ref(), " 1");
    }

    #[test]
    fn border_style_maps_reward_settle_phases() {
        let idle = border_style(CounterRewardFrame {
            active: false,
            intensity: 0,
        });
        let low = border_style(CounterRewardFrame {
            active: true,
            intensity: 25,
        });
        let medium = border_style(CounterRewardFrame {
            active: true,
            intensity: 55,
        });
        let peak = border_style(CounterRewardFrame {
            active: true,
            intensity: 100,
        });

        assert_eq!(idle.fg, Some(RUST_REPS_DARK.border_normal));
        assert_ne!(low.fg, idle.fg);
        assert_ne!(medium.fg, low.fg);
        assert_ne!(peak.fg, medium.fg);
        for style in [low, medium, peak] {
            assert!(!style.add_modifier.contains(Modifier::BOLD));
        }
        assert_ne!(peak.fg, idle.fg);
    }

    #[test]
    fn feedback_height_and_layout_stay_stable_across_default_states() {
        let mut wrong_app = TuiApp::new();
        for ch in "wrong".chars() {
            wrong_app.handle_action(ClientAction::TypeChar(ch));
        }
        wrong_app.handle_action(ClientAction::Submit);

        let mut success_app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            success_app.handle_action(ClientAction::TypeChar(ch));
        }
        success_app.handle_action(ClientAction::Submit);

        let idle_height = feedback_height(&feedback_chrome_lines(&TuiApp::new().frame()));
        let wrong_height = feedback_height(&feedback_chrome_lines(&wrong_app.frame()));
        let success_height = feedback_height(&feedback_chrome_lines(&success_app.frame()));

        assert_eq!(idle_height, FEEDBACK_CHROME_HEIGHT);
        assert_eq!(wrong_height, FEEDBACK_CHROME_HEIGHT);
        assert_eq!(success_height, FEEDBACK_CHROME_HEIGHT);

        let area = Rect::new(0, 0, 80, 24);
        let idle_chunks = layout_chunks(area, idle_height);
        let wrong_chunks = layout_chunks(area, wrong_height);
        let success_chunks = layout_chunks(area, success_height);
        assert_eq!(idle_chunks, wrong_chunks);
        assert_eq!(wrong_chunks, success_chunks);
    }

    #[test]
    fn wrong_answer_does_not_render_reward_checkmark_or_increment_counter() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.bottom_chrome.iter().any(|line| line == "● 0"));
        assert!(!snapshot.bottom_chrome.iter().any(|line| line.contains("✓")));
    }

    #[test]
    fn status_theme_maps_pass_try_again_and_unknown_command_distinctly() {
        assert_eq!(
            status_tone_style(StatusTone::Pass).fg,
            Some(RUST_REPS_DARK.success)
        );
        assert_eq!(
            status_tone_style(StatusTone::TryAgain).fg,
            Some(RUST_REPS_DARK.error)
        );
        assert_eq!(
            status_tone_style(StatusTone::UnknownCommand).fg,
            Some(RUST_REPS_DARK.warning_unknown)
        );
        assert_ne!(
            status_tone_style(StatusTone::Pass).fg,
            status_tone_style(StatusTone::TryAgain).fg
        );
    }

    #[test]
    fn try_again_feedback_renders_red() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let lines = feedback_chrome_lines(&frame);
        let try_again = lines
            .iter()
            .find(|line| line_text(line) == "not quite")
            .expect("expected not quite feedback");

        assert_eq!(try_again.spans[0].style.fg, Some(RUST_REPS_DARK.error));
    }

    #[test]
    fn code_prompt_renders_with_non_default_syntax_styles() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = problem_lines(&frame);
        let prompt_line = lines
            .iter()
            .find(|line| line_text(line) == "fn main() {}")
            .expect("expected prompt line");

        assert_eq!(line_text(prompt_line), "fn main() {}");
        assert!(
            prompt_line
                .spans
                .iter()
                .any(|span| span.content.as_ref() == "fn"
                    && span.style.fg == Some(RUST_REPS_DARK.keyword))
        );
        assert!(
            prompt_line
                .spans
                .iter()
                .any(|span| span.style != base_style())
        );
    }

    #[test]
    fn render_snapshot_shows_typed_input_text() {
        let mut app = TuiApp::new();
        for ch in "hello".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.game.iter().any(|line| line == "1 │ hello|"));
    }

    #[test]
    fn render_snapshot_hides_cursor_when_cursor_state_is_hidden() {
        let mut app = TuiApp::new();
        app.tick_cursor();
        let frame = app.frame();
        let empty_snapshot = render_snapshot(&frame);
        assert!(empty_snapshot.game.iter().any(|line| line == "1 │  "));

        app.handle_action(ClientAction::TypeChar('x'));
        app.tick_cursor();
        let frame = app.frame();
        let typed_snapshot = render_snapshot(&frame);
        assert!(typed_snapshot.game.iter().any(|line| line == "1 │ x "));
    }

    #[test]
    fn render_snapshot_shows_help_only_when_toggled() {
        let mut app = TuiApp::new();
        let frame = app.frame();
        let default_snapshot = render_snapshot(&frame);
        assert!(
            !default_snapshot
                .feedback
                .iter()
                .any(|line| line == "Commands:")
        );
        assert!(
            !default_snapshot
                .feedback
                .iter()
                .any(|line| line == "Backspace: delete char")
        );

        app.handle_action(ClientAction::ToggleHelp);
        let frame = app.frame();
        let help_snapshot = render_snapshot(&frame);
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line == "Commands:")
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line == "Enter: submit input")
        );
        assert!(!help_snapshot.game.iter().any(|line| line == "Commands:"));
    }

    #[test]
    fn render_snapshot_keeps_wrong_answer_clear_with_expected_answer() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.feedback.iter().any(|line| line == "not quite"));
        assert!(snapshot.feedback.iter().any(|line| line == "expected:"));
        assert!(snapshot.feedback.iter().any(|line| line == "fn main() {}"));
        assert!(!snapshot.game.iter().any(|line| line == "TRY AGAIN"));
        assert!(!snapshot.game.iter().any(|line| line == "not quite"));
        assert!(!snapshot.game.iter().any(|line| line == "expected:"));
    }

    #[test]
    fn render_snapshot_makes_unknown_command_clear() {
        let mut app = TuiApp::new();
        for ch in ":unknown".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(
            snapshot
                .feedback
                .iter()
                .any(|line| line == "UNKNOWN COMMAND")
        );
        assert!(
            snapshot
                .feedback
                .iter()
                .any(|line| line == "Type :help for commands.")
        );
        assert!(
            !snapshot
                .game
                .iter()
                .any(|line| line == ">> UNKNOWN COMMAND")
        );
    }

    #[test]
    fn render_snapshot_hides_pass_and_shows_reward_checkmark_after_success() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + COUNTER_REWARD_DURATION / 2);
        let snapshot = render_snapshot(&frame);

        assert!(!snapshot.feedback.iter().any(|line| line == "PASS"));
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == "println!(\"Rust Reps\");")
        );
        assert!(snapshot.game.iter().any(|line| line == "1 │ |"));
        assert!(snapshot.bottom_chrome.iter().any(|line| line == "✓ 1"));
        assert!(!snapshot.game.iter().any(|line| line == "PASS"));
    }
}
