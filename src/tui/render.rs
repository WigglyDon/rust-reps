use crate::client::CursorPosition;
use crate::{
    client::{
        ClientFrame, CodeToken, EditorAnchor, FailureFrame, ProblemDisplayLine, ProblemLineKind,
        RewardFrame, STIMULUS_GUTTER_WIDTH, StatusTone, StimulusRowKind, ViewportOverflow,
    },
    tui::{
        app::TuiApp,
        input::MousePane,
        theme::{
            base_style, border_sweep_style, code_token_style, control_hint_style, counter_style,
            editor_canvas_style, editor_gutter_feedback_style, editor_line_style, muted_style,
            plus_one_style, primary_action_style, problem_header_style, status_tone_style,
            surface_style,
        },
    },
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Margin, Position, Rect},
    style::Modifier,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

const FEEDBACK_CHROME_HEIGHT: u16 = 0;
const COMMAND_RAIL_HEIGHT: u16 = 1;
const TOP_COUNTER_HEIGHT: u16 = 1;
const MIN_GUTTER_WIDTH: usize = 2;
const EDITOR_SOURCE_SEPARATOR_WIDTH: usize = 3;
const EDITOR_TRAILING_INDICATOR_WIDTH: usize = 1;
const PROBLEM_HORIZONTAL_PADDING: u16 = 1;
const PROBLEM_TRAILING_RESERVE: usize = 3;
const FORMAT_ACTION_HINT: &str = "F1 format";
const PRIMARY_ACTION_HINT: &str = "F5 submit";
const SKIP_ACTION_HINT: &str = "F7 skip";
const QUIT_ACTION_HINT: &str = "Esc quit";
const STIMULUS_HARD_LINE_GUTTER: &str = "  ";
const STIMULUS_CONTINUATION_GUTTER: &str = "↪ ";
#[cfg(test)]
const SNAPSHOT_PROBLEM_HEIGHT: u16 = 8;
#[cfg(test)]
const SNAPSHOT_WORKBOX_HEIGHT: u16 = 8;

pub(crate) fn render_ui(frame: &mut Frame<'_>, app: &mut TuiApp, now: std::time::Instant) {
    let preliminary_frame = app.frame_at(now);
    let feedback_lines = feedback_chrome_lines(&preliminary_frame);
    let feedback_height = feedback_height(&feedback_lines);
    let width_layout = shell_layout_for_content(
        frame.area(),
        feedback_height,
        preliminary_frame.game_window.workbox.lines.len().max(1),
        TOP_COUNTER_HEIGHT,
    );
    app.set_editor_viewport_width(editor_text_columns_for_area(
        width_layout.editor,
        preliminary_frame.game_window.workbox.lines.len().max(1),
    ));

    let problem_wrap_width = problem_wrap_width(problem_content_area(width_layout.problem));
    let projected_frame = app.frame_at_for_layout(now, problem_wrap_width);
    let final_layout =
        shell_layout_for_client_frame(frame.area(), feedback_height, &projected_frame);
    app.set_editor_viewport_height(visible_editor_rows(final_layout.editor.height));

    let client_frame = app.frame_at_for_layout(now, problem_wrap_width);
    render_client_frame(frame, &client_frame);
}

fn render_client_frame(frame: &mut Frame<'_>, client_frame: &ClientFrame) {
    let feedback_lines = feedback_chrome_lines(client_frame);
    let shell_area = frame.area();
    let layout =
        shell_layout_for_client_frame(shell_area, feedback_height(&feedback_lines), client_frame);

    frame.render_widget(Block::default().style(base_style()), shell_area);

    let game_block = Block::default().style(surface_style());
    frame.render_widget(game_block, shell_area);
    render_game_border(frame, shell_area, client_frame.reward);

    let instruction_area = problem_content_area(layout.instruction);
    frame.render_widget(
        Paragraph::new(Text::from(instruction_lines_for_height(
            client_frame,
            instruction_area.height,
        )))
        .style(editor_canvas_style()),
        instruction_area,
    );

    let problem_area = problem_content_area(layout.problem);
    frame.render_widget(
        Paragraph::new(Text::from(problem_lines_for_height(
            client_frame,
            problem_area.height,
        )))
        .style(editor_canvas_style()),
        problem_area,
    );
    render_viewport_indicator(
        frame,
        layout.problem,
        client_frame
            .game_window
            .problem
            .overflow(usize::from(problem_area.height)),
    );

    frame.render_widget(
        Paragraph::new(Text::from(workbox_lines_for_area(
            client_frame,
            layout.editor,
        )))
        .style(editor_canvas_style()),
        layout.editor,
    );
    render_editor_horizontal_indicators(frame, layout.editor, client_frame);
    render_editor_cursor(frame, layout.editor, client_frame);
    render_editor_toasts(frame, layout.editor, client_frame);
    render_viewport_indicator(
        frame,
        layout.editor,
        client_frame
            .game_window
            .workbox
            .overflow(visible_editor_rows(layout.editor.height)),
    );
    render_editor_controls(frame, layout.controls, client_frame.format_available);

    if !feedback_lines.is_empty() {
        frame.render_widget(
            Paragraph::new(Text::from(feedback_lines)).style(base_style()),
            layout.feedback,
        );
    }

    render_top_banner(frame, layout.rail, client_frame);
}

pub(crate) fn input_layout(area: Rect, app: &TuiApp, now: std::time::Instant) -> ShellLayout {
    let preliminary_frame = app.frame_at(now);
    let feedback_height = feedback_height(&feedback_chrome_lines(&preliminary_frame));
    let width_layout = shell_layout_for_content(
        area,
        feedback_height,
        preliminary_frame.game_window.workbox.lines.len().max(1),
        TOP_COUNTER_HEIGHT,
    );
    let client_frame = app.frame_at_for_layout(
        now,
        problem_wrap_width(problem_content_area(width_layout.problem)),
    );
    shell_layout_for_client_frame(area, feedback_height, &client_frame)
}

#[cfg(test)]
fn layout_chunks(area: Rect, feedback_height: u16) -> std::rc::Rc<[Rect]> {
    let layout = shell_layout(area, feedback_height);
    vec![
        layout.problem,
        layout.editor,
        layout.controls,
        layout.feedback,
        layout.rail,
        layout.instruction,
    ]
    .into()
}

#[cfg(test)]
fn shell_layout(area: Rect, feedback_height: u16) -> ShellLayout {
    shell_layout_for_content(area, feedback_height, 1, TOP_COUNTER_HEIGHT)
}

fn shell_layout_for_client_frame(
    area: Rect,
    feedback_height: u16,
    client_frame: &ClientFrame,
) -> ShellLayout {
    shell_layout_for_content(
        area,
        feedback_height,
        client_frame.game_window.workbox.lines.len().max(1),
        shared_header_height(client_frame),
    )
}

fn shared_header_height(client_frame: &ClientFrame) -> u16 {
    usize_to_u16(
        client_frame
            .game_window
            .problem
            .instruction_lines()
            .len()
            .max(usize::from(TOP_COUNTER_HEIGHT)),
    )
}

fn shell_layout_for_content(
    area: Rect,
    feedback_height: u16,
    editor_line_count: usize,
    header_rows: u16,
) -> ShellLayout {
    let body = shell_body(area);
    let reserved_panes = body.height.min(2);
    let header_height = header_rows
        .max(TOP_COUNTER_HEIGHT)
        .min(body.height.saturating_sub(reserved_panes));
    let feedback_height =
        feedback_height.min(body.height.saturating_sub(header_height + reserved_panes));
    let controls_height = COMMAND_RAIL_HEIGHT.min(
        body.height
            .saturating_sub(header_height + feedback_height + reserved_panes),
    );
    let panel_height = body
        .height
        .saturating_sub(header_height + controls_height + feedback_height);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Length(panel_height),
            Constraint::Length(controls_height),
            Constraint::Length(feedback_height),
        ])
        .split(body);
    let panels = panel_layout(vertical[1], editor_line_count);
    let editor = panels[0];
    let controls = Rect::new(body.x, vertical[2].y, body.width, controls_height);
    let rail = Rect::new(panels[0].x, vertical[0].y, panels[0].width, header_height);
    let instruction = Rect::new(panels[1].x, vertical[0].y, panels[1].width, header_height);
    ShellLayout {
        editor,
        problem: panels[1],
        controls,
        feedback: vertical[3],
        rail,
        instruction,
    }
}

fn panel_layout(area: Rect, editor_line_count: usize) -> [Rect; 2] {
    if area.width == 0 {
        return [area, area];
    }

    if area.width <= 1 {
        let editor = Rect::new(area.x, area.y, area.width, area.height);
        let problem = Rect::new(area.right(), area.y, 0, area.height);
        return [editor, problem];
    }

    let mut editor_width = 1;
    let mut gap_width = 0;
    let mut best_difference = usize::MAX;
    let mut best_shared_width = 0usize;
    let mut best_total_width = 0usize;
    let mut best_balance = u16::MAX;

    let gap_candidates: &[u16] = if area.width >= 42 { &[1, 2] } else { &[0] };
    for &candidate_gap_width in gap_candidates {
        let content_width = area.width.saturating_sub(candidate_gap_width);
        for candidate_editor_width in 1..content_width {
            let candidate_problem_width = content_width - candidate_editor_width;
            let editor_source = editor_source_capacity(candidate_editor_width, editor_line_count);
            let problem_source = problem_source_capacity(candidate_problem_width);
            let difference = editor_source.abs_diff(problem_source);
            let shared_width = editor_source.min(problem_source);
            let total_width = editor_source.saturating_add(problem_source);
            let balance = candidate_editor_width.abs_diff(candidate_problem_width);
            let is_better = difference < best_difference
                || (difference == best_difference && shared_width > best_shared_width)
                || (difference == best_difference
                    && shared_width == best_shared_width
                    && total_width > best_total_width)
                || (difference == best_difference
                    && shared_width == best_shared_width
                    && total_width == best_total_width
                    && balance < best_balance);

            if is_better {
                editor_width = candidate_editor_width;
                gap_width = candidate_gap_width;
                best_difference = difference;
                best_shared_width = shared_width;
                best_total_width = total_width;
                best_balance = balance;
            }
        }
    }

    let content_width = area.width.saturating_sub(gap_width);
    let problem_width = content_width.saturating_sub(editor_width);
    let editor = Rect::new(area.x, area.y, editor_width, area.height);
    let problem = Rect::new(
        area.x + editor_width + gap_width,
        area.y,
        problem_width,
        area.height,
    );
    [editor, problem]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellLayout {
    pub problem: Rect,
    pub editor: Rect,
    pub instruction: Rect,
    pub controls: Rect,
    pub feedback: Rect,
    pub rail: Rect,
}

impl ShellLayout {
    pub(crate) fn mouse_pane_at(self, column: u16, row: u16) -> MousePane {
        if rect_contains(self.problem, column, row) || rect_contains(self.instruction, column, row)
        {
            MousePane::Problem
        } else if rect_contains(self.editor, column, row) {
            MousePane::Editor
        } else {
            MousePane::Outside
        }
    }
}

fn rect_contains(rect: Rect, column: u16, row: u16) -> bool {
    column >= rect.left() && column < rect.right() && row >= rect.top() && row < rect.bottom()
}

fn shell_body(area: Rect) -> Rect {
    area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    })
}

fn problem_content_area(area: Rect) -> Rect {
    area.inner(Margin {
        vertical: 0,
        horizontal: if area.width > 4 {
            PROBLEM_HORIZONTAL_PADDING
        } else {
            0
        },
    })
}

fn problem_wrap_width(area: Rect) -> usize {
    usize::from(area.width).saturating_sub(PROBLEM_TRAILING_RESERVE)
}

fn problem_source_capacity(area_width: u16) -> usize {
    let area = Rect::new(0, 0, area_width, 1);
    problem_wrap_width(problem_content_area(area)).saturating_sub(STIMULUS_GUTTER_WIDTH)
}

fn render_viewport_indicator(frame: &mut Frame<'_>, area: Rect, overflow: ViewportOverflow) {
    if area.width == 0 || area.height == 0 || !overflow.has_overflow() {
        return;
    }

    let x = area.right() - 1;
    let buffer = frame.buffer_mut();
    for (row, symbol) in viewport_indicator_cells(area.height, overflow) {
        buffer[(x, area.top() + row)]
            .set_symbol(symbol)
            .set_style(problem_header_style());
    }
}

