use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiIntent {
    Append(char),
    Backspace,
    Submit,
    Quit,
    ToggleHelp,
    Ignore,
}

pub fn classify_key_event(key: KeyEvent) -> UiIntent {
    match key.code {
        KeyCode::Esc => UiIntent::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => UiIntent::Quit,
        KeyCode::Char('?') => UiIntent::ToggleHelp,
        KeyCode::F(1) => UiIntent::ToggleHelp,
        KeyCode::Backspace => UiIntent::Backspace,
        KeyCode::Enter => UiIntent::Submit,
        KeyCode::Char(ch) => UiIntent::Append(ch),
        KeyCode::Tab => UiIntent::Ignore,
        _ => UiIntent::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::{UiIntent, classify_key_event};
    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn character_key_classifies_as_append() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Char('a')));
        assert_eq!(intent, UiIntent::Append('a'));
    }

    #[test]
    fn backspace_classifies_as_backspace() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(intent, UiIntent::Backspace);
    }

    #[test]
    fn enter_classifies_as_submit() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Enter));
        assert_eq!(intent, UiIntent::Submit);
    }

    #[test]
    fn esc_classifies_as_quit() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Esc));
        assert_eq!(intent, UiIntent::Quit);
    }

    #[test]
    fn ctrl_c_classifies_as_quit() {
        let intent = classify_key_event(KeyEvent::new(
            KeyCode::Char('c'),
            crossterm::event::KeyModifiers::CONTROL,
        ));
        assert_eq!(intent, UiIntent::Quit);
    }

    #[test]
    fn question_mark_toggles_help() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Char('?')));
        assert_eq!(intent, UiIntent::ToggleHelp);
    }

    #[test]
    fn f1_toggles_help() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(1)));
        assert_eq!(intent, UiIntent::ToggleHelp);
    }
}
