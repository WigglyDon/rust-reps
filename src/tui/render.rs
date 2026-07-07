use crate::{
    client::{ClientFrame, CodeToken, EditorViewport, RewardFrame, StatusLine, StatusTone},
    tui::{
        app::TuiApp,
        theme::{
            base_style, border_sweep_style, caret_style, code_token_style, control_hint_style,
            counter_style, editor_active_line_style, editor_canvas_style, editor_gutter_style,
            plus_one_style, primary_action_style, problem_header_style, status_tone_style,
            surface_style,
        },
    },
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

const FEEDBACK_CHROME_HEIGHT: u16 = 3;
const PROBLEM_ZONE_HEIGHT: u16 = 6;
const COMMAND_RAIL_HEIGHT: u16 = 4;
const MIN_GUTTER_WIDTH: usize = 2;
const PRIMARY_ACTION_HINT: &str = "F5 submit";
const SECONDARY_CONTROL_HINTS: &str = "Enter newline · Esc quit · ? help";
#[cfg(test)]
const SNAPSHOT_WORKBOX_HEIGHT: u16 = 6;

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &TuiApp, now: std::time::Instant) {
    let client_frame = app.frame_at(now);
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let feedback_lines = feedback_chrome_lines(client_frame);
    let shell_area = frame.area();
    let chunks = layout_chunks(shell_area, feedback_height(&feedback_lines));

    frame.render_widget(Block::default().style(base_style()), shell_area);

    let game_block = Block::default().style(surface_style());
    frame.render_widget(game_block, shell_area);
    render_game_border(frame, shell_area, client_frame.reward);

    frame.render_widget(
        Paragraph::new(Text::from(problem_lines(client_frame))).style(base_style()),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(workbox_lines(client_frame, chunks[1].height)))
            .style(editor_canvas_style()),
        chunks[1],
    );

    if !feedback_lines.is_empty() {
        frame.render_widget(
            Paragraph::new(Text::from(feedback_lines)).style(base_style()),
            chunks[2],
        );
    }

    render_command_rail(frame, chunks[3], client_frame);
}

fn layout_chunks(area: Rect, feedback_height: u16) -> std::rc::Rc<[Rect]> {
    let body = shell_body(area);
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(PROBLEM_ZONE_HEIGHT),
            Constraint::Min(6),
            Constraint::Length(feedback_height),
            Constraint::Length(COMMAND_RAIL_HEIGHT),
        ])
        .split(body)
}

fn shell_body(area: Rect) -> Rect {
    area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    })
}

fn render_game_border(frame: &mut Frame<'_>, area: Rect, reward: RewardFrame) {
    if area.width < 2 || area.height < 2 {
        return;
    }

    let buffer = frame.buffer_mut();
    for x in area.left()..area.right() {
        set_border_cell(
            buffer,
            area,
            x,
            area.top(),
            border_symbol(area, x, area.top()),
            reward,
        );
        set_border_cell(
            buffer,
            area,
            x,
            area.bottom() - 1,
            border_symbol(area, x, area.bottom() - 1),
            reward,
        );
    }
    for y in area.top()..area.bottom() {
        set_border_cell(
            buffer,
            area,
            area.left(),
            y,
            border_symbol(area, area.left(), y),
            reward,
        );
        set_border_cell(
            buffer,
            area,
            area.right() - 1,
            y,
            border_symbol(area, area.right() - 1, y),
            reward,
        );
    }
}

fn set_border_cell(
    buffer: &mut Buffer,
    area: Rect,
    x: u16,
    y: u16,
    symbol: &'static str,
    reward: RewardFrame,
) {
    buffer[(x, y)]
        .set_symbol(symbol)
        .set_style(border_sweep_style(reward_sweep_intensity_at(
            x, y, area, reward,
        )));
}

fn border_symbol(area: Rect, x: u16, y: u16) -> &'static str {
    let left = area.left();
    let right = area.right() - 1;
    let top = area.top();
    let bottom = area.bottom() - 1;
    match (x, y) {
        (x, y) if x == left && y == top => "┌",
        (x, y) if x == right && y == top => "┐",
        (x, y) if x == left && y == bottom => "└",
        (x, y) if x == right && y == bottom => "┘",
        (_, y) if y == top || y == bottom => "─",
        _ => "│",
    }
}

