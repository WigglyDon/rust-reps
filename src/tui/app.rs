use crate::{catalog, practice};

use super::input::UiIntent;

pub struct TuiApp {
    session: practice::PracticeSession,
    input: String,
    status_lines: Vec<String>,
    show_help_panel: bool,
    should_quit: bool,
    completed: bool,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            session: practice::PracticeSession::new(),
            input: String::new(),
            status_lines: vec!["Type an exact answer and press Enter".to_string()],
            show_help_panel: true,
            should_quit: false,
            completed: false,
        }
    }

    pub fn session(&self) -> &practice::PracticeSession {
        &self.session
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn show_help_panel(&self) -> bool {
        self.show_help_panel
    }

    pub fn active_drill(&self) -> &'static catalog::Drill {
        self.session
            .current_drill()
            .unwrap_or(&catalog::drills()[0])
    }

    pub fn status_lines(&self) -> &[String] {
        &self.status_lines
    }

    pub fn handle_intent(&mut self, intent: UiIntent) {
        match intent {
            UiIntent::Quit => self.quit(),
            UiIntent::ToggleHelp => self.show_help_panel = !self.show_help_panel,
            UiIntent::Backspace => {
                self.input.pop();
            }
            UiIntent::Submit => self.submit_current_input(),
            UiIntent::Append(char) => {
                self.input.push(char);
            }
            UiIntent::Ignore => {}
        }
    }

    fn submit_current_input(&mut self) {
        if self.completed {
            return;
        }

        let submitted = std::mem::take(&mut self.input);
        self.status_lines = match self.session.submit_input(&submitted) {
            practice::PracticeEvent::Quit => {
                self.should_quit = true;
                collect_summary_lines("QUIT", &self.session)
            }
            practice::PracticeEvent::Help => vec![
                ":help    show commands".to_string(),
                ":status  show session status".to_string(),
                ":list    show drills".to_string(),
                ":skip    skip current drill".to_string(),
                ":q       quit".to_string(),
                ":quit    quit".to_string(),
            ],
            practice::PracticeEvent::Status => vec![
                format!(
                    "Current: {}/{}",
                    self.active_drill().id,
                    self.session.total_drills()
                ),
                format!(
                    "Passed: {}/{}",
                    self.session.passed(),
                    self.session.total_drills()
                ),
                format!("Skipped: {}", self.session.skipped()),
                format!("Attempts: {}", self.session.attempts()),
            ],
            practice::PracticeEvent::List => catalog::drills()
                .iter()
                .map(|drill| format!("{}  {}", drill.id, drill.expected))
                .collect(),
            practice::PracticeEvent::Skip => {
                if self.session.is_complete() {
                    self.completed = true;
                    collect_complete_lines(&self.session)
                } else {
                    vec!["SKIP".to_string()]
                }
            }
            practice::PracticeEvent::UnknownCommand => {
                vec![
                    "UNKNOWN COMMAND".to_string(),
                    "Type :help for commands.".to_string(),
                ]
            }
            practice::PracticeEvent::Correct => {
                if self.session.is_complete() {
                    self.completed = true;
                    let mut lines = Vec::from(["PASS".to_string()]);
                    lines.extend(collect_complete_lines(&self.session));
                    lines
                } else {
                    vec!["PASS".to_string()]
                }
            }
            practice::PracticeEvent::TryAgain => {
                let expected = self.active_drill().expected;
                vec![
                    "TRY AGAIN".to_string(),
                    "Expected:".to_string(),
                    expected.to_string(),
                ]
            }
        };
    }

    fn quit(&mut self) {
        self.should_quit = true;
        self.status_lines = collect_summary_lines("QUIT", &self.session);
    }
}

fn collect_complete_lines(session: &practice::PracticeSession) -> Vec<String> {
    let mut lines = vec!["COMPLETE".to_string(), "Summary:".to_string()];
    lines.extend(collect_summary_pairs(session));
    lines
}

fn collect_summary_lines(action: &str, session: &practice::PracticeSession) -> Vec<String> {
    let mut lines = vec![action.to_string()];
    if action == "QUIT" || action == "COMPLETE" {
        lines.push("Summary:".to_string());
        lines.extend(collect_summary_pairs(session));
    }
    lines
}

fn collect_summary_pairs(session: &practice::PracticeSession) -> Vec<String> {
    vec![
        format!("Passed: {}/{}", session.passed(), session.total_drills()),
        format!("Skipped: {}", session.skipped()),
        format!("Attempts: {}", session.attempts()),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::input::UiIntent;
    use super::{TuiApp, collect_summary_pairs};

    #[test]
    fn correct_input_advances_and_clears_input() {
        let mut app = TuiApp::new();
        for ch in "fn main() {}".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);

        assert_eq!(app.input(), "");
        assert_eq!(app.session().passed(), 1);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(
            app.session()
                .current_drill()
                .expect("expected second drill")
                .id,
            "002"
        );
    }

    #[test]
    fn wrong_input_tracks_attempt_but_stays_on_current_drill() {
        let mut app = TuiApp::new();
        for ch in "wrong".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Submit);

        assert_eq!(app.input(), "");
        assert_eq!(app.session().passed(), 0);
        assert_eq!(app.session().attempts(), 1);
        assert_eq!(
            app.session()
                .current_drill()
                .expect("expected first drill")
                .id,
            "001"
        );
        assert_eq!(
            app.status_lines(),
            &[
                "TRY AGAIN".to_string(),
                "Expected:".to_string(),
                "fn main() {}".to_string(),
            ]
        );
    }

    #[test]
    fn backspace_edits_input_line() {
        let mut app = TuiApp::new();

        for ch in "abc".chars() {
            app.handle_intent(UiIntent::Append(ch));
        }
        app.handle_intent(UiIntent::Backspace);

        assert_eq!(app.input(), "ab");
    }

    #[test]
    fn final_answer_marks_completion_state() {
        let mut app = TuiApp::new();
        for drill_index in 0..12 {
            let drill = app
                .session()
                .current_drill()
                .expect("expected active drill");
            for ch in drill.expected.chars() {
                app.handle_intent(UiIntent::Append(ch));
            }
            app.handle_intent(UiIntent::Submit);
            if drill_index < 11 {
                continue;
            }
        }

        assert_eq!(app.session().passed(), 12);
        assert!(app.completed());
        assert!(app.session().is_complete());
        assert_eq!(app.status_lines()[0].as_str(), "PASS");
        assert!(app.status_lines().iter().any(|line| line == "Summary:"));
        assert!(
            app.status_lines()
                .iter()
                .any(|line| line == &format!("Passed: {}/12", app.session().passed()))
        );
        assert_eq!(collect_summary_pairs(app.session())[1], "Skipped: 0");
        assert_eq!(collect_summary_pairs(app.session())[2], "Attempts: 12");
    }
}
