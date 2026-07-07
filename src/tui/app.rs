use std::time::Instant;

use crate::{
    answer,
    client::{ClientAction, ClientFrame, ClientUiState, project_client_frame_at},
    engine,
};

pub struct TuiApp {
    session: engine::Session,
    ui_state: ClientUiState,
    should_quit: bool,
    completed: bool,
}

impl TuiApp {
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

    #[cfg(test)]
    pub fn handle_action(&mut self, action: ClientAction) {
        self.handle_action_at(action, Instant::now());
    }

    pub fn handle_action_at(&mut self, action: ClientAction, now: Instant) {
        match action {
            ClientAction::Quit => self.quit(),
            ClientAction::Submit => self.submit_current_input(now),
            ClientAction::Skip => self.skip_current_exercise(now),
            ClientAction::Ignore => {}
            ClientAction::TypeChar(_)
            | ClientAction::Backspace
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
    }

    pub fn reward_active_at(&self, now: Instant) -> bool {
        self.ui_state.reward_active_at(now)
    }

    pub fn visual_feedback_active_at(&self, now: Instant) -> bool {
        self.reward_active_at(now)
            || self.ui_state.failure_active_at(now)
            || self.ui_state.skip_active_at(now)
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
            self.ui_state.clear_draft_input();
            {
                self.should_quit = true;
                collect_summary_lines("QUIT", &self.session)
            }
        } else if command == ":help" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
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
            self.ui_state.clear_draft_input();
            vec![
                "STATUS".to_string(),
                format!(
                    "Current: {}/{}",
                    self.active_exercise().id,
                    self.session.total_drills()
                ),
                format!(
                    "Passed: {}/{}",
                    self.session.passed(),
                    self.session.total_drills()
                ),
                format!("Skipped: {}", self.session.skipped()),
                format!("Attempts: {}", self.session.attempts()),
            ]
        } else if command == ":list" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.clear_skip_toast();
            self.ui_state.clear_draft_input();
            engine::exercise_packets()
                .iter()
                .map(|exercise| format!("{}  {}", exercise.id, exercise.expected_answer))
                .collect()
        } else if command == ":skip" {
            self.ui_state.clear_failure_pulse();
            self.ui_state.start_skip_toast(now);
            self.ui_state.clear_draft_input();
            let _ = self.session.skip();
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
                    self.ui_state.start_reward_event(now);
                    self.ui_state.clear_draft_input();
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
                    self.ui_state.start_failure_pulse(now);
                    let expected = self.active_exercise().expected_answer;
                    vec![
                        "not quite".to_string(),
                        "expected:".to_string(),
                        expected.to_string(),
                    ]
                }
                engine::SubmitResult::NoActiveDrill => {
                    self.ui_state.clear_failure_pulse();
                    self.ui_state.clear_skip_toast();
                    self.ui_state.clear_draft_input();
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
        self.ui_state.start_skip_toast(now);
        self.ui_state.clear_draft_input();
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
        ClientAction, CursorPosition, FAILURE_PULSE_DURATION, REWARD_ATTACK_DURATION,
        REWARD_DURATION, REWARD_PLUS_ONE_BODY_DURATION,
    };
    use crate::tui::input::classify_key_event;
    use crossterm::event::{KeyCode, KeyEvent};

    fn test_now() -> Instant {
        Instant::now()
    }

    fn lab_app() -> TuiApp {
        TuiApp::with_session(crate::engine::Session::lab())
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
        assert_eq!(
            app.status_lines(),
            &[
                "not quite".to_string(),
                "expected:".to_string(),
                "fn main() {}".to_string(),
            ]
        );
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
        assert_eq!(
            app.status_lines(),
            &[
                "not quite".to_string(),
                "expected:".to_string(),
                "fn main() {}".to_string(),
            ]
        );
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
        type_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);

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
    fn lab_correct_multiline_answer_with_one_final_newline_passes() {
        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        type_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);
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
        type_text(&mut app, crate::lab::MULTILINE_SOLUTION_EXPECTED);
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
        assert_eq!(
            app.status_lines(),
            &[
                "not quite".to_string(),
                "expected:".to_string(),
                crate::lab::MULTILINE_SOLUTION_EXPECTED.to_string(),
            ]
        );
    }

    #[test]
    fn lab_wrong_multiline_answer_preserves_editor_buffer() {
        let mut app = lab_app();
        advance_lab_to_family(&mut app, crate::lab::MULTILINE_SOLUTION_FAMILY_INDEX);
        type_text(&mut app, "fn add_one(x: i32) -> i32 {\n    x\n}");
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
        assert_eq!(
            app.status_lines(),
            &[
                "not quite".to_string(),
                "expected:".to_string(),
                "fn main() {}".to_string(),
            ]
        );
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
        assert_eq!(
            app.status_lines(),
            &[
                "not quite".to_string(),
                "expected:".to_string(),
                "fn main() {}".to_string(),
            ]
        );
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
        for drill_index in 0..12 {
            let exercise = app
                .session()
                .current_exercise()
                .expect("expected active exercise");
            for ch in exercise.expected_answer.chars() {
                app.handle_action(ClientAction::TypeChar(ch));
            }
            app.handle_action(ClientAction::Submit);
            if drill_index < 11 {
                continue;
            }
        }

        assert_eq!(app.session().passed(), 12);
        assert!(app.completed());
        assert!(app.session().is_complete());
        assert_eq!(
            app.status_lines(),
            &["PASS".to_string(), "COMPLETE".to_string()]
        );
    }
}