fn viewport_indicator_cells(height: u16, overflow: ViewportOverflow) -> Vec<(u16, &'static str)> {
    if height == 0 || !overflow.has_overflow() {
        return Vec::new();
    }

    let mut cells = Vec::new();
    if overflow.above {
        cells.push((0, "↑"));
    }
    if overflow.below {
        cells.push((height - 1, "↓"));
    }

    let track_start = if overflow.above { 1 } else { 0 };
    let track_end = height.saturating_sub(if overflow.below { 1 } else { 0 });
    if track_start < track_end {
        let track_height = track_end - track_start;
        let max_offset = overflow.total.saturating_sub(overflow.visible);
        let thumb_row = if max_offset == 0 || track_height <= 1 {
            track_start
        } else {
            let track_span = usize::from(track_height - 1);
            track_start + ((overflow.offset.min(max_offset) * track_span / max_offset) as u16)
        };
        cells.push((thumb_row, "┃"));
    }

    cells.sort_by_key(|(row, _)| *row);
    cells.dedup_by_key(|(row, _)| *row);
    cells
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
        (x, y) if x == left && y == top => "┏",
        (x, y) if x == right && y == top => "┓",
        (x, y) if x == left && y == bottom => "┗",
        (x, y) if x == right && y == bottom => "┛",
        (_, y) if y == top || y == bottom => "━",
        _ => "┃",
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
    let vertical_from_top = f32::from(y.saturating_sub(area.top())) / f32::from(area.height - 1);
    ((horizontal + vertical_from_top) / 2.0).clamp(0.0, 1.0)
}

fn smoothstep(progress: f32) -> f32 {
    progress * progress * (3.0 - (2.0 * progress))
}

#[cfg(test)]
fn problem_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    let mut lines = instruction_lines_for_height(client_frame, SNAPSHOT_PROBLEM_HEIGHT);
    let source_height = SNAPSHOT_PROBLEM_HEIGHT.saturating_sub(usize_to_u16(lines.len()));
    lines.extend(problem_lines_for_height(client_frame, source_height));
    lines
}

fn instruction_lines_for_height(
    client_frame: &ClientFrame,
    instruction_height: u16,
) -> Vec<Line<'static>> {
    client_frame
        .game_window
        .problem
        .instruction_lines()
        .iter()
        .take(usize::from(instruction_height))
        .map(problem_display_line)
        .collect()
}

fn problem_lines_for_height(client_frame: &ClientFrame, problem_height: u16) -> Vec<Line<'static>> {
    if problem_height == 0 {
        return Vec::new();
    }

    let problem = &client_frame.game_window.problem;
    let available_rows = usize::from(problem_height);
    problem
        .visible_lines(available_rows)
        .iter()
        .map(problem_display_line)
        .collect::<Vec<_>>()
}

fn problem_display_line(line: &ProblemDisplayLine) -> Line<'static> {
    let mut spans = match line.kind {
        ProblemLineKind::Blank => Vec::new(),
        ProblemLineKind::Header | ProblemLineKind::AnswerShape => {
            vec![Span::styled(line.text.clone(), problem_header_style())]
        }
        ProblemLineKind::Code => code_spans(&line.tokens),
        ProblemLineKind::Bullet | ProblemLineKind::Plain => {
            vec![Span::raw(line.text.clone())]
        }
    };

    if let Some(row_kind) = line.stimulus_row {
        let gutter = match row_kind {
            StimulusRowKind::HardLineStart => STIMULUS_HARD_LINE_GUTTER,
            StimulusRowKind::SoftContinuation => STIMULUS_CONTINUATION_GUTTER,
        };
        spans.insert(0, Span::styled(gutter, muted_style()));
    }

    Line::from(spans)
}

#[cfg(test)]
fn workbox_lines(client_frame: &ClientFrame, workbox_height: u16) -> Vec<Line<'static>> {
    workbox_lines_for_width(client_frame, workbox_height, u16::MAX)
}

fn workbox_lines_for_area(client_frame: &ClientFrame, area: Rect) -> Vec<Line<'static>> {
    workbox_lines_for_width(client_frame, area.height, area.width)
}

fn workbox_lines_for_width(
    client_frame: &ClientFrame,
    workbox_height: u16,
    workbox_width: u16,
) -> Vec<Line<'static>> {
    let visible_editor_rows = visible_editor_rows(workbox_height);
    let source_viewport = client_frame.game_window.workbox.viewport;
    let total_lines = client_frame.game_window.workbox.lines.len().max(1);
    let visible_lines = source_viewport
        .visible_lines
        .min(visible_editor_rows)
        .max(1);
    let max_first_line = total_lines.saturating_sub(visible_lines);
    let first_line = source_viewport.first_line.min(max_first_line);
    let visible_end = (first_line + visible_lines).min(total_lines);
    let gutter_width = gutter_width(total_lines.max(visible_end));
    let visible_columns = source_viewport
        .visible_columns
        .min(editor_text_columns(workbox_width, total_lines))
        .max(1);
    let first_column = source_viewport.first_column;
    let mut lines = Vec::new();

    for line_index in first_line..visible_end {
        let tokens = client_frame
            .game_window
            .workbox
            .line_tokens
            .get(line_index)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let active = line_index == client_frame.game_window.workbox.cursor.line;
        lines.push(editor_line(
            line_index,
            gutter_width,
            tokens,
            active,
            client_frame.failure,
            first_column,
            visible_columns,
        ));
    }

    while lines.len() < usize::from(workbox_height) {
        lines.push(Line::from("").style(editor_canvas_style()));
    }

    lines
}

fn editor_text_columns_for_area(area: Rect, total_lines: usize) -> usize {
    editor_text_columns(area.width, total_lines)
}

fn editor_text_columns(area_width: u16, total_lines: usize) -> usize {
    editor_source_capacity(area_width, total_lines).max(1)
}

fn editor_source_capacity(area_width: u16, total_lines: usize) -> usize {
    usize::from(area_width)
        .saturating_sub(gutter_width(total_lines.max(1)))
        .saturating_sub(EDITOR_SOURCE_SEPARATOR_WIDTH)
        .saturating_sub(EDITOR_TRAILING_INDICATOR_WIDTH)
}

fn render_editor_cursor(frame: &mut Frame<'_>, area: Rect, client_frame: &ClientFrame) {
    let workbox = &client_frame.game_window.workbox;
    if !workbox.cursor_visible {
        return;
    }

    let Some(anchor) = visible_cursor_anchor(area, client_frame) else {
        return;
    };
    let Some((x, y)) = editor_anchor_position(area, client_frame, anchor, 1) else {
        return;
    };

    frame.set_cursor_position(Position::new(x, y));
}

fn visible_cursor_anchor(area: Rect, client_frame: &ClientFrame) -> Option<EditorAnchor> {
    let workbox = &client_frame.game_window.workbox;
    let visible_editor_rows = visible_editor_rows(area.height);
    let total_lines = workbox.lines.len().max(1);
    let visible_lines = workbox
        .viewport
        .visible_lines
        .min(visible_editor_rows)
        .max(1)
        .min(total_lines);
    let max_first_line = total_lines.saturating_sub(visible_lines);
    let first_line = workbox.viewport.first_line.min(max_first_line);
    let end_line = first_line + visible_lines;

    if workbox.cursor.line < first_line || workbox.cursor.line >= end_line {
        return None;
    }
    if workbox.cursor.column < workbox.viewport.first_column {
        return None;
    }

    let visible_columns =
        editor_text_columns_for_area(area, total_lines).min(workbox.viewport.visible_columns);
    if workbox.cursor.column >= workbox.viewport.first_column + visible_columns.max(1) {
        return None;
    }

    Some(EditorAnchor {
        row: workbox.cursor.line - first_line,
        column: workbox.cursor.column - workbox.viewport.first_column,
    })
}

pub(crate) fn editor_cursor_at(
    area: Rect,
    client_frame: &ClientFrame,
    column: u16,
    row: u16,
) -> Option<CursorPosition> {
    if !rect_contains(area, column, row) || area.height == 0 {
        return None;
    }

    let workbox = &client_frame.game_window.workbox;
    let total_lines = workbox.lines.len().max(1);
    let visible_rows = visible_editor_rows(area.height);
    let visible_lines = workbox
        .viewport
        .visible_lines
        .min(visible_rows)
        .max(1)
        .min(total_lines);
    let first_line = workbox
        .viewport
        .first_line
        .min(total_lines.saturating_sub(visible_lines));
    let clicked_row = usize::from(row.saturating_sub(area.top()));
    let line_index = first_line.saturating_add(clicked_row).min(total_lines - 1);
    let content_left = area
        .left()
        .saturating_add(usize_to_u16(gutter_width(total_lines)))
        .saturating_add(3);
    let clicked_column = usize::from(column.saturating_sub(content_left))
        .saturating_add(workbox.viewport.first_column);
    let line_len = workbox
        .lines
        .get(line_index)
        .map(|line| line.chars().count())
        .unwrap_or(0);

    Some(CursorPosition {
        line: line_index,
        column: clicked_column.min(line_len),
    })
}

fn render_editor_horizontal_indicators(
    frame: &mut Frame<'_>,
    area: Rect,
    client_frame: &ClientFrame,
) {
    let workbox = &client_frame.game_window.workbox;
    let Some(anchor) = visible_cursor_anchor(area, client_frame) else {
        return;
    };
    let Some((content_left, row_y)) = editor_anchor_position(
        area,
        client_frame,
        EditorAnchor {
            row: anchor.row,
            column: 0,
        },
        1,
    ) else {
        return;
    };
    let visible_columns = editor_text_columns_for_area(area, workbox.lines.len().max(1))
        .min(workbox.viewport.visible_columns)
        .max(1);
    let (hidden_left, hidden_right) =
        editor_horizontal_overflow(client_frame, workbox.cursor.line, visible_columns);
    let buffer = frame.buffer_mut();
    if hidden_left && content_left > area.left() {
        buffer[(content_left - 1, row_y)]
            .set_symbol("‹")
            .set_style(problem_header_style());
    }
    if hidden_right && area.right() > area.left() {
        buffer[(area.right() - 1, row_y)]
            .set_symbol("›")
            .set_style(problem_header_style());
    }
}

fn editor_horizontal_overflow(
    client_frame: &ClientFrame,
    line_index: usize,
    visible_columns: usize,
) -> (bool, bool) {
    let workbox = &client_frame.game_window.workbox;
    let line_len = workbox
        .lines
        .get(line_index)
        .map(|line| line.chars().count())
        .unwrap_or(0);
    let first_column = workbox.viewport.first_column;
    (
        first_column > 0,
        line_len > first_column.saturating_add(visible_columns.max(1)),
    )
}

