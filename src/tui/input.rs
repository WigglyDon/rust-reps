use crate::client::ClientAction;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn classify_key_event(key: KeyEvent) -> ClientAction {
    match key.code {
        KeyCode::Esc => ClientAction::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => ClientAction::Quit,
        KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => ClientAction::Submit,
        KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
            ClientAction::InsertNewline
        }
        KeyCode::Enter if key.modifiers.is_empty() => ClientAction::InsertNewline,
        KeyCode::F(5) => ClientAction::Submit,
        KeyCode::F(6) => ClientAction::Skip,
        KeyCode::F(1) => ClientAction::Ignore,
        KeyCode::Backspace => ClientAction::Backspace,
        KeyCode::Left => ClientAction::MoveCursorLeft,
        KeyCode::Right => ClientAction::MoveCursorRight,
        KeyCode::Up => ClientAction::MoveCursorUp,
        KeyCode::Down => ClientAction::MoveCursorDown,
        KeyCode::PageUp => ClientAction::ScrollProblemUp,
        KeyCode::PageDown => ClientAction::ScrollProblemDown,
        KeyCode::Home => ClientAction::MoveLineStart,
        KeyCode::End => ClientAction::MoveLineEnd,
        KeyCode::Char(ch)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            ClientAction::TypeChar(ch)
        }
        KeyCode::Tab => ClientAction::Ignore,
        _ => ClientAction::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::{ClientAction, classify_key_event};
    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn character_key_classifies_as_append() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Char('a')));
        assert_eq!(intent, ClientAction::TypeChar('a'));
    }

    #[test]
    fn backspace_classifies_as_backspace() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(intent, ClientAction::Backspace);
    }

    #[test]
    fn enter_classifies_as_insert_newline() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Enter));
        assert_eq!(intent, ClientAction::InsertNewline);
    }

    #[test]
    fn ctrl_enter_classifies_as_submit() {
        let intent = classify_key_event(KeyEvent::new(
            KeyCode::Enter,
            crossterm::event::KeyModifiers::CONTROL,
        ));
        assert_eq!(intent, ClientAction::Submit);
    }

    #[test]
    fn f5_classifies_as_submit() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(5)));
        assert_eq!(intent, ClientAction::Submit);
    }

    #[test]
    fn shift_enter_classifies_as_newline_when_reported() {
        let intent = classify_key_event(KeyEvent::new(
            KeyCode::Enter,
            crossterm::event::KeyModifiers::SHIFT,
        ));
        assert_eq!(intent, ClientAction::InsertNewline);
    }

    #[test]
    fn arrow_keys_classify_as_editor_navigation() {
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::Left)),
            ClientAction::MoveCursorLeft
        );
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::Right)),
            ClientAction::MoveCursorRight
        );
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::Up)),
            ClientAction::MoveCursorUp
        );
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::Down)),
            ClientAction::MoveCursorDown
        );
    }

    #[test]
    fn home_and_end_classify_as_line_navigation() {
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::Home)),
            ClientAction::MoveLineStart
        );
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::End)),
            ClientAction::MoveLineEnd
        );
    }

    #[test]
    fn page_up_and_page_down_classify_as_problem_scroll() {
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::PageUp)),
            ClientAction::ScrollProblemUp
        );
        assert_eq!(
            classify_key_event(KeyEvent::from(KeyCode::PageDown)),
            ClientAction::ScrollProblemDown
        );
    }

    #[test]
    fn esc_classifies_as_quit() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Esc));
        assert_eq!(intent, ClientAction::Quit);
    }

    #[test]
    fn ctrl_c_classifies_as_quit() {
        let intent = classify_key_event(KeyEvent::new(
            KeyCode::Char('c'),
            crossterm::event::KeyModifiers::CONTROL,
        ));
        assert_eq!(intent, ClientAction::Quit);
    }

    #[test]
    fn question_mark_inserts_like_normal_editor_text() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Char('?')));
        assert_eq!(intent, ClientAction::TypeChar('?'));
    }

    #[test]
    fn f1_is_ignored_without_default_help_overlay() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(1)));
        assert_eq!(intent, ClientAction::Ignore);
    }

    #[test]
    fn f6_classifies_as_skip() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(6)));
        assert_eq!(intent, ClientAction::Skip);
    }
}
