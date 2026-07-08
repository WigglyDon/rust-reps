use crate::client::{ClientAction, CursorPosition};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MousePane {
    Problem,
    Editor,
    Outside,
}

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
        KeyCode::F(7) => ClientAction::Skip,
        KeyCode::F(1) => ClientAction::Ignore,
        KeyCode::Backspace => ClientAction::Backspace,
        KeyCode::Delete => ClientAction::DeleteForward,
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

pub(crate) fn classify_mouse_event(
    mouse: MouseEvent,
    pane: MousePane,
    editor_cursor: Option<CursorPosition>,
) -> ClientAction {
    match (mouse.kind, pane) {
        (MouseEventKind::ScrollUp, MousePane::Problem) => ClientAction::ScrollProblemLineUp,
        (MouseEventKind::ScrollDown, MousePane::Problem) => ClientAction::ScrollProblemLineDown,
        (MouseEventKind::ScrollUp, MousePane::Editor) => ClientAction::ScrollEditorUp,
        (MouseEventKind::ScrollDown, MousePane::Editor) => ClientAction::ScrollEditorDown,
        (MouseEventKind::Down(MouseButton::Left), MousePane::Editor) => editor_cursor
            .map(ClientAction::MoveCursorTo)
            .unwrap_or(ClientAction::Ignore),
        _ => ClientAction::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::{ClientAction, MousePane, classify_key_event, classify_mouse_event};
    use crate::client::CursorPosition;
    use crossterm::event::{
        KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    };

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
    fn delete_classifies_as_forward_delete() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::Delete));
        assert_eq!(intent, ClientAction::DeleteForward);
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
    fn f7_classifies_as_skip() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(7)));
        assert_eq!(intent, ClientAction::Skip);
    }

    #[test]
    fn f6_is_not_a_hidden_skip_alias() {
        let intent = classify_key_event(KeyEvent::from(KeyCode::F(6)));
        assert_eq!(intent, ClientAction::Ignore);
    }

    #[test]
    fn mouse_wheel_classifies_by_target_pane() {
        let scroll_down = MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 4,
            row: 2,
            modifiers: KeyModifiers::empty(),
        };
        let scroll_up = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 4,
            row: 2,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(
            classify_mouse_event(scroll_down, MousePane::Problem, None),
            ClientAction::ScrollProblemLineDown
        );
        assert_eq!(
            classify_mouse_event(scroll_up, MousePane::Problem, None),
            ClientAction::ScrollProblemLineUp
        );
        assert_eq!(
            classify_mouse_event(scroll_down, MousePane::Editor, None),
            ClientAction::ScrollEditorDown
        );
        assert_eq!(
            classify_mouse_event(scroll_up, MousePane::Editor, None),
            ClientAction::ScrollEditorUp
        );
        assert_eq!(
            classify_mouse_event(scroll_down, MousePane::Outside, None),
            ClientAction::Ignore
        );
    }

    #[test]
    fn non_wheel_mouse_events_are_ignored() {
        let event = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 4,
            row: 2,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(
            classify_mouse_event(event, MousePane::Problem, None),
            ClientAction::Ignore
        );
    }

    #[test]
    fn editor_left_click_classifies_as_cursor_move_when_position_is_known() {
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 4,
            row: 2,
            modifiers: KeyModifiers::empty(),
        };
        let cursor = CursorPosition { line: 1, column: 3 };

        assert_eq!(
            classify_mouse_event(event, MousePane::Editor, Some(cursor)),
            ClientAction::MoveCursorTo(cursor)
        );
    }

    #[test]
    fn editor_left_click_without_position_is_ignored() {
        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 4,
            row: 2,
            modifiers: KeyModifiers::empty(),
        };

        assert_eq!(
            classify_mouse_event(event, MousePane::Editor, None),
            ClientAction::Ignore
        );
    }
}