fn reward_sweep_intensity_at(x: u16, y: u16, area: Rect, reward: RewardFrame) -> u8 {
    if !reward.active {
        return 0;
    }

    let cell_progress = border_sweep_cell_progress(x, y, area);
    let sweep_progress = f32::from(reward.sweep_progress) / 100.0;
    let wave_width = 0.24;
    let wave_distance = (cell_progress - sweep_progress).abs();
    let wave = 1.0 - smoothstep((wave_distance / wave_width).clamp(0.0, 1.0));
    let tail = if cell_progress <= sweep_progress {
        let tail_distance = sweep_progress - cell_progress;
        (1.0 - smoothstep((tail_distance / 0.58).clamp(0.0, 1.0))) * 0.28
    } else {
        0.0
    };
    let spatial_intensity = wave.max(tail);

    (spatial_intensity * f32::from(reward.game_glow_intensity))
        .round()
        .clamp(0.0, 100.0) as u8
}

fn border_sweep_cell_progress(x: u16, y: u16, area: Rect) -> f32 {
    if area.width <= 1 || area.height <= 1 {
        return 0.0;
    }

    let horizontal = f32::from(x.saturating_sub(area.left())) / f32::from(area.width - 1);
    let vertical_from_bottom =
        f32::from((area.bottom() - 1).saturating_sub(y)) / f32::from(area.height - 1);
    ((horizontal + vertical_from_bottom) / 2.0).clamp(0.0, 1.0)
}

fn smoothstep(progress: f32) -> f32 {
    progress * progress * (3.0 - (2.0 * progress))
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

fn workbox_lines(client_frame: &ClientFrame, workbox_height: u16) -> Vec<Line<'static>> {
    let visible_editor_rows = visible_editor_rows(workbox_height);
    let viewport = EditorViewport::around_cursor(
        client_frame.game_window.workbox.lines.len(),
        client_frame.game_window.workbox.cursor.line,
        visible_editor_rows,
    );
    let visible_end = viewport.first_line + viewport.visible_lines;
    let gutter_width = gutter_width(visible_end);
    let mut lines = vec![Line::from("").style(editor_canvas_style())];

    for line_index in viewport.first_line..visible_end {
        let tokens = client_frame
            .game_window
            .workbox
            .line_tokens
            .get(line_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let cursor_column = (line_index == client_frame.game_window.workbox.cursor.line)
            .then_some(client_frame.game_window.workbox.cursor.column);
        let active = line_index == client_frame.game_window.workbox.cursor.line;
        lines.push(editor_line(
            line_index,
            gutter_width,
            tokens,
            cursor_column,
            client_frame.game_window.workbox.cursor_visible,
            active,
        ));
    }

    while lines.len() < usize::from(workbox_height) {
        lines.push(Line::from("").style(editor_canvas_style()));
    }

    lines
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

fn render_command_rail(frame: &mut Frame<'_>, area: Rect, client_frame: &ClientFrame) {
    frame.render_widget(Block::default().style(base_style()), area);

    let rail = area.inner(Margin {
        vertical: 0,
        horizontal: 1,
    });
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(rail);
    let groups = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(rows[1]);

    frame.render_widget(
        Paragraph::new(Line::from(primary_rail_spans(client_frame))).style(base_style()),
        groups[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(secondary_rail_spans()))
            .alignment(Alignment::Right)
            .style(base_style()),
        groups[1],
    );
}

fn visible_editor_rows(workbox_height: u16) -> usize {
    usize::from(workbox_height.saturating_sub(1).max(1))
}

fn gutter_width(total_lines: usize) -> usize {
    total_lines.max(1).to_string().len().max(MIN_GUTTER_WIDTH)
}

fn editor_line(
    line_index: usize,
    gutter_width: usize,
    tokens: &[CodeToken],
    cursor_column: Option<usize>,
    cursor_visible: bool,
    active: bool,
) -> Line<'static> {
    let style = if active {
        editor_active_line_style()
    } else {
        editor_canvas_style()
    };
    Line::from(editor_line_spans(
        line_index,
        gutter_width,
        tokens,
        cursor_column,
        cursor_visible,
        active,
    ))
    .style(style)
}

fn editor_line_spans(
    line_index: usize,
    gutter_width: usize,
    tokens: &[CodeToken],
    cursor_column: Option<usize>,
    cursor_visible: bool,
    active: bool,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(
        format!(
            "{line_number:>gutter_width$} │ ",
            line_number = line_index + 1
        ),
        editor_gutter_style(active),
    )];

    if let Some(cursor_column) = cursor_column {
        let cursor = if cursor_visible { "|" } else { " " };
        spans.extend(code_spans_with_cursor(tokens, cursor_column, cursor));
    } else {
        spans.extend(code_spans(tokens));
    }

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

#[cfg(test)]
fn bottom_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(primary_rail_spans(client_frame)),
        Line::from(secondary_rail_spans()),
        Line::from(""),
    ]
}

