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
        Self {
            session: engine::Session::new(),
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
            .unwrap_or_else(engine::first_exercise_packet)
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

    #[cfg(test)]
    pub fn handle_action(&mut self, action: ClientAction) {
        self.handle_action_at(action, Instant::now());
    }

    pub fn handle_action_at(&mut self, action: ClientAction, now: Instant) {
        match action {
            ClientAction::Quit => self.quit(),
            ClientAction::Submit => self.submit_current_input(now),
            ClientAction::Ignore => {}
            ClientAction::TypeChar(_)
            | ClientAction::Backspace
            | ClientAction::MoveCursorLeft
            | ClientAction::MoveCursorRight
            | ClientAction::MoveCursorUp
            | ClientAction::MoveCursorDown
            | ClientAction::MoveLineStart
            | ClientAction::MoveLineEnd
            | ClientAction::ToggleHelp
            | ClientAction::CursorTick => self.ui_state.apply_action(action),
        }
    }

    pub fn tick_cursor(&mut self) {
        self.ui_state.apply_action(ClientAction::CursorTick);
    }

    pub fn settle_animations(&mut self, now: Instant) {
        self.ui_state.settle_counter_reward(now);
    }

    pub fn counter_reward_active_at(&self, now: Instant) -> bool {
        self.ui_state.counter_reward_active_at(now)
    }

    fn submit_current_input(&mut self, now: Instant) {
        if self.completed {
            return;
        }

        let submitted = self.ui_state.take_draft_input();
        let command = answer::trim_line_ending(&submitted);
        self.ui_state.clear_counter_reward();
        let status_lines = if command == ":q" || command == ":quit" {
            {
                self.should_quit = true;
                collect_summary_lines("QUIT", &self.session)
            }
        } else if command == ":help" {
            vec![
                ":help    show commands".to_string(),
                ":status  show session status".to_string(),
                ":list    show drills".to_string(),
                ":skip    skip current drill".to_string(),
                ":q       quit".to_string(),
                ":quit    quit".to_string(),
            ]
        } else if command == ":status" {
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
            engine::exercise_packets()
                .iter()
                .map(|exercise| format!("{}  {}", exercise.id, exercise.expected_answer))
                .collect()
        } else if command == ":skip" {
            let _ = self.session.skip();
            if self.session.is_complete() {
                self.completed = true;
                collect_complete_lines(&self.session)
            } else {
                vec!["SKIP".to_string()]
            }
        } else if command.starts_with(':') {
            vec![
                "UNKNOWN COMMAND".to_string(),
                "Type :help for commands.".to_string(),
            ]
        } else {
            match self.session.submit_answer(&submitted) {
                engine::SubmitResult::Correct => {
                    self.ui_state.start_counter_reward(now);
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
                    let expected = self.active_exercise().expected_answer;
                    vec![
                        "not quite".to_string(),
                        "expected:".to_string(),
                        expected.to_string(),
                    ]
                }
                engine::SubmitResult::NoActiveDrill => {
                    self.should_quit = true;
                    collect_summary_lines("QUIT", &self.session)
                }
            }
        };
        self.ui_state.set_status_lines(status_lines);
    }

    fn quit(&mut self) {
        self.should_quit = true;
        self.ui_state.clear_counter_reward();
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
        COUNTER_REWARD_ATTACK_DURATION, COUNTER_REWARD_DURATION, COUNTER_REWARD_HOLD_DURATION,
        ClientAction,
    };

    fn test_now() -> Instant {
        Instant::now()
    }

    #[test]
    fn new_session_starts_without_counter_reward() {
        let app = TuiApp::new();
        let now = test_now();

        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).counter_reward_active);
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
        assert_eq!(app.frame_at(now).counter_reward.intensity, 0);
        assert!(app.frame_at(now).counter_reward_active);
        assert_eq!(
            app.session()
                .current_exercise()
                .expect("expected second exercise")
                .id,
            "002"
        );
    }

    #[test]
    fn successful_answer_counter_reward_expires_after_duration() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert!(app.frame_at(now).counter_reward_active);
        assert!(
            app.frame_at(now + Duration::from_millis(50))
                .counter_reward_active
        );
        assert_eq!(
            app.frame_at(now + COUNTER_REWARD_ATTACK_DURATION)
                .counter_reward
                .intensity,
            100
        );
        assert_eq!(
            app.frame_at(now + COUNTER_REWARD_ATTACK_DURATION + (COUNTER_REWARD_HOLD_DURATION / 2))
                .counter_reward
                .intensity,
            100
        );
        assert!(
            app.frame_at(now + COUNTER_REWARD_DURATION - Duration::from_millis(100))
                .counter_reward
                .intensity
                < 10
        );

        let expired_at = now + COUNTER_REWARD_DURATION;
        assert!(!app.frame_at(expired_at).counter_reward_active);
        app.settle_animations(expired_at);
        assert!(!app.counter_reward_active_at(expired_at));
    }

    #[test]
    fn wrong_input_tracks_attempt_but_stays_on_current_drill() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.input(), "");
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert!(!app.frame_at(now).counter_reward_active);
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
    fn skip_does_not_start_counter_reward() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in ":skip".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);

        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().skipped(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 0);
        assert!(!app.frame_at(now).counter_reward_active);
    }

    #[test]
    fn wrong_input_clears_existing_counter_reward() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        assert!(app.frame_at(now).counter_reward_active);

        for ch in "wrong".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now + Duration::from_millis(50));

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.frame_at(now).completed_reps_this_run, 1);
        assert!(!app.frame_at(now).counter_reward_active);
    }

    #[test]
    fn skip_clears_existing_counter_reward() {
        let mut app = TuiApp::new();
        let now = test_now();
        for ch in "fn main() {}".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now);
        assert!(app.frame_at(now).counter_reward_active);

        for ch in ":skip".chars() {
            app.handle_action(ClientAction::TypeChar(ch));
        }
        app.handle_action_at(ClientAction::Submit, now + Duration::from_millis(50));

        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().skipped(), 1);
        assert!(!app.frame_at(now).counter_reward_active);
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
    fn correct_submit_clears_editor_and_resets_cursor() {
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
