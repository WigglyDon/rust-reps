use std::time::{Duration, Instant};

use crate::engine;

mod editor;

pub use editor::{CursorPosition, EditorAction, EditorBuffer, EditorView, EditorViewport};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientAction {
    TypeChar(char),
    Backspace,
    InsertNewline,
    MoveCursorTo(CursorPosition),
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveLineStart,
    MoveLineEnd,
    ScrollProblemLineUp,
    ScrollProblemLineDown,
    ScrollProblemUp,
    ScrollProblemDown,
    ScrollEditorUp,
    ScrollEditorDown,
    Submit,
    Skip,
    Quit,
    CursorTick,
    Ignore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeTokenKind {
    Keyword,
    Type,
    Number,
    StringLiteral,
    Macro,
    Identifier,
    Punctuation,
    Whitespace,
    Comment,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeToken {
    pub kind: CodeTokenKind,
    pub text: String,
}

impl CodeToken {
    fn new(kind: CodeTokenKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

pub const REWARD_ATTACK_DURATION: Duration = Duration::from_millis(100);
pub const REWARD_PLUS_ONE_BODY_DURATION: Duration = Duration::from_millis(450);
pub const REWARD_RELEASE_DURATION: Duration = Duration::from_millis(550);
pub const REWARD_PLUS_ONE_DURATION: Duration = Duration::from_millis(1_100);
pub const REWARD_DURATION: Duration = Duration::from_millis(2_250);
pub const REWARD_GAME_ATTACK_DURATION: Duration = Duration::from_millis(150);
pub const REWARD_GAME_BODY_DURATION: Duration = Duration::from_millis(1_050);
pub const REWARD_GAME_RELEASE_DURATION: Duration = Duration::from_millis(1_050);
pub const REWARD_SWEEP_DURATION: Duration = Duration::from_millis(850);
pub const REWARD_GAME_GLOW_DELAY: Duration = Duration::from_millis(80);
pub const REWARD_GAME_GLOW_MAX_INTENSITY: u8 = 58;
pub const FAILURE_PULSE_DURATION: Duration = Duration::from_millis(520);
pub const SKIP_TOAST_DURATION: Duration = Duration::from_millis(900);
pub const PROBLEM_WHEEL_SCROLL_LINES: usize = 1;
pub const PROBLEM_SCROLL_LINES: usize = 4;
pub const DEFAULT_PROBLEM_WRAP_WIDTH: usize = 80;
pub const MIN_PROBLEM_WRAP_WIDTH: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProblemLineKind {
    Blank,
    Header,
    AnswerShape,
    Bullet,
    Code,
    #[allow(dead_code)]
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemDisplayLine {
    pub kind: ProblemLineKind,
    pub text: String,
    pub tokens: Vec<CodeToken>,
}

impl ProblemDisplayLine {
    pub fn blank() -> Self {
        Self {
            kind: ProblemLineKind::Blank,
            text: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn header(text: impl Into<String>) -> Self {
        Self {
            kind: ProblemLineKind::Header,
            text: text.into(),
            tokens: Vec::new(),
        }
    }

    pub fn answer_shape(text: impl Into<String>) -> Self {
        Self {
            kind: ProblemLineKind::AnswerShape,
            text: text.into(),
            tokens: Vec::new(),
        }
    }

    pub fn bullet(text: impl Into<String>) -> Self {
        Self {
            kind: ProblemLineKind::Bullet,
            text: text.into(),
            tokens: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn code(text: &str) -> Self {
        Self {
            kind: ProblemLineKind::Code,
            text: text.to_string(),
            tokens: tokenize_rust_line(text),
        }
    }

    fn code_with_tokens(tokens: Vec<CodeToken>) -> Self {
        let text = tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<String>();
        Self {
            kind: ProblemLineKind::Code,
            text,
            tokens,
        }
    }

    #[allow(dead_code)]
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            kind: ProblemLineKind::Plain,
            text: text.into(),
            tokens: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemViewport {
    pub scroll_offset: usize,
}

impl ProblemViewport {
    pub fn new() -> Self {
        Self { scroll_offset: 0 }
    }

    #[cfg(test)]
    pub fn with_offset(scroll_offset: usize) -> Self {
        Self { scroll_offset }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_up_by(PROBLEM_SCROLL_LINES);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_down_by(PROBLEM_SCROLL_LINES);
    }

    pub fn scroll_line_up(&mut self) {
        self.scroll_up_by(PROBLEM_WHEEL_SCROLL_LINES);
    }

    pub fn scroll_line_down(&mut self) {
        self.scroll_down_by(PROBLEM_WHEEL_SCROLL_LINES);
    }

    fn scroll_up_by(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    fn scroll_down_by(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
    }

    pub fn clamped(self, total_lines: usize) -> Self {
        Self {
            scroll_offset: self.scroll_offset.min(max_scroll_offset(total_lines)),
        }
    }

    pub fn visible_range(self, total_lines: usize, visible_lines: usize) -> ProblemVisibleRange {
        if total_lines == 0 || visible_lines == 0 {
            return ProblemVisibleRange { start: 0, end: 0 };
        }

        let visible_lines = visible_lines.min(total_lines);
        let max_offset = total_lines.saturating_sub(visible_lines);
        let start = self.scroll_offset.min(max_offset);
        let end = (start + visible_lines).min(total_lines);
        ProblemVisibleRange { start, end }
    }
}

impl Default for ProblemViewport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProblemVisibleRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewportOverflow {
    pub above: bool,
    pub below: bool,
    pub offset: usize,
    pub visible: usize,
    pub total: usize,
}

impl ViewportOverflow {
    pub fn from_parts(total: usize, visible: usize, offset: usize) -> Self {
        if total == 0 || visible == 0 || total <= visible {
            return Self {
                above: false,
                below: false,
                offset: 0,
                visible,
                total,
            };
        }

        let visible = visible.min(total);
        let max_offset = total.saturating_sub(visible);
        let offset = offset.min(max_offset);
        Self {
            above: offset > 0,
            below: offset < max_offset,
            offset,
            visible,
            total,
        }
    }

    pub fn has_overflow(self) -> bool {
        self.above || self.below
    }
}

fn max_scroll_offset(total_lines: usize) -> usize {
    total_lines.saturating_sub(1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditorAnchor {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RewardFrame {
    pub active: bool,
    pub plus_one_intensity: u8,
    pub game_glow_intensity: u8,
    pub sweep_progress: u8,
    pub show_plus_one: bool,
    pub anchor: Option<EditorAnchor>,
}

impl RewardFrame {
    fn idle() -> Self {
        Self {
            active: false,
            plus_one_intensity: 0,
            game_glow_intensity: 0,
            sweep_progress: 0,
            show_plus_one: false,
            anchor: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FailureFrame {
    pub active: bool,
    pub intensity: u8,
    pub anchor: Option<EditorAnchor>,
}

impl FailureFrame {
    fn idle() -> Self {
        Self {
            active: false,
            intensity: 0,
            anchor: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkipFrame {
    pub active: bool,
    pub intensity: u8,
    pub anchor: Option<EditorAnchor>,
}

impl SkipFrame {
    fn idle() -> Self {
        Self {
            active: false,
            intensity: 0,
            anchor: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FailurePulse {
    started_at: Option<Instant>,
    duration: Duration,
    anchor: Option<EditorAnchor>,
}

impl FailurePulse {
    fn new() -> Self {
        Self {
            started_at: None,
            duration: FAILURE_PULSE_DURATION,
            anchor: None,
        }
    }

    fn start(&mut self, now: Instant, anchor: EditorAnchor) {
        self.started_at = Some(now);
        self.anchor = Some(anchor);
    }

    fn clear(&mut self) {
        self.started_at = None;
        self.anchor = None;
    }

    fn settle(&mut self, now: Instant) {
        if !self.frame_at(now).active {
            self.clear();
        }
    }

    fn frame_at(self, now: Instant) -> FailureFrame {
        let Some(started_at) = self.started_at else {
            return FailureFrame::idle();
        };
        let Some(elapsed) = now.checked_duration_since(started_at) else {
            return FailureFrame::idle();
        };
        if elapsed >= self.duration {
            return FailureFrame::idle();
        }

        FailureFrame {
            active: true,
            intensity: failure_pulse_intensity_at(elapsed),
            anchor: self.anchor,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RewardAnimation {
    started_at: Option<Instant>,
    duration: Duration,
    anchor: Option<EditorAnchor>,
}

impl RewardAnimation {
    fn new() -> Self {
        Self {
            started_at: None,
            duration: REWARD_DURATION,
            anchor: None,
        }
    }

    fn start(&mut self, now: Instant, anchor: EditorAnchor) {
        self.started_at = Some(now);
        self.anchor = Some(anchor);
    }

    fn clear(&mut self) {
        self.started_at = None;
        self.anchor = None;
    }

    fn settle(&mut self, now: Instant) {
        if !self.frame_at(now).active {
            self.clear();
        }
    }

    fn frame_at(self, now: Instant) -> RewardFrame {
        let Some(started_at) = self.started_at else {
            return RewardFrame::idle();
        };
        let elapsed = now.saturating_duration_since(started_at);
        if elapsed >= self.duration {
            return RewardFrame::idle();
        }

        RewardFrame {
            active: true,
            plus_one_intensity: plus_one_intensity_at(elapsed),
            game_glow_intensity: game_glow_intensity_at(elapsed),
            sweep_progress: reward_sweep_progress_at(elapsed),
            show_plus_one: elapsed < REWARD_PLUS_ONE_DURATION,
            anchor: self.anchor,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SkipToast {
    started_at: Option<Instant>,
    duration: Duration,
    anchor: Option<EditorAnchor>,
}

impl SkipToast {
    fn new() -> Self {
        Self {
            started_at: None,
            duration: SKIP_TOAST_DURATION,
            anchor: None,
        }
    }

    fn start(&mut self, now: Instant, anchor: EditorAnchor) {
        self.started_at = Some(now);
        self.anchor = Some(anchor);
    }

    fn clear(&mut self) {
        self.started_at = None;
        self.anchor = None;
    }

    fn settle(&mut self, now: Instant) {
        if !self.frame_at(now).active {
            self.clear();
        }
    }

    fn frame_at(self, now: Instant) -> SkipFrame {
        let Some(started_at) = self.started_at else {
            return SkipFrame::idle();
        };
        let elapsed = now.saturating_duration_since(started_at);
        if elapsed >= self.duration {
            return SkipFrame::idle();
        }

        SkipFrame {
            active: true,
            intensity: skip_toast_intensity_at(elapsed),
            anchor: self.anchor,
        }
    }
}

fn plus_one_intensity_at(elapsed: Duration) -> u8 {
    if elapsed >= REWARD_PLUS_ONE_DURATION {
        return 0;
    }

    pulse_intensity_at(
        elapsed,
        REWARD_ATTACK_DURATION,
        REWARD_PLUS_ONE_BODY_DURATION,
        REWARD_RELEASE_DURATION,
        REWARD_PLUS_ONE_DURATION,
    )
}

fn game_glow_intensity_at(elapsed: Duration) -> u8 {
    if elapsed < REWARD_GAME_GLOW_DELAY || elapsed >= REWARD_DURATION {
        return 0;
    }

    let delayed_elapsed = elapsed.saturating_sub(REWARD_GAME_GLOW_DELAY);
    let glow_intensity = pulse_intensity_at(
        delayed_elapsed,
        REWARD_GAME_ATTACK_DURATION,
        REWARD_GAME_BODY_DURATION,
        REWARD_GAME_RELEASE_DURATION,
        REWARD_DURATION,
    );
    ((f32::from(glow_intensity) * f32::from(REWARD_GAME_GLOW_MAX_INTENSITY)) / 100.0)
        .round()
        .clamp(0.0, f32::from(REWARD_GAME_GLOW_MAX_INTENSITY)) as u8
}

fn pulse_intensity_at(
    elapsed: Duration,
    attack: Duration,
    body: Duration,
    release: Duration,
    total: Duration,
) -> u8 {
    if elapsed >= total {
        return 0;
    }

    let release_starts_at = attack + body;
    let intensity = if elapsed < attack {
        ease_out_cubic(duration_progress(elapsed, attack))
    } else if elapsed < release_starts_at {
        1.0
    } else {
        let release_elapsed = elapsed.saturating_sub(release_starts_at);
        1.0 - smoothstep(duration_progress(release_elapsed, release))
    };

    (intensity * 100.0).round().clamp(0.0, 100.0) as u8
}

fn reward_sweep_progress_at(elapsed: Duration) -> u8 {
    if elapsed >= REWARD_SWEEP_DURATION {
        return 100;
    }

    (smoothstep(duration_progress(elapsed, REWARD_SWEEP_DURATION)) * 100.0)
        .round()
        .clamp(0.0, 100.0) as u8
}

fn failure_pulse_intensity_at(elapsed: Duration) -> u8 {
    if elapsed >= FAILURE_PULSE_DURATION {
        return 0;
    }

    let hold = Duration::from_millis(120);
    let intensity = if elapsed < hold {
        1.0
    } else {
        let release_elapsed = elapsed.saturating_sub(hold);
        let release_duration = FAILURE_PULSE_DURATION.saturating_sub(hold);
        1.0 - smoothstep(duration_progress(release_elapsed, release_duration))
    };

    (intensity * 100.0).round().clamp(0.0, 100.0) as u8
}

fn skip_toast_intensity_at(elapsed: Duration) -> u8 {
    if elapsed >= SKIP_TOAST_DURATION {
        return 0;
    }

    let hold = Duration::from_millis(220);
    let intensity = if elapsed < hold {
        1.0
    } else {
        let release_elapsed = elapsed.saturating_sub(hold);
        let release_duration = SKIP_TOAST_DURATION.saturating_sub(hold);
        1.0 - smoothstep(duration_progress(release_elapsed, release_duration))
    };

    (intensity * 100.0).round().clamp(0.0, 100.0) as u8
}

fn duration_progress(elapsed: Duration, duration: Duration) -> f32 {
    if duration.is_zero() {
        return 1.0;
    }

    (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0)
}

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress).powi(3)
}

fn smoothstep(progress: f32) -> f32 {
    progress * progress * (3.0 - (2.0 * progress))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientUiState {
    editor: EditorBuffer,
    problem_viewport: ProblemViewport,
    cursor_visible: bool,
    help_open: bool,
    status_lines: Vec<String>,
    reward_animation: RewardAnimation,
    failure_pulse: FailurePulse,
    skip_toast: SkipToast,
}

impl ClientUiState {
    pub fn new() -> Self {
        Self {
            editor: EditorBuffer::new(),
            problem_viewport: ProblemViewport::new(),
            cursor_visible: true,
            help_open: false,
            status_lines: Vec::new(),
            reward_animation: RewardAnimation::new(),
            failure_pulse: FailurePulse::new(),
            skip_toast: SkipToast::new(),
        }
    }

    pub fn editor_view(&self) -> EditorView {
        self.editor.view()
    }

    pub fn set_editor_viewport_height(&mut self, visible_lines: usize) {
        self.editor.set_viewport_height(visible_lines);
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn help_open(&self) -> bool {
        self.help_open
    }

    pub fn problem_viewport(&self) -> ProblemViewport {
        self.problem_viewport
    }

    pub fn status_lines(&self) -> &[String] {
        &self.status_lines
    }

    pub fn reward_frame_at(&self, now: Instant) -> RewardFrame {
        self.reward_animation.frame_at(now)
    }

    pub fn reward_active_at(&self, now: Instant) -> bool {
        self.reward_frame_at(now).active
    }

    pub fn failure_frame_at(&self, now: Instant) -> FailureFrame {
        self.failure_pulse.frame_at(now)
    }

    pub fn failure_active_at(&self, now: Instant) -> bool {
        self.failure_frame_at(now).active
    }

    pub fn skip_frame_at(&self, now: Instant) -> SkipFrame {
        self.skip_toast.frame_at(now)
    }

    pub fn skip_active_at(&self, now: Instant) -> bool {
        self.skip_frame_at(now).active
    }

    pub fn apply_action(&mut self, action: ClientAction) {
        match action {
            ClientAction::TypeChar(ch) => {
                self.editor.apply_action(EditorAction::InsertChar(ch));
                self.show_cursor();
            }
            ClientAction::Backspace => {
                self.editor.apply_action(EditorAction::Backspace);
                self.show_cursor();
            }
            ClientAction::InsertNewline => {
                self.editor.apply_action(EditorAction::InsertNewline);
                self.show_cursor();
            }
            ClientAction::MoveCursorTo(position) => {
                self.editor.set_cursor(position);
                self.show_cursor();
            }
            ClientAction::MoveCursorLeft => {
                self.editor.apply_action(EditorAction::MoveLeft);
                self.show_cursor();
            }
            ClientAction::MoveCursorRight => {
                self.editor.apply_action(EditorAction::MoveRight);
                self.show_cursor();
            }
            ClientAction::MoveCursorUp => {
                self.editor.apply_action(EditorAction::MoveUp);
                self.show_cursor();
            }
            ClientAction::MoveCursorDown => {
                self.editor.apply_action(EditorAction::MoveDown);
                self.show_cursor();
            }
            ClientAction::MoveLineStart => {
                self.editor.apply_action(EditorAction::MoveLineStart);
                self.show_cursor();
            }
            ClientAction::MoveLineEnd => {
                self.editor.apply_action(EditorAction::MoveLineEnd);
                self.show_cursor();
            }
            ClientAction::ScrollProblemLineUp => {
                self.problem_viewport.scroll_line_up();
                self.show_cursor();
            }
            ClientAction::ScrollProblemLineDown => {
                self.problem_viewport.scroll_line_down();
                self.show_cursor();
            }
            ClientAction::ScrollProblemUp => {
                self.problem_viewport.scroll_up();
                self.show_cursor();
            }
            ClientAction::ScrollProblemDown => {
                self.problem_viewport.scroll_down();
                self.show_cursor();
            }
            ClientAction::ScrollEditorUp => {
                self.editor.apply_action(EditorAction::ScrollViewportUp);
                self.show_cursor();
            }
            ClientAction::ScrollEditorDown => {
                self.editor.apply_action(EditorAction::ScrollViewportDown);
                self.show_cursor();
            }
            ClientAction::CursorTick => {
                self.cursor_visible = !self.cursor_visible;
            }
            ClientAction::Submit
            | ClientAction::Skip
            | ClientAction::Quit
            | ClientAction::Ignore => {}
        }
    }

    pub fn draft_input(&self) -> String {
        self.editor.text()
    }

    pub fn clear_draft_input(&mut self) {
        self.show_cursor();
        self.editor.clear();
    }

    pub fn set_status_lines(&mut self, status_lines: Vec<String>) {
        self.status_lines = status_lines;
    }

    pub fn show_cursor(&mut self) {
        self.cursor_visible = true;
    }

    pub fn reset_problem_scroll(&mut self) {
        self.problem_viewport = ProblemViewport::new();
    }

    pub fn start_reward_event(&mut self, now: Instant) {
        self.reward_animation
            .start(now, editor_anchor_from_view(&self.editor.view()));
    }

    pub fn clear_reward_event(&mut self) {
        self.reward_animation.clear();
    }

    pub fn settle_reward_event(&mut self, now: Instant) {
        self.reward_animation.settle(now);
    }

    pub fn start_failure_pulse(&mut self, now: Instant) {
        self.failure_pulse
            .start(now, editor_anchor_from_view(&self.editor.view()));
    }

    pub fn clear_failure_pulse(&mut self) {
        self.failure_pulse.clear();
    }

    pub fn settle_failure_pulse(&mut self, now: Instant) {
        self.failure_pulse.settle(now);
    }

    pub fn start_skip_toast(&mut self, now: Instant) {
        self.skip_toast
            .start(now, editor_anchor_from_view(&self.editor.view()));
    }

    pub fn clear_skip_toast(&mut self) {
        self.skip_toast.clear();
    }

    pub fn settle_skip_toast(&mut self, now: Instant) {
        self.skip_toast.settle(now);
    }
}

fn editor_anchor_from_view(view: &EditorView) -> EditorAnchor {
    let total_lines = view.lines.len().max(1);
    let visible_lines = view.viewport.visible_lines.max(1).min(total_lines);
    let max_first_line = total_lines.saturating_sub(visible_lines);
    let first_line = view.viewport.first_line.min(max_first_line);
    let last_visible_line = first_line + visible_lines - 1;
    let line = view.cursor.line.clamp(first_line, last_visible_line);
    let column = if line == view.cursor.line {
        view.cursor.column
    } else {
        view.lines
            .get(line)
            .map(|line| line.chars().count())
            .unwrap_or(0)
    };

    EditorAnchor {
        row: line.saturating_sub(first_line),
        column,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientFrame {
    pub game_window: GameWindowFrame,
    pub status_chrome: StatusChromeFrame,
    pub completed_reps_this_run: usize,
    pub reward: RewardFrame,
    pub reward_active: bool,
    pub failure: FailureFrame,
    pub failure_active: bool,
    pub skip: SkipFrame,
    pub skip_active: bool,
    pub help_chrome: HelpChromeFrame,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameWindowFrame {
    pub problem: ProblemView,
    pub workbox: WorkboxView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemView {
    pub instruction: String,
    pub prompt: String,
    pub prompt_tokens: Vec<CodeToken>,
    pub lines: Vec<ProblemDisplayLine>,
    pub viewport: ProblemViewport,
}

impl ProblemView {
    pub fn visible_lines(&self, visible_lines: usize) -> &[ProblemDisplayLine] {
        let range = self.visible_range(visible_lines);
        &self.lines[range.start..range.end]
    }

    pub fn visible_range(&self, visible_lines: usize) -> ProblemVisibleRange {
        self.viewport.visible_range(self.lines.len(), visible_lines)
    }

    pub fn overflow(&self, visible_lines: usize) -> ViewportOverflow {
        let range = self.visible_range(visible_lines);
        ViewportOverflow::from_parts(self.lines.len(), visible_lines, range.start)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkboxView {
    pub input: String,
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
    pub viewport: EditorViewport,
    pub input_tokens: Vec<CodeToken>,
    pub line_tokens: Vec<Vec<CodeToken>>,
    pub cursor_visible: bool,
}

impl WorkboxView {
    pub fn overflow(&self, visible_lines: usize) -> ViewportOverflow {
        ViewportOverflow::from_parts(
            self.lines.len().max(1),
            visible_lines,
            self.viewport.first_line,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusChromeFrame {
    pub lines: Vec<StatusLine>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLine {
    pub tone: StatusTone,
    pub text: String,
    pub code_tokens: Vec<CodeToken>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusTone {
    Pass,
    Complete,
    TryAgain,
    ExpectedLabel,
    UnknownCommand,
    Warning,
    Plain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpChromeFrame {
    pub open: bool,
    pub lines: Vec<String>,
}

#[cfg(test)]
pub fn project_client_frame(session: &engine::Session, ui_state: &ClientUiState) -> ClientFrame {
    project_client_frame_at(session, ui_state, Instant::now())
}

pub fn project_client_frame_at(
    session: &engine::Session,
    ui_state: &ClientUiState,
    now: Instant,
) -> ClientFrame {
    project_client_frame_for_layout(session, ui_state, now, DEFAULT_PROBLEM_WRAP_WIDTH)
}

pub fn project_client_frame_for_layout(
    session: &engine::Session,
    ui_state: &ClientUiState,
    now: Instant,
    problem_wrap_width: usize,
) -> ClientFrame {
    let snapshot = session.snapshot();
    let exercise = snapshot
        .exercise
        .unwrap_or_else(|| session.first_exercise_packet());
    let editor_view = ui_state.editor_view();
    let input = editor_view.text.clone();
    let line_tokens = editor_view
        .lines
        .iter()
        .map(|line| tokenize_rust_line(line))
        .collect();
    let reward = ui_state.reward_frame_at(now);
    let failure = ui_state.failure_frame_at(now);
    let skip = ui_state.skip_frame_at(now);
    let problem_lines =
        problem_display_lines("do this:", exercise.prompt.as_ref(), problem_wrap_width);
    let problem_viewport = ui_state.problem_viewport().clamped(problem_lines.len());

    ClientFrame {
        game_window: GameWindowFrame {
            problem: ProblemView {
                instruction: "do this:".to_string(),
                prompt: exercise.prompt.to_string(),
                prompt_tokens: tokenize_rust_line(exercise.prompt.as_ref()),
                lines: problem_lines,
                viewport: problem_viewport,
            },
            workbox: WorkboxView {
                input_tokens: tokenize_rust_line(&input),
                input,
                lines: editor_view.lines,
                cursor: editor_view.cursor,
                viewport: editor_view.viewport,
                line_tokens,
                cursor_visible: ui_state.cursor_visible(),
            },
        },
        status_chrome: StatusChromeFrame {
            lines: status_lines(ui_state.status_lines()),
        },
        completed_reps_this_run: snapshot.stats.passed,
        reward,
        reward_active: reward.active,
        failure,
        failure_active: failure.active,
        skip,
        skip_active: skip.active,
        help_chrome: HelpChromeFrame {
            open: ui_state.help_open(),
            lines: help_lines(ui_state.help_open()),
        },
    }
}

fn problem_display_lines(
    instruction: &str,
    prompt: &str,
    wrap_width: usize,
) -> Vec<ProblemDisplayLine> {
    let mut lines = Vec::new();
    if should_render_problem_instruction(prompt) {
        lines.extend([
            ProblemDisplayLine::blank(),
            ProblemDisplayLine::header(instruction),
            ProblemDisplayLine::blank(),
        ]);
    } else {
        lines.push(ProblemDisplayLine::blank());
    }
    for line in prompt.lines() {
        if line.is_empty() {
            lines.push(ProblemDisplayLine::blank());
        } else {
            lines.extend(problem_display_lines_for_source_line(line, wrap_width));
        }
    }
    lines
}

fn should_render_problem_instruction(prompt: &str) -> bool {
    prompt
        .lines()
        .find(|line| !line.trim().is_empty())
        .is_none_or(|line| !is_problem_header(line.trim()))
}

fn problem_display_lines_for_source_line(line: &str, wrap_width: usize) -> Vec<ProblemDisplayLine> {
    let kind = problem_line_kind(line);
    match kind {
        ProblemLineKind::Blank => vec![ProblemDisplayLine::blank()],
        ProblemLineKind::Header => split_display_line_for_kind(line, wrap_width, kind)
            .into_iter()
            .map(ProblemDisplayLine::header)
            .collect(),
        ProblemLineKind::AnswerShape => split_display_line_for_kind(line, wrap_width, kind)
            .into_iter()
            .map(ProblemDisplayLine::answer_shape)
            .collect(),
        ProblemLineKind::Bullet => split_display_line_for_kind(line, wrap_width, kind)
            .into_iter()
            .map(ProblemDisplayLine::bullet)
            .collect(),
        ProblemLineKind::Code => split_code_display_line(line, wrap_width),
        ProblemLineKind::Plain => split_display_line_for_kind(line, wrap_width, kind)
            .into_iter()
            .map(ProblemDisplayLine::plain)
            .collect(),
    }
}

fn split_display_line_for_kind(
    line: &str,
    wrap_width: usize,
    kind: ProblemLineKind,
) -> Vec<String> {
    match kind {
        ProblemLineKind::Code => split_code_display_line(line, wrap_width)
            .into_iter()
            .map(|line| line.text)
            .collect(),
        ProblemLineKind::Bullet => wrap_problem_bullet(line, wrap_width),
        _ => split_display_line(line, wrap_width),
    }
}

fn split_display_line(line: &str, wrap_width: usize) -> Vec<String> {
    let wrap_width = wrap_width.max(MIN_PROBLEM_WRAP_WIDTH);
    if line.is_empty() {
        return vec![String::new()];
    }

    if preserves_problem_spacing(line) {
        return split_long_segment(line, wrap_width);
    }

    wrap_problem_prose(line, wrap_width)
}

fn problem_line_kind(line: &str) -> ProblemLineKind {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        ProblemLineKind::Blank
    } else if is_answer_shape_header(trimmed) {
        ProblemLineKind::AnswerShape
    } else if is_problem_header(trimmed) {
        ProblemLineKind::Header
    } else if is_bullet_line(line) {
        ProblemLineKind::Bullet
    } else if is_standalone_path_or_token(trimmed) {
        ProblemLineKind::Plain
    } else if preserves_problem_spacing(line) || looks_like_rust_code(trimmed) {
        ProblemLineKind::Code
    } else {
        ProblemLineKind::Plain
    }
}

fn is_problem_header(trimmed: &str) -> bool {
    trimmed.ends_with(':')
        && trimmed
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch.is_whitespace() || ch == ':' || ch == '-')
}

fn is_answer_shape_header(trimmed: &str) -> bool {
    let lowercase = trimmed.to_ascii_lowercase();
    is_problem_header(trimmed)
        && (lowercase == "answer:"
            || lowercase == "answer shape:"
            || lowercase == "expected:"
            || lowercase == "expected answer:"
            || lowercase == "expected shape:")
}

fn preserves_problem_spacing(line: &str) -> bool {
    line.starts_with(char::is_whitespace) || line.contains('\t')
}

fn looks_like_rust_code(line: &str) -> bool {
    line.starts_with("fn ")
        || line.starts_with("let ")
        || line.starts_with("if ")
        || line.starts_with("while ")
        || line.starts_with("match ")
        || line.starts_with("use ")
        || line.starts_with("//")
        || line.contains("::")
        || line.contains("=>")
        || looks_like_rust_statement_fragment(line)
        || line.contains('{')
        || line.contains('}')
}

fn looks_like_rust_statement_fragment(line: &str) -> bool {
    line.ends_with(';')
        && (line.contains("+=")
            || line.contains("-=")
            || line.contains("*=")
            || line.contains("/=")
            || line.contains('=')
            || line.contains('(')
            || line.chars().any(|ch| ch.is_ascii_digit()))
}

fn is_standalone_path_or_token(line: &str) -> bool {
    !line.chars().any(char::is_whitespace)
        && (line.contains("::")
            || line.contains('/')
            || line.contains('.')
            || line.contains('_')
            || line.contains('-'))
}

fn is_bullet_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- ") || trimmed.starts_with("* ")
}

fn wrap_problem_prose(line: &str, wrap_width: usize) -> Vec<String> {
    let wrap_width = wrap_width.max(MIN_PROBLEM_WRAP_WIDTH);
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in line.split_whitespace() {
        if word.chars().count() > wrap_width {
            if !current.is_empty() {
                lines.push(current);
                current = String::new();
            }
            lines.extend(split_long_segment(word, wrap_width));
            continue;
        }

        let next_len = if current.is_empty() {
            word.chars().count()
        } else {
            current.chars().count() + 1 + word.chars().count()
        };

        if next_len <= wrap_width {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn wrap_problem_bullet(line: &str, wrap_width: usize) -> Vec<String> {
    let wrap_width = wrap_width.max(MIN_PROBLEM_WRAP_WIDTH);
    let leading = line
        .chars()
        .take_while(|ch| ch.is_whitespace())
        .collect::<String>();
    let trimmed = line.trim_start();
    let Some(rest) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
    else {
        return split_display_line(line, wrap_width);
    };
    let marker = &trimmed[..2];
    let first_prefix = format!("{leading}{marker}");
    let continuation_prefix = format!("{leading}  ");
    let prefix_width = first_prefix.chars().count();
    if prefix_width >= wrap_width {
        return split_long_segment(line, wrap_width);
    }

    let wrapped = wrap_problem_prose(rest, wrap_width.saturating_sub(prefix_width));
    wrapped
        .into_iter()
        .enumerate()
        .map(|(index, chunk)| {
            if index == 0 {
                format!("{first_prefix}{chunk}")
            } else {
                format!("{continuation_prefix}{chunk}")
            }
        })
        .collect()
}

fn split_long_segment(text: &str, wrap_width: usize) -> Vec<String> {
    let wrap_width = wrap_width.max(MIN_PROBLEM_WRAP_WIDTH);
    let chars = text.chars().collect::<Vec<_>>();
    let mut lines = Vec::new();
    let mut start = 0usize;

    while start < chars.len() {
        let hard_end = (start + wrap_width).min(chars.len());
        if hard_end == chars.len() {
            lines.push(chars[start..hard_end].iter().collect::<String>());
            break;
        }

        let end = split_at_preferred_separator(&chars, start, hard_end).unwrap_or(hard_end);
        lines.push(chars[start..end].iter().collect::<String>());
        start = end;
    }

    lines
}

fn split_at_preferred_separator(chars: &[char], start: usize, hard_end: usize) -> Option<usize> {
    ((start + 2)..=hard_end)
        .rev()
        .find(|&end| is_preferred_break_char(chars[end - 1], chars.get(end).copied()))
}

fn is_preferred_break_char(ch: char, next: Option<char>) -> bool {
    matches!(ch, '/' | '.' | '_' | '-') || (ch == ':' && next != Some(':'))
}

fn split_code_display_line(line: &str, wrap_width: usize) -> Vec<ProblemDisplayLine> {
    split_code_tokens_for_display(&tokenize_rust_line(line), wrap_width)
        .into_iter()
        .map(ProblemDisplayLine::code_with_tokens)
        .collect()
}

fn split_code_tokens_for_display(tokens: &[CodeToken], wrap_width: usize) -> Vec<Vec<CodeToken>> {
    let wrap_width = wrap_width.max(MIN_PROBLEM_WRAP_WIDTH);
    let mut lines = Vec::new();
    let mut current = Vec::new();
    let mut current_width = 0usize;

    for token in tokens {
        let chars = token.text.chars().collect::<Vec<_>>();
        let mut start = 0usize;
        while start < chars.len() {
            if current_width == wrap_width {
                lines.push(current);
                current = Vec::new();
                current_width = 0;
            }

            let available = wrap_width.saturating_sub(current_width).max(1);
            let take = available.min(chars.len().saturating_sub(start));
            let segment = chars[start..start + take].iter().collect::<String>();
            current.push(CodeToken::new(token.kind.clone(), segment));
            current_width += take;
            start += take;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        vec![Vec::new()]
    } else {
        lines
    }
}

pub fn tokenize_rust_line(line: &str) -> Vec<CodeToken> {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];

        if ch.is_whitespace() {
            let start = index;
            while index < chars.len() && chars[index].is_whitespace() {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Whitespace,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if ch == '/' && chars.get(index + 1) == Some(&'/') {
            tokens.push(CodeToken::new(
                CodeTokenKind::Comment,
                chars[index..].iter().collect::<String>(),
            ));
            break;
        } else if ch == '"' {
            let start = index;
            index += 1;
            let mut escaped = false;
            while index < chars.len() {
                let current = chars[index];
                index += 1;
                if escaped {
                    escaped = false;
                } else if current == '\\' {
                    escaped = true;
                } else if current == '"' {
                    break;
                }
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::StringLiteral,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if ch.is_ascii_digit() {
            let start = index;
            index += 1;
            while index < chars.len()
                && (chars[index].is_ascii_alphanumeric() || chars[index] == '_')
            {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Number,
                chars[start..index].iter().collect::<String>(),
            ));
        } else if is_identifier_start(ch) {
            let start = index;
            index += 1;
            while index < chars.len() && is_identifier_continue(chars[index]) {
                index += 1;
            }
            let identifier: String = chars[start..index].iter().collect();
            if chars.get(index) == Some(&'!') {
                index += 1;
                tokens.push(CodeToken::new(
                    CodeTokenKind::Macro,
                    chars[start..index].iter().collect::<String>(),
                ));
            } else if is_keyword(&identifier) {
                tokens.push(CodeToken::new(CodeTokenKind::Keyword, identifier));
            } else if is_type_name(&identifier) {
                tokens.push(CodeToken::new(CodeTokenKind::Type, identifier));
            } else {
                tokens.push(CodeToken::new(CodeTokenKind::Identifier, identifier));
            }
        } else if ch.is_ascii_punctuation() {
            let start = index;
            index += 1;
            while index < chars.len()
                && chars[index].is_ascii_punctuation()
                && chars[index] != '"'
                && !(chars[index] == '/' && chars.get(index + 1) == Some(&'/'))
                && !is_identifier_start(chars[index])
            {
                index += 1;
            }
            tokens.push(CodeToken::new(
                CodeTokenKind::Punctuation,
                chars[start..index].iter().collect::<String>(),
            ));
        } else {
            tokens.push(CodeToken::new(CodeTokenKind::Plain, ch.to_string()));
            index += 1;
        }
    }

    tokens
}

fn status_lines(lines: &[String]) -> Vec<StatusLine> {
    let mut projected = Vec::new();
    let mut previous_was_expected_label = false;

    for line in lines {
        let code_tokens = if previous_was_expected_label {
            tokenize_rust_line(line)
        } else {
            Vec::new()
        };
        let tone = if previous_was_expected_label {
            StatusTone::Plain
        } else {
            status_tone(line)
        };

        projected.push(StatusLine {
            tone,
            text: line.to_string(),
            code_tokens,
        });
        previous_was_expected_label = is_expected_label(line);
    }

    projected
}

fn status_tone(line: &str) -> StatusTone {
    match line {
        "PASS" => StatusTone::Pass,
        "COMPLETE" => StatusTone::Complete,
        "TRY AGAIN" | "not quite" => StatusTone::TryAgain,
        "Expected:" | "expected:" => StatusTone::ExpectedLabel,
        "UNKNOWN COMMAND" => StatusTone::UnknownCommand,
        "SKIP" | "skipped" | "QUIT" | "STATUS" => StatusTone::Warning,
        _ => StatusTone::Plain,
    }
}

fn is_expected_label(line: &str) -> bool {
    matches!(line, "Expected:" | "expected:")
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

fn is_keyword(identifier: &str) -> bool {
    matches!(
        identifier,
        "fn" | "let"
            | "mut"
            | "if"
            | "else"
            | "while"
            | "for"
            | "loop"
            | "match"
            | "return"
            | "true"
            | "false"
    )
}

fn is_type_name(identifier: &str) -> bool {
    matches!(
        identifier,
        "i32" | "u32" | "usize" | "bool" | "String" | "str" | "char" | "Option" | "Vec" | "Result"
    )
}

fn help_lines(open: bool) -> Vec<String> {
    let _ = open;
    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{
        ClientAction, ClientUiState, CodeTokenKind, CursorPosition, FAILURE_PULSE_DURATION,
        PROBLEM_SCROLL_LINES, PROBLEM_WHEEL_SCROLL_LINES, ProblemDisplayLine, ProblemLineKind,
        ProblemView, ProblemViewport, REWARD_ATTACK_DURATION, REWARD_DURATION,
        REWARD_GAME_ATTACK_DURATION, REWARD_GAME_BODY_DURATION, REWARD_GAME_GLOW_DELAY,
        REWARD_GAME_GLOW_MAX_INTENSITY, REWARD_GAME_RELEASE_DURATION,
        REWARD_PLUS_ONE_BODY_DURATION, REWARD_PLUS_ONE_DURATION, REWARD_RELEASE_DURATION,
        REWARD_SWEEP_DURATION, SKIP_TOAST_DURATION, StatusTone, project_client_frame,
        project_client_frame_at, project_client_frame_for_layout, tokenize_rust_line,
    };
    use crate::engine;

    fn token_text(tokens: &[super::CodeToken]) -> String {
        tokens
            .iter()
            .map(|token| token.text.as_str())
            .collect::<String>()
    }

    fn token_kinds_and_text(tokens: &[super::CodeToken]) -> Vec<(CodeTokenKind, String)> {
        tokens
            .iter()
            .map(|token| (token.kind.clone(), token.text.clone()))
            .collect()
    }

    fn test_now() -> Instant {
        Instant::now()
    }

    #[test]
    fn tokenizer_highlights_main_function_line() {
        let tokens = tokenize_rust_line("fn main() {}");

        assert_eq!(token_text(&tokens), "fn main() {}");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword && token.text == "fn")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Identifier && token.text == "main")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Punctuation && token.text.contains('{'))
        );
    }

    #[test]
    fn tokenizer_highlights_let_type_and_number() {
        let tokens = tokenize_rust_line("let score: i32 = 0;");

        assert_eq!(token_text(&tokens), "let score: i32 = 0;");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword && token.text == "let")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "i32")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "0")
        );
    }

    #[test]
    fn tokenizer_highlights_mut_binding_array_and_control_flow_lines() {
        for (line, expected_tokens) in [
            (
                "let mut score = 0;",
                vec![
                    (CodeTokenKind::Keyword, "let"),
                    (CodeTokenKind::Keyword, "mut"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "0"),
                ],
            ),
            (
                "let scores = [1, 2, 3];",
                vec![
                    (CodeTokenKind::Keyword, "let"),
                    (CodeTokenKind::Identifier, "scores"),
                    (CodeTokenKind::Number, "1"),
                    (CodeTokenKind::Number, "2"),
                    (CodeTokenKind::Number, "3"),
                ],
            ),
            (
                "if score > 0 {}",
                vec![
                    (CodeTokenKind::Keyword, "if"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "0"),
                ],
            ),
            (
                "while score < 10 {}",
                vec![
                    (CodeTokenKind::Keyword, "while"),
                    (CodeTokenKind::Identifier, "score"),
                    (CodeTokenKind::Number, "10"),
                ],
            ),
        ] {
            let tokens = tokenize_rust_line(line);
            assert_eq!(token_text(&tokens), line);
            for (kind, text) in expected_tokens {
                assert!(
                    tokens
                        .iter()
                        .any(|token| token.kind == kind && token.text == text),
                    "expected token {kind:?} {text:?} in {line:?}"
                );
            }
        }
    }

    #[test]
    fn tokenizer_highlights_macro_and_string_literal() {
        let tokens = tokenize_rust_line("println!(\"Rust Reps\");");

        assert_eq!(token_text(&tokens), "println!(\"Rust Reps\");");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Macro && token.text == "println!")
        );
        assert!(tokens.iter().any(
            |token| token.kind == CodeTokenKind::StringLiteral && token.text == "\"Rust Reps\""
        ));
    }

    #[test]
    fn tokenizer_highlights_comment_references_and_generic_type_names() {
        let comment_tokens = tokenize_rust_line("// comment");
        assert_eq!(token_text(&comment_tokens), "// comment");
        assert_eq!(comment_tokens[0].kind, CodeTokenKind::Comment);

        let reference_tokens = tokenize_rust_line("&str");
        assert_eq!(token_text(&reference_tokens), "&str");
        assert!(
            reference_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Punctuation && token.text == "&")
        );
        assert!(
            reference_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "str")
        );

        let generic_tokens = tokenize_rust_line("Vec<Option<Result<String, i32>>>");
        assert_eq!(
            token_text(&generic_tokens),
            "Vec<Option<Result<String, i32>>>"
        );
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "Vec")
        );
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "Option")
        );
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "Result")
        );
        assert!(
            generic_tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "String")
        );
    }

    #[test]
    fn tokenizer_preserves_exact_text_when_concatenated() {
        for line in [
            "fn main() {}",
            "let score: i32 = 0;",
            "let mut score = 0;",
            "println!(\"Rust Reps\");",
            "let scores = [1, 2, 3];",
            "if score > 0 {}",
            "while score < 10 {}",
            "// note",
            "&str",
            "Option<String>",
            "Vec<Option<Result<String, i32>>>",
        ] {
            assert_eq!(token_text(&tokenize_rust_line(line)), line);
        }
    }

    #[test]
    fn tokenizer_handles_unknown_identifiers_safely() {
        let tokens = tokenize_rust_line("mystery_value @");

        assert_eq!(token_text(&tokens), "mystery_value @");
        assert_eq!(
            token_kinds_and_text(&tokens)[0],
            (CodeTokenKind::Identifier, "mystery_value".to_string())
        );
    }

    #[test]
    fn frame_projection_includes_problem_prompt() {
        let session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.game_window.problem.instruction, "do this:");
        assert_eq!(frame.game_window.problem.prompt, "fn main() {}");
        assert_eq!(
            token_text(&frame.game_window.problem.prompt_tokens),
            "fn main() {}"
        );
        assert_eq!(frame.game_window.problem.viewport.scroll_offset, 0);
        assert_eq!(
            frame
                .game_window
                .problem
                .lines
                .iter()
                .map(|line| (&line.kind, line.text.as_str()))
                .collect::<Vec<_>>(),
            vec![
                (&ProblemLineKind::Blank, ""),
                (&ProblemLineKind::Header, "do this:"),
                (&ProblemLineKind::Blank, ""),
                (&ProblemLineKind::Code, "fn main() {}"),
            ]
        );
    }

    #[test]
    fn problem_viewport_starts_at_offset_zero() {
        let ui_state = ClientUiState::new();

        assert_eq!(ui_state.problem_viewport().scroll_offset, 0);
    }

    #[test]
    fn problem_viewport_scrolls_and_clamps_safely() {
        let mut viewport = ProblemViewport::new();

        viewport.scroll_up();
        assert_eq!(viewport.scroll_offset, 0);

        viewport.scroll_down();
        assert_eq!(viewport.scroll_offset, PROBLEM_SCROLL_LINES);

        assert_eq!(ProblemViewport::with_offset(99).clamped(4).scroll_offset, 3);
        assert_eq!(
            ProblemViewport::with_offset(99).visible_range(6, 2),
            super::ProblemVisibleRange { start: 4, end: 6 }
        );
    }

    #[test]
    fn problem_line_scroll_moves_by_one_visual_line() {
        let mut viewport = ProblemViewport::new();

        viewport.scroll_line_up();
        assert_eq!(viewport.scroll_offset, 0);

        viewport.scroll_line_down();
        assert_eq!(viewport.scroll_offset, PROBLEM_WHEEL_SCROLL_LINES);

        viewport.scroll_line_down();
        assert_eq!(viewport.scroll_offset, PROBLEM_WHEEL_SCROLL_LINES * 2);

        viewport.scroll_up();
        assert_eq!(viewport.scroll_offset, 0);
    }

    #[test]
    fn visible_problem_lines_and_overflow_are_derived_from_viewport() {
        let problem = ProblemView {
            instruction: "do this:".to_string(),
            prompt: "line 1".to_string(),
            prompt_tokens: Vec::new(),
            lines: (1..=8)
                .map(|index| ProblemDisplayLine::plain(format!("line {index}")))
                .collect(),
            viewport: ProblemViewport::with_offset(4),
        };

        let visible = problem
            .visible_lines(3)
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(visible, vec!["line 5", "line 6", "line 7"]);
        assert_eq!(
            problem.overflow(3),
            super::ViewportOverflow {
                above: true,
                below: true,
                offset: 4,
                visible: 3,
                total: 8,
            }
        );
        let fitting_problem = ProblemView {
            viewport: ProblemViewport::new(),
            lines: vec![
                ProblemDisplayLine::plain("line 1"),
                ProblemDisplayLine::plain("line 2"),
            ],
            ..problem
        };
        assert_eq!(
            fitting_problem.overflow(3),
            super::ViewportOverflow {
                above: false,
                below: false,
                offset: 0,
                visible: 3,
                total: 2,
            }
        );
    }

    #[test]
    fn lab_long_problem_overflows_and_exposes_viewport_state() {
        let session = engine::Session::lab();
        let ui_state = ClientUiState::new();
        let frame = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        let problem = &frame.game_window.problem;

        assert!(problem.prompt.contains("lab case 1"));
        assert!(problem.lines.len() > 20);
        assert_eq!(problem.viewport.scroll_offset, 0);
        let visible = problem
            .visible_lines(5)
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>();
        assert_eq!(&visible[..2], &["", "task:"]);
        assert!(visible[2].starts_with("Build the smallest Rust entry point"));
        assert_eq!(
            problem.overflow(5),
            super::ViewportOverflow {
                above: false,
                below: true,
                offset: 0,
                visible: 5,
                total: problem.lines.len(),
            }
        );
    }

    #[test]
    fn editor_overflow_projection_tracks_visible_buffer_slice() {
        let mut ui_state = ClientUiState::new();
        ui_state.set_editor_viewport_height(3);
        ui_state.editor.set_text("one\ntwo\nthree\nfour\nfive");
        for _ in 0..4 {
            ui_state.apply_action(ClientAction::MoveCursorDown);
        }

        let session = engine::Session::new();
        let frame = project_client_frame(&session, &ui_state);
        let workbox = &frame.game_window.workbox;

        assert_eq!(
            workbox.overflow(3),
            super::ViewportOverflow {
                above: true,
                below: false,
                offset: 2,
                visible: 3,
                total: 5,
            }
        );
        assert_eq!(workbox.lines.join("\n"), "one\ntwo\nthree\nfour\nfive");
    }

    #[test]
    fn editor_scroll_actions_move_viewport_without_changing_buffer_or_cursor() {
        let mut ui_state = ClientUiState::new();
        ui_state.set_editor_viewport_height(3);
        ui_state.editor.set_text("one\ntwo\nthree\nfour\nfive");
        ui_state
            .editor
            .set_cursor(CursorPosition { line: 2, column: 0 });
        let before = ui_state.editor_view();

        ui_state.apply_action(ClientAction::ScrollEditorUp);
        let scrolled = ui_state.editor_view();

        assert_eq!(scrolled.viewport.first_line, 1);
        assert_eq!(scrolled.text, before.text);
        assert_eq!(scrolled.cursor, before.cursor);
        assert_eq!(scrolled.visible_lines(), &["two", "three", "four"]);

        ui_state.apply_action(ClientAction::ScrollEditorDown);
        let returned = ui_state.editor_view();
        assert_eq!(returned.viewport.first_line, 2);
        assert_eq!(returned.text, before.text);
        assert_eq!(returned.cursor, before.cursor);
    }

    #[test]
    fn editor_scroll_overscroll_clamps_and_recovers() {
        let mut ui_state = ClientUiState::new();
        ui_state.set_editor_viewport_height(2);
        ui_state.editor.set_text("one\ntwo\nthree\nfour");
        ui_state
            .editor
            .set_cursor(CursorPosition { line: 1, column: 0 });

        for _ in 0..10 {
            ui_state.apply_action(ClientAction::ScrollEditorDown);
        }
        assert_eq!(ui_state.editor_view().viewport.first_line, 2);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 1, column: 0 }
        );

        ui_state.apply_action(ClientAction::ScrollEditorUp);
        assert_eq!(ui_state.editor_view().viewport.first_line, 1);
        ui_state.apply_action(ClientAction::ScrollEditorUp);
        ui_state.apply_action(ClientAction::ScrollEditorUp);
        assert_eq!(ui_state.editor_view().viewport.first_line, 0);
        assert_eq!(ui_state.editor_view().text, "one\ntwo\nthree\nfour");
    }

    #[test]
    fn typing_after_editor_viewport_scrolls_away_from_cursor_brings_cursor_back() {
        let mut ui_state = ClientUiState::new();
        ui_state.set_editor_viewport_height(2);
        ui_state.editor.set_text("one\ntwo\nthree\nfour");
        ui_state
            .editor
            .set_cursor(CursorPosition { line: 0, column: 3 });
        ui_state.apply_action(ClientAction::ScrollEditorDown);
        ui_state.apply_action(ClientAction::ScrollEditorDown);
        assert_eq!(ui_state.editor_view().viewport.first_line, 2);

        ui_state.apply_action(ClientAction::TypeChar('!'));

        assert_eq!(ui_state.editor_view().text, "one!\ntwo\nthree\nfour");
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 4 }
        );
        assert_eq!(ui_state.editor_view().viewport.first_line, 0);
    }

    #[test]
    fn reward_anchor_is_captured_before_editor_clear() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }

        ui_state.start_reward_event(now);
        ui_state.clear_draft_input();

        let frame = ui_state.reward_frame_at(now + REWARD_ATTACK_DURATION);
        assert_eq!(ui_state.editor_view().text, "");
        assert_eq!(
            frame.anchor,
            Some(super::EditorAnchor { row: 0, column: 12 })
        );
        assert!(frame.show_plus_one);
    }

    #[test]
    fn skip_toast_uses_warning_timing_and_editor_anchor() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        ui_state.apply_action(ClientAction::TypeChar('x'));

        ui_state.start_skip_toast(now);

        let frame = ui_state.skip_frame_at(now);
        assert!(frame.active);
        assert_eq!(frame.intensity, 100);
        assert_eq!(
            frame.anchor,
            Some(super::EditorAnchor { row: 0, column: 1 })
        );
        assert!(!ui_state.skip_active_at(now + SKIP_TOAST_DURATION));
    }

    #[test]
    fn page_down_and_page_up_scroll_lab_problem_without_editor_changes() {
        let session = engine::Session::lab();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        let before_editor = ui_state.editor_view();

        ui_state.apply_action(ClientAction::ScrollProblemDown);
        let scrolled = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        assert_eq!(
            scrolled.game_window.problem.viewport.scroll_offset,
            PROBLEM_SCROLL_LINES
        );
        let top_visible =
            project_client_frame_for_layout(&session, &ClientUiState::new(), test_now(), 48)
                .game_window
                .problem
                .visible_lines(5)
                .iter()
                .map(|line| line.text.clone())
                .collect::<Vec<_>>();
        let scrolled_visible = scrolled
            .game_window
            .problem
            .visible_lines(5)
            .iter()
            .map(|line| line.text.clone())
            .collect::<Vec<_>>();
        assert_ne!(scrolled_visible, top_visible);
        assert_eq!(ui_state.editor_view(), before_editor);

        ui_state.apply_action(ClientAction::ScrollProblemUp);
        let returned = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        assert_eq!(returned.game_window.problem.viewport.scroll_offset, 0);
        assert_eq!(ui_state.editor_view(), before_editor);
    }

    #[test]
    fn wheel_problem_scroll_uses_small_repeated_deltas_without_editor_changes() {
        let session = engine::Session::lab();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        let before_editor = ui_state.editor_view();

        ui_state.apply_action(ClientAction::ScrollProblemLineDown);
        let once = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        assert_eq!(
            once.game_window.problem.viewport.scroll_offset,
            PROBLEM_WHEEL_SCROLL_LINES
        );

        ui_state.apply_action(ClientAction::ScrollProblemLineDown);
        let twice = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        assert_eq!(
            twice.game_window.problem.viewport.scroll_offset,
            PROBLEM_WHEEL_SCROLL_LINES * 2
        );
        assert!(twice.game_window.problem.viewport.scroll_offset < PROBLEM_SCROLL_LINES);
        assert_eq!(ui_state.editor_view(), before_editor);

        ui_state.apply_action(ClientAction::ScrollProblemLineUp);
        let returned = project_client_frame_for_layout(&session, &ui_state, test_now(), 48);
        assert_eq!(
            returned.game_window.problem.viewport.scroll_offset,
            PROBLEM_WHEEL_SCROLL_LINES
        );
    }

    #[test]
    fn long_problem_text_wraps_before_viewport_slicing() {
        let session = engine::Session::lab();
        let ui_state = ClientUiState::new();

        let wide = project_client_frame_for_layout(&session, &ui_state, test_now(), 90);
        let narrow = project_client_frame_for_layout(&session, &ui_state, test_now(), 24);

        assert!(narrow.game_window.problem.lines.len() > wide.game_window.problem.lines.len());
        assert!(
            narrow
                .game_window
                .problem
                .lines
                .iter()
                .filter(|line| !line.text.is_empty())
                .all(|line| line.text.chars().count() <= 24)
        );
    }

    #[test]
    fn problem_prose_wrapping_is_deterministic_and_word_aware() {
        let line = "The problem panel should wrap polished prose into stable display lines without changing the stored prompt.";
        let wrapped = super::split_display_line(line, 28);

        assert_eq!(
            wrapped,
            vec![
                "The problem panel should".to_string(),
                "wrap polished prose into".to_string(),
                "stable display lines without".to_string(),
                "changing the stored prompt.".to_string(),
            ]
        );
        assert!(wrapped.iter().all(|line| line.chars().count() <= 28));
    }

    #[test]
    fn problem_long_unbroken_token_splits_inside_pane_width() {
        let line = "crate::very::long::module::path::that::must::stay::inside::the::pane";
        let wrapped = super::split_display_line(line, 18);

        assert!(wrapped.len() > 1);
        assert!(wrapped.iter().all(|line| line.chars().count() <= 18));
        assert_eq!(wrapped.join(""), line);
    }

    #[test]
    fn problem_standalone_paths_split_at_stable_separators_without_code_highlight() {
        let line = "crate::stress::very::long::module";
        let wrapped = super::split_display_line(line, 16);
        let display_lines = super::problem_display_lines_for_source_line(line, 16);

        assert_eq!(
            wrapped,
            vec![
                "crate::stress::".to_string(),
                "very::long::".to_string(),
                "module".to_string(),
            ]
        );
        assert!(wrapped.iter().all(|line| line.chars().count() <= 16));
        assert_eq!(wrapped.join(""), line);
        assert!(
            display_lines
                .iter()
                .all(|line| line.kind == ProblemLineKind::Plain)
        );
        assert!(display_lines.iter().all(|line| line.tokens.is_empty()));
    }

    #[test]
    fn prompt_that_already_has_task_label_omits_duplicate_instruction_header() {
        let lines = super::problem_display_lines("do this:", "task:\nCopy this.", 40);
        let nonblank = lines
            .iter()
            .filter(|line| !line.text.is_empty())
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(nonblank, vec!["task:", "Copy this."]);
        assert!(!lines.iter().any(|line| line.text == "do this:"));
    }

    #[test]
    fn simple_one_line_task_keeps_minimal_instruction_header() {
        let lines = super::problem_display_lines("do this:", "fn main() {}", 40);
        let visible = lines
            .iter()
            .map(|line| (&line.kind, line.text.as_str()))
            .collect::<Vec<_>>();

        assert_eq!(
            visible,
            vec![
                (&ProblemLineKind::Blank, ""),
                (&ProblemLineKind::Header, "do this:"),
                (&ProblemLineKind::Blank, ""),
                (&ProblemLineKind::Code, "fn main() {}"),
            ]
        );
    }

    #[test]
    fn problem_code_wrapping_preserves_spacing_without_changing_answer_buffer() {
        let code = "fn main() {    println!(\"Rust Reps\"); }";
        let wrapped = super::split_display_line_for_kind(code, 12, ProblemLineKind::Code);

        assert!(wrapped.len() > 1);
        assert_eq!(wrapped.join(""), code);

        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        let before = ui_state.editor_view().text;
        let _ = project_client_frame_for_layout(&engine::Session::lab(), &ui_state, test_now(), 12);
        assert_eq!(ui_state.editor_view().text, before);
    }

    #[test]
    fn problem_bullet_continuations_align_deterministically() {
        let line = "- keep this bullet readable when the problem pane is narrow";
        let wrapped = super::split_display_line_for_kind(line, 24, ProblemLineKind::Bullet);

        assert_eq!(
            wrapped,
            vec![
                "- keep this bullet".to_string(),
                "  readable when the".to_string(),
                "  problem pane is narrow".to_string(),
            ]
        );
        assert!(wrapped.iter().all(|line| line.chars().count() <= 24));
    }

    #[test]
    fn problem_answer_shape_header_is_distinct_without_mutating_expected_answer() {
        let expected = "fn main() {}";
        let lines = super::problem_display_lines("do this:", "answer shape:\nfn main() {}", 24);

        assert!(
            lines
                .iter()
                .any(|line| line.kind == ProblemLineKind::AnswerShape
                    && line.text == "answer shape:")
        );
        assert_eq!(expected, "fn main() {}");
    }

    #[test]
    fn task_code_fragment_uses_same_tokens_as_editor_code() {
        let line = "score += 1;";
        let lines = super::problem_display_lines_for_source_line(line, 80);
        let editor_tokens = tokenize_rust_line(line);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].kind, ProblemLineKind::Code);
        assert_eq!(lines[0].tokens, editor_tokens);
        assert!(
            lines[0]
                .tokens
                .iter()
                .any(|token| { token.kind == CodeTokenKind::Punctuation && token.text == "+=" })
        );
        assert!(
            lines[0]
                .tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "1")
        );
    }

    #[test]
    fn task_let_statement_gets_keyword_and_number_tokens() {
        let line = "let answer = 42;";
        let lines = super::problem_display_lines_for_source_line(line, 80);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].kind, ProblemLineKind::Code);
        assert!(
            lines[0]
                .tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Keyword && token.text == "let")
        );
        assert!(
            lines[0]
                .tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "42")
        );
    }

    #[test]
    fn answer_shape_snippet_is_code_styled() {
        let lines = super::problem_display_lines("do this:", "answer shape:\nscore += 1;", 24);
        let snippet = lines
            .iter()
            .find(|line| line.text == "score += 1;")
            .expect("expected answer-shape code snippet");

        assert_eq!(snippet.kind, ProblemLineKind::Code);
        assert!(
            snippet
                .tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "1")
        );
    }

    #[test]
    fn problem_prose_with_rust_words_is_not_syntax_highlighted() {
        let line = "This prose mentions fn let i32 and match without being code.";
        let lines = super::problem_display_lines_for_source_line(line, 80);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].kind, ProblemLineKind::Plain);
        assert!(lines[0].tokens.is_empty());
    }

    #[test]
    fn wrapped_code_slices_full_line_tokens_instead_of_retokenizing_fragments() {
        let line = "let score: i32 = 1000;";
        let wrapped = super::split_code_display_line(line, 12);

        assert_eq!(
            wrapped
                .iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>(),
            vec!["let score: i", "32 = 1000;"]
        );
        assert_eq!(
            wrapped
                .iter()
                .map(|line| line.text.as_str())
                .collect::<String>(),
            line
        );
        assert!(
            wrapped[1]
                .tokens
                .iter()
                .filter(|token| !token.text.trim().is_empty())
                .any(|token| token.kind == CodeTokenKind::Type && token.text == "32")
        );
        assert!(
            !wrapped[1]
                .tokens
                .iter()
                .any(|token| token.kind == CodeTokenKind::Number && token.text == "32")
        );
    }

    #[test]
    fn problem_viewport_offset_is_capped_to_valid_bounds_after_wrapping() {
        let session = engine::Session::lab();
        let mut ui_state = ClientUiState::new();
        for _ in 0..200 {
            ui_state.apply_action(ClientAction::ScrollProblemDown);
        }

        let frame = project_client_frame_for_layout(&session, &ui_state, test_now(), 36);
        let problem = &frame.game_window.problem;
        let range = problem.visible_range(5);

        assert!(range.end <= problem.lines.len());
        assert_eq!(range.end.saturating_sub(range.start), 5);
        assert_eq!(range.end, problem.lines.len());
    }

    #[test]
    fn problem_scroll_actions_do_not_affect_editor_buffer_or_cursor() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        let before = ui_state.editor_view();

        ui_state.apply_action(ClientAction::ScrollProblemDown);
        ui_state.apply_action(ClientAction::ScrollProblemUp);

        assert_eq!(ui_state.editor_view(), before);
    }

    #[test]
    fn initial_client_frame_counter_is_zero() {
        let session = engine::Session::new();
        let ui_state = ClientUiState::new();
        let now = test_now();

        let frame = project_client_frame_at(&session, &ui_state, now);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.reward.plus_one_intensity, 0);
        assert_eq!(frame.reward.game_glow_intensity, 0);
        assert!(!frame.reward.show_plus_one);
        assert!(!frame.reward_active);
        assert_eq!(frame.failure.intensity, 0);
        assert!(!frame.failure_active);
    }

    #[test]
    fn reward_timing_uses_shorter_middle_heavy_boop() {
        assert_eq!(REWARD_ATTACK_DURATION, Duration::from_millis(100));
        assert_eq!(REWARD_PLUS_ONE_BODY_DURATION, Duration::from_millis(450));
        assert_eq!(REWARD_RELEASE_DURATION, Duration::from_millis(550));
        assert_eq!(REWARD_PLUS_ONE_DURATION, Duration::from_millis(1_100));
        assert_eq!(REWARD_DURATION, Duration::from_millis(2_250));
        assert_eq!(REWARD_GAME_ATTACK_DURATION, Duration::from_millis(150));
        assert_eq!(REWARD_GAME_BODY_DURATION, Duration::from_millis(1_050));
        assert_eq!(REWARD_GAME_RELEASE_DURATION, Duration::from_millis(1_050));
        assert_eq!(REWARD_SWEEP_DURATION, Duration::from_millis(850));
        assert_eq!(REWARD_GAME_GLOW_DELAY, Duration::from_millis(80));
        assert_eq!(REWARD_GAME_GLOW_MAX_INTENSITY, 58);
        assert!(REWARD_PLUS_ONE_DURATION <= Duration::from_millis(1_150));
        assert_eq!(REWARD_PLUS_ONE_DURATION * 2, Duration::from_millis(2_200));
        assert!(REWARD_DURATION >= Duration::from_millis(2_200));
        assert!(REWARD_RELEASE_DURATION > REWARD_ATTACK_DURATION);
        assert!(REWARD_PLUS_ONE_BODY_DURATION > REWARD_ATTACK_DURATION);
        assert!(REWARD_PLUS_ONE_DURATION < REWARD_DURATION);
        assert!(REWARD_SWEEP_DURATION > REWARD_ATTACK_DURATION);
        assert!(REWARD_SWEEP_DURATION < REWARD_DURATION);
        assert!(REWARD_GAME_GLOW_MAX_INTENSITY < 100);
        assert_eq!(FAILURE_PULSE_DURATION, Duration::from_millis(520));
        assert_eq!(SKIP_TOAST_DURATION, Duration::from_millis(900));
    }

    #[test]
    fn plus_one_boop_reaches_readable_strength_quickly() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        assert!(!ui_state.reward_active_at(now));
        ui_state.start_reward_event(now);

        let start = ui_state.reward_frame_at(now);
        let quick_hit = ui_state.reward_frame_at(now + REWARD_ATTACK_DURATION / 2);
        let attack_end = ui_state.reward_frame_at(now + REWARD_ATTACK_DURATION);

        assert!(start.active);
        assert_eq!(start.plus_one_intensity, 0);
        assert_eq!(start.game_glow_intensity, 0);
        assert!(start.show_plus_one);
        assert!(quick_hit.active);
        assert!(quick_hit.plus_one_intensity >= 80);
        assert_eq!(quick_hit.game_glow_intensity, 0);
        assert!(quick_hit.sweep_progress > start.sweep_progress);
        assert!(quick_hit.show_plus_one);
        assert!(attack_end.active);
        assert_eq!(attack_end.plus_one_intensity, 100);
        assert!(attack_end.game_glow_intensity > 0);
        assert!(attack_end.plus_one_intensity > attack_end.game_glow_intensity);
        assert!(attack_end.sweep_progress > quick_hit.sweep_progress);
        assert!(attack_end.show_plus_one);
    }

    #[test]
    fn reward_sweep_progress_increases_over_time() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        ui_state.start_reward_event(now);

        let start = ui_state.reward_frame_at(now);
        let middle = ui_state.reward_frame_at(now + REWARD_SWEEP_DURATION / 2);
        let end = ui_state.reward_frame_at(now + REWARD_SWEEP_DURATION);
        let later =
            ui_state.reward_frame_at(now + REWARD_SWEEP_DURATION + Duration::from_millis(300));

        assert_eq!(start.sweep_progress, 0);
        assert!(middle.sweep_progress > start.sweep_progress);
        assert_eq!(end.sweep_progress, 100);
        assert_eq!(later.sweep_progress, 100);
    }

    #[test]
    fn game_glow_lags_plus_one_and_stays_secondary() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        ui_state.start_reward_event(now);

        let before_glow = ui_state.reward_frame_at(now + REWARD_GAME_GLOW_DELAY / 2);
        let glow_peak =
            ui_state.reward_frame_at(now + REWARD_GAME_ATTACK_DURATION + REWARD_GAME_GLOW_DELAY);

        assert!(before_glow.plus_one_intensity > 0);
        assert_eq!(before_glow.game_glow_intensity, 0);
        assert_eq!(glow_peak.plus_one_intensity, 100);
        assert!(glow_peak.game_glow_intensity > 0);
        assert_eq!(
            glow_peak.game_glow_intensity,
            REWARD_GAME_GLOW_MAX_INTENSITY
        );
        assert!(glow_peak.plus_one_intensity > glow_peak.game_glow_intensity);
    }

    #[test]
    fn plus_one_boop_holds_body_then_releases_cleanly() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();
        ui_state.start_reward_event(now);

        let hold_mid = ui_state
            .reward_frame_at(now + REWARD_ATTACK_DURATION + (REWARD_PLUS_ONE_BODY_DURATION / 2));
        let release_starts_at = REWARD_ATTACK_DURATION + REWARD_PLUS_ONE_BODY_DURATION;
        let early_release =
            ui_state.reward_frame_at(now + release_starts_at + Duration::from_millis(150));
        let late_release =
            ui_state.reward_frame_at(now + release_starts_at + Duration::from_millis(450));

        assert_eq!(hold_mid.plus_one_intensity, 100);
        assert!(hold_mid.plus_one_intensity > hold_mid.game_glow_intensity);
        assert!(hold_mid.show_plus_one);
        assert!(REWARD_RELEASE_DURATION > REWARD_ATTACK_DURATION);
        assert!(early_release.active);
        assert!(early_release.show_plus_one);
        assert!(late_release.active);
        assert!(late_release.show_plus_one);
        assert!(early_release.plus_one_intensity < hold_mid.plus_one_intensity);
        assert!(early_release.plus_one_intensity > late_release.plus_one_intensity);
        assert!(late_release.plus_one_intensity > 0);
        assert!(early_release.plus_one_intensity > early_release.game_glow_intensity);

        let expired_plus_one = ui_state.reward_frame_at(now + REWARD_PLUS_ONE_DURATION);
        assert!(expired_plus_one.active);
        assert_eq!(expired_plus_one.plus_one_intensity, 0);
        assert!(!expired_plus_one.show_plus_one);
        assert!(expired_plus_one.game_glow_intensity > 0);
    }

    #[test]
    fn reward_event_expires_after_duration() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        ui_state.start_reward_event(now);
        assert!(ui_state.reward_active_at(now + Duration::from_millis(1)));
        assert!(!ui_state.reward_active_at(now + REWARD_DURATION));
        assert!(
            !ui_state
                .reward_frame_at(now + REWARD_DURATION)
                .show_plus_one
        );

        ui_state.settle_reward_event(now + REWARD_DURATION);
        assert!(!ui_state.reward_active_at(now + REWARD_DURATION));
    }

    #[test]
    fn reward_event_can_be_cleared_without_session_truth() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        ui_state.start_reward_event(now);
        assert!(ui_state.reward_active_at(now));

        ui_state.clear_reward_event();
        assert!(!ui_state.reward_active_at(now));
    }

    #[test]
    fn failure_pulse_starts_short_and_expires() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        assert_eq!(FAILURE_PULSE_DURATION, Duration::from_millis(520));
        assert!(FAILURE_PULSE_DURATION >= Duration::from_millis(350));
        assert!(FAILURE_PULSE_DURATION <= Duration::from_millis(700));
        assert!(FAILURE_PULSE_DURATION < REWARD_DURATION);
        assert!(!ui_state.failure_active_at(now));

        ui_state.start_failure_pulse(now);
        let start = ui_state.failure_frame_at(now);
        let mid = ui_state.failure_frame_at(now + FAILURE_PULSE_DURATION / 2);
        let expired = ui_state.failure_frame_at(now + FAILURE_PULSE_DURATION);

        assert!(start.active);
        assert_eq!(start.intensity, 100);
        assert!(mid.active);
        assert!(mid.intensity < start.intensity);
        assert!(mid.intensity > 0);
        assert!(!expired.active);
        assert_eq!(expired.intensity, 0);

        ui_state.settle_failure_pulse(now + FAILURE_PULSE_DURATION);
        assert!(!ui_state.failure_active_at(now + FAILURE_PULSE_DURATION));
    }

    #[test]
    fn failure_pulse_can_be_cleared_without_session_truth() {
        let mut ui_state = ClientUiState::new();
        let now = test_now();

        ui_state.start_failure_pulse(now);
        assert!(ui_state.failure_active_at(now));

        ui_state.clear_failure_pulse();
        assert!(!ui_state.failure_active_at(now));
    }

    #[test]
    fn correct_answer_increments_counter_to_one() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        assert!(matches!(
            session.submit_answer("fn main() {}"),
            engine::SubmitResult::Correct
        ));
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 1);
        assert_eq!(frame.game_window.problem.prompt, "println!(\"Rust Reps\");");
    }

    #[test]
    fn multiple_correct_answers_increment_counter_one_two_three() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        for (answer, expected_counter) in [
            ("fn main() {}", 1),
            ("println!(\"Rust Reps\");", 2),
            ("let answer = 42;", 3),
        ] {
            assert!(matches!(
                session.submit_answer(answer),
                engine::SubmitResult::Correct
            ));
            let frame = project_client_frame(&session, &ui_state);
            assert_eq!(frame.completed_reps_this_run, expected_counter);
        }
    }

    #[test]
    fn wrong_answer_does_not_increment_counter() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        assert!(matches!(
            session.submit_answer("wrong"),
            engine::SubmitResult::TryAgain
        ));
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
    }

    #[test]
    fn skip_does_not_increment_counter() {
        let mut session = engine::Session::new();
        let ui_state = ClientUiState::new();

        let _ = session.skip();
        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.game_window.problem.prompt, "println!(\"Rust Reps\");");
    }

    #[test]
    fn new_session_starts_counter_at_zero() {
        let mut completed_session = engine::Session::new();
        assert!(matches!(
            completed_session.submit_answer("fn main() {}"),
            engine::SubmitResult::Correct
        ));

        let fresh_session = engine::Session::new();
        let ui_state = ClientUiState::new();
        let now = test_now();
        let frame = project_client_frame_at(&fresh_session, &ui_state, now);

        assert_eq!(frame.completed_reps_this_run, 0);
        assert_eq!(frame.reward.plus_one_intensity, 0);
        assert_eq!(frame.reward.game_glow_intensity, 0);
        assert!(!frame.reward.show_plus_one);
        assert!(!frame.reward_active);
    }

    #[test]
    fn frame_projection_includes_workbox_input_and_cursor_state() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('y'));
        ui_state.apply_action(ClientAction::CursorTick);

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(frame.game_window.workbox.input, "x\ny");
        assert_eq!(frame.game_window.workbox.lines, vec!["x", "y"]);
        assert_eq!(
            frame
                .game_window
                .workbox
                .line_tokens
                .iter()
                .map(|tokens| token_text(tokens))
                .collect::<Vec<_>>(),
            vec!["x".to_string(), "y".to_string()]
        );
        assert!(!frame.game_window.workbox.cursor_visible);
    }

    #[test]
    fn frame_projection_keeps_status_result_outside_game_window() {
        let session = engine::Session::new();
        let mut ui_state = ClientUiState::new();
        ui_state.set_status_lines(vec![
            "not quite".to_string(),
            "expected:".to_string(),
            "fn main() {}".to_string(),
        ]);

        let frame = project_client_frame(&session, &ui_state);

        assert_eq!(
            frame
                .status_chrome
                .lines
                .iter()
                .map(|line| (
                    &line.tone,
                    line.text.as_str(),
                    token_text(&line.code_tokens)
                ))
                .collect::<Vec<_>>(),
            vec![
                (&StatusTone::TryAgain, "not quite", String::new()),
                (&StatusTone::ExpectedLabel, "expected:", String::new()),
                (
                    &StatusTone::Plain,
                    "fn main() {}",
                    "fn main() {}".to_string()
                ),
            ]
        );
        assert_eq!(frame.game_window.problem.prompt, "fn main() {}");
        assert_eq!(frame.game_window.workbox.input, "");
    }

    #[test]
    fn ui_state_handles_type_backspace_and_cursor_tick() {
        let mut ui_state = ClientUiState::new();

        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('b'));
        ui_state.apply_action(ClientAction::Backspace);
        ui_state.apply_action(ClientAction::CursorTick);

        assert_eq!(ui_state.editor_view().text, "a\n");
        assert!(!ui_state.cursor_visible());
    }

    #[test]
    fn client_action_insert_newline_updates_editor_buffer() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::InsertNewline);
        ui_state.apply_action(ClientAction::TypeChar('b'));

        let view = ui_state.editor_view();
        assert_eq!(view.text, "a\nb");
        assert_eq!(view.lines, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(view.cursor, CursorPosition { line: 1, column: 1 });
    }

    #[test]
    fn client_action_move_left_and_right_change_editor_cursor() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }

        ui_state.apply_action(ClientAction::MoveCursorLeft);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 2 }
        );
        ui_state.apply_action(ClientAction::MoveCursorRight);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 3 }
        );
    }

    #[test]
    fn printable_char_inserts_at_cursor() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('a'));
        ui_state.apply_action(ClientAction::TypeChar('c'));
        ui_state.apply_action(ClientAction::MoveCursorLeft);

        ui_state.apply_action(ClientAction::TypeChar('b'));

        assert_eq!(ui_state.editor_view().text, "abc");
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 2 }
        );
    }

    #[test]
    fn backspace_deletes_before_cursor() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }
        ui_state.apply_action(ClientAction::MoveCursorLeft);

        ui_state.apply_action(ClientAction::Backspace);

        assert_eq!(ui_state.editor_view().text, "ac");
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
    }

    #[test]
    fn cursor_navigation_clamps_safely_on_single_line_editor() {
        let mut ui_state = ClientUiState::new();
        ui_state.apply_action(ClientAction::TypeChar('x'));

        ui_state.apply_action(ClientAction::MoveCursorLeft);
        ui_state.apply_action(ClientAction::MoveCursorLeft);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 0 }
        );

        ui_state.apply_action(ClientAction::MoveCursorRight);
        ui_state.apply_action(ClientAction::MoveCursorRight);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );

        ui_state.apply_action(ClientAction::MoveCursorUp);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
        ui_state.apply_action(ClientAction::MoveCursorDown);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 1 }
        );
    }

    #[test]
    fn home_and_end_move_to_line_boundaries() {
        let mut ui_state = ClientUiState::new();
        for ch in "abc".chars() {
            ui_state.apply_action(ClientAction::TypeChar(ch));
        }

        ui_state.apply_action(ClientAction::MoveLineStart);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 0 }
        );
        ui_state.apply_action(ClientAction::MoveLineEnd);
        assert_eq!(
            ui_state.editor_view().cursor,
            CursorPosition { line: 0, column: 3 }
        );
    }

    #[test]
    fn client_public_api_stays_terminal_free() {
        let public_source = include_str!("mod.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected client public source");

        for forbidden in [
            format!("{}{}", "rata", "tui"),
            format!("{}{}", "cross", "term"),
        ] {
            assert!(
                !public_source.contains(&forbidden),
                "client public API should not mention terminal crate {forbidden}"
            );
        }
    }
}