fn render_editor_toasts(frame: &mut Frame<'_>, area: Rect, client_frame: &ClientFrame) {
    if client_frame.failure.active {
        render_editor_toast(
            frame,
            area,
            client_frame,
            client_frame.failure.anchor,
            "not quite",
            status_tone_style(StatusTone::TryAgain),
            ToastPlacement::AfterVisibleLine,
        );
    }
    if client_frame.skip.active {
        render_editor_toast(
            frame,
            area,
            client_frame,
            client_frame.skip.anchor,
            "skipped",
            status_tone_style(StatusTone::Warning),
            ToastPlacement::AfterVisibleLine,
        );
    }
    if client_frame.format.active {
        let Some(kind) = client_frame.format.kind else {
            return;
        };
        let style = if kind.is_success() {
            status_tone_style(StatusTone::Pass)
        } else {
            status_tone_style(StatusTone::Warning)
        };
        render_editor_toast(
            frame,
            area,
            client_frame,
            client_frame.format.anchor,
            kind.message(),
            style,
            ToastPlacement::AfterVisibleLine,
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToastPlacement {
    AfterVisibleLine,
}

fn render_editor_toast(
    frame: &mut Frame<'_>,
    area: Rect,
    client_frame: &ClientFrame,
    anchor: Option<EditorAnchor>,
    text: &str,
    style: ratatui::style::Style,
    placement: ToastPlacement,
) {
    let Some(anchor) = anchor else {
        return;
    };
    let text_width = text.chars().count().max(1);
    let Some((anchor_x, y)) = editor_anchor_position(area, client_frame, anchor, 1) else {
        return;
    };
    let right_limit = area.right().saturating_sub(1);
    let total_lines = client_frame.game_window.workbox.lines.len().max(1);
    let content_left = editor_content_left(area, total_lines);
    if content_left >= right_limit {
        return;
    }
    let row_width = usize::from(right_limit.saturating_sub(content_left));
    if placement == ToastPlacement::AfterVisibleLine {
        let after_cursor = anchor_x.saturating_add(1);
        let after_line = editor_visible_line_end_x(area, client_frame, anchor)
            .map(|line_end| line_end.saturating_add(1))
            .unwrap_or(after_cursor);
        let preferred_x = after_cursor.max(after_line);
        if text_width <= row_width {
            let max_x = right_limit.saturating_sub(usize_to_u16(text_width));
            let x = preferred_x.min(max_x).max(content_left);
            render_editor_toast_text(frame, x, y, right_limit, area.bottom(), text, style);
            return;
        }
    }

    let wrapped_y = if y.saturating_add(1) < area.bottom() {
        y + 1
    } else {
        y
    };
    render_editor_toast_text(
        frame,
        content_left,
        wrapped_y,
        right_limit,
        area.bottom(),
        text,
        style,
    );
}

fn render_editor_toast_text(
    frame: &mut Frame<'_>,
    start_x: u16,
    start_y: u16,
    right_limit: u16,
    bottom_limit: u16,
    text: &str,
    style: ratatui::style::Style,
) {
    let mut x = start_x;
    let mut y = start_y;
    let buffer = frame.buffer_mut();
    for ch in text.chars() {
        if x >= right_limit {
            x = start_x;
            y = y.saturating_add(1);
        }
        if y >= bottom_limit {
            break;
        }
        buffer[(x, y)].set_symbol(&ch.to_string()).set_style(style);
        x = x.saturating_add(1);
    }
}

fn editor_anchor_position(
    area: Rect,
    client_frame: &ClientFrame,
    anchor: EditorAnchor,
    text_width: usize,
) -> Option<(u16, u16)> {
    if area.width == 0 || area.height == 0 {
        return None;
    }

    let max_row = usize::from(area.height.saturating_sub(1));
    let row = usize_to_u16(anchor.row.min(max_row));
    let y = area.top().saturating_add(row);
    if y >= area.bottom() {
        return None;
    }

    let total_lines = client_frame.game_window.workbox.lines.len().max(1);
    let content_left = editor_content_left(area, total_lines);
    let indicator_safe_right = area.right().saturating_sub(1);
    if content_left >= indicator_safe_right {
        return None;
    }

    let desired_x = content_left.saturating_add(usize_to_u16(anchor.column));
    let max_x = indicator_safe_right.saturating_sub(usize_to_u16(text_width));
    Some((desired_x.min(max_x).max(content_left), y))
}

fn editor_visible_line_end_x(
    area: Rect,
    client_frame: &ClientFrame,
    anchor: EditorAnchor,
) -> Option<u16> {
    if area.width == 0 {
        return None;
    }

    let workbox = &client_frame.game_window.workbox;
    let total_lines = workbox.lines.len().max(1);
    let content_left = editor_content_left(area, total_lines);
    let line_index = workbox.viewport.first_line.saturating_add(anchor.row);
    let line_len = workbox
        .lines
        .get(line_index)
        .map(|line| line.chars().count())
        .unwrap_or(0);
    let visible_columns = editor_text_columns_for_area(area, total_lines);
    let visible_len = line_len
        .saturating_sub(workbox.viewport.first_column)
        .min(visible_columns);

    Some(content_left.saturating_add(usize_to_u16(visible_len)))
}

fn editor_content_left(area: Rect, total_lines: usize) -> u16 {
    area.left()
        .saturating_add(usize_to_u16(gutter_width(total_lines)))
        .saturating_add(3)
}

fn usize_to_u16(value: usize) -> u16 {
    value.min(usize::from(u16::MAX)) as u16
}

fn feedback_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    let _ = client_frame;
    Vec::new()
}

fn feedback_height(lines: &[Line<'static>]) -> u16 {
    let _ = lines;
    FEEDBACK_CHROME_HEIGHT
}

fn render_top_banner(frame: &mut Frame<'_>, area: Rect, client_frame: &ClientFrame) {
    let rail = area.inner(Margin {
        vertical: 0,
        horizontal: 1,
    });
    frame.render_widget(
        Paragraph::new(Line::from(counter_spans(client_frame))).style(surface_style()),
        rail,
    );
}

fn render_editor_controls(frame: &mut Frame<'_>, area: Rect, format_available: bool) {
    if area.height == 0 {
        return;
    }

    frame.render_widget(
        Paragraph::new(Line::from(command_spans(format_available))).style(base_style()),
        area.inner(Margin {
            vertical: 0,
            horizontal: if area.width > 4 { 1 } else { 0 },
        }),
    );
}

pub(crate) fn visible_editor_rows(workbox_height: u16) -> usize {
    usize::from(workbox_height.max(1))
}

fn gutter_width(total_lines: usize) -> usize {
    total_lines.max(1).to_string().len().max(MIN_GUTTER_WIDTH)
}

fn editor_line(
    line_index: usize,
    gutter_width: usize,
    tokens: &[CodeToken],
    active: bool,
    failure: FailureFrame,
    first_column: usize,
    visible_columns: usize,
) -> Line<'static> {
    let style = editor_line_style(active, failure);
    Line::from(editor_line_spans(
        line_index,
        gutter_width,
        tokens,
        active,
        failure,
        first_column,
        visible_columns,
    ))
    .style(style)
}

fn editor_line_spans(
    line_index: usize,
    gutter_width: usize,
    tokens: &[CodeToken],
    active: bool,
    failure: FailureFrame,
    first_column: usize,
    visible_columns: usize,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(
        format!(
            "{line_number:>gutter_width$} │ ",
            line_number = line_index + 1
        ),
        editor_gutter_feedback_style(active, failure),
    )];

    spans.extend(code_spans_for_columns(
        tokens,
        first_column,
        visible_columns,
    ));

    spans
}

#[cfg(test)]
fn command_chrome_lines(client_frame: &ClientFrame) -> Vec<Line<'static>> {
    vec![Line::from(command_rail_spans(
        client_frame.format_available,
    ))]
}

#[cfg(test)]
fn command_rail_spans(format_available: bool) -> Vec<Span<'static>> {
    command_spans(format_available)
}

fn counter_spans(client_frame: &ClientFrame) -> Vec<Span<'static>> {
    let mut spans = vec![Span::styled(
        format!("● {}", client_frame.completed_reps_this_run),
        counter_style(),
    )];
    if client_frame.reward.show_plus_one {
        spans.push(Span::styled(" ", counter_style()));
        spans.push(Span::styled("+1", plus_one_style(client_frame.reward)));
    }
    spans
}

fn command_spans(format_available: bool) -> Vec<Span<'static>> {
    let format_style = if format_available {
        control_hint_style()
    } else {
        control_hint_style().add_modifier(Modifier::DIM)
    };
    vec![
        Span::styled(FORMAT_ACTION_HINT, format_style),
        Span::styled(" · ", control_hint_style()),
        Span::styled(PRIMARY_ACTION_HINT, primary_action_style()),
        Span::styled(" · ", control_hint_style()),
        Span::styled(SKIP_ACTION_HINT, control_hint_style()),
        Span::styled(" · ", control_hint_style()),
        Span::styled(QUIT_ACTION_HINT, control_hint_style()),
    ]
}

fn code_spans(tokens: &[CodeToken]) -> Vec<Span<'static>> {
    tokens
        .iter()
        .map(|token| Span::styled(token.text.clone(), code_token_style(token.kind)))
        .collect()
}

fn code_spans_for_columns(
    tokens: &[CodeToken],
    first_column: usize,
    visible_columns: usize,
) -> Vec<Span<'static>> {
    tokens_for_columns(tokens, first_column, visible_columns)
        .iter()
        .map(|token| Span::styled(token.text.clone(), code_token_style(token.kind)))
        .collect()
}

fn tokens_for_columns(
    tokens: &[CodeToken],
    first_column: usize,
    visible_columns: usize,
) -> Vec<CodeToken> {
    let end_column = first_column.saturating_add(visible_columns.max(1));
    let mut cursor = 0usize;
    let mut visible = Vec::new();

    for token in tokens {
        let token_width = token.text.chars().count();
        let token_start = cursor;
        let token_end = token_start.saturating_add(token_width);
        if token_end > first_column && token_start < end_column {
            let start = first_column.saturating_sub(token_start);
            let end = end_column.min(token_end).saturating_sub(token_start);
            let text = token
                .text
                .chars()
                .skip(start)
                .take(end.saturating_sub(start))
                .collect::<String>();
            if !text.is_empty() {
                visible.push(CodeToken {
                    kind: token.kind,
                    text,
                });
            }
        }
        cursor = token_end;
    }

    visible
}

#[cfg(test)]
fn command_chrome_alignment() -> ratatui::layout::Alignment {
    ratatui::layout::Alignment::Left
}

#[cfg(test)]
struct RenderSnapshot {
    game: Vec<String>,
    feedback: Vec<String>,
    command_chrome: Vec<String>,
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
        command_chrome: command_chrome_lines(client_frame)
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
    use std::borrow::Cow;
    use std::time::{Duration, Instant};

    use super::{
        COMMAND_RAIL_HEIGHT, FEEDBACK_CHROME_HEIGHT, FORMAT_ACTION_HINT, PRIMARY_ACTION_HINT,
        QUIT_ACTION_HINT, SKIP_ACTION_HINT, SNAPSHOT_PROBLEM_HEIGHT, SNAPSHOT_WORKBOX_HEIGHT,
        STIMULUS_CONTINUATION_GUTTER, STIMULUS_HARD_LINE_GUTTER, TOP_COUNTER_HEIGHT,
        border_sweep_cell_progress, command_chrome_alignment, command_chrome_lines,
        editor_cursor_at, editor_horizontal_overflow, feedback_chrome_lines, feedback_height,
        gutter_width, layout_chunks, line_text, problem_display_line, problem_lines,
        render_client_frame, render_editor_controls, render_editor_cursor,
        render_editor_horizontal_indicators, render_snapshot, render_snapshot_with_workbox_height,
        render_top_banner, reward_sweep_intensity_at, shell_layout, usize_to_u16,
        viewport_indicator_cells, visible_editor_rows, workbox_lines, workbox_lines_for_area,
        workbox_lines_for_width,
    };
    use crate::{
        client::{
            ClientAction, CursorPosition, EditorAnchor, FormatFrame, FormatNoticeKind,
            ProblemDisplayLine, ProblemViewport, REWARD_ATTACK_DURATION, REWARD_DURATION,
            REWARD_PLUS_ONE_BODY_DURATION, REWARD_PLUS_ONE_DURATION, RewardFrame, SkipFrame,
            StatusTone, StimulusRowKind, ViewportOverflow,
        },
        engine::{
            self, ExerciseAnswerRule, ExerciseKind, ExercisePacket, ExerciseValidator,
            FormatCapability,
        },
        lab,
        tui::{
            TuiApp,
            theme::{
                RUST_REPS_DARK, base_style, border_style, counter_style, editor_canvas_style,
                muted_style, plus_one_style, status_tone_style, surface_style,
            },
        },
    };
    use ratatui::{
        Terminal,
        backend::TestBackend,
        layout::{Alignment, Position, Rect},
        style::{Color, Modifier},
        text::Text,
        widgets::{Block, Paragraph},
    };

    fn test_now() -> Instant {
        Instant::now()
    }

    fn lab_app() -> TuiApp {
        TuiApp::with_session(engine::Session::lab())
    }

    fn projection_app(
        instruction: &'static str,
        stimulus: &'static str,
        expected: &'static str,
    ) -> TuiApp {
        TuiApp::with_session(engine::Session::from_exercises(vec![ExercisePacket::new(
            Cow::Borrowed("TEST-PROJECTION"),
            1,
            1,
            Cow::Borrowed(instruction),
            Cow::Borrowed(stimulus),
            Cow::Borrowed(expected),
            FormatCapability::Disabled,
            ExerciseKind::TypeExact,
            ExerciseValidator::Exact,
            ExerciseAnswerRule::ExactTrimFinalNewline,
        )]))
    }

    fn project_app_for_area(
        app: &mut TuiApp,
        area: Rect,
    ) -> (crate::client::ClientFrame, super::ShellLayout) {
        let now = test_now();
        let preliminary = app.frame_at(now);
        let feedback_height = feedback_height(&feedback_chrome_lines(&preliminary));
        let width_layout = super::shell_layout_for_content(
            area,
            feedback_height,
            preliminary.game_window.workbox.lines.len().max(1),
            super::TOP_COUNTER_HEIGHT,
        );
        let editor_width = super::editor_source_capacity(
            width_layout.editor.width,
            preliminary.game_window.workbox.lines.len().max(1),
        );
        app.set_editor_viewport_width(editor_width);
        let wrap_width =
            super::problem_wrap_width(super::problem_content_area(width_layout.problem));
        let projected = app.frame_at_for_layout(now, wrap_width);
        let layout = super::shell_layout_for_client_frame(area, feedback_height, &projected);
        app.set_editor_viewport_height(visible_editor_rows(layout.editor.height));
        (app.frame_at_for_layout(now, wrap_width), layout)
    }

    fn type_text(app: &mut TuiApp, text: &str) {
        for ch in text.chars() {
            if ch == '\n' {
                app.handle_action(ClientAction::InsertNewline);
            } else {
                app.handle_action(ClientAction::TypeChar(ch));
            }
        }
    }

