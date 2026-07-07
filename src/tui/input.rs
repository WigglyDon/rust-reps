use crate::client::ClientAction;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn classify_key_event(key: KeyEvent) -> ClientAction {
    match key.code {
        KeyCode::Esc => ClientAction::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => ClientAction::Quit,
        KeyCode::Char('?') => ClientAction::ToggleHelp,
        KeyCode::F(1) => ClientAction::ToggleHelp,
        KeyCode::Backspace => ClientAction::Backspace,
        KeyCode::Enter => ClientAction::Submit,
        KeyCode::Left => ClientAction::MoveCursorLeft,
        KeyCode::Right => ClientAction::MoveCursorRight,
        KeyCode::Up => ClientAction::MoveCursorUp,
        KeyCode::Down => ClientAction::MoveCursorDown,
        KeyCode::Home => ClientAction::MoveLineStart,
        KeyCode::End => ClientAction::MoveLineEnd,
        KeyCode::Char(ch) => ClientAction::TypeChar(ch),
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
    fn enter_classifies_as_submit() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Enter));
        assert_eq!(intent, ClientAction::Submit);
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
    fn question_mark_toggles_help() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Char('?')));
        assert_eq!(intent, ClientAction::ToggleHelp);
    }

    #[test]
    fn f1_toggles_help() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(1)));
        assert_eq!(intent, ClientAction::ToggleHelp);
    }
}
