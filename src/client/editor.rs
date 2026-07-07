#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn origin() -> Self {
        Self { line: 0, column: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditorViewport {
    pub first_line: usize,
    pub visible_lines: usize,
}

impl EditorViewport {
    fn new() -> Self {
        Self {
            first_line: 0,
            visible_lines: 0,
        }
    }

    pub fn around_cursor(total_lines: usize, cursor_line: usize, visible_lines: usize) -> Self {
        if visible_lines == 0 {
            return Self {
                first_line: 0,
                visible_lines: 0,
            };
        }

        let total_lines = total_lines.max(1);
        if total_lines <= visible_lines {
            return Self {
                first_line: 0,
                visible_lines,
            };
        }

        let cursor_line = cursor_line.min(total_lines - 1);
        let half_window = visible_lines / 2;
        let max_first_line = total_lines - visible_lines;
        let first_line = cursor_line.saturating_sub(half_window).min(max_first_line);

        Self {
            first_line,
            visible_lines,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorAction {
    InsertChar(char),
    InsertString(String),
    InsertNewline,
    Backspace,
    Clear,
    SetText(String),
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveLineStart,
    MoveLineEnd,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorView {
    pub text: String,
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
    pub viewport: EditorViewport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorBuffer {
    lines: Vec<String>,
    cursor: CursorPosition,
    viewport: EditorViewport,
}

#[allow(dead_code)]
impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: CursorPosition::origin(),
            viewport: EditorViewport::new(),
        }
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn cursor(&self) -> CursorPosition {
        self.cursor
    }

    pub fn viewport(&self) -> EditorViewport {
        self.viewport
    }

    pub fn view(&self) -> EditorView {
        EditorView {
            text: self.text(),
            lines: self.lines.clone(),
            cursor: self.cursor,
            viewport: self.viewport,
        }
    }

    pub fn apply_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::InsertChar(ch) => self.insert_char(ch),
            EditorAction::InsertString(text) => self.insert_string(&text),
            EditorAction::InsertNewline => self.insert_newline(),
            EditorAction::Backspace => self.backspace(),
            EditorAction::Clear => self.clear(),
            EditorAction::SetText(text) => self.set_text(&text),
            EditorAction::MoveLeft => self.move_left(),
            EditorAction::MoveRight => self.move_right(),
            EditorAction::MoveUp => self.move_up(),
            EditorAction::MoveDown => self.move_down(),
            EditorAction::MoveLineStart => self.move_line_start(),
            EditorAction::MoveLineEnd => self.move_line_end(),
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.clamp_cursor();
        if ch == '\n' {
            self.insert_newline();
            return;
        }

        let line = &mut self.lines[self.cursor.line];
        let byte_index = byte_index_at_char_column(line, self.cursor.column);
        line.insert(byte_index, ch);
        self.cursor.column += 1;
    }

    pub fn insert_string(&mut self, text: &str) {
        for ch in text.chars() {
            self.insert_char(ch);
        }
    }

    pub fn insert_newline(&mut self) {
        self.clamp_cursor();
        let line = &mut self.lines[self.cursor.line];
        let byte_index = byte_index_at_char_column(line, self.cursor.column);
        let remainder = line.split_off(byte_index);
        self.lines.insert(self.cursor.line + 1, remainder);
        self.cursor.line += 1;
        self.cursor.column = 0;
    }

    pub fn backspace(&mut self) {
        self.clamp_cursor();
        if self.cursor.line == 0 && self.cursor.column == 0 {
            return;
        }

        if self.cursor.column > 0 {
            let line = &mut self.lines[self.cursor.line];
            let remove_start = byte_index_at_char_column(line, self.cursor.column - 1);
            let remove_end = byte_index_at_char_column(line, self.cursor.column);
            line.replace_range(remove_start..remove_end, "");
            self.cursor.column -= 1;
            return;
        }

        let current_line = self.lines.remove(self.cursor.line);
        self.cursor.line -= 1;
        self.cursor.column = char_count(&self.lines[self.cursor.line]);
        self.lines[self.cursor.line].push_str(&current_line);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.lines.push(String::new());
        self.cursor = CursorPosition::origin();
        self.viewport.first_line = 0;
    }

    pub fn set_text(&mut self, text: &str) {
        self.lines = text.split('\n').map(ToString::to_string).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        let last_line = self.lines.len() - 1;
        self.cursor = CursorPosition {
            line: last_line,
            column: char_count(&self.lines[last_line]),
        };
        self.clamp_viewport();
    }

    pub fn set_cursor(&mut self, cursor: CursorPosition) {
        self.cursor = cursor;
        self.clamp_cursor();
    }

    pub fn move_left(&mut self) {
        self.clamp_cursor();
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = char_count(&self.lines[self.cursor.line]);
        }
    }

    pub fn move_right(&mut self) {
        self.clamp_cursor();
        let line_len = char_count(&self.lines[self.cursor.line]);
        if self.cursor.column < line_len {
            self.cursor.column += 1;
        } else if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    pub fn move_up(&mut self) {
        self.clamp_cursor();
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self
                .cursor
                .column
                .min(char_count(&self.lines[self.cursor.line]));
        }
    }

    pub fn move_down(&mut self) {
        self.clamp_cursor();
        if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            self.cursor.column = self
                .cursor
                .column
                .min(char_count(&self.lines[self.cursor.line]));
        }
    }

    pub fn move_line_start(&mut self) {
        self.clamp_cursor();
        self.cursor.column = 0;
    }

    pub fn move_line_end(&mut self) {
        self.clamp_cursor();
        self.cursor.column = char_count(&self.lines[self.cursor.line]);
    }

    pub fn clamp_cursor(&mut self) {
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor.line = self.cursor.line.min(self.lines.len() - 1);
        self.cursor.column = self
            .cursor
            .column
            .min(char_count(&self.lines[self.cursor.line]));
    }

    fn clamp_viewport(&mut self) {
        self.viewport.first_line = self.viewport.first_line.min(self.lines.len() - 1);
    }
}

impl Default for EditorBuffer {
    fn default() -> Self {
        Self::new()
    }
}

fn byte_index_at_char_column(text: &str, column: usize) -> usize {
    text.char_indices()
        .map(|(index, _)| index)
        .nth(column)
        .unwrap_or(text.len())
}

fn char_count(text: &str) -> usize {
    text.chars().count()
}

#[cfg(test)]
mod tests {
    use super::{CursorPosition, EditorAction, EditorBuffer};

    #[test]
    fn new_editor_starts_empty_at_origin() {
        let editor = EditorBuffer::new();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.lines(), &["".to_string()]);
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
        assert_eq!(editor.viewport().first_line, 0);
    }

    #[test]
    fn viewport_around_cursor_keeps_cursor_line_visible() {
        let viewport = super::EditorViewport::around_cursor(8, 6, 3);

        assert_eq!(viewport.visible_lines, 3);
        assert!(viewport.first_line <= 6);
        assert!(6 < viewport.first_line + viewport.visible_lines);
    }

    #[test]
    fn insert_char_updates_text_and_cursor() {
        let mut editor = EditorBuffer::new();

        editor.insert_char('a');

        assert_eq!(editor.text(), "a");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn insert_string_updates_text_and_cursor() {
        let mut editor = EditorBuffer::new();

        editor.insert_string("fn main()");

        assert_eq!(editor.text(), "fn main()");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 9 });
    }

    #[test]
    fn backspace_removes_previous_char() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("abc");

        editor.backspace();

        assert_eq!(editor.text(), "ab");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn backspace_at_beginning_is_safe() {
        let mut editor = EditorBuffer::new();

        editor.backspace();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn insert_newline_creates_multiline_buffer() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("let");

        editor.insert_newline();
        editor.insert_string("score");

        assert_eq!(editor.text(), "let\nscore");
        assert_eq!(editor.lines(), &["let".to_string(), "score".to_string()]);
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 5 });
    }

    #[test]
    fn backspace_at_start_of_second_line_joins_lines() {
        let mut editor = EditorBuffer::new();
        editor.set_text("let\nscore");
        editor.set_cursor(CursorPosition { line: 1, column: 0 });

        editor.backspace();

        assert_eq!(editor.text(), "letscore");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 3 });
    }

    #[test]
    fn cursor_left_and_right_move_and_clamp() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("ab");

        editor.move_left();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
        editor.move_left();
        editor.move_left();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
        editor.move_right();
        editor.move_right();
        editor.move_right();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn cursor_up_and_down_move_and_clamp_across_uneven_lines() {
        let mut editor = EditorBuffer::new();
        editor.set_text("abcd\nef\nghijk");
        editor.set_cursor(CursorPosition { line: 0, column: 4 });

        editor.move_down();
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 2 });
        editor.move_down();
        assert_eq!(editor.cursor(), CursorPosition { line: 2, column: 2 });
        editor.move_up();
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 2 });
        editor.move_up();
        editor.move_up();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn cursor_line_start_and_end_move_and_clamp() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("abcd");

        editor.move_line_start();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
        editor.move_line_end();
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 4 });
    }

    #[test]
    fn set_text_handles_single_line_text() {
        let mut editor = EditorBuffer::new();

        editor.set_text("let score = 0;");

        assert_eq!(editor.text(), "let score = 0;");
        assert_eq!(editor.lines(), &["let score = 0;".to_string()]);
        assert_eq!(
            editor.cursor(),
            CursorPosition {
                line: 0,
                column: 14
            }
        );
    }

    #[test]
    fn set_text_handles_multiline_text() {
        let mut editor = EditorBuffer::new();

        editor.set_text("fn main() {\n}");

        assert_eq!(editor.text(), "fn main() {\n}");
        assert_eq!(
            editor.lines(),
            &["fn main() {".to_string(), "}".to_string()]
        );
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 1 });
    }

    #[test]
    fn clear_resets_text_and_cursor() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() {\n}");

        editor.clear();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.lines(), &["".to_string()]);
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn text_reconstruction_preserves_exact_content() {
        let mut editor = EditorBuffer::new();
        for text in ["", "a", "a\n", "a\nb\n", "fn main() {}\r\nnext"] {
            editor.set_text(text);
            assert_eq!(editor.text(), text);
        }
    }

    #[test]
    fn unicode_columns_are_char_based_and_do_not_panic() {
        let mut editor = EditorBuffer::new();

        editor.insert_string("é🙂x");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 3 });
        editor.backspace();
        editor.move_left();
        editor.insert_char('!');

        assert_eq!(editor.text(), "é!🙂");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn apply_action_routes_basic_editor_actions() {
        let mut editor = EditorBuffer::new();

        editor.apply_action(EditorAction::InsertString("ab".to_string()));
        editor.apply_action(EditorAction::MoveLeft);
        editor.apply_action(EditorAction::InsertChar('!'));
        editor.apply_action(EditorAction::MoveRight);
        editor.apply_action(EditorAction::InsertNewline);
        editor.apply_action(EditorAction::InsertString("cd".to_string()));
        editor.apply_action(EditorAction::MoveUp);
        editor.apply_action(EditorAction::MoveDown);
        editor.apply_action(EditorAction::MoveLineStart);
        editor.apply_action(EditorAction::MoveLineEnd);
        editor.apply_action(EditorAction::Backspace);
        editor.apply_action(EditorAction::SetText("reset".to_string()));
        editor.apply_action(EditorAction::Clear);

        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn view_contains_read_only_projection_data() {
        let mut editor = EditorBuffer::new();
        editor.set_text("one\ntwo");

        let view = editor.view();

        assert_eq!(view.text, "one\ntwo");
        assert_eq!(view.lines, vec!["one".to_string(), "two".to_string()]);
        assert_eq!(view.cursor, CursorPosition { line: 1, column: 3 });
        assert_eq!(view.viewport.first_line, 0);
    }

    #[test]
    fn editor_public_api_stays_terminal_free() {
        let public_source = include_str!("editor.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("expected editor public source");

        for forbidden in [
            format!("{}{}", "rata", "tui"),
            format!("{}{}", "cross", "term"),
            "tree_sitter".to_string(),
            "tree-sitter".to_string(),
        ] {
            assert!(
                !public_source.contains(&forbidden),
                "client editor public API should not mention platform/parser crate {forbidden}"
            );
        }
    }
}