    fn load_text(app: &mut TuiApp, text: &str) {
        app.replace_input_for_test(text);
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
            anchor: None,
        }
    }

    fn frame_with_long_problem(scroll_offset: usize) -> crate::client::ClientFrame {
        let mut frame = TuiApp::new().frame();
        frame.game_window.problem.lines = (1..=10)
            .map(|index| ProblemDisplayLine {
                stimulus_row: Some(StimulusRowKind::HardLineStart),
                ..ProblemDisplayLine::plain(format!("problem line {index}"))
            })
            .collect();
        frame.game_window.problem.viewport = ProblemViewport::with_offset(scroll_offset);
        frame
    }

    fn render_top_banner_line(client_frame: &crate::client::ClientFrame, width: u16) -> String {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| render_top_banner(frame, Rect::new(0, 0, width, 1), client_frame))
            .expect("expected banner render");
        let buffer = terminal.backend().buffer();
        (0..width)
            .map(|x| buffer[(x, 0)].symbol())
            .collect::<Vec<_>>()
            .concat()
    }

    fn render_top_banner_backgrounds(
        client_frame: &crate::client::ClientFrame,
        width: u16,
    ) -> Vec<Color> {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| {
                frame.render_widget(Block::default().style(surface_style()), frame.area());
                render_top_banner(frame, Rect::new(0, 0, width, 1), client_frame);
            })
            .expect("expected banner render");
        let buffer = terminal.backend().buffer();
        (0..width).map(|x| buffer[(x, 0)].bg).collect()
    }

    fn render_editor_controls_line(width: u16, format_available: bool) -> String {
        let backend = TestBackend::new(width, 1);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| {
                render_editor_controls(frame, Rect::new(0, 0, width, 1), format_available)
            })
            .expect("expected controls render");
        let buffer = terminal.backend().buffer();
        (0..width)
            .map(|x| buffer[(x, 0)].symbol())
            .collect::<Vec<_>>()
            .concat()
    }

    fn render_frame_text(
        client_frame: &crate::client::ClientFrame,
        width: u16,
        height: u16,
    ) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| render_client_frame(frame, client_frame))
            .expect("expected frame render");
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<Vec<_>>()
                    .concat()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_editor_buffer_line(client_frame: &crate::client::ClientFrame, area: Rect) -> String {
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| {
                frame.render_widget(
                    Paragraph::new(Text::from(workbox_lines_for_area(client_frame, area)))
                        .style(editor_canvas_style()),
                    area,
                );
                render_editor_horizontal_indicators(frame, area, client_frame);
                render_editor_cursor(frame, area, client_frame);
            })
            .expect("expected editor render");
        let buffer = terminal.backend().buffer();
        (0..area.width)
            .map(|x| buffer[(x, 0)].symbol())
            .collect::<Vec<_>>()
            .concat()
    }

    fn red_dominant(color: Color) -> bool {
        matches!(color, Color::Rgb(red, green, blue) if red > green && red > blue)
    }

    #[test]
    fn render_snapshot_keeps_only_problem_and_workbox_in_game_window() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let instruction_position = snapshot
            .game
            .iter()
            .position(|line| line == "Copy this line exactly:")
            .expect("expected instruction in game snapshot");
        let prompt_value_position = snapshot
            .game
            .iter()
            .position(|line| line == "  fn main() {}")
            .expect("expected prompt in game snapshot");
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == " 1 │ ")
            .expect("expected editor gutter in game snapshot");

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
            .position(|line| line == "  fn main() {}")
            .expect("expected problem prompt");
        let editor_position = snapshot
            .game
            .iter()
            .position(|line| line == " 1 │ ")
            .expect("expected editor gutter");
        let rail_position = snapshot
            .command_chrome
            .iter()
            .position(|line| line.contains(PRIMARY_ACTION_HINT))
            .expect("expected command strip primary action");

        assert!(problem_position < editor_position);
        assert_eq!(rail_position, 0);
        assert!(
            snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains(SKIP_ACTION_HINT) && line.contains(QUIT_ACTION_HINT))
        );
    }

    #[test]
    fn normal_terminal_layout_uses_top_banner_and_side_by_side_panels() {
        let layout = shell_layout(Rect::new(0, 0, 100, 32), FEEDBACK_CHROME_HEIGHT);

        assert_eq!(layout.problem.y, layout.editor.y);
        assert_eq!(layout.problem.height, layout.editor.height);
        assert!(layout.editor.width > 45);
        assert!(layout.problem.width > 30);
        assert_eq!(
            super::editor_source_capacity(layout.editor.width, 1),
            super::problem_source_capacity(layout.problem.width)
        );
        assert!(layout.editor.right() < layout.problem.left());
        assert_eq!(layout.rail.height, TOP_COUNTER_HEIGHT);
        assert!(layout.rail.bottom() <= layout.editor.top());
        assert!(layout.rail.bottom() <= layout.problem.top());
        assert_eq!(layout.controls.height, COMMAND_RAIL_HEIGHT);
        assert_eq!(layout.controls.left(), layout.editor.left());
        assert!(layout.controls.right() > layout.editor.right());
        assert!(layout.controls.right() >= layout.problem.right());
        assert_eq!(layout.controls.top(), layout.editor.bottom());
        assert_eq!(layout.problem.bottom(), layout.controls.top());
        assert!(layout.feedback.top() >= layout.problem.bottom());
    }

    #[test]
    fn normal_panel_widths_keep_equal_source_capacity_across_terminal_parities() {
        for panel_width in 42..=120 {
            for editor_line_count in [1, 9, 100] {
                let panels =
                    super::panel_layout(Rect::new(0, 0, panel_width, 8), editor_line_count);
                assert_eq!(
                    super::editor_source_capacity(panels[0].width, editor_line_count),
                    super::problem_source_capacity(panels[1].width),
                    "source capacity differs at panel width {panel_width} with {editor_line_count} editor lines"
                );
                assert!(panels[0].right() < panels[1].left());
            }
        }
    }

    #[test]
    fn narrow_panel_fallback_is_deterministic_and_non_overlapping() {
        for panel_width in 0..42 {
            let first = super::panel_layout(Rect::new(0, 0, panel_width, 8), 1);
            let second = super::panel_layout(Rect::new(0, 0, panel_width, 8), 1);

            assert_eq!(first, second);
            assert!(first[0].right() <= first[1].left());
            assert!(first[1].right() <= panel_width);
        }
    }

    #[test]
    fn top_counter_replaces_global_bottom_strip() {
        let layout = shell_layout(Rect::new(0, 0, 100, 32), FEEDBACK_CHROME_HEIGHT);

        assert_eq!(layout.rail.height, TOP_COUNTER_HEIGHT);
        assert!(layout.rail.top() < layout.editor.top());
        assert_eq!(layout.rail.bottom(), layout.editor.top());
        assert_eq!(layout.feedback.height, 0);
        assert_eq!(layout.controls.bottom(), layout.feedback.top());
        assert_eq!(layout.problem.bottom(), layout.controls.top());
        assert_eq!(layout.controls.height, COMMAND_RAIL_HEIGHT);
        assert_eq!(layout.controls.left(), layout.editor.left());
        assert!(layout.controls.right() > layout.editor.right());
        assert!(layout.controls.right() >= layout.problem.right());
    }

    #[test]
    fn side_by_side_panel_hitboxes_route_mouse_points() {
        let layout = shell_layout(Rect::new(0, 0, 100, 32), FEEDBACK_CHROME_HEIGHT);

        assert_eq!(
            layout.mouse_pane_at(layout.problem.left(), layout.problem.top()),
            crate::tui::input::MousePane::Problem
        );
        assert_eq!(
            layout.mouse_pane_at(layout.instruction.left(), layout.instruction.top()),
            crate::tui::input::MousePane::Problem
        );
        assert_eq!(
            layout.mouse_pane_at(layout.editor.left(), layout.editor.top()),
            crate::tui::input::MousePane::Editor
        );
        assert_eq!(
            layout.mouse_pane_at(layout.rail.left(), layout.rail.top()),
            crate::tui::input::MousePane::Outside
        );
    }

    #[test]
    fn editor_click_maps_visible_cells_to_buffer_cursor_positions() {
        let mut app = TuiApp::new();
        type_text(&mut app, "abcd\nef");
        let frame = app.frame();
        let area = Rect::new(10, 4, 40, 8);
        let content_left = area.left() + usize_to_u16(gutter_width(2)) + 3;
        let first_row = area.top();

        assert_eq!(
            editor_cursor_at(area, &frame, content_left, first_row),
            Some(CursorPosition { line: 0, column: 0 })
        );
        assert_eq!(
            editor_cursor_at(area, &frame, content_left + 2, first_row),
            Some(CursorPosition { line: 0, column: 2 })
        );
        assert_eq!(
            editor_cursor_at(area, &frame, content_left + 10, first_row),
            Some(CursorPosition { line: 0, column: 4 })
        );
    }

    #[test]
    fn editor_click_below_real_lines_clamps_to_last_editable_line() {
        let mut app = TuiApp::new();
        type_text(&mut app, "abcd\nef");
        let frame = app.frame();
        let area = Rect::new(10, 4, 40, 8);
        let content_left = area.left() + usize_to_u16(gutter_width(2)) + 3;

        assert_eq!(
            editor_cursor_at(area, &frame, content_left + 1, area.top() + 6),
            Some(CursorPosition { line: 1, column: 1 })
        );
    }

    #[test]
    fn editor_click_accounts_for_editor_viewport_offset() {
        let mut app = TuiApp::new();
        type_text(&mut app, "one\ntwo\nthree\nfour");
        app.set_editor_viewport_height(2);
        app.handle_action(ClientAction::ScrollEditorDown);
        app.handle_action(ClientAction::ScrollEditorDown);
        let frame = app.frame();
        let area = Rect::new(10, 4, 40, 4);
        let content_left = area.left() + usize_to_u16(gutter_width(4)) + 3;

        assert_eq!(frame.game_window.workbox.viewport.first_line, 2);
        assert_eq!(
            editor_cursor_at(area, &frame, content_left, area.top()),
            Some(CursorPosition { line: 2, column: 0 })
        );
    }

    #[test]
    fn small_terminal_layout_keeps_rail_reserved_and_panels_non_overlapping() {
        let layout = shell_layout(Rect::new(0, 0, 34, 12), FEEDBACK_CHROME_HEIGHT);

        assert_eq!(layout.rail.height, TOP_COUNTER_HEIGHT);
        assert!(layout.controls.height <= COMMAND_RAIL_HEIGHT);
        assert!(layout.problem.width > 0);
        assert!(layout.editor.width > 0);
        assert!(layout.editor.right() <= layout.problem.left());
        assert!(layout.rail.bottom() <= 12);
    }

    #[test]
    fn editor_surface_snapshot_includes_gutter_spacing_and_reserved_space() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        let input_position = snapshot
            .game
            .iter()
            .position(|line| line == " 1 │ ")
            .expect("expected editor input line");

        assert!(input_position > 0);
        assert_eq!(line_text(&workbox_lines(&frame, 1)[0]), " 1 │ ");
        assert!(!snapshot.game.iter().any(|line| line == " 2 │ "));
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

        assert_eq!(rendered[0], " 1 │ ");
        assert_eq!(rendered[1], "");
        assert_eq!(rendered[2], "");
        assert_eq!(rendered[3], "");
        assert!(!rendered.iter().any(|line| line == " 2 │ "));
        assert!(!rendered.iter().any(|line| line.contains("fn main")));
    }

    #[test]
    fn active_line_style_is_mapped_through_editor_theme() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = workbox_lines(&frame, SNAPSHOT_WORKBOX_HEIGHT);
        let active_line = &lines[0];
        let blank_line = &lines[1];

        assert_eq!(
            active_line.style.bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(
            active_line.spans[0].style.bg,
            Some(RUST_REPS_DARK.editor_active_line)
        );
        assert_eq!(blank_line.style.bg, Some(RUST_REPS_DARK.surface));
        assert!(blank_line.spans.is_empty());
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
    fn render_snapshot_projects_instruction_before_simple_stimulus() {
        let app = TuiApp::new();
        let frame = app.frame();
        let problem = problem_lines(&frame)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();
        let editor = workbox_lines(&frame, SNAPSHOT_WORKBOX_HEIGHT)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();

        let instruction_position = problem
            .iter()
            .position(|line| line == "Copy this line exactly:")
            .expect("expected instruction line");
        let stimulus_position = problem
            .iter()
            .position(|line| line == "  fn main() {}")
            .expect("expected stimulus line");
        let editor_line_position = editor
            .iter()
            .position(|line| line == " 1 │ ")
            .expect("expected editor line 1");

        assert_eq!(instruction_position, 0);
        assert_eq!(editor_line_position, 0);
        assert!(instruction_position < stimulus_position);
    }

    #[test]
    fn screenshot_width_gives_editor_and_stimulus_equal_source_columns() {
        let source = "let scores = [1, 2, 3];";
        let area = Rect::new(0, 0, 70, 18);
        let mut app = projection_app("Copy this line exactly:", source, source);
        load_text(&mut app, source);

        let (frame, layout) = project_app_for_area(&mut app, area);
        let editor_width = super::editor_source_capacity(
            layout.editor.width,
            frame.game_window.workbox.lines.len(),
        );
        let stimulus_width = super::problem_source_capacity(layout.problem.width);
        let source_rows = frame
            .game_window
            .problem
            .lines
            .iter()
            .filter(|line| line.stimulus_row.is_some())
            .collect::<Vec<_>>();

        assert_eq!(editor_width, stimulus_width);
        assert_eq!(editor_width, 26);
        assert!(source.chars().count() <= editor_width);
        assert_eq!(frame.game_window.workbox.lines, vec![source.to_string()]);
        assert_eq!(frame.game_window.workbox.viewport.first_column, 0);
        assert_eq!(source_rows.len(), 1);
        assert_eq!(source_rows[0].text, source);
        assert_eq!(
            source_rows[0].stimulus_row,
            Some(StimulusRowKind::HardLineStart)
        );
    }

    #[test]
    fn narrower_equal_source_columns_keep_soft_continuation_unambiguous() {
        let source = "fn add_one(x: i32) -> i32 { x + 1 }";
        let area = Rect::new(0, 0, 52, 18);
        let mut app = projection_app("Copy this line exactly:", source, source);
        load_text(&mut app, source);

        let (frame, layout) = project_app_for_area(&mut app, area);
        let editor_width = super::editor_source_capacity(
            layout.editor.width,
            frame.game_window.workbox.lines.len(),
        );
        let stimulus_width = super::problem_source_capacity(layout.problem.width);
        let source_rows = frame
            .game_window
            .problem
            .lines
            .iter()
            .filter(|line| line.stimulus_row.is_some())
            .collect::<Vec<_>>();

        assert_eq!(editor_width, stimulus_width);
        assert!(source_rows.len() > 1);
        assert_eq!(
            source_rows[0].stimulus_row,
            Some(StimulusRowKind::HardLineStart)
        );
        assert!(source_rows[1..].iter().all(|line| {
            line.stimulus_row == Some(StimulusRowKind::SoftContinuation)
                && !line.text.contains(STIMULUS_CONTINUATION_GUTTER)
        }));
        assert_eq!(
            source_rows
                .iter()
                .map(|line| line.text.as_str())
                .collect::<String>(),
            source
        );
        assert_eq!(
            line_text(&problem_display_line(source_rows[1])),
            format!("{STIMULUS_CONTINUATION_GUTTER}{}", source_rows[1].text)
        );
    }

    #[test]
    fn shared_header_band_aligns_editor_and_first_stimulus_source_rows() {
        let source = "fn main() {}";
        let area = Rect::new(0, 0, 70, 18);
        let mut app = projection_app("Copy this line exactly:", source, source);
        load_text(&mut app, source);

        let (frame, layout) = project_app_for_area(&mut app, area);
        assert_eq!(layout.editor.y, layout.problem.y);
        assert_eq!(layout.rail.y, layout.instruction.y);
        assert_eq!(layout.rail.bottom(), layout.editor.y);
        assert_eq!(layout.instruction.bottom(), layout.problem.y);

        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|draw_frame| render_client_frame(draw_frame, &frame))
            .expect("expected frame render");
        let buffer = terminal.backend().buffer();
        let editor_source_x = layout.editor.left()
            + usize_to_u16(gutter_width(frame.game_window.workbox.lines.len()))
            + usize_to_u16(super::EDITOR_SOURCE_SEPARATOR_WIDTH);
        let stimulus_source_x = super::problem_content_area(layout.problem).left()
            + usize_to_u16(crate::client::STIMULUS_GUTTER_WIDTH);

        assert_eq!(buffer[(editor_source_x, layout.editor.y)].symbol(), "f");
        assert_eq!(buffer[(stimulus_source_x, layout.problem.y)].symbol(), "f");
        assert_eq!(layout.editor.y, layout.problem.y);
    }

    #[test]
    fn wrapped_instruction_moves_both_source_origins_without_changing_width() {
        let instruction = "Write a Rust function named five with no parameters that returns the i32 value 5 using a tail expression:";
        let source = "fn five() -> i32 { 5 }";
        let area = Rect::new(0, 0, 52, 18);
        let mut long = projection_app(instruction, source, source);
        load_text(&mut long, source);
        let (long_frame, long_layout) = project_app_for_area(&mut long, area);
        let mut short = projection_app("Copy this line exactly:", source, source);
        load_text(&mut short, source);
        let (short_frame, short_layout) = project_app_for_area(&mut short, area);

        assert!(long_frame.game_window.problem.instruction_lines().len() > 1);
        assert!(long_layout.editor.y > short_layout.editor.y);
        assert_eq!(long_layout.editor.y, long_layout.problem.y);
        assert_eq!(long_layout.rail.height, long_layout.instruction.height);
        assert_eq!(
            super::editor_source_capacity(
                long_layout.editor.width,
                long_frame.game_window.workbox.lines.len()
            ),
            super::problem_source_capacity(long_layout.problem.width)
        );
        assert_eq!(
            super::editor_source_capacity(
                long_layout.editor.width,
                long_frame.game_window.workbox.lines.len()
            ),
            super::editor_source_capacity(
                short_layout.editor.width,
                short_frame.game_window.workbox.lines.len()
            )
        );
        assert_eq!(long_frame.game_window.problem.stimulus, source);
    }

    #[test]
    fn five_operation_task_panels_never_render_non_copy_expected_answers() {
        let fixtures = [
            (
                "Copy this line exactly:",
                "fn five() -> i32 { 5 }",
                "fn five() -> i32 { 5 }",
            ),
            (
                "Complete the missing return value:",
                "fn five() -> i32 { ___ }",
                "fn five() -> i32 { 5 }",
            ),
            (
                "Write a Rust function named five with no parameters that returns the i32 value 5 using a tail expression:",
                "",
                "fn five() -> i32 { 5 }",
            ),
            (
                "Repair the return-type syntax:",
                "fn five(): i32 { 5 }",
                "fn five() -> i32 { 5 }",
            ),
            (
                "Rewrite the function to use a tail expression:",
                "fn five() -> i32 { return 5; }",
                "fn five() -> i32 { 5 }",
            ),
        ];

        for (index, (instruction, stimulus, expected)) in fixtures.iter().enumerate() {
            let frame = projection_app(instruction, stimulus, expected)
                .frame_at_for_layout(test_now(), 160);
            let lines = problem_lines(&frame)
                .iter()
                .map(line_text)
                .collect::<Vec<_>>();
            let rendered = lines.join("\n");
            let instruction_position = lines
                .iter()
                .position(|line| line == instruction)
                .expect("task panel must contain exact instruction");

            if stimulus.is_empty() {
                assert_eq!(lines.last(), Some(&instruction.to_string()));
                assert!(!rendered.contains(STIMULUS_CONTINUATION_GUTTER));
            } else {
                let rendered_stimulus = format!("{STIMULUS_HARD_LINE_GUTTER}{stimulus}");
                let stimulus_position = lines
                    .iter()
                    .position(|line| line == &rendered_stimulus)
                    .expect("task panel must contain exact stimulus");
                assert!(instruction_position < stimulus_position);
            }
            if index > 0 {
                assert!(!rendered.contains(expected));
            }
        }
    }

    #[test]
    fn viewport_indicator_appears_only_when_pane_content_overflows() {
        let short_snapshot = render_snapshot(&TuiApp::new().frame());
        assert!(
            !short_snapshot
                .game
                .iter()
                .any(|line| line.contains("PgUp/PgDn"))
        );

        assert!(
            viewport_indicator_cells(
                8,
                ViewportOverflow {
                    above: false,
                    below: false,
                    offset: 0,
                    visible: 8,
                    total: 8,
                }
            )
            .is_empty()
        );
        assert_eq!(
            viewport_indicator_cells(
                8,
                ViewportOverflow {
                    above: false,
                    below: true,
                    offset: 0,
                    visible: 8,
                    total: 12,
                }
            ),
            vec![(0, "┃"), (7, "↓")]
        );
    }

    #[test]
    fn viewport_indicator_marks_middle_and_bottom_scroll_positions() {
        assert_eq!(
            viewport_indicator_cells(
                8,
                ViewportOverflow {
                    above: true,
                    below: true,
                    offset: 4,
                    visible: 3,
                    total: 10,
                }
            ),
            vec![(0, "↑"), (3, "┃"), (7, "↓")]
        );
        assert_eq!(
            viewport_indicator_cells(
                8,
                ViewportOverflow {
                    above: true,
                    below: false,
                    offset: 7,
                    visible: 3,
                    total: 10,
                }
            ),
            vec![(0, "↑"), (7, "┃")]
        );
    }

    #[test]
    fn editor_overflow_states_map_to_pane_local_indicators() {
        let mut app = TuiApp::new();
        type_text(&mut app, "one\ntwo\nthree");
        let fitting = app.frame().game_window.workbox.overflow(3);
        assert!(!fitting.has_overflow());
        assert!(viewport_indicator_cells(4, fitting).is_empty());

        type_text(&mut app, "\nfour\nfive\nsix");
        app.set_editor_viewport_height(3);
        while app.frame().game_window.workbox.cursor.line > 0 {
            app.handle_action(ClientAction::MoveCursorUp);
        }
        let top = app.frame().game_window.workbox.overflow(3);
        assert_eq!(top.above, false);
        assert_eq!(top.below, true);
        assert!(
            viewport_indicator_cells(4, top)
                .iter()
                .any(|(_, symbol)| *symbol == "↓")
        );

        app.handle_action(ClientAction::MoveCursorDown);
        app.handle_action(ClientAction::MoveCursorDown);
        app.handle_action(ClientAction::MoveCursorDown);
        let middle = app.frame().game_window.workbox.overflow(3);
        assert!(middle.above);
        assert!(middle.below);
        let middle_symbols = viewport_indicator_cells(4, middle)
            .into_iter()
            .map(|(_, symbol)| symbol)
            .collect::<Vec<_>>();
        assert!(middle_symbols.contains(&"↑"));
        assert!(middle_symbols.contains(&"↓"));

        while app.frame().game_window.workbox.cursor.line
            < app
                .frame()
                .game_window
                .workbox
                .lines
                .len()
                .saturating_sub(1)
        {
            app.handle_action(ClientAction::MoveCursorDown);
        }
        let bottom = app.frame().game_window.workbox.overflow(3);
        assert!(bottom.above);
        assert!(!bottom.below);
        let bottom_symbols = viewport_indicator_cells(4, bottom)
            .into_iter()
            .map(|(_, symbol)| symbol)
            .collect::<Vec<_>>();
        assert!(bottom_symbols.contains(&"↑"));
        assert!(!bottom_symbols.contains(&"↓"));
    }

    #[test]
    fn render_snapshot_shows_long_problem_visible_subset() {
        let snapshot = render_snapshot(&frame_with_long_problem(4));

        assert!(snapshot.game.iter().any(|line| line == "  problem line 5"));
        assert!(snapshot.game.iter().any(|line| line == "  problem line 10"));
        assert!(!snapshot.game.iter().any(|line| line == "  problem line 1"));
        assert!(!snapshot.game.iter().any(|line| line.contains("PgUp/PgDn")));
    }

    #[test]
    fn problem_overflow_states_map_to_pane_local_indicators() {
        let top_frame = frame_with_long_problem(0);
        let middle_frame = frame_with_long_problem(4);
        let bottom_frame = frame_with_long_problem(99);

        let visible_rows = 5;
        let top = top_frame.game_window.problem.overflow(visible_rows);
        let middle = middle_frame.game_window.problem.overflow(visible_rows);
        let bottom = bottom_frame.game_window.problem.overflow(visible_rows);

        assert_eq!((top.above, top.below), (false, true));
        assert_eq!((middle.above, middle.below), (true, true));
        assert_eq!((bottom.above, bottom.below), (true, false));
        assert!(
            viewport_indicator_cells(visible_rows as u16, top)
                .iter()
                .any(|(_, symbol)| *symbol == "↓")
        );
        assert!(
            viewport_indicator_cells(visible_rows as u16, middle)
                .iter()
                .any(|(_, symbol)| *symbol == "↑")
        );
        assert!(
            viewport_indicator_cells(visible_rows as u16, middle)
                .iter()
                .any(|(_, symbol)| *symbol == "↓")
        );
        assert!(
            !viewport_indicator_cells(visible_rows as u16, bottom)
                .iter()
                .any(|(_, symbol)| *symbol == "↓")
        );
    }

    #[test]
    fn long_problem_rendering_is_clipped_to_reserved_pane() {
        let session = engine::Session::lab();
        let ui_state = crate::client::ClientUiState::new();
        let frame =
            crate::client::project_client_frame_for_layout(&session, &ui_state, test_now(), 18);
        let lines = problem_lines(&frame)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();

        assert_eq!(lines.len(), usize::from(SNAPSHOT_PROBLEM_HEIGHT));
        assert!(
            lines
                .iter()
                .filter(|line| !line.is_empty())
                .all(|line| line.chars().count() <= 18)
        );
    }

    #[test]
    fn problem_scroll_keeps_editor_and_rail_layout_stable() {
        let top_frame = frame_with_long_problem(0);
        let scrolled_frame = frame_with_long_problem(4);

        assert_eq!(
            workbox_lines(&top_frame, SNAPSHOT_WORKBOX_HEIGHT)
                .iter()
                .map(line_text)
                .collect::<Vec<_>>(),
            workbox_lines(&scrolled_frame, SNAPSHOT_WORKBOX_HEIGHT)
                .iter()
                .map(line_text)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            command_chrome_lines(&top_frame)
                .iter()
                .map(line_text)
                .collect::<Vec<_>>(),
            command_chrome_lines(&scrolled_frame)
                .iter()
                .map(line_text)
                .collect::<Vec<_>>()
        );

        let top_snapshot = render_snapshot(&top_frame);
        let scrolled_snapshot = render_snapshot(&scrolled_frame);
        assert_eq!(top_snapshot.game.len(), scrolled_snapshot.game.len());
        assert_eq!(
            top_snapshot.command_chrome,
            scrolled_snapshot.command_chrome
        );
    }

    #[test]
    fn lab_long_problem_render_snapshot_shows_visible_subset_and_stable_editor_rail() {
        let mut app = lab_app();
        let top_snapshot = render_snapshot(&app.frame());

        app.handle_action(ClientAction::ScrollProblemDown);
        let scrolled_snapshot = render_snapshot(&app.frame());

        assert!(top_snapshot.game.iter().any(|line| line == "  task:"));
        assert!(
            top_snapshot
                .game
                .iter()
                .any(|line| line.starts_with("  Build the smallest Rust entry point"))
        );
        assert!(
            top_snapshot
                .game
                .iter()
                .all(|line| !line.contains("PgUp/PgDn"))
        );
        assert!(
            scrolled_snapshot
                .game
                .iter()
                .any(|line| line == "  requirements:")
        );
        assert!(!scrolled_snapshot.game.iter().any(|line| line == "  task:"));
        assert_eq!(top_snapshot.game.len(), scrolled_snapshot.game.len());
        assert_eq!(
            top_snapshot.command_chrome,
            scrolled_snapshot.command_chrome
        );
        assert!(scrolled_snapshot.game.iter().any(|line| line == " 1 │ "));
    }

    #[test]
    fn lab_multiline_editor_render_snapshot_shows_multiple_lines_and_gutter() {
        let mut app = TuiApp::with_session(engine::Session::lab_at(
            lab::MULTILINE_SOLUTION_FAMILY_INDEX,
        ));
        load_text(&mut app, lab::MULTILINE_SOLUTION_EXPECTED);

        let snapshot = render_snapshot_with_workbox_height(&app.frame(), 8);

        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == " 1 │ fn add_one(x: i32) -> i32 {")
        );
        assert!(snapshot.game.iter().any(|line| line == " 2 │     x + 1"));
        assert!(snapshot.game.iter().any(|line| line == " 3 │ }"));
        assert!(
            snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains(PRIMARY_ACTION_HINT))
        );
        assert_eq!(snapshot.feedback, Vec::<String>::new());
    }

    #[test]
    fn idle_command_rail_renders_only_controls() {
        let app = TuiApp::new();
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);
        assert_eq!(
            snapshot.command_chrome,
            vec![format!(
                "{FORMAT_ACTION_HINT} · {PRIMARY_ACTION_HINT} · {SKIP_ACTION_HINT} · {QUIT_ACTION_HINT}"
            )]
        );
        assert_eq!(command_chrome_alignment(), Alignment::Left);
        assert!(snapshot.command_chrome[0].starts_with(FORMAT_ACTION_HINT));
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("reps"))
        );
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains('✓'))
        );
        assert!(!snapshot.game.iter().any(|line| line.contains("Passed")));
        assert!(!snapshot.feedback.iter().any(|line| line.contains("Passed")));
    }

    #[test]
    fn top_banner_keeps_only_counter_at_editor_left() {
        let app = TuiApp::new();
        let line = render_top_banner_line(&app.frame(), 72);

        assert!(line.trim_start().starts_with("● 0"));
        assert_eq!(line.trim(), "● 0");
        assert!(!line.contains(FORMAT_ACTION_HINT));
        assert!(!line.contains(PRIMARY_ACTION_HINT));
        assert!(!line.contains(SKIP_ACTION_HINT));
        assert!(!line.contains(QUIT_ACTION_HINT));
        assert!(!line.contains("F6 skip"));
        assert!(!line.contains("wheel"));
        assert!(!line.contains("PageUp"));
        assert!(!line.contains("PageDown"));
    }

    #[test]
    fn counter_banner_background_matches_shell_surface() {
        let app = TuiApp::new();
        let backgrounds = render_top_banner_backgrounds(&app.frame(), 24);

        assert_eq!(counter_style().bg, Some(RUST_REPS_DARK.surface));
        assert!(
            backgrounds
                .iter()
                .all(|background| *background == RUST_REPS_DARK.surface)
        );
    }

    #[test]
    fn counter_plus_one_background_matches_shell_surface() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        let line = render_top_banner_line(&frame, 24);
        let backgrounds = render_top_banner_backgrounds(&frame, 24);
        let plus_one_start = line.find("+1").expect("expected counter-local +1");

        assert_eq!(
            plus_one_style(frame.reward).bg,
            Some(RUST_REPS_DARK.surface)
        );
        assert_eq!(backgrounds[plus_one_start], RUST_REPS_DARK.surface);
        assert_eq!(backgrounds[plus_one_start + 1], RUST_REPS_DARK.surface);
        assert!(line.trim_start().starts_with("● 1 +1"));
    }

    #[test]
    fn counter_style_is_stronger_and_outer_border_uses_heavy_glyphs() {
        let app = TuiApp::new();
        let rendered = render_frame_text(&app.frame(), 80, 20);

        assert_eq!(counter_style().fg, Some(RUST_REPS_DARK.reward_plus_one));
        assert!(rendered.contains('┏'));
        assert!(rendered.contains('┓'));
        assert!(rendered.contains('┗'));
        assert!(rendered.contains('┛'));
        assert!(rendered.contains('━'));
        assert!(rendered.contains('┃'));
    }

    #[test]
    fn editor_local_controls_render_under_editor_in_order() {
        let controls = render_editor_controls_line(80, false);
        let f1 = controls
            .find(FORMAT_ACTION_HINT)
            .expect("missing F1 format");
        let f5 = controls
            .find(PRIMARY_ACTION_HINT)
            .expect("missing F5 submit");
        let f7 = controls.find(SKIP_ACTION_HINT).expect("missing F7 skip");
        let esc = controls.find(QUIT_ACTION_HINT).expect("missing Esc quit");

        assert!(f1 < f5);
        assert!(f5 < f7);
        assert!(f7 < esc);
        assert_eq!(controls.trim_end().rfind(QUIT_ACTION_HINT), Some(esc));
        assert!(!controls.contains("F6 skip"));
        assert!(!controls.contains("wheel"));
        assert!(!controls.contains("PageUp"));
        assert!(!controls.contains("PageDown"));
    }

    #[test]
    fn command_row_uses_full_inner_width_to_keep_esc_visible() {
        let layout = shell_layout(Rect::new(0, 0, 72, 18), FEEDBACK_CHROME_HEIGHT);
        let controls = render_editor_controls_line(layout.controls.width, false);

        assert!(layout.controls.width > layout.editor.width);
        assert!(controls.contains(FORMAT_ACTION_HINT));
        assert!(controls.contains(PRIMARY_ACTION_HINT));
        assert!(controls.contains(SKIP_ACTION_HINT));
        assert!(controls.contains(QUIT_ACTION_HINT));
        assert_eq!(
            controls.trim_end().rfind(QUIT_ACTION_HINT),
            controls.find(QUIT_ACTION_HINT)
        );
        assert!(!controls.trim_end().ends_with('·'));
    }

    #[test]
    fn format_feedback_renders_locally_without_result_block() {
        let mut frame = TuiApp::new().frame();
        frame.format = FormatFrame {
            active: true,
            intensity: 100,
            kind: Some(FormatNoticeKind::Formatted),
            anchor: Some(EditorAnchor { row: 0, column: 4 }),
        };
        frame.format_active = true;

        let rendered = render_frame_text(&frame, 96, 24);
        let snapshot = render_snapshot(&frame);

        assert!(rendered.contains("formatted"));
        assert!(snapshot.feedback.is_empty());
        assert!(snapshot.command_chrome[0].contains(FORMAT_ACTION_HINT));
        assert!(!snapshot.command_chrome[0].contains("formatted"));
        assert!(!snapshot.command_chrome[0].contains("format failed"));
    }

    #[test]
    fn disabled_exact_task_format_feedback_renders_exactly_and_locally() {
        let mut frame = TuiApp::new().frame();
        frame.format = FormatFrame {
            active: true,
            intensity: 100,
            kind: Some(FormatNoticeKind::DisabledExactTask),
            anchor: Some(EditorAnchor { row: 0, column: 0 }),
        };
        frame.format_active = true;

        let rendered = render_frame_text(&frame, 70, 18);
        let snapshot = render_snapshot(&frame);

        assert!(rendered.contains("format off for exact task"));
        assert!(snapshot.feedback.is_empty());
        assert!(!snapshot.command_chrome[0].contains("format off"));
    }

    #[test]
    fn local_toasts_keep_one_cell_gap_from_editor_text() {
        let now = test_now();
        let mut wrong_app = TuiApp::new();
        type_text(&mut wrong_app, "wrong");
        wrong_app.handle_action_at(ClientAction::Submit, now);
        let wrong_frame = wrong_app.frame_at(now);
        let wrong_rendered = render_frame_text(&wrong_frame, 96, 24);
        assert!(wrong_rendered.contains("wrong not quite"));
        assert!(!wrong_rendered.contains("wrongnot quite"));
        assert_eq!(wrong_frame.game_window.workbox.input, "wrong");

        let mut skip_frame = TuiApp::new().frame();
        skip_frame.skip = SkipFrame {
            active: true,
            intensity: 100,
            anchor: Some(EditorAnchor { row: 0, column: 0 }),
        };
        let skip_rendered = render_frame_text(&skip_frame, 96, 24);
        let skip_line = skip_rendered
            .lines()
            .find(|line| line.contains("skipped"))
            .expect("expected skipped toast");
        assert!(skip_line.contains("│  skipped"));

        let mut format_app = TuiApp::new();
        type_text(&mut format_app, "abc");
        let mut format_frame = format_app.frame();
        format_frame.format = FormatFrame {
            active: true,
            intensity: 100,
            kind: Some(FormatNoticeKind::Formatted),
            anchor: Some(EditorAnchor { row: 0, column: 3 }),
        };
        format_frame.format_active = true;
        let format_rendered = render_frame_text(&format_frame, 96, 24);
        assert!(format_rendered.contains("abc formatted"));
        assert!(!format_rendered.contains("abcformatted"));

        format_frame.format.kind = Some(FormatNoticeKind::Unavailable);
        let unavailable_rendered = render_frame_text(&format_frame, 96, 24);
        assert!(unavailable_rendered.contains("abc rustfmt unavailable"));
        assert!(!unavailable_rendered.contains("abcrustfmt unavailable"));
        assert_eq!(format_frame.game_window.workbox.input, "abc");
    }

    #[test]
    fn command_chrome_shows_minimal_control_hints_without_dashboard_stats() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app.frame());
        let rail = snapshot.command_chrome.join("\n");

        for expected in ["F1 format", "F5 submit", "F7 skip", "Esc quit"] {
            assert!(rail.contains(expected), "missing {expected}");
        }
        let f1 = rail.find("F1 format").expect("missing F1 format");
        let f5 = rail.find("F5 submit").expect("missing F5 submit");
        let skip = rail.find("F7 skip").expect("missing F7 skip");
        let esc = rail.find("Esc quit").expect("missing Esc quit");
        assert!(f1 < f5);
        assert!(f5 < skip);
        assert!(skip < esc);
        assert_eq!(rail.trim_end().rfind("Esc quit"), Some(esc));
        assert!(!rail.contains("F6 skip"));
        assert!(!rail.contains("Enter newline"));
        assert!(!rail.contains("? help"));
        assert!(!rail.contains("wheel scroll"));
        assert!(!rail.contains("Pg scroll"));
        assert!(!rail.contains("reps"));
        assert!(!rail.contains("PageUp"));
        assert!(!rail.contains("PageDown"));

        for line in &snapshot.command_chrome {
            for forbidden in ["Passed", "Skipped", "Attempts", "READY", "%", "/12", "reps"] {
                assert!(
                    !line.contains(forbidden),
                    "command row should not contain {forbidden}"
                );
            }
        }
        assert!(snapshot.command_chrome.iter().any(|line| line
            == &format!("{FORMAT_ACTION_HINT} · {PRIMARY_ACTION_HINT} · {SKIP_ACTION_HINT} · {QUIT_ACTION_HINT}")));
    }

    #[test]
    fn command_rail_groups_primary_skip_and_quit_controls() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = command_chrome_lines(&frame);
        let reward_line = lines.first().expect("expected command row line");

        assert_eq!(lines.len(), usize::from(COMMAND_RAIL_HEIGHT));
        assert_eq!(reward_line.spans[0].content.as_ref(), FORMAT_ACTION_HINT);
        assert_eq!(reward_line.spans[1].content.as_ref(), " · ");
        assert_eq!(reward_line.spans[2].content.as_ref(), PRIMARY_ACTION_HINT);
        assert!(
            reward_line.spans[2]
                .style
                .add_modifier
                .contains(Modifier::BOLD)
        );
        assert_eq!(reward_line.spans[1].content.as_ref(), " · ");
        assert_eq!(reward_line.spans[3].content.as_ref(), " · ");
        assert_eq!(reward_line.spans[4].content.as_ref(), SKIP_ACTION_HINT);
        assert_eq!(reward_line.spans[5].content.as_ref(), " · ");
        assert_eq!(reward_line.spans[6].content.as_ref(), QUIT_ACTION_HINT);
        assert_eq!(
            reward_line.spans[6].style.fg,
            Some(RUST_REPS_DARK.text_muted)
        );
    }

    #[test]
    fn format_control_is_dimmed_only_when_task_formatting_is_disabled() {
        let disabled = command_chrome_lines(&TuiApp::new().frame())
            .into_iter()
            .next()
            .expect("expected disabled command row");
        let enabled = command_chrome_lines(&lab_app().frame())
            .into_iter()
            .next()
            .expect("expected enabled command row");

        assert_eq!(line_text(&disabled), line_text(&enabled));
        assert_eq!(
            line_text(&disabled),
            format!(
                "{FORMAT_ACTION_HINT} · {PRIMARY_ACTION_HINT} · {SKIP_ACTION_HINT} · {QUIT_ACTION_HINT}"
            )
        );
        assert!(disabled.spans[0].style.add_modifier.contains(Modifier::DIM));
        assert!(!enabled.spans[0].style.add_modifier.contains(Modifier::DIM));
        assert_eq!(&disabled.spans[1..], &enabled.spans[1..]);
    }

    #[test]
    fn success_reward_renders_plus_one_adjacent_to_counter_not_editor() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        let banner = render_top_banner_line(&frame, 40);
        let rendered = render_frame_text(&frame, 96, 24);

        assert_eq!(frame.reward.anchor, None);
        assert!(frame.reward.show_plus_one);
        assert_eq!(frame.reward.plus_one_intensity, 100);
        assert!(banner.contains("● 1 +1"));
        assert!(banner.find("+1").expect("expected +1") > banner.find("● 1").unwrap());
        assert!(!rendered.contains(" 1 │ +1"));
        assert!(!rendered.contains("fn main() {}+1"));
    }

    #[test]
    fn reward_plus_one_uses_counter_row_and_not_bottom_controls() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        let controls = line_text(&command_chrome_lines(&frame)[0]);
        let banner = render_top_banner_line(&frame, 40);

        assert!(banner.trim_start().starts_with("● 1 +1"));
        assert!(!controls.contains("+1"));
        assert!(controls.contains(PRIMARY_ACTION_HINT));
    }

    #[test]
    fn reward_plus_one_expires_from_counter_row() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        let active = render_top_banner_line(&app.frame_at(now + REWARD_ATTACK_DURATION), 40);
        let expired = render_top_banner_line(&app.frame_at(now + REWARD_PLUS_ONE_DURATION), 40);

        assert!(active.contains("● 1 +1"));
        assert_eq!(expired.trim(), "● 1");
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
        let lines = command_chrome_lines(&frame);
        let line = lines.first().expect("expected command row line");
        assert_eq!(frame.reward.anchor, None);
        assert_eq!(frame.reward.plus_one_intensity, 0);
        assert!(!line_text(line).contains("+1"));

        let frame = app.frame_at(now + REWARD_ATTACK_DURATION);
        assert_eq!(frame.reward.plus_one_intensity, 100);
        assert!(frame.reward.show_plus_one);
        assert!(render_top_banner_line(&frame, 40).contains("● 1 +1"));

        let frame =
            app.frame_at(now + REWARD_ATTACK_DURATION + (REWARD_PLUS_ONE_BODY_DURATION / 2));
        assert_eq!(frame.reward.plus_one_intensity, 100);
        assert!(frame.reward.show_plus_one);

        let frame = app.frame_at(now + REWARD_PLUS_ONE_DURATION - Duration::from_millis(250));
        assert!(frame.reward.plus_one_intensity < 50);
        assert!(frame.reward.show_plus_one);

        let frame = app.frame_at(now + REWARD_PLUS_ONE_DURATION);
        assert!(frame.reward.active);
        assert_eq!(frame.reward.plus_one_intensity, 0);
        assert!(!frame.reward.show_plus_one);
        assert_eq!(render_top_banner_line(&frame, 40).trim(), "● 1");

        let frame = app.frame_at(now + REWARD_DURATION);
        let lines = command_chrome_lines(&frame);
        let line = lines.first().expect("expected command row line");
        assert!(!frame.reward.show_plus_one);
        assert!(!line_text(line).contains("+1"));
        assert!(line_text(line).contains(PRIMARY_ACTION_HINT));
    }

    #[test]
    fn plus_one_expires_after_reward_duration() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_PLUS_ONE_DURATION);
        let lines = command_chrome_lines(&frame);
        let line = lines.first().expect("expected command row line");

        assert!(frame.reward.active);
        assert!(!frame.reward.show_plus_one);
        assert_eq!(render_top_banner_line(&frame, 40).trim(), "● 1");
        assert!(!line_text(line).contains("+1"));
        assert!(line_text(line).contains(PRIMARY_ACTION_HINT));
        assert!(!line_text(line).contains("reps"));
    }

    #[test]
    fn primary_action_position_stays_stable_across_result_states() {
        let mut success_app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            success_app.handle_action(ClientAction::TypeChar(ch));
        }
        success_app.handle_action_at(ClientAction::Submit, now);

        let mut wrong_app = TuiApp::new();
        for ch in "wrong".chars() {
            wrong_app.handle_action(ClientAction::TypeChar(ch));
        }
        wrong_app.handle_action(ClientAction::Submit);

        let mut skip_app = TuiApp::new();
        skip_app.handle_action(ClientAction::Skip);

        let idle_line = line_text(&command_chrome_lines(&TuiApp::new().frame())[0]);
        let success_line = line_text(
            &command_chrome_lines(&success_app.frame_at(now + REWARD_ATTACK_DURATION))[0],
        );
        let wrong_line = line_text(&command_chrome_lines(&wrong_app.frame())[0]);
        let skip_line = line_text(&command_chrome_lines(&skip_app.frame())[0]);

        let primary_position = idle_line
            .find(PRIMARY_ACTION_HINT)
            .expect("expected F5 submit");
        assert_eq!(
            success_line.find(PRIMARY_ACTION_HINT),
            Some(primary_position)
        );
        assert_eq!(wrong_line.find(PRIMARY_ACTION_HINT), Some(primary_position));
        assert_eq!(skip_line.find(PRIMARY_ACTION_HINT), Some(primary_position));
        assert!(!idle_line.contains("+1"));
        assert!(!success_line.contains("+1"));
        assert!(!wrong_line.contains("+1"));
        assert!(!skip_line.contains("+1"));
        assert!(
            render_top_banner_line(&success_app.frame_at(now + REWARD_ATTACK_DURATION), 40)
                .contains("+1")
        );
        assert!(!render_top_banner_line(&wrong_app.frame(), 40).contains("+1"));
        assert!(!render_top_banner_line(&skip_app.frame(), 40).contains("+1"));
        assert!(!idle_line.contains("reps"));
        assert!(!success_line.contains("reps"));
        assert!(!wrong_line.contains("reps"));
        assert!(!skip_line.contains("reps"));
    }

    #[test]
    fn border_style_maps_reward_settle_phases() {
        let idle = border_style(RewardFrame {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
            anchor: None,
        });
        let low = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 80,
            game_glow_intensity: 20,
            sweep_progress: 25,
            show_plus_one: true,
            anchor: None,
        });
        let medium = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 40,
            sweep_progress: 55,
            show_plus_one: true,
            anchor: None,
        });
        let peak = border_style(RewardFrame {
            active: true,
            plus_one_intensity: 100,
            game_glow_intensity: 58,
            sweep_progress: 100,
            show_plus_one: true,
            anchor: None,
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
    fn sweep_cell_progress_runs_from_top_left_to_bottom_right() {
        let area = Rect::new(2, 3, 20, 10);
        let bottom_left = border_sweep_cell_progress(area.left(), area.bottom() - 1, area);
        let bottom_right = border_sweep_cell_progress(area.right() - 1, area.bottom() - 1, area);
        let top_left = border_sweep_cell_progress(area.left(), area.top(), area);
        let top_right = border_sweep_cell_progress(area.right() - 1, area.top(), area);

        assert_eq!(top_left, 0.0);
        assert_eq!(bottom_right, 1.0);
        assert!(bottom_left > top_left);
        assert!(top_right > top_left);
        assert!(bottom_left < bottom_right);
        assert!(top_right < bottom_right);
    }

    #[test]
    fn directional_sweep_lights_top_left_before_bottom_right() {
        let area = Rect::new(0, 0, 80, 20);
        let early = reward_frame(100, 58, 10);
        let top_left = reward_sweep_intensity_at(area.left(), area.top(), area, early);
        let bottom_right =
            reward_sweep_intensity_at(area.right() - 1, area.bottom() - 1, area, early);

        assert!(top_left > 30);
        assert!(bottom_right < 5);
    }

    #[test]
    fn directional_sweep_reaches_bottom_right_later() {
        let area = Rect::new(0, 0, 80, 20);
        let early = reward_frame(100, 58, 10);
        let later = reward_frame(100, 58, 100);
        let early_bottom_right =
            reward_sweep_intensity_at(area.right() - 1, area.bottom() - 1, area, early);
        let later_bottom_right =
            reward_sweep_intensity_at(area.right() - 1, area.bottom() - 1, area, later);

        assert!(later_bottom_right > early_bottom_right);
        assert!(later_bottom_right > 45);
    }

    #[test]
    fn directional_sweep_fades_after_passing() {
        let area = Rect::new(0, 0, 80, 20);
        let peak = reward_frame(100, 58, 100);
        let faded = reward_frame(20, 12, 100);
        let peak_bottom_right =
            reward_sweep_intensity_at(area.right() - 1, area.bottom() - 1, area, peak);
        let faded_bottom_right =
            reward_sweep_intensity_at(area.right() - 1, area.bottom() - 1, area, faded);

        assert!(peak_bottom_right > faded_bottom_right);
        assert!(faded_bottom_right > 0);
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
            anchor: None,
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
    fn feedback_height_and_side_by_side_layout_stay_stable_across_default_states() {
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
        let idle_layout = shell_layout(area, idle_height);
        let wrong_layout = shell_layout(area, wrong_height);
        let success_layout = shell_layout(area, success_height);
        assert_eq!(idle_layout, wrong_layout);
        assert_eq!(wrong_layout, success_layout);
        assert_eq!(idle_layout.problem.y, idle_layout.editor.y);
        assert_eq!(idle_layout.problem.height, idle_layout.editor.height);
        assert!(idle_layout.editor.right() < idle_layout.problem.left());
        assert!(idle_layout.problem.height > SNAPSHOT_PROBLEM_HEIGHT);
        assert!(idle_layout.editor.height > SNAPSHOT_WORKBOX_HEIGHT);
        assert_eq!(idle_layout.feedback.height, FEEDBACK_CHROME_HEIGHT);
        assert_eq!(idle_layout.rail.height, TOP_COUNTER_HEIGHT);
        assert_eq!(idle_layout.controls.height, COMMAND_RAIL_HEIGHT);
        assert!(idle_layout.rail.bottom() <= idle_layout.editor.top());
        assert!(idle_layout.rail.bottom() <= idle_layout.problem.top());
        assert!(idle_layout.feedback.top() >= idle_layout.problem.bottom());

        let chunks = layout_chunks(area, idle_height);
        assert_eq!(chunks[0], idle_layout.problem);
        assert_eq!(chunks[1], idle_layout.editor);
        assert_eq!(chunks[2], idle_layout.controls);
        assert_eq!(chunks[3], idle_layout.feedback);
        assert_eq!(chunks[4], idle_layout.rail);
        assert_eq!(chunks[5], idle_layout.instruction);
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

        assert_eq!(frame.completed_reps_this_run, 0);
        assert!(frame.failure.active);
        assert!(!frame.reward.show_plus_one);
        assert!(!frame.skip.active);
        assert!(!render_top_banner_line(&frame, 40).contains("+1"));
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("reps"))
        );
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("PASS"))
        );
    }

    #[test]
    fn skip_and_quit_do_not_render_plus_one() {
        let mut skip_app = TuiApp::new();
        skip_app.handle_action(ClientAction::Skip);
        let skip_snapshot = render_snapshot(&skip_app.frame());

        let mut quit_app = TuiApp::new();
        quit_app.handle_action(ClientAction::Quit);
        let quit_snapshot = render_snapshot(&quit_app.frame());

        assert!(skip_app.frame().skip.active);
        assert!(!skip_app.frame().reward.show_plus_one);
        assert!(!render_top_banner_line(&skip_app.frame(), 40).contains("+1"));
        assert!(
            !skip_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(
            !skip_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("reps"))
        );
        assert!(!quit_app.frame().skip.active);
        assert!(
            !quit_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(
            !quit_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("reps"))
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
    fn wrong_answer_feedback_uses_local_failure_state_without_global_result_text() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let lines = feedback_chrome_lines(&frame);

        assert!(frame.failure.active);
        assert!(lines.is_empty());
        assert_eq!(
            status_tone_style(StatusTone::TryAgain).fg,
            Some(RUST_REPS_DARK.error)
        );
    }

    #[test]
    fn wrong_answer_failure_pulse_tints_active_editor_caret_and_line() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        let frame = app.frame_at(now);
        let lines = workbox_lines(&frame, SNAPSHOT_WORKBOX_HEIGHT);
        let active_line = lines
            .iter()
            .find(|line| line_text(line) == " 1 │ wrong")
            .expect("expected active editor line with preserved wrong answer");

        assert_eq!(
            active_line.style.bg,
            Some(RUST_REPS_DARK.editor_failure_line)
        );
        assert_eq!(
            frame.failure.anchor,
            Some(EditorAnchor { row: 0, column: 5 })
        );
        assert_ne!(
            active_line.spans[0].style.fg,
            Some(RUST_REPS_DARK.text_muted)
        );
        assert!(red_dominant(
            active_line.spans[0]
                .style
                .fg
                .expect("expected failure gutter foreground")
        ));
    }

    #[test]
    fn command_area_does_not_allocate_status_feedback_block() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);

        let lines = feedback_chrome_lines(&app.frame());

        assert!(lines.is_empty());
        assert_eq!(feedback_height(&lines), FEEDBACK_CHROME_HEIGHT);
    }

    #[test]
    fn skip_and_complete_do_not_allocate_status_feedback_block() {
        let mut skip_app = TuiApp::new();
        skip_app.handle_action(ClientAction::Skip);
        let skip_lines = feedback_chrome_lines(&skip_app.frame());

        assert!(skip_lines.is_empty());
        assert!(skip_app.frame().skip.active);

        let mut complete_app = TuiApp::new();
        for _ in 0..12 {
            complete_app.handle_action(ClientAction::Skip);
        }
        let complete_lines = feedback_chrome_lines(&complete_app.frame());

        assert!(complete_lines.is_empty());
    }

    #[test]
    fn code_prompt_renders_with_non_default_syntax_styles() {
        let app = TuiApp::new();
        let frame = app.frame();
        let lines = problem_lines(&frame);
        let prompt_line = lines
            .iter()
            .find(|line| line_text(line) == "  fn main() {}")
            .expect("expected prompt line");

        assert_eq!(line_text(prompt_line), "  fn main() {}");
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
    fn task_code_snippet_uses_editor_consistent_token_styles() {
        let line = problem_display_line(&ProblemDisplayLine::code("score += 1;"));

        assert!(line.spans.iter().any(|span| {
            span.content.as_ref() == "+="
                && span.style.fg == Some(RUST_REPS_DARK.punctuation_operator)
        }));
        assert!(line.spans.iter().any(|span| {
            span.content.as_ref() == "1" && span.style.fg == Some(RUST_REPS_DARK.number)
        }));
    }

    #[test]
    fn wrapped_stimulus_continuation_has_a_distinct_non_source_gutter() {
        let stimulus = "fn add_one(x: i32) -> i32 { x + 1 }";
        let frame = projection_app("Copy this line exactly:", stimulus, stimulus)
            .frame_at_for_layout(test_now(), 26);
        let source_rows = frame
            .game_window
            .problem
            .lines
            .iter()
            .filter(|line| line.stimulus_row.is_some())
            .collect::<Vec<_>>();
        let hard_start = source_rows
            .iter()
            .find(|line| line.stimulus_row == Some(StimulusRowKind::HardLineStart))
            .expect("expected hard source-line start");
        let continuation = source_rows
            .iter()
            .find(|line| line.stimulus_row == Some(StimulusRowKind::SoftContinuation))
            .expect("expected soft continuation");

        let hard_line = problem_display_line(hard_start);
        let continuation_line = problem_display_line(continuation);
        assert_eq!(
            hard_line.spans[0].content.as_ref(),
            STIMULUS_HARD_LINE_GUTTER
        );
        assert_eq!(
            continuation_line.spans[0].content.as_ref(),
            STIMULUS_CONTINUATION_GUTTER
        );
        assert_eq!(continuation_line.spans[0].style, muted_style());
        assert_ne!(
            continuation_line.spans[0].style,
            continuation_line.spans[1].style
        );
        assert_eq!(
            continuation_line
                .spans
                .iter()
                .skip(1)
                .map(|span| span.content.as_ref())
                .collect::<String>(),
            continuation.text
        );
        assert!(!continuation.text.contains('↪'));

        let backend = TestBackend::new(40, 1);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|frame| {
                frame.render_widget(
                    Paragraph::new(problem_display_line(continuation)).style(surface_style()),
                    Rect::new(0, 0, 40, 1),
                );
            })
            .expect("expected continuation render");
        let buffer = terminal.backend().buffer();

        assert_eq!(buffer[(0, 0)].symbol(), "↪");
        assert_eq!(buffer[(1, 0)].symbol(), " ");
        assert_eq!(buffer[(2, 0)].symbol(), "i");
        assert_eq!(
            buffer[(0, 0)].fg,
            muted_style().fg.expect("muted foreground")
        );
        assert_ne!(buffer[(0, 0)].fg, buffer[(2, 0)].fg);
    }

    #[test]
    fn render_snapshot_shows_typed_input_text() {
        let mut app = TuiApp::new();
        for ch in "hello".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.game.iter().any(|line| line == " 1 │ hello"));
        assert_eq!(frame.game_window.workbox.cursor.column, 5);
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

        assert!(snapshot.game.iter().any(|line| line == " 1 │ abc"));
        assert_eq!(frame.game_window.workbox.cursor.column, 2);
    }

    #[test]
    fn moving_cursor_does_not_change_rendered_code_text_or_width() {
        let mut app = TuiApp::new();
        for ch in "let score: i32 = 1;".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        let end_snapshot = render_snapshot(&app.frame());
        let end_line = end_snapshot
            .game
            .iter()
            .find(|line| line.starts_with(" 1 │ let score"))
            .expect("expected rendered editor line")
            .clone();

        for _ in 0..8 {
            app.handle_action(ClientAction::MoveCursorLeft);
        }
        let middle_snapshot = render_snapshot(&app.frame());
        let middle_line = middle_snapshot
            .game
            .iter()
            .find(|line| line.starts_with(" 1 │ let score"))
            .expect("expected rendered editor line")
            .clone();

        assert_eq!(end_line, " 1 │ let score: i32 = 1;");
        assert_eq!(middle_line, end_line);
        assert_eq!(middle_line.chars().count(), end_line.chars().count());
        assert_eq!(app.input(), "let score: i32 = 1;");
    }

    #[test]
    fn cursor_overlay_is_slim_and_does_not_displace_rendered_text() {
        let mut app = TuiApp::new();
        type_text(&mut app, "abc");
        app.handle_action(ClientAction::MoveCursorLeft);
        let client_frame = app.frame();
        let snapshot = render_snapshot(&client_frame);
        let area = Rect::new(0, 0, 16, 4);
        let rendered = render_editor_buffer_line(&client_frame, area);
        let backend = TestBackend::new(area.width, area.height);
        let mut terminal = Terminal::new(backend).expect("expected test terminal");
        terminal
            .draw(|draw_frame| {
                draw_frame.render_widget(
                    Paragraph::new(Text::from(workbox_lines_for_area(&client_frame, area)))
                        .style(editor_canvas_style()),
                    area,
                );
                render_editor_cursor(draw_frame, area, &client_frame);
            })
            .expect("expected editor render");
        terminal
            .backend_mut()
            .assert_cursor_position(Position::new(usize_to_u16(gutter_width(1) + 3 + 2), 0));

        assert!(snapshot.game.iter().any(|line| line == " 1 │ abc"));
        assert!(rendered.contains(" 1 │ abc"));
        assert!(!rendered.contains('▏'));
        assert!(!rendered.contains(" 1 │ ab c"));
        assert_eq!(app.input(), "abc");
    }

    #[test]
    fn long_editor_line_uses_horizontal_viewport_and_overflow_cues() {
        let mut app = TuiApp::new();
        app.set_editor_viewport_width(4);
        type_text(&mut app, "abcdef");
        let frame = app.frame();
        let lines = workbox_lines_for_width(&frame, 4, 10)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();
        let rendered = render_editor_buffer_line(&frame, Rect::new(0, 0, 10, 4));

        assert_eq!(frame.game_window.workbox.viewport.first_column, 3);
        assert!(lines.iter().any(|line| line == " 1 │ def"));
        assert_eq!(editor_horizontal_overflow(&frame, 0, 4), (true, false));
        assert!(rendered.contains("‹"));
        assert!(!rendered.contains("abcdef"));
    }

    #[test]
    fn horizontal_viewport_retreats_when_cursor_moves_left() {
        let mut app = TuiApp::new();
        app.set_editor_viewport_width(4);
        type_text(&mut app, "abcdef");
        for _ in 0..5 {
            app.handle_action(ClientAction::MoveCursorLeft);
        }
        let frame = app.frame();
        let lines = workbox_lines_for_width(&frame, 4, 10)
            .iter()
            .map(line_text)
            .collect::<Vec<_>>();

        assert_eq!(frame.game_window.workbox.cursor.column, 1);
        assert_eq!(frame.game_window.workbox.viewport.first_column, 1);
        assert!(lines.iter().any(|line| line == " 1 │ bcde"));
        assert_eq!(editor_horizontal_overflow(&frame, 0, 4), (true, true));
    }

    #[test]
    fn render_snapshot_hides_cursor_when_cursor_state_is_hidden() {
        let mut app = TuiApp::new();
        app.tick_cursor();
        let frame = app.frame();
        let empty_snapshot = render_snapshot(&frame);
        assert!(empty_snapshot.game.iter().any(|line| line == " 1 │ "));

        app.handle_action(ClientAction::TypeChar('x'));
        app.tick_cursor();
        let frame = app.frame();
        let typed_snapshot = render_snapshot(&frame);
        assert!(typed_snapshot.game.iter().any(|line| line == " 1 │ x"));
    }

    #[test]
    fn render_snapshot_has_no_default_help_overlay() {
        let app = TuiApp::new();
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
        assert!(
            !default_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("? help"))
        );
        assert!(
            !default_snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("Enter newline"))
        );
    }

    #[test]
    fn render_snapshot_keeps_wrong_answer_local_without_global_result_block() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(frame.failure.active);
        assert!(snapshot.feedback.is_empty());
        assert!(!snapshot.game.iter().any(|line| line == "TRY AGAIN"));
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("PASS"))
        );
    }

    #[test]
    fn unknown_command_keeps_status_data_without_global_result_block() {
        let mut app = TuiApp::new();
        for ch in ":unknown".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Submit);
        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(
            frame
                .status_chrome
                .lines
                .iter()
                .any(|line| line.text == "UNKNOWN COMMAND")
        );
        assert!(
            frame
                .status_chrome
                .lines
                .iter()
                .any(|line| line.text == "Type :help for commands.")
        );
        assert!(snapshot.feedback.is_empty());
        assert!(
            !snapshot
                .game
                .iter()
                .any(|line| line == ">> UNKNOWN COMMAND")
        );
    }

    #[test]
    fn render_snapshot_hides_pass_and_keeps_reward_counter_local_after_success() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        let frame = app.frame_at(now + REWARD_PLUS_ONE_DURATION / 2);
        let snapshot = render_snapshot(&frame);

        assert!(!snapshot.feedback.iter().any(|line| line == "PASS"));
        assert!(
            snapshot
                .game
                .iter()
                .any(|line| line == "  println!(\"Rust Reps\");")
        );
        assert!(snapshot.game.iter().any(|line| line == " 1 │ "));
        assert!(frame.reward.show_plus_one);
        assert_eq!(frame.reward.anchor, None);
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("+1"))
        );
        assert!(render_top_banner_line(&frame, 40).contains("+1"));
        assert!(
            !snapshot
                .command_chrome
                .iter()
                .any(|line| line.contains("reps"))
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
        assert!(snapshot.game.iter().any(|line| line == " 2 │ b"));
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

        let frame = app.frame();
        let snapshot = render_snapshot(&frame);

        assert!(snapshot.game.iter().any(|line| line == " 1 │ let"));
        assert!(snapshot.game.iter().any(|line| line == " 2 │ score"));
        assert_eq!(frame.game_window.workbox.cursor.column, 4);
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
        assert!(snapshot.game.iter().any(|line| line == " 3 │ b"));
    }

    #[test]
    fn render_snapshot_keeps_one_line_case_clean() {
        let app = TuiApp::new();
        let snapshot = render_snapshot(&app.frame());

        assert!(snapshot.game.iter().any(|line| line == " 1 │ "));
        assert!(!snapshot.game.iter().any(|line| line == " 2 │ "));
    }

    #[test]
    fn viewport_snapshot_keeps_cursor_line_visible_when_editor_is_taller_than_workbox() {
        let mut app = TuiApp::new();
        app.set_editor_viewport_height(visible_editor_rows(3));
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
        assert!(snapshot.game.iter().any(|line| line == " 5 │ e"));
    }
}
