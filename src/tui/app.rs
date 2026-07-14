use std::time::Instant;

use crate::{
    answer,
    client::{ClientAction, ClientFrame, ClientUiState, FormatNoticeKind, project_client_frame_at},
    engine,
    formatter::{self, FormatResult},
};

pub struct TuiApp {
    session: engine::Session,
    ui_state: ClientUiState,
    should_quit: bool,
    completed: bool,
}

impl TuiApp {
    #[cfg(test)]
    pub fn new() -> Self {
        Self::with_session(engine::Session::new())
    }

    pub fn with_session(session: engine::Session) -> Self {
        Self {
            session,
            ui_state: ClientUiState::new(),
            should_quit: false,
            completed: false,
        }
    }

    #[cfg(test)]
    pub fn session(&self) -> &engine::Session {
        &self.session
    }

    #[cfg(test)]
    pub fn input(&self) -> String {
        self.ui_state.editor_view().text
    }

    #[cfg(test)]
    pub fn replace_input_for_test(&mut self, text: &str) {
        self.ui_state.replace_draft_input(text);
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    #[cfg(test)]
    pub fn completed(&self) -> bool {
        self.completed
    }

    #[cfg(test)]
    pub fn cursor_visible(&self) -> bool {
        self.ui_state.cursor_visible()
    }

    pub fn active_exercise(&self) -> engine::ExercisePacket {
        self.session
            .current_exercise()
            .unwrap_or_else(|| self.session.first_exercise_packet())
    }

    #[cfg(test)]
    pub fn status_lines(&self) -> &[String] {
        self.ui_state.status_lines()
    }

    #[cfg(test)]
    pub fn frame(&self) -> ClientFrame {
        self.frame_at(Instant::now())
    }

    pub fn frame_at(&self, now: Instant) -> ClientFrame {
        project_client_frame_at(&self.session, &self.ui_state, now)
    }

    pub fn frame_at_for_layout(&self, now: Instant, problem_wrap_width: usize) -> ClientFrame {
        crate::client::project_client_frame_for_layout(
            &self.session,
            &self.ui_state,
            now,
            problem_wrap_width,
        )
    }

    pub fn set_editor_viewport_height(&mut self, visible_lines: usize) {
        self.ui_state.set_editor_viewport_height(visible_lines);
    }

    pub fn set_editor_viewport_width(&mut self, visible_columns: usize) {
        self.ui_state.set_editor_viewport_width(visible_columns);
    }

    #[cfg(test)]
    pub fn handle_action(&mut self, action: ClientAction) {
        self.handle_action_at(action, Instant::now());
    }

    pub fn handle_action_at(&mut self, action: ClientAction, now: Instant) {
        match action {
            ClientAction::Quit => self.quit(),
            ClientAction::Format => self.format_current_input(now),
            ClientAction::Help => self.toggle_help(),
            ClientAction::Submit => self.submit_current_input(now),
            ClientAction::Skip => self.skip_current_exercise(now),
            ClientAction::Ignore => {}
            ClientAction::TypeChar(_)
            | ClientAction::Backspace
            | ClientAction::DeleteForward
            | ClientAction::InsertNewline
            | ClientAction::MoveCursorTo(_)
            | ClientAction::MoveCursorLeft
            | ClientAction::MoveCursorRight
            | ClientAction::MoveCursorUp
            | ClientAction::MoveCursorDown
            | ClientAction::MoveLineStart
            | ClientAction::MoveLineEnd
            | ClientAction::ScrollProblemLineUp
            | ClientAction::ScrollProblemLineDown
            | ClientAction::ScrollProblemUp
            | ClientAction::ScrollProblemDown
            | ClientAction::ScrollEditorUp
            | ClientAction::ScrollEditorDown
            | ClientAction::CursorTick => self.ui_state.apply_action(action),
        }
    }

    pub fn tick_cursor(&mut self) {
        self.ui_state.apply_action(ClientAction::CursorTick);
    }

    pub fn settle_animations(&mut self, now: Instant) {
        self.ui_state.settle_reward_event(now);
        self.ui_state.settle_failure_pulse(now);
        self.ui_state.settle_skip_toast(now);
        self.ui_state.settle_format_toast(now);
    }

    pub fn reward_active_at(&self, now: Instant) -> bool {
        self.ui_state.reward_active_at(now)
    }

    pub fn visual_feedback_active_at(&self, now: Instant) -> bool {
        self.reward_active_at(now)
            || self.ui_state.failure_active_at(now)
            || self.ui_state.skip_active_at(now)
            || self.ui_state.format_active_at(now)
    }

    fn format_current_input(&mut self, now: Instant) {
        self.format_current_input_with(now, formatter::format_rust_source);
    }

    fn toggle_help(&mut self) {
        let available = self.active_exercise().support.is_some();
        self.ui_state.toggle_help(available);
    }

    fn format_current_input_with(
        &mut self,
        now: Instant,
        format_source: impl FnOnce(&str) -> FormatResult,
    ) {
        if self.completed {
            return;
        }

        if !self.frame_at(now).format_available {
            self.finish_format_action(now, FormatNoticeKind::DisabledExactTask);
            return;
        }

        let result = format_source(&self.ui_state.draft_input());
        self.apply_format_result(now, result);
    }

    fn apply_format_result(&mut self, now: Instant, result: FormatResult) {
        let notice = match result {
            FormatResult::Formatted(formatted) => {
                self.ui_state.replace_draft_input(&formatted);
                FormatNoticeKind::Formatted
            }
            FormatResult::Unavailable => FormatNoticeKind::Unavailable,
            FormatResult::Failed(_) => FormatNoticeKind::Failed,
            FormatResult::TimedOut => FormatNoticeKind::TimedOut,
        };

        self.finish_format_action(now, notice);
    }

    fn finish_format_action(&mut self, now: Instant, notice: FormatNoticeKind) {
        self.ui_state.clear_reward_event();
        self.ui_state.clear_failure_pulse();
        self.ui_state.clear_skip_toast();
        self.ui_state.start_format_toast(now, notice);
        self.ui_state.set_status_lines(Vec::new());
    }

    fn submit_current_input(&mut self, now: Instant) {
        if self.completed {
            return;
        }

        let submitted = self.ui_state.draft_input();
        let command = answer::trim_line_ending(&submitted);
        self.ui_state.clear_reward_event();
        let status_lines = if command == ":q" || command == ":quit" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_format_toast();
            self.ui_state.clear_draft_input();
            {
                self.should_quit = true;
                collect_summary_lines("QUIT", &self.session)
            }
        } else if command == ":help" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_format_toast();
            self.ui_state.clear_draft_input();
            vec![
                ":help    show commands".to_string(),
                ":status  show session status".to_string(),
                ":list    show drills".to_string(),
                ":skip    skip current drill".to_string(),
                ":q       quit".to_string(),
                ":quit    quit".to_string(),
            ]
        } else if command == ":status" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_format_toast();
            self.ui_state.clear_draft_input();
            let mut lines = vec!["STATUS".to_string()];
            if let Some((position, total)) = self.session.round_position() {
                lines.push(format!(
                    "Current: {} ({position}/{total})",
                    self.active_exercise().id
                ));
                lines.push(format!("Passed: {}", self.session.passed()));
            } else {
                lines.push(format!(
                    "Current: {}/{}",
                    self.active_exercise().id,
                    self.session.total_drills()
                ));
                lines.push(format!(
                    "Passed: {}/{}",
                    self.session.passed(),
                    self.session.total_drills()
                ));
            }
            lines.push(format!("Skipped: {}", self.session.skipped()));
            lines.push(format!("Attempts: {}", self.session.attempts()));
            lines
        } else if command == ":list" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_format_toast();
            self.ui_state.clear_draft_input();
            self.session
                .product_exercises()
                .iter()
                .map(|exercise| format!("{}  {}", exercise.id, exercise.stimulus))
                .collect()
        } else if command == ":skip" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_format_toast();
            self.ui_state.start_skip_toast(now);
            self.ui_state.clear_draft_input();
            let _ = self.session.skip();
            self.ui_state.reset_task_support_state();
            self.ui_state.reset_problem_scroll();
            if self.session.is_complete() {
                self.completed = true;
                collect_complete_lines(&self.session)
            } else {
                vec!["skipped".to_string()]
            }
        } else if command.starts_with(':') {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_format_toast();
            self.ui_state.clear_draft_input();
            vec![
                "UNKNOWN COMMAND".to_string(),
                "Type :help for commands.".to_string(),
            ]
        } else {
            match self.session.submit_answer(&submitted) {
                engine::SubmitResult::Correct => {
                    self.ui_state.clear_failure_pulse();
                    self.ui_state.clear_skip_toast();
                    self.ui_state.clear_format_toast();
                    self.ui_state.start_reward_event(now);
                    self.ui_state.clear_draft_input();
                    self.ui_state.reset_task_support_state();
                    self.ui_state.reset_problem_scroll();
                    if self.session.is_complete() {
                        self.completed = true;
                        let mut lines = Vec::from(["PASS".to_string()]);
                        lines.extend(collect_complete_lines(&self.session));
                        lines
                    } else {
                        vec!["PASS".to_string()]
                    }
                }
                engine::SubmitResult::TryAgain => {
                    self.ui_state.clear_skip_toast();
                    self.ui_state.clear_format_toast();
                    self.ui_state.start_failure_pulse(now);
                    vec!["not quite".to_string()]
                }
                engine::SubmitResult::ValidationUnavailable => {
                    self.ui_state.clear_failure_pulse();
                    self.ui_state.clear_skip_toast();
                    self.ui_state.clear_format_toast();
                    vec!["validation unavailable".to_string()]
                }
                engine::SubmitResult::NoActiveDrill => {
                    self.ui_state.clear_failure_pulse();
                    self.ui_state.clear_skip_toast();
                    self.ui_state.clear_format_toast();
                    self.ui_state.clear_draft_input();
                    self.ui_state.reset_task_support_state();
                    self.should_quit = true;
                    collect_summary_lines("QUIT", &self.session)
                }
            }
        };
        self.ui_state.set_status_lines(status_lines);
    }

    fn skip_current_exercise(&mut self, now: Instant) {
        if self.completed {
            return;
        }

        self.ui_state.clear_reward_event();
        self.ui_state.clear_failure_pulse();
        self.ui_state.clear_format_toast();
        self.ui_state.start_skip_toast(now);
        self.ui_state.clear_draft_input();
        self.ui_state.reset_task_support_state();
        self.ui_state.reset_problem_scroll();
        let _ = self.session.skip();
        let status_lines = if self.session.is_complete() {
            self.completed = true;
            collect_complete_lines(&self.session)
        } else {
            vec!["skipped".to_string()]
        };
        self.ui_state.set_status_lines(status_lines);
    }

    fn quit(&mut self) {
        self.should_quit = true;
        self.ui_state.clear_reward_event();
        self.ui_state.clear_failure_pulse();
        self.ui_state.clear_skip_toast();
        self.ui_state.clear_format_toast();
        self.ui_state
            .set_status_lines(collect_summary_lines("QUIT", &self.session));
    }
}