fn primary_rail_spans(client_frame: &ClientFrame) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(
        format!("reps {}", client_frame.completed_reps_this_run),
        counter_style(),
    )];
    if client_frame.reward.show_plus_one {
        spans.push(Span::styled("   +1", plus_one_style(client_frame.reward)));
    }

    spans.push(Span::raw("   "));
    spans.push(Span::styled(PRIMARY_ACTION_HINT, primary_action_style()));
    spans
}

fn secondary_rail_spans() -> Vec<Span<'static>> {
    vec![Span::styled(
        SECONDARY_CONTROL_HINTS.to_string(),
        control_hint_style(),
    )]
}

fn code_spans(tokens: &[CodeToken]) -> Vec<Span<'static>> {
    tokens
        .iter()
        .map(|token| Span::styled(token.text.clone(), code_token_style(token.kind)))
        .collect()
}

fn code_spans_with_cursor(
    tokens: &[CodeToken],
    cursor_column: usize,
    cursor: &'static str,
) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut chars_before_cursor = 0;
    let mut cursor_inserted = false;

    for token in tokens {
        let token_len = token.text.chars().count();
        if !cursor_inserted && cursor_column <= chars_before_cursor + token_len {
            let split_column = cursor_column.saturating_sub(chars_before_cursor);
            let (before, after) = split_at_char_column(&token.text, split_column);
            if !before.is_empty() {
                spans.push(Span::styled(before, code_token_style(token.kind)));
            }
            spans.push(Span::styled(cursor.to_string(), caret_style()));
            if !after.is_empty() {
                spans.push(Span::styled(after, code_token_style(token.kind)));
            }
            cursor_inserted = true;
        } else {
            spans.push(Span::styled(
                token.text.clone(),
                code_token_style(token.kind),
            ));
        }
        chars_before_cursor += token_len;
    }

    if !cursor_inserted {
        spans.push(Span::styled(cursor.to_string(), caret_style()));
    }

    spans
}

fn split_at_char_column(text: &str, column: usize) -> (String, String) {
    let split_index = text
        .char_indices()
        .map(|(index, _)| index)
        .nth(column)
        .unwrap_or(text.len());
    (
        text[..split_index].to_string(),
        text[split_index..].to_string(),
    )
}

#[cfg(test)]
fn bottom_chrome_alignment() -> Alignment {
    Alignment::Left
}

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    feedback: Vec<String>,
    bottom_chrome: Vec<String>,
}

#[cfg(test)]
fn render_snapshot(client_frame: &ClientFrame) -> RenderSnapshot {
    render_snapshot_with_workbox_height(client_frame, SNAPSHOT_WORKBOX_HEIGHT)
}

#[cfg(test)]
fn render_snapshot_with_workbox_height(
    client_frame: &ClientFrame,
    workbox_height: u16,
) -> RenderSnapshot {
    RenderSnapshot {
        game: [
            problem_lines(client_frame),
            workbox_lines(client_frame, workbox_height),
        ]
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
        COMMAND_RAIL_HEIGHT, FEEDBACK_CHROME_HEIGHT, PRIMARY_ACTION_HINT, PROBLEM_ZONE_HEIGHT,
        SECONDARY_CONTROL_HINTS, SNAPSHOT_WORKBOX_HEIGHT, border_sweep_cell_progress,
        bottom_chrome_alignment, bottom_chrome_lines, feedback_chrome_lines, feedback_height,
        gutter_width, layout_chunks, line_text, problem_lines, render_snapshot,
        render_snapshot_with_workbox_height, reward_sweep_intensity_at, workbox_lines,
    };
    use crate::{
        client::{
            ClientAction, REWARD_ATTACK_DURATION, REWARD_DURATION, REWARD_PLUS_ONE_BODY_DURATION,
            RewardFrame, StatusTone,
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

    fn reward_frame(
        plus_one_intensity: u8,
        game_glow_intensity: u8,
        sweep_progress: u8,
    ) -> RewardFrame {
        RewardFrame {
            active: plus_one_intensity > 0 || game_glow_intensity > 0,
            plus_one_intensity,
            game_glow_intensity,
            sweep_progress,
            show_plus_one: plus_one_intensity > 0,
        }
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
            .position(|line| line == " 1 │ |")
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
    fn render_snapshot_shows_problem_editor_and_command_rail_zones() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app.frame());
        let problem_position = snapshot
            .game
            .iter()
            .position(|line| line == "do this:")
            .expect("expected problem header");
        let editor_position = snapshot
            .game
            .iter()
            .position(|line| line == " 1 │ |")
            .expect("expected editor gutter and caret");
        let rail_position = snapshot
            .bottom_chrome
            .iter()
            .position(|line| line.contains(PRIMARY_ACTION_HINT))
            .expect("expected command rail primary action");

        assert!(problem_position < editor_position);
        assert_eq!(rail_position, 1);
        assert!(
            snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.contains(SECONDARY_CONTROL_HINTS))
        );
    }

    #[test]
    fn editor_surface_snapshot_includes_gutter_spacing_and_reserved_space() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == " 1 │ |")
            .expect("expected editor input line");

        assert!(input_position > 0);
        assert_eq!(snapshot.game[input_position - 1], "");
        assert!(snapshot.game.iter().any(|line| line == " 2 │ "));
        assert!(
            !snapshot.game.iter().any(|line| line.contains('─')),
            "editor surface should not use a partial horizontal divider"
        );
    }

    #[test]
    fn editor_gutter_width_stays_stable_for_small_line_counts() {
        assert_eq!(gutter_width(1), 2);
        assert_eq!(gutter_width(2), 2);
        assert_eq!(gutter_width(10), 2);
        assert_eq!(gutter_width(100), 3);
    }

    #[test]
    fn editor_canvas_renders_blank_rows_below_current_input() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = workbox_lines(&frame, SNAPSHOT_WORKBOX_HEIGHT);
        let rendered: Vec<String> = lines.iter().map(line_text).collect();

        assert_eq!(rendered[0], "");
        assert_eq!(rendered[1], " 1 │ |");
        assert_eq!(rendered[2], " 2 │ ");
        assert_eq!(rendered[3], " 3 │ ");
        assert!(!rendered.iter().any(|line| line.contains("fn main")));
    }

    #[test]
    fn active_line_style_is_mapped_through_editor_theme() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = workbox_lines(&frame, SNAPSHOT_WORKBOX_HEIGHT);
        let active_line = &lines[1];
        let blank_line = &lines[2];

        assert_eq!(
            active_line.style.bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(
            active_line.spans[0].style.bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(blank_line.style.bg, Some(RUST_REPS_DARK.surface));
        assert_eq!(blank_line.spans[0].style.bg, Some(RUST_REPS_DARK.surface));
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
            .collect();
        for forbidden in ["Passed", "Skipped", "Attempts", "READY"] {
            assert!(
                !visible_lines.iter().any(|line| line.contains(forbidden)),
                "default game/feedback render should not contain {forbidden}"
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
    fn idle_counter_renders_inside_command_rail() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert_eq!(
            snapshot.bottom_chrome,
            vec![
                "".to_string(),
                format!("reps 0   {PRIMARY_ACTION_HINT}"),
                SECONDARY_CONTROL_HINTS.to_string(),
                "".to_string(),
            ]
        );
        assert_eq!(bottom_chrome_alignment(), Alignment::Left);
        assert!(!snapshot.bottom_chrome.iter().any(|line| line.contains('●')));
        assert!(!snapshot.bottom_chrome.iter().any(|line| line.contains('✓')));
        assert!(!snapshot.game.iter().any(|line| line.contains("Passed")));
        assert!(!snapshot.feedback.iter().any(|line| line.contains("Passed")));
    }

    #[test]
    fn bottom_chrome_shows_minimal_control_hints_without_dashboard_stats() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app.frame());
        let rail = snapshot.bottom_chrome.join("\n");

        for expected in ["F5 submit", "Enter newline", "Esc quit", "? help"] {
            assert!(rail.contains(expected), "missing {expected}");
        }
        let f5 = rail.find("F5 submit").expect("missing F5 submit");
        let enter = rail.find("Enter newline").expect("missing Enter newline");
        let esc = rail.find("Esc quit").expect("missing Esc quit");
        let help = rail.find("? help").expect("missing ? help");
        assert!(f5 < enter);
        assert!(enter < esc);
        assert!(esc < help);

        for line in &snapshot.bottom_chrome {
            for forbidden in ["Passed", "Skipped", "Attempts", "READY", "%", "/12"] {
                assert!(
                    !line.contains(forbidden),
                    "bottom rail should not contain {forbidden}"
                );
            }
        }
        assert!(
            snapshot
                .bottom_chrome
                .iter()
                .any(|line| line == &format!("reps 0   {PRIMARY_ACTION_HINT}"))
        );
    }

    #[test]
    fn command_rail_groups_counter_reward_and_primary_action() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = bottom_chrome_lines(&frame);
        let reward_line = lines.get(1).expect("expected bottom rail reward line");
        let controls_line = lines.get(2).expect("expected bottom rail controls line");

        assert_eq!(lines.len(), usize::from(COMMAND_RAIL_HEIGHT));
        assert_eq!(reward_line.spans[0].content.as_ref(), "reps 0");
        assert_eq!(
            reward_line.spans[0].style.fg,
            Some(RUST_REPS_DARK.counter_normal)
        );
        assert!(
            reward_line.spans[0]
                .style
                .add_modifier
                .contains(Modifier::BOLD)
        );
        assert_eq!(reward_line.spans[1].content.as_ref(), "   ");
        assert_eq!(reward_line.spans[2].content.as_ref(), PRIMARY_ACTION_HINT);
        assert!(
            reward_line.spans[2]
                .style
                .add_modifier
                .contains(Modifier::BOLD)
        );
        assert_eq!(
            controls_line.spans[0].content.as_ref(),
            SECONDARY_CONTROL_HINTS
        );
        assert_eq!(
            controls_line.spans[0].style.fg,
            Some(RUST_REPS_DARK.text_muted)
        );
    }

    #[test]
    fn success_reward_shows_plus_one_next_to_stable_counter() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");

        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert!(line.spans[0].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[1].content.as_ref(), "   +1");
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.reward_plus_one));
        assert!(line.spans[1].style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(line.spans[3].content.as_ref(), PRIMARY_ACTION_HINT);
        assert_eq!(lines[2].spans[0].content.as_ref(), SECONDARY_CONTROL_HINTS);
    }

    #[test]
    fn reward_plus_one_sequence_hits_fast_holds_then_releases_slowly() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");
        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[1].content.as_ref(), "   +1");
        assert_ne!(line.spans[1].style.fg, Some(RUST_REPS_DARK.reward_plus_one));

        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.reward_plus_one));

        let frame =
            app.frame_at(now + REWARD_ATTACK_DURATION + (REWARD_PLUS_ONE_BODY_DURATION / 2));
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");
        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert_eq!(line.spans[1].content.as_ref(), "   +1");
        assert_eq!(line.spans[1].style.fg, Some(RUST_REPS_DARK.reward_plus_one));

        let frame = app.frame_at(now + REWARD_DURATION - Duration::from_millis(500));
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");
        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[1].content.as_ref(), "   +1");
        assert_ne!(line.spans[1].style.fg, Some(RUST_REPS_DARK.reward_plus_one));

        let frame = app.frame_at(now + REWARD_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");
        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert!(!line_text(line).contains("+1"));
        assert!(line_text(line).contains(PRIMARY_ACTION_HINT));
        assert_eq!(lines[2].spans[0].content.as_ref(), SECONDARY_CONTROL_HINTS);
    }

    #[test]
    fn plus_one_expires_after_reward_duration() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_DURATION);
        let lines = bottom_chrome_lines(&frame);
        let line = lines.get(1).expect("expected bottom rail reward line");

        assert_eq!(line.spans[0].content.as_ref(), "reps 1");
        assert_eq!(line.spans[0].style.fg, Some(RUST_REPS_DARK.counter_normal));
        assert!(!line_text(line).contains("+1"));
        assert!(line_text(line).contains(PRIMARY_ACTION_HINT));
    }

    #[test]
    fn border_style_maps_reward_settle_phases() {
        let idle = border_style(RewardFrame {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
        });
        let low = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 80,
            game_glow_intensity: 20,
            sweep_progress: 25,
            show_plus_one: true,
        });
        let medium = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 40,
            sweep_progress: 55,
            show_plus_one: true,
        });
        let peak = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 58,
            sweep_progress: 100,
            show_plus_one: true,
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
    fn sweep_cell_progress_runs_from_bottom_left_to_top_right() {
        let area = Rect::new(2, 3, 20, 10);
        let bottom_left = border_sweep_cell_progress(area.left(), area.bottom() - 1, area);
        let top_right = border_sweep_cell_progress(area.right() - 1, area.top(), area);
        let top_left = border_sweep_cell_progress(area.left(), area.top(), area);
        let bottom_right = border_sweep_cell_progress(area.right() - 1, area.bottom() - 1, area);

        assert_eq!(bottom_left, 0.0);
        assert_eq!(top_right, 1.0);
        assert!(top_left > bottom_left);
        assert!(bottom_right > bottom_left);
        assert!(top_left < top_right);
        assert!(bottom_right < top_right);
    }

    #[test]
    fn directional_sweep_lights_bottom_left_before_top_right() {
        let area = Rect::new(0, 0, 80, 20);
        let early = reward_frame(100, 58, 10);
        let bottom_left = reward_sweep_intensity_at(area.left(), area.bottom() - 1, area, early);
        let top_right = reward_sweep_intensity_at(area.right() - 1, area.top(), area, early);

        assert!(bottom_left > 30);
        assert!(top_right < 5);
    }

    #[test]
    fn directional_sweep_reaches_top_right_later() {
        let area = Rect::new(0, 0, 80, 20);
        let early = reward_frame(100, 58, 10);
        let later = reward_frame(100, 58, 100);
        let early_top_right = reward_sweep_intensity_at(area.right() - 1, area.top(), area, early);
        let later_top_right = reward_sweep_intensity_at(area.right() - 1, area.top(), area, later);

        assert!(later_top_right > early_top_right);
        assert!(later_top_right > 45);
    }

    #[test]
    fn directional_sweep_fades_after_passing() {
        let area = Rect::new(0, 0, 80, 20);
        let peak = reward_frame(100, 58, 100);
        let faded = reward_frame(20, 12, 100);
        let peak_top_right = reward_sweep_intensity_at(area.right() - 1, area.top(), area, peak);
        let faded_top_right = reward_sweep_intensity_at(area.right() - 1, area.top(), area, faded);

        assert!(peak_top_right > faded_top_right);
        assert!(faded_top_right > 0);
    }

    #[test]
    fn idle_border_sweep_intensity_is_normal() {
        let area = Rect::new(0, 0, 80, 20);
        let idle = RewardFrame {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
        };

        assert_eq!(
            reward_sweep_intensity_at(area.left(), area.bottom() - 1, area, idle),
            0
        );
        assert_eq!(
            reward_sweep_intensity_at(area.right() - 1, area.top(), area, idle),
            0
        );
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
        assert_eq!(idle_chunks.len(), 4);
        assert_eq!(idle_chunks[0].height, PROBLEM_ZONE_HEIGHT);
        assert_eq!(idle_chunks[2].height, FEEDBACK_CHROME_HEIGHT);
        assert_eq!(idle_chunks[3].height, COMMAND_RAIL_HEIGHT);
    }

    #[test]
    fn wrong_answer_does_not_render_plus_one_or_increment_counter() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(
            snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.starts_with("reps 0"))
        );
        assert!(
            !snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
    }

    #[test]
    fn skip_and_quit_do_not_render_plus_one() {
        let mut skip_app = TuiApp::new();
        for ch in ":skip".chars() {
            skip_app.handle_action(ClientAction::TypeChar(ch));
        }
        skip_app.handle_action(ClientAction::Submit);
        let skip_snapshot = render_snapshot(&skip_app.frame());

        let mut quit_app = TuiApp::new();
        quit_app.handle_action(ClientAction::Quit);
        let quit_snapshot = render_snapshot(&quit_app.frame());

        assert!(
            skip_snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.starts_with("reps 0"))
        );
        assert!(
            !skip_snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(
            quit_snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.starts_with("reps 0"))
        );
        assert!(
            !quit_snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
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

        assert!(snapshot.game.iter().any(|line| line == " 1 │ hello|"));
    }

    #[test]
    fn render_snapshot_shows_caret_in_middle_of_input() {
        let mut app = TuiApp::new();
        for ch in "abc".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::MoveCursorLeft);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.game.iter().any(|line| line == " 1 │ ab|c"));
    }

    #[test]
    fn render_snapshot_hides_cursor_when_cursor_state_is_hidden() {
        let mut app = TuiApp::new();
        app.tick_cursor();
        let frame = app.frame();
        let empty_snapshot = render_snapshot(&frame);
        assert!(empty_snapshot.game.iter().any(|line| line == " 1 │  "));

        app.handle_action(ClientAction::TypeChar('x'));
        app.tick_cursor();
        let frame = app.frame();
        let typed_snapshot = render_snapshot(&frame);
        assert!(typed_snapshot.game.iter().any(|line| line == " 1 │ x "));
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
                .any(|line| line == "Controls:")
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
                .any(|line| line == "Controls:")
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("Enter newline"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("F5 submit"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("Esc quit"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("? help"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("Arrows/Home/End navigate"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("Shift+Enter newline if reported"))
        );
        assert!(
            help_snapshot
                .feedback
                .iter()
                .any(|line| line.contains("Ctrl+Enter submit if supported"))
        );
        assert!(!help_snapshot.game.iter().any(|line| line == "Controls:"));
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
    fn render_snapshot_hides_pass_and_shows_plus_one_after_success() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_DURATION / 2);
        let snapshot = render_snapshot(&frame);

        assert!(!snapshot.feedback.iter().any(|line| line == "PASS"));
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == "println!(\"Rust Reps\");")
        );
        assert!(snapshot.game.iter().any(|line| line == " 1 │ |"));
        assert!(
            snapshot
                .bottom_chrome
                .iter()
                .any(|line| line.starts_with("reps 1") && line.contains("+1"))
        );
        assert!(!snapshot.game.iter().any(|line| line == "PASS"));
    }

    #[test]
    fn render_snapshot_shows_two_editor_lines() {
        let mut app = TuiApp::new();
        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('b'));

        let snapshot = render_snapshot(&app.frame());

        assert!(snapshot.game.iter().any(|line| line == " 1 │ a"));
        assert!(snapshot.game.iter().any(|line| line == " 2 │ b|"));
    }

    #[test]
    fn render_snapshot_shows_caret_on_second_line() {
        let mut app = TuiApp::new();
        for ch in "let".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::InsertNewline);
        for ch in "score".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::MoveCursorLeft);

        let snapshot = render_snapshot(&app.frame());

        assert!(snapshot.game.iter().any(|line| line == " 1 │ let"));
        assert!(snapshot.game.iter().any(|line| line == " 2 │ scor|e"));
    }

    #[test]
    fn render_snapshot_shows_empty_middle_line() {
        let mut app = TuiApp::new();
        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('b'));

        let snapshot = render_snapshot(&app.frame());

        assert!(snapshot.game.iter().any(|line| line == " 1 │ a"));
        assert!(snapshot.game.iter().any(|line| line == " 2 │ "));
        assert!(snapshot.game.iter().any(|line| line == " 3 │ b|"));
    }

    #[test]
    fn render_snapshot_keeps_one_line_case_clean() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app.frame());

        assert!(snapshot.game.iter().any(|line| line == " 1 │ |"));
        assert!(snapshot.game.iter().any(|line| line == " 2 │ "));
    }

    #[test]
    fn viewport_snapshot_keeps_cursor_line_visible_when_editor_is_taller_than_workbox() {
        let mut app = TuiApp::new();
        for ch in "a\nb\nc\nd\ne".chars() {
            if ch == '\n' {
                app.handle_action(ClientAction::InsertNewline);
            } else {
                app.handle_action(ClientAction::TypeChar(ch));
            }
        }

        let snapshot = render_snapshot_with_workbox_height(&app.frame(), 3);

        assert!(!snapshot.game.iter().any(|line| line == " 1 │ a"));
        assert!(snapshot.game.iter().any(|line| line == " 4 │ d"));
        assert!(snapshot.game.iter().any(|line| line == " 5 │ e|"));
    }
}