fn collect_complete_lines(_session: &engine::Session) -> Vec<String> {
    vec!["COMPLETE".to_string()]
}

fn collect_summary_lines(action: &str, _session: &engine::Session) -> Vec<String> {
    vec![action.to_string()]
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::TuiApp;
    use crate::client::{
        ClientAction, CursorPosition, FAILURE_PULSE_DURATION, FormatNoticeKind,
        REWARD_ATTACK_DURATION, REWARD_DURATION, REWARD_PLUS_ONE_BODY_DURATION,
    };
    use crate::formatter::FormatResult;
    use crate::tui::input::classify_key_event;
    use crossterm::event::{KeyCode, KeyEvent};

    fn test_now() -> Instant {
        Instant::now()
    }

    fn lab_app() -> TuiApp {
        TuiApp::with_session(crate::engine::Session::lab())
    }

    fn interactive_app() -> TuiApp {
        TuiApp::with_session(
            crate::orchestration::current_interactive_product_session()
                .expect("interactive product session must resolve"),
        )
    }

    fn advance_lab_to_family(app: &mut TuiApp, family_index: usize) {
        for _ in 0..family_index {
            app.handle_action(ClientAction::Skip);
        }
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

    fn advance_to_product_id(app: &mut TuiApp, id: &str) {
        while app.active_exercise().id != id {
            app.handle_action(ClientAction::Skip);
        }
    }

    #[test]
    fn new_session_starts_without_reward_event() {
        let app = TuiApp::new();
        let now = test_now();

        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert!(!app.frame_at(now).failure_active);
    }

    #[test]
    fn correct_input_advances_and_clears_input() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.input(), "");
        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.frame_at(now).reward.plus_one_intensity, 0);
        assert_eq!(app.frame_at(now).reward.game_glow_intensity, 0);
        assert!(app.frame_at(now).reward_active);
        assert!(app.frame_at(now).reward.show_plus_one);
        assert!(!app.frame_at(now).failure_active);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn f5_submits_correct_answer_and_clears_editor() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        let action = classify_key_event(KeyEvent::from(KeyCode::F(5)));
        assert_eq!(action, ClientAction::Submit);
        app.handle_action_at(action, now);

        let workbox = app.frame_at(now).game_window.workbox;
        assert_eq!(workbox.input, "");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 0);
        assert_eq!(app.session().passed(), 1);
        assert!(app.frame_at(now).reward_active);
        assert!(app.frame_at(now).reward.show_plus_one);
        assert!(!app.frame_at(now).failure_active);
    }

    #[test]
    fn successful_answer_reward_event_expires_after_duration() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert!(app.frame_at(now).reward_active);
        assert!(app.frame_at(now + Duration::from_millis(50)).reward_active);
        assert_eq!(
            app.frame_at(now + REWARD_ATTACK_DURATION)
                .reward
                .plus_one_intensity,
            100
        );
        assert_eq!(
            app.frame_at(now + REWARD_ATTACK_DURATION + (REWARD_PLUS_ONE_BODY_DURATION / 2))
                .reward
                .plus_one_intensity,
            100
        );
        assert!(
            app.frame_at(now + REWARD_DURATION - Duration::from_millis(100))
                .reward
                .plus_one_intensity
                < 10
        );

        let expired_at = now + REWARD_DURATION;
        assert!(!app.frame_at(expired_at).reward_active);
        app.settle_animations(expired_at);
        assert!(!app.reward_active_at(expired_at));
    }

    #[test]
    fn wrong_input_tracks_attempt_but_stays_on_current_drill() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::MoveCursorLeft);
        let before_cursor = app.frame().game_window.workbox.cursor;
        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.input(), "wrong");
        assert_eq!(app.frame_at(now).game_window.workbox.cursor, before_cursor);
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert_eq!(app.frame_at(now).reward.game_glow_intensity, 0);
        assert!(app.frame_at(now).failure_active);
        assert_eq!(app.frame_at(now).failure.intensity, 100);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected first exercise")
                .id,
            "001"
        );
        assert_eq!(app.status_lines(), &["not quite".to_string()]);
    }

    #[test]
    fn f5_submits_wrong_answer_without_reward() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        let action = classify_key_event(KeyEvent::from(KeyCode::F(5)));
        assert_eq!(action, ClientAction::Submit);
        app.handle_action_at(action, now);

        assert_eq!(app.input(), "wrong");
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert_eq!(app.frame_at(now).reward.game_glow_intensity, 0);
        assert!(app.frame_at(now).failure_active);
        assert_eq!(app.status_lines(), &["not quite".to_string()]);
    }

    #[test]
    fn unknown_command_does_not_start_answer_failure_pulse() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in ":unknown".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.session().attempts(), 0);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).failure_active);
        assert_eq!(
            app.status_lines(),
            &[
                "UNKNOWN COMMAND".to_string(),
                "Type :help for commands.".to_string(),
            ]
        );
    }

    #[test]
    fn f7_skips_current_exercise_without_reward_or_rep_increment() {
        let mut app = TuiApp::new();
        let now = test_now();
        app.handle_action(ClientAction::TypeChar('x'));

        let action = classify_key_event(KeyEvent::from(KeyCode::F(7)));
        assert_eq!(action, ClientAction::Skip);
        app.handle_action_at(action, now);

        assert_eq!(app.input(), "");
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert_eq!(app.frame_at(now).reward.game_glow_intensity, 0);
        assert!(!app.frame_at(now).failure_active);
        assert!(app.frame_at(now).skip_active);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn format_success_replaces_editor_buffer_without_session_transition() {
        let mut app = lab_app();
        let now = test_now();
        type_text(&mut app, "fn main(){}");

        app.apply_format_result(now, FormatResult::Formatted("fn main() {}".to_string()));

        let frame = app.frame_at(now);
        assert_eq!(app.input(), "fn main() {}");
        assert_eq!(frame.game_window.workbox.cursor.line, 0);
        assert_eq!(
            frame.game_window.workbox.cursor.column,
            "fn main() {}".len()
        );
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 0);
        assert_eq!(app.session().attempts(), 0);
        assert_eq!(app.active_exercise().id, "LAB-000001");
        assert!(!frame.reward_active);
        assert!(!frame.failure_active);
        assert!(!frame.skip_active);
        assert!(frame.format_active);
        assert!(
            frame
                .format
                .kind
                .expect("expected format kind")
                .is_success()
        );
    }

    #[test]
    fn enabled_f1_calls_formatter_once_and_applies_output_without_submission() {
        let mut app = lab_app();
        let now = test_now();
        type_text(&mut app, "unformatted input");
        let before_stats = app.session().stats();
        let mut calls = 0;

        app.format_current_input_with(now, |_| {
            calls += 1;
            FormatResult::Formatted("formatted output unrelated to the model".to_string())
        });

        assert_eq!(calls, 1);
        assert_eq!(app.input(), "formatted output unrelated to the model");
        assert_eq!(app.session().stats(), before_stats);
        assert_eq!(
            app.frame_at(now)
                .format
                .kind
                .expect("expected format kind")
                .message(),
            "formatted"
        );
    }

    #[test]
    fn disabled_exact_task_f1_does_not_call_formatter_or_mutate_session() {
        let model = "fn add_one(x: i32) -> i32 { x + 1 }";
        let mut app = TuiApp::new();
        advance_to_product_id(&mut app, "010");
        load_text(&mut app, model);
        let before_stats = app.session().stats();
        let now = test_now();

        app.format_current_input_with(now, |_| panic!("disabled F1 must not invoke rustfmt"));

        let frame = app.frame_at(now);
        assert_eq!(app.input(), model);
        assert_eq!(app.session().stats(), before_stats);
        assert!(!frame.reward_active);
        assert!(!frame.failure_active);
        assert!(!frame.skip_active);
        assert!(frame.format_active);
        assert_eq!(frame.format.kind, Some(FormatNoticeKind::DisabledExactTask));
        assert_eq!(
            frame.format.kind.expect("expected format kind").message(),
            "format off for exact task"
        );
    }

    #[test]
    fn varied_copy_and_fill_keep_f1_disabled_without_attempts() {
        for id in ["013", "014"] {
            let mut app = TuiApp::new();
            advance_to_product_id(&mut app, id);
            load_text(&mut app, "unchanged learner text");
            let before = app.session().stats();

            app.format_current_input_with(test_now(), |_| {
                panic!("exact varied task {id} must not invoke rustfmt")
            });

            assert_eq!(app.input(), "unchanged learner text");
            assert_eq!(app.session().stats(), before);
            assert_eq!(
                app.frame().format.kind.unwrap().message(),
                "format off for exact task"
            );
        }
    }

    #[test]
    fn varied_full_code_tasks_enable_f1_without_submitting() {
        for id in ["015", "016", "017"] {
            let mut app = TuiApp::new();
            advance_to_product_id(&mut app, id);
            load_text(&mut app, "fn five()->i32{5}");
            let before = app.session().stats();
            let mut calls = 0;

            app.format_current_input_with(test_now(), |_| {
                calls += 1;
                FormatResult::Formatted("fn five() -> i32 {\n    5\n}".to_string())
            });

            assert_eq!(calls, 1);
            assert_eq!(app.input(), "fn five() -> i32 {\n    5\n}");
            assert_eq!(app.session().stats(), before);
            assert_eq!(app.active_exercise().id, id);
        }
    }

    #[test]
    fn interactive_counter_resets_and_no_completion_screen_appears_between_rows() {
        let mut app = interactive_app();
        assert_eq!(app.active_exercise().id, "013");
        assert_eq!(app.frame().round_position, Some((1, 5)));

        for expected_position in 2..=5 {
            app.handle_action(ClientAction::Skip);
            assert_eq!(app.frame().round_position, Some((expected_position, 5)));
            assert!(!app.completed());
        }
        assert_eq!(app.active_exercise().id, "017");

        app.handle_action(ClientAction::Skip);
        assert_eq!(app.active_exercise().id, "018");
        assert_eq!(app.frame().round_position, Some((1, 5)));
        assert!(!app.completed());
        assert!(!app.should_quit());
    }

    #[test]
    fn varied_fill_contract_is_visible_empty_and_wrong_stays_at_two_of_five() {
        let mut app = interactive_app();
        app.handle_action(ClientAction::Skip);

        let frame = app.frame();
        let rendered_problem = frame
            .game_window
            .problem
            .lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(app.active_exercise().id, "014");
        assert_eq!(frame.round_position, Some((2, 5)));
        assert_eq!(
            frame.game_window.problem.instruction,
            "Complete the missing return-type arrow: Type only the code replacing ___."
        );
        assert_eq!(
            frame.game_window.problem.stimulus,
            "fn five() ___ i32 { 5 }"
        );
        assert!(!rendered_problem.contains("fn five() -> i32 { 5 }"));
        assert!(app.input().is_empty());
        assert!(!frame.format_available);

        let before = app.session().stats();
        load_text(&mut app, "fn five() -> i32 { 5 }");
        app.handle_action(ClientAction::Submit);

        assert_eq!(app.active_exercise().id, "014");
        assert_eq!(app.frame().round_position, Some((2, 5)));
        assert_eq!(app.session().stats().attempts, before.attempts + 1);
        assert_eq!(app.session().stats().passed, before.passed);

        load_text(&mut app, "->");
        app.handle_action(ClientAction::Submit);

        assert_eq!(app.active_exercise().id, "015");
        assert_eq!(app.frame().round_position, Some((3, 5)));
        assert_eq!(app.session().stats().attempts, before.attempts + 2);
        assert_eq!(app.session().stats().passed, before.passed + 1);
    }

    #[test]
    fn f2_help_toggles_without_editor_cursor_or_session_evidence() {
        let mut app = interactive_app();
        advance_to_product_id(&mut app, "016");
        app.set_editor_viewport_height(1);
        load_text(&mut app, "first\nsecond\nlearner draft");
        app.handle_action(ClientAction::MoveCursorLeft);
        let before_input = app.input();
        let before_frame = app.frame();
        let before_cursor = before_frame.game_window.workbox.cursor;
        let before_editor_viewport = before_frame.game_window.workbox.viewport;
        let before_problem_viewport = before_frame.game_window.problem.viewport;
        let before_stats = app.session().stats();
        let before_id = app.active_exercise().id.clone();
        let expected = app.active_exercise().expected_answer().to_owned();

        app.handle_action(ClientAction::Help);
        let open = app.frame();
        let visible_problem = open
            .game_window
            .problem
            .lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(open.help_available);
        assert!(open.help_open);
        assert_eq!(app.input(), before_input);
        assert_eq!(open.game_window.workbox.cursor, before_cursor);
        assert_eq!(open.game_window.workbox.viewport, before_editor_viewport);
        assert_eq!(open.game_window.problem.viewport, before_problem_viewport);
        assert_eq!(app.session().stats(), before_stats);
        assert_eq!(app.active_exercise().id, before_id);
        assert!(!open.reward_active);
        assert!(!visible_problem.contains(&expected));
        assert!(
            open.game_window
                .problem
                .lines
                .iter()
                .any(|line| line.kind == crate::client::ProblemLineKind::HelpExample)
        );

        app.handle_action(ClientAction::Help);
        let closed = app.frame();
        assert!(!closed.help_open);
        assert_eq!(app.input(), before_input);
        assert_eq!(closed.game_window.workbox.cursor, before_cursor);
        assert_eq!(closed.game_window.workbox.viewport, before_editor_viewport);
        assert_eq!(closed.game_window.problem.viewport, before_problem_viewport);
        assert_eq!(app.session().stats(), before_stats);
        assert_eq!(app.active_exercise().id, before_id);
    }

    #[test]
    fn problem_first_support_is_optional_and_f5_keeps_the_single_step_fast_path() {
        let mut intro = interactive_app();
        assert!(
            intro
                .frame()
                .game_window
                .problem
                .lines
                .iter()
                .any(|line| { line.text.starts_with("FUNCTION —") })
        );
        let intro_expected = intro.active_exercise().expected_answer().to_owned();
        let intro_before = intro.session().stats();
        load_text(&mut intro, &intro_expected);
        intro.handle_action(ClientAction::Submit);
        assert_eq!(intro.active_exercise().id, "014");
        assert_eq!(intro.session().stats().attempts, intro_before.attempts + 1);
        assert_eq!(intro.session().stats().passed, intro_before.passed + 1);

        let mut with_help = interactive_app();
        advance_to_product_id(&mut with_help, "016");
        with_help.handle_action(ClientAction::Help);
        assert!(with_help.frame().help_open);
        let expected = with_help.active_exercise().expected_answer().to_owned();
        let before = with_help.session().stats();
        load_text(&mut with_help, &expected);
        with_help.handle_action(ClientAction::Submit);
        assert_eq!(with_help.active_exercise().id, "017");
        assert_eq!(with_help.session().stats().attempts, before.attempts + 1);
        assert_eq!(with_help.session().stats().passed, before.passed + 1);
        assert!(!with_help.frame().help_open);
    }

    #[test]
    fn help_stays_after_wrong_and_resets_on_correct_skip_and_round_transition() {
        let mut app = interactive_app();
        advance_to_product_id(&mut app, "016");
        app.handle_action(ClientAction::Help);
        load_text(&mut app, "not valid Rust");
        let before_wrong = app.session().stats();
        app.handle_action(ClientAction::Submit);
        assert_eq!(app.active_exercise().id, "016");
        assert!(app.frame().help_open);
        assert_eq!(app.session().stats().attempts, before_wrong.attempts + 1);
        assert_eq!(app.session().stats().passed, before_wrong.passed);

        let correct = app.active_exercise().expected_answer().to_owned();
        load_text(&mut app, &correct);
        app.handle_action(ClientAction::Submit);
        assert_eq!(app.active_exercise().id, "017");
        assert!(!app.frame().help_open);

        app.handle_action(ClientAction::Help);
        assert!(app.frame().help_open);
        app.handle_action(ClientAction::Skip);
        assert_eq!(app.active_exercise().id, "018");
        assert_eq!(app.frame().round_position, Some((1, 5)));
        assert!(!app.frame().help_open);

        app.handle_action(ClientAction::Help);
        assert!(app.frame().help_open);
        app.handle_action(ClientAction::Quit);
        assert!(app.should_quit());
    }

    #[test]
    fn unsupported_f2_reports_exact_local_notice_and_changes_nothing_else() {
        let mut app = TuiApp::new();
        load_text(&mut app, "draft");
        app.handle_action(ClientAction::MoveCursorLeft);
        let before_input = app.input();
        let before_cursor = app.frame().game_window.workbox.cursor;
        let before_stats = app.session().stats();
        let before_id = app.active_exercise().id.clone();

        app.handle_action(ClientAction::Help);
        let frame = app.frame();
        assert!(!frame.help_available);
        assert!(!frame.help_open);
        assert!(frame.game_window.problem.lines.iter().any(|line| {
            line.kind == crate::client::ProblemLineKind::HelpNotice
                && line.text == "help unavailable"
        }));
        assert_eq!(app.input(), before_input);
        assert_eq!(frame.game_window.workbox.cursor, before_cursor);
        assert_eq!(app.session().stats(), before_stats);
        assert_eq!(app.active_exercise().id, before_id);
        assert!(!frame.reward_active);
        assert!(!frame.failure_active);
        assert!(!frame.skip_active);
    }

    #[test]
    fn fill_help_does_not_change_exact_replacement_validation() {
        let mut app = interactive_app();
        app.handle_action(ClientAction::Skip);
        assert_eq!(app.active_exercise().id, "014");
        app.handle_action(ClientAction::Help);
        assert!(app.frame().help_open);

        load_text(&mut app, "fn five() -> i32 { 5 }");
        app.handle_action(ClientAction::Submit);
        assert_eq!(app.active_exercise().id, "014");
        assert!(app.frame().help_open);

        load_text(&mut app, "->");
        app.handle_action(ClientAction::Submit);
        assert_eq!(app.active_exercise().id, "015");
        assert!(!app.frame().help_open);
    }

    #[test]
    fn f5_keeps_internal_newline_differences_wrong_and_exact_model_correct() {
        let model = "fn add_one(x: i32) -> i32 { x + 1 }";
        let formatted = "fn add_one(x: i32) -> i32 {\n    x + 1\n}";
        let mut app = TuiApp::new();
        advance_to_product_id(&mut app, "010");
        load_text(&mut app, formatted);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.active_exercise().id, "010");

        load_text(&mut app, model);
        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().attempts(), 2);
        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.active_exercise().id, "011");
    }

    #[test]
    fn f5_still_ignores_only_one_final_line_ending_for_exact_copy() {
        let mut app = TuiApp::new();
        load_text(&mut app, "fn main() {}\r\n");

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.session().passed(), 1);
    }

    #[test]
    fn production_f1_path_uses_capability_without_answer_truth() {
        let production = include_str!("app.rs")
            .split("#[cfg(test)]\nmod tests")
            .next()
            .expect("expected production app source");
        let format_application = production
            .split("fn format_current_input_with")
            .nth(1)
            .expect("expected format application")
            .split("fn submit_current_input")
            .next()
            .expect("expected format application boundary");

        assert!(format_application.contains("format_available"));
        for forbidden in [
            "active_exercise",
            "stimulus",
            "expected",
            "answer_rule",
            "validator",
            "check_answer",
            "submit_answer",
            "RustfmtExact",
        ] {
            assert!(
                !format_application.contains(forbidden),
                "F1 application must not inspect {forbidden}"
            );
        }
    }

    #[test]
    fn production_tui_never_reads_expected_answer_truth() {
        let production = include_str!("app.rs")
            .split("#[cfg(test)]\nmod tests")
            .next()
            .expect("expected production app source");

        for forbidden in ["expected_answer", "wrong_answer_feedback"] {
            assert!(
                !production.contains(forbidden),
                "TUI production code must not read {forbidden}"
            );
        }
    }

    #[test]
    fn format_failure_preserves_editor_buffer_and_session_state() {
        let mut app = lab_app();
        let now = test_now();
        type_text(&mut app, "let = ;");
        app.handle_action(ClientAction::MoveCursorLeft);
        let before_cursor = app.frame().game_window.workbox.cursor;

        app.format_current_input_with(now, |_| FormatResult::Failed("parse error".to_string()));

        let frame = app.frame_at(now);
        assert_eq!(app.input(), "let = ;");
        assert_eq!(frame.game_window.workbox.cursor, before_cursor);
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 0);
        assert_eq!(app.session().attempts(), 0);
        assert_eq!(app.active_exercise().id, "LAB-000001");
        assert!(!frame.reward_active);
        assert!(!frame.failure_active);
        assert!(!frame.skip_active);
        assert!(frame.format_active);
        assert_eq!(
            frame.format.kind.expect("expected format kind").message(),
            "format failed"
        );
    }

    #[test]
    fn format_unavailable_preserves_editor_buffer() {
        let mut app = lab_app();
        let now = test_now();
        type_text(&mut app, "fn main(){}");

        app.format_current_input_with(now, |_| FormatResult::Unavailable);

        let frame = app.frame_at(now);
        assert_eq!(app.input(), "fn main(){}");
        assert_eq!(app.session().passed(), 0);
        assert!(frame.format_active);
        assert_eq!(
            frame.format.kind.expect("expected format kind").message(),
            "rustfmt unavailable"
        );
    }

    #[test]
    fn smart_enter_works_without_formatter_availability() {
        let mut app = lab_app();
        let now = test_now();
        type_text(&mut app, "fn main() {");

        app.format_current_input_with(now, |_| FormatResult::Unavailable);
        app.handle_action(ClientAction::InsertNewline);

        let frame = app.frame();
        assert_eq!(app.input(), "fn main() {\n    \n}");
        assert_eq!(
            frame.game_window.workbox.cursor,
            CursorPosition { line: 1, column: 4 }
        );
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 0);
    }

    #[test]
    fn smart_enter_buffer_still_submits_with_exact_answer_rules() {
        let mut app = TuiApp::new();
        type_text(&mut app, "fn main() {");
        app.handle_action(ClientAction::InsertNewline);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.input(), "fn main() {\n    \n}");
    }

    #[test]
    fn f6_does_not_skip_as_a_hidden_alias() {
        let mut app = TuiApp::new();
        let now = test_now();
        let action = classify_key_event(KeyEvent::from(KeyCode::F(6)));

        assert_eq!(action, ClientAction::Ignore);
        app.handle_action_at(action, now);

        assert_eq!(app.session().skipped(), 0);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected first exercise")
                .id,
            "001"
        );
    }

    #[test]
    fn move_cursor_to_action_places_next_insert_at_clicked_position() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        app.handle_action(ClientAction::MoveCursorTo(CursorPosition {
            line: 0,
            column: 3,
        }));
        assert_eq!(app.input(), "fn main() {}");
        assert_eq!(
            app.frame().game_window.workbox.cursor,
            CursorPosition { line: 0, column: 3 }
        );

        app.handle_action(ClientAction::TypeChar('X'));

        assert_eq!(app.input(), "fn Xmain() {}");
    }

    #[test]
    fn lab_session_starts_at_first_lab_exercise() {
        let app = lab_app();

        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected first lab exercise")
                .id,
            "LAB-000001"
        );
        assert_eq!(app.session().total_drills(), usize::MAX);
        assert_eq!(app.frame().completed_reps_this_run, 0);
    }

    #[test]
    fn lab_skip_advances_without_reward() {
        let mut app = lab_app();
        let now = test_now();

        app.handle_action_at(ClientAction::Skip, now);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert!(app.frame_at(now).skip_active);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected second lab exercise")
                .id,
            "LAB-000002"
        );
    }

    #[test]
    fn lab_correct_multiline_answer_passes() {
        let mut app = lab_app();
        let now = test_now();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        load_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);

        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.input(), "");
        assert!(app.frame_at(now).reward_active);
        assert!(app.frame_at(now).reward.show_plus_one);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected syntax sampler lab exercise")
                .id,
            "LAB-000006"
        );
    }

    #[test]
    fn lab_f1_formatted_full_function_answer_matches_expected_and_passes() {
        let unformatted = "fn add_one(x: i32) -> i32 { x + 1 }";
        match crate::formatter::format_rust_source(unformatted) {
            FormatResult::Formatted(formatted) => {
                assert_eq!(formatted, crate::lab::MULTILINE_SOLUTION_EXPECTED);
            }
            FormatResult::Unavailable => return,
            other => panic!("expected rustfmt to format lab function, got {other:?}"),
        }

        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        load_text(&mut app, unformatted);

        app.handle_action(ClientAction::Format);
        assert_eq!(app.input(), crate::lab::MULTILINE_SOLUTION_EXPECTED);
        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
    }

    #[test]
    fn lab_correct_multiline_answer_with_one_final_newline_passes() {
        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        load_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);
        app.handle_action(ClientAction::InsertNewline);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.input(), "");
    }

    #[test]
    fn lab_extra_blank_line_is_wrong_and_preserves_buffer() {
        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        load_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::InsertNewline);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected multiline lab exercise")
                .id,
            "LAB-000005"
        );
        assert_eq!(
            app.input(),
            format!("{}\n\n", crate::lab::MULTILINE_SOLUTION_EXPECTED)
        );
        assert_eq!(app.status_lines(), &["not quite".to_string()]);
    }

    #[test]
    fn lab_wrong_multiline_answer_preserves_editor_buffer() {
        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        load_text(&mut app, "fn add_one(x: i32) -> i32 {\n    x\n}");
        let before_cursor = app.frame().game_window.workbox.cursor;

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.input(), "fn add_one(x: i32) -> i32 {\n    x\n}");
        assert_eq!(app.frame().game_window.workbox.cursor, before_cursor);
        assert!(!app.frame().reward_active);
        assert!(app.frame().failure_active);
    }

    #[test]
    fn problem_scroll_does_not_affect_editor_or_session_state() {
        let mut app = lab_app();
        app.handle_action(ClientAction::TypeChar('x'));
        let before_input = app.input();
        let before_cursor = app.frame().game_window.workbox.cursor;

        app.handle_action(ClientAction::ScrollProblemLineDown);
        assert_eq!(app.frame().game_window.problem.viewport.scroll_offset, 1);
        app.handle_action(ClientAction::ScrollProblemDown);
        assert_eq!(app.frame().game_window.problem.viewport.scroll_offset, 5);
        app.handle_action(ClientAction::ScrollProblemLineUp);
        assert_eq!(app.frame().game_window.problem.viewport.scroll_offset, 4);
        app.handle_action(ClientAction::ScrollProblemUp);

        assert_eq!(app.input(), before_input);
        assert_eq!(app.frame().game_window.workbox.cursor, before_cursor);
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 0);
        assert_eq!(app.session().attempts(), 0);
    }

    #[test]
    fn editor_scroll_does_not_affect_buffer_or_session_state() {
        let mut app = TuiApp::new();
        type_text(&mut app, "one\ntwo\nthree\nfour\nfive");
        app.set_editor_viewport_height(3);
        while app.frame().game_window.workbox.cursor.line > 2 {
            app.handle_action(ClientAction::MoveCursorUp);
        }
        let before_input = app.input();
        let before_cursor = app.frame().game_window.workbox.cursor;

        app.handle_action(ClientAction::ScrollEditorDown);
        let frame = app.frame();

        assert_eq!(frame.game_window.workbox.viewport.first_line, 2);
        assert_eq!(app.input(), before_input);
        assert_eq!(frame.game_window.workbox.cursor, before_cursor);
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 0);
        assert_eq!(app.session().attempts(), 0);

        app.handle_action(ClientAction::ScrollEditorUp);
        assert_eq!(app.frame().game_window.workbox.viewport.first_line, 1);
    }

    #[test]
    fn skip_does_not_start_reward_event() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in ":skip".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert_eq!(app.frame_at(now).reward.game_glow_intensity, 0);
        assert!(!app.frame_at(now).failure_active);
    }

    #[test]
    fn wrong_answer_failure_pulse_expires_after_duration() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        app.handle_action_at(ClientAction::Submit, now);

        assert!(app.frame_at(now).failure_active);
        assert!(app.visual_feedback_active_at(now));

        let expired_at = now + FAILURE_PULSE_DURATION;
        assert!(!app.frame_at(expired_at).failure_active);
        app.settle_animations(expired_at);
        assert!(!app.visual_feedback_active_at(expired_at));
    }

    #[test]
    fn wrong_input_clears_existing_reward_event() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        assert!(app.frame_at(now).reward_active);

        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now + Duration::from_millis(50));

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 1);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert!(app.frame_at(now + Duration::from_millis(50)).failure_active);
    }

    #[test]
    fn skip_clears_existing_reward_event() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        assert!(app.frame_at(now).reward_active);

        for ch in ":skip".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now + Duration::from_millis(50));

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().skipped(), 1);
        assert!(!app.frame_at(now).reward_active);
        assert!(!app.frame_at(now).reward.show_plus_one);
        assert!(!app.frame_at(now + Duration::from_millis(50)).failure_active);
    }

    #[test]
    fn backspace_edits_input_line() {
        let mut app = TuiApp::new();

        for ch in "abc".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::Backspace);

        assert_eq!(app.input(), "ab");
    }

    #[test]
    fn arrow_navigation_edits_at_cursor() {
        let mut app = TuiApp::new();

        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::TypeChar('c'));
        app.handle_action(ClientAction::MoveCursorLeft);
        app.handle_action(ClientAction::TypeChar('b'));

        assert_eq!(app.input(), "abc");
        assert_eq!(app.frame().game_window.workbox.cursor.column, 2);
    }

    #[test]
    fn newline_action_inserts_line_in_editor_buffer() {
        let mut app = TuiApp::new();

        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('b'));

        let workbox = app.frame().game_window.workbox;
        assert_eq!(workbox.input, "a\nb");
        assert_eq!(workbox.lines, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(workbox.cursor.line, 1);
        assert_eq!(workbox.cursor.column, 1);
    }

    #[test]
    fn up_and_down_move_across_lines_through_app_path() {
        let mut app = TuiApp::new();

        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('b'));
        app.handle_action(ClientAction::MoveCursorUp);
        assert_eq!(app.frame().game_window.workbox.cursor.line, 0);

        app.handle_action(ClientAction::MoveCursorDown);
        assert_eq!(app.frame().game_window.workbox.cursor.line, 1);
    }

    #[test]
    fn backspace_at_start_of_second_line_joins_lines_through_app_path() {
        let mut app = TuiApp::new();

        app.handle_action(ClientAction::TypeChar('a'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('b'));
        app.handle_action(ClientAction::MoveLineStart);
        app.handle_action(ClientAction::Backspace);

        let workbox = app.frame().game_window.workbox;
        assert_eq!(workbox.input, "ab");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 1);
    }

    #[test]
    fn delete_forward_removes_next_character_through_app_path() {
        let mut app = TuiApp::new();
        type_text(&mut app, "abc");
        app.handle_action(ClientAction::MoveCursorLeft);
        app.handle_action(ClientAction::DeleteForward);

        let workbox = app.frame().game_window.workbox;
        assert_eq!(workbox.input, "ab");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 2);
    }

    #[test]
    fn delete_forward_at_line_end_joins_next_line_through_app_path() {
        let mut app = TuiApp::new();
        type_text(&mut app, "ab\ncd");
        app.handle_action(ClientAction::MoveCursorUp);
        app.handle_action(ClientAction::MoveLineEnd);
        app.handle_action(ClientAction::DeleteForward);

        let workbox = app.frame().game_window.workbox;
        assert_eq!(workbox.input, "abcd");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 2);
    }

    #[test]
    fn long_one_line_answer_submit_uses_full_buffer_after_horizontal_viewport() {
        let mut app = TuiApp::new();
        app.set_editor_viewport_width(4);
        type_text(&mut app, "fn main() {}");
        assert!(app.frame().game_window.workbox.viewport.first_column > 0);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.input(), "");
    }

    #[test]
    fn enter_alone_inserts_newline_and_does_not_submit() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        app.handle_action(ClientAction::InsertNewline);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 0);
        assert_eq!(
            app.frame().game_window.workbox.lines,
            vec!["fn main() {}".to_string(), "".to_string()]
        );
    }

    #[test]
    fn ctrl_enter_submit_clears_editor_and_resets_cursor_after_correct_answer() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }

        app.handle_action(ClientAction::Submit);

        let workbox = app.frame().game_window.workbox;
        assert_eq!(workbox.input, "");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 0);
    }

    #[test]
    fn ctrl_enter_submits_multiline_buffer_with_existing_exact_answer_rules() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::InsertNewline);

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.frame().game_window.workbox.lines, vec!["".to_string()]);
    }

    #[test]
    fn f5_submits_correct_one_line_answer_with_one_editor_newline() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::InsertNewline);

        let action = classify_key_event(KeyEvent::from(KeyCode::F(5)));
        assert_eq!(action, ClientAction::Submit);
        app.handle_action(action);

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(app.frame().game_window.workbox.lines, vec!["".to_string()]);
    }

    #[test]
    fn f5_submits_extra_blank_line_as_wrong_answer() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::InsertNewline);

        let action = classify_key_event(KeyEvent::from(KeyCode::F(5)));
        assert_eq!(action, ClientAction::Submit);
        app.handle_action(action);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected active exercise")
                .id,
            "001"
        );
        assert_eq!(app.status_lines(), &["not quite".to_string()]);
        assert_eq!(app.input(), "fn main() {}\n\n");
        assert_eq!(
            app.frame().game_window.workbox.lines,
            vec!["fn main() {}".to_string(), "".to_string(), "".to_string()]
        );
    }

    #[test]
    fn ctrl_enter_wrong_answer_stays_on_same_drill_and_reports_feedback() {
        let mut app = TuiApp::new();
        app.handle_action(ClientAction::TypeChar('x'));
        app.handle_action(ClientAction::InsertNewline);
        app.handle_action(ClientAction::TypeChar('y'));

        app.handle_action(ClientAction::Submit);

        assert_eq!(app.input(), "x\ny");
        assert_eq!(
            app.frame().game_window.workbox.cursor,
            crate::client::CursorPosition { line: 1, column: 1 }
        );
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected active exercise")
                .id,
            "001"
        );
        assert_eq!(app.status_lines(), &["not quite".to_string()]);
    }

    #[test]
    fn wrong_submit_can_be_edited_into_correct_answer() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() }".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action(ClientAction::MoveCursorLeft);

        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.input(), "fn main() }");
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert!(app.frame_at(now).failure_active);

        app.handle_action(ClientAction::TypeChar('{'));
        assert_eq!(app.input(), "fn main() {}}");
        app.handle_action(ClientAction::DeleteForward);
        assert_eq!(app.input(), "fn main() {}");
        app.handle_action_at(ClientAction::Submit, now + Duration::from_millis(1));

        let workbox = app.frame().game_window.workbox;
        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 2);
        assert_eq!(workbox.input, "");
        assert_eq!(workbox.cursor.line, 0);
        assert_eq!(workbox.cursor.column, 0);
        assert!(!app.frame_at(now + Duration::from_millis(1)).failure_active);
    }

    #[test]
    fn app_input_buffer_tracks_characters_and_backspace() {
        let mut app = TuiApp::new();
        app.tick_cursor();
        assert!(!app.cursor_visible());

        app.handle_action(ClientAction::TypeChar('h'));
        app.handle_action(ClientAction::TypeChar('i'));

        assert_eq!(app.input(), "hi");
        assert!(app.cursor_visible());

        app.tick_cursor();
        assert!(!app.cursor_visible());

        app.handle_action(ClientAction::Backspace);
        assert_eq!(app.input(), "h");
        assert!(app.cursor_visible());
    }

    #[test]
    fn cursor_tick_toggles_visibility_without_session_changes() {
        let mut app = TuiApp::new();
        assert!(app.cursor_visible());
        assert_eq!(app.session().attempts(), 0);

        app.tick_cursor();
        assert!(!app.cursor_visible());
        assert_eq!(app.session().attempts(), 0);

        app.tick_cursor();
        assert!(app.cursor_visible());
        assert_eq!(app.session().attempts(), 0);
    }

    #[test]
    fn quit_does_not_increment_completed_rep_counter() {
        let mut app = TuiApp::new();

        app.handle_action(ClientAction::Quit);

        assert!(app.should_quit());
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.frame().completed_reps_this_run, 0);
        assert_eq!(app.status_lines(), &["QUIT".to_string()]);
    }

    #[test]
    fn final_answer_marks_completion_state() {
        let mut app = TuiApp::new();
        for drill_index in 0..22 {
            let exercise = app
                .session()
                .current_exercise()
                .expect("expected active exercise");
            for ch in exercise.expected_answer().chars() {
                app.handle_action(ClientAction::TypeChar(ch));
            }
            app.handle_action(ClientAction::Submit);
            if drill_index < 21 {
                continue;
            }
        }

        assert_eq!(app.session().passed(), 22);
        assert!(app.completed());
        assert!(app.session().is_complete());
        assert_eq!(
            app.status_lines(),
            &["PASS".to_string(), "COMPLETE".to_string()]
        );
    }
}
