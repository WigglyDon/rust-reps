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
    pub first_column: usize,
    pub visible_columns: usize,
}

impl EditorViewport {
    fn new() -> Self {
        Self {
            first_line: 0,
            visible_lines: 5,
            first_column: 0,
            visible_columns: 80,
        }
    }

    #[cfg(test)]
    pub fn around_cursor(total_lines: usize, cursor_line: usize, visible_lines: usize) -> Self {
        if visible_lines == 0 {
            return Self {
                first_line: 0,
                visible_lines: 0,
                first_column: 0,
                visible_columns: 80,
            };
        }

        let total_lines = total_lines.max(1);
        if total_lines <= visible_lines {
            return Self {
                first_line: 0,
                visible_lines,
                first_column: 0,
                visible_columns: 80,
            };
        }

        let cursor_line = cursor_line.min(total_lines - 1);
        let half_window = visible_lines / 2;
        let max_first_line = total_lines - visible_lines;
        let first_line = cursor_line.saturating_sub(half_window).min(max_first_line);

        Self {
            first_line,
            visible_lines,
            first_column: 0,
            visible_columns: 80,
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
    DeleteForward,
    Clear,
    SetText(String),
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveLineStart,
    MoveLineEnd,
    ScrollViewportUp,
    ScrollViewportDown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorView {
    pub text: String,
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
    pub viewport: EditorViewport,
}

#[allow(dead_code)]
impl EditorView {
    pub fn visible_lines(&self) -> &[String] {
        let first_line = self
            .viewport
            .first_line
            .min(self.lines.len().saturating_sub(1));
        let end = first_line
            .saturating_add(self.viewport.visible_lines)
            .min(self.lines.len());
        &self.lines[first_line..end]
    }

    pub fn caret_row(&self) -> usize {
        self.cursor.line.saturating_sub(self.viewport.first_line)
    }
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
            EditorAction::DeleteForward => self.delete_forward(),
            EditorAction::Clear => self.clear(),
            EditorAction::SetText(text) => self.set_text(&text),
            EditorAction::MoveLeft => self.move_left(),
            EditorAction::MoveRight => self.move_right(),
            EditorAction::MoveUp => self.move_up(),
            EditorAction::MoveDown => self.move_down(),
            EditorAction::MoveLineStart => self.move_line_start(),
            EditorAction::MoveLineEnd => self.move_line_end(),
            EditorAction::ScrollViewportUp => self.scroll_viewport_up(),
            EditorAction::ScrollViewportDown => self.scroll_viewport_down(),
        }
    }

    pub fn set_viewport_height(&mut self, visible_lines: usize) {
        self.viewport.visible_lines = visible_lines;
        self.clamp_viewport();
        self.ensure_cursor_visible();
    }

    pub fn set_viewport_width(&mut self, visible_columns: usize) {
        self.viewport.visible_columns = visible_columns.max(1);
        self.clamp_viewport();
        self.ensure_cursor_visible();
    }

    pub fn scroll_viewport_up(&mut self) {
        self.viewport.first_line = self.viewport.first_line.saturating_sub(1);
        self.clamp_viewport();
    }

    pub fn scroll_viewport_down(&mut self) {
        self.viewport.first_line = self.viewport.first_line.saturating_add(1);
        self.clamp_viewport();
    }

    pub fn insert_char(&mut self, ch: char) {
        self.clamp_cursor();
        if ch == '\n' {
            self.insert_newline();
            return;
        }

        if let Some(close) = matching_close_for_open(ch) {
            if self.next_char_is(close) {
                self.insert_raw_char(ch);
            } else {
                self.insert_pair(ch, close);
            }
            return;
        }

        if is_closing_delimiter(ch) && self.next_char_is(ch) {
            self.cursor.column += 1;
            self.ensure_cursor_visible();
            return;
        }

        self.insert_raw_char(ch);
    }

    pub fn insert_string(&mut self, text: &str) {
        for ch in text.chars() {
            self.insert_char(ch);
        }
    }

    pub fn insert_newline(&mut self) {
        self.clamp_cursor();
        let line_index = self.cursor.line;
        let byte_index = byte_index_at_char_column(&self.lines[line_index], self.cursor.column);
        let before_cursor = self.lines[line_index][..byte_index].to_string();
        let after_cursor = self.lines[line_index][byte_index..].to_string();
        let base_indent = leading_spaces(&self.lines[line_index]);
        let after_trimmed = after_cursor.trim();

        if before_cursor.trim_end().ends_with('{') && after_trimmed == "}" {
            self.expand_brace_block(line_index, before_cursor, base_indent, true);
            return;
        }

        if before_cursor.trim_end().ends_with('{') && after_trimmed.is_empty() {
            let insert_closing_brace =
                !self.next_meaningful_line_is_closing_brace_at_indent(&base_indent);
            self.expand_brace_block(line_index, before_cursor, base_indent, insert_closing_brace);
            return;
        }

        self.lines[line_index] = before_cursor;
        self.lines
            .insert(line_index + 1, format!("{base_indent}{after_cursor}"));
        self.cursor.line = line_index + 1;
        self.cursor.column = char_count(&base_indent);
        self.ensure_cursor_visible();
    }

    fn insert_pair(&mut self, open: char, close: char) {
        let line = &mut self.lines[self.cursor.line];
        let byte_index = byte_index_at_char_column(line, self.cursor.column);
        line.insert(byte_index, open);
        line.insert(byte_index + open.len_utf8(), close);
        self.cursor.column += 1;
        self.ensure_cursor_visible();
    }

    fn insert_raw_char(&mut self, ch: char) {
        let line = &mut self.lines[self.cursor.line];
        let byte_index = byte_index_at_char_column(line, self.cursor.column);
        line.insert(byte_index, ch);
        self.cursor.column += 1;
        self.ensure_cursor_visible();
    }

    fn expand_brace_block(
        &mut self,
        line_index: usize,
        before_cursor: String,
        base_indent: String,
        insert_closing_brace: bool,
    ) {
        let inner_indent = format!("{base_indent}    ");
        self.lines[line_index] = before_cursor;
        self.lines.insert(line_index + 1, inner_indent.clone());
        if insert_closing_brace {
            self.lines
                .insert(line_index + 2, format!("{base_indent}}}"));
        }
        self.cursor.line = line_index + 1;
        self.cursor.column = char_count(&inner_indent);
        self.ensure_cursor_visible();
    }

    pub fn backspace(&mut self) {
        self.clamp_cursor();
        if self.cursor.line == 0 && self.cursor.column == 0 {
            return;
        }

        if self.cursor.column > 0 {
            if self.remove_empty_pair_before_cursor() {
                self.ensure_cursor_visible();
                return;
            }
            let line = &mut self.lines[self.cursor.line];
            let remove_start = byte_index_at_char_column(line, self.cursor.column - 1);
            let remove_end = byte_index_at_char_column(line, self.cursor.column);
            line.replace_range(remove_start..remove_end, "");
            self.cursor.column -= 1;
            self.ensure_cursor_visible();
            return;
        }

        let current_line = self.lines.remove(self.cursor.line);
        self.cursor.line -= 1;
        self.cursor.column = char_count(&self.lines[self.cursor.line]);
        self.lines[self.cursor.line].push_str(&current_line);
        self.ensure_cursor_visible();
    }

    pub fn delete_forward(&mut self) {
        self.clamp_cursor();
        let line_len = char_count(&self.lines[self.cursor.line]);
        if self.cursor.column < line_len {
            let line = &mut self.lines[self.cursor.line];
            let remove_start = byte_index_at_char_column(line, self.cursor.column);
            let remove_end = byte_index_at_char_column(line, self.cursor.column + 1);
            line.replace_range(remove_start..remove_end, "");
            self.ensure_cursor_visible();
            return;
        }

        if self.cursor.line + 1 >= self.lines.len() {
            self.ensure_cursor_visible();
            return;
        }

        let next_line = self.lines.remove(self.cursor.line + 1);
        self.lines[self.cursor.line].push_str(&next_line);
        self.ensure_cursor_visible();
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.lines.push(String::new());
        self.cursor = CursorPosition::origin();
        self.viewport.first_line = 0;
        self.viewport.first_column = 0;
        self.ensure_cursor_visible();
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
        self.ensure_cursor_visible();
    }

    pub fn set_cursor(&mut self, cursor: CursorPosition) {
        self.cursor = cursor;
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub fn move_left(&mut self) {
        self.clamp_cursor();
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = char_count(&self.lines[self.cursor.line]);
        }
        self.ensure_cursor_visible();
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
        self.ensure_cursor_visible();
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
        self.ensure_cursor_visible();
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
        self.ensure_cursor_visible();
    }

    pub fn move_line_start(&mut self) {
        self.clamp_cursor();
        self.cursor.column = 0;
        self.ensure_cursor_visible();
    }

    pub fn move_line_end(&mut self) {
        self.clamp_cursor();
        self.cursor.column = char_count(&self.lines[self.cursor.line]);
        self.ensure_cursor_visible();
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

    fn ensure_cursor_visible(&mut self) {
        self.clamp_cursor();
        if self.viewport.visible_lines == 0 {
            self.viewport.first_line = 0;
            return;
        }

        let max_first_line = self.lines.len().saturating_sub(self.viewport.visible_lines);
        let line_len = char_count(&self.lines[self.cursor.line]);
        let max_first_column = line_len
            .saturating_add(1)
            .saturating_sub(self.viewport.visible_columns.max(1));
        if self.cursor.line < self.viewport.first_line {
            self.viewport.first_line = self.cursor.line;
        } else if self.cursor.line >= self.viewport.first_line + self.viewport.visible_lines {
            self.viewport.first_line = self.cursor.line + 1 - self.viewport.visible_lines;
        }
        if self.cursor.column < self.viewport.first_column {
            self.viewport.first_column = self.cursor.column;
        } else if self.cursor.column >= self.viewport.first_column + self.viewport.visible_columns {
            self.viewport.first_column =
                self.cursor.column + 1 - self.viewport.visible_columns.max(1);
        }

        self.viewport.first_line = self.viewport.first_line.min(max_first_line);
        self.viewport.first_column = self.viewport.first_column.min(max_first_column);
    }

    fn clamp_viewport(&mut self) {
        let max_first_line = self.lines.len().saturating_sub(self.viewport.visible_lines);
        let line_len = self
            .lines
            .get(self.cursor.line.min(self.lines.len().saturating_sub(1)))
            .map(|line| char_count(line))
            .unwrap_or(0);
        let max_first_column = line_len
            .saturating_add(1)
            .saturating_sub(self.viewport.visible_columns.max(1));
        self.viewport.first_line = self.viewport.first_line.min(max_first_line);
        self.viewport.first_column = self.viewport.first_column.min(max_first_column);
    }

    fn next_char_is(&self, ch: char) -> bool {
        self.lines[self.cursor.line].chars().nth(self.cursor.column) == Some(ch)
    }

    fn remove_empty_pair_before_cursor(&mut self) -> bool {
        let line = &mut self.lines[self.cursor.line];
        let Some(open) = char_at(line, self.cursor.column - 1) else {
            return false;
        };
        let Some(close) = matching_close_for_open(open) else {
            return false;
        };
        if char_at(line, self.cursor.column) != Some(close) {
            return false;
        }

        let remove_start = byte_index_at_char_column(line, self.cursor.column - 1);
        let remove_end = byte_index_at_char_column(line, self.cursor.column + 1);
        line.replace_range(remove_start..remove_end, "");
        self.cursor.column -= 1;
        true
    }

    fn next_meaningful_line_is_closing_brace_at_indent(&self, indent: &str) -> bool {
        self.lines
            .iter()
            .skip(self.cursor.line + 1)
            .find(|line| !line.trim().is_empty())
            .is_some_and(|line| line.trim() == "}" && leading_spaces(line) == indent)
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

fn leading_spaces(text: &str) -> String {
    text.chars().take_while(|ch| *ch == ' ').collect()
}

fn char_at(text: &str, column: usize) -> Option<char> {
    text.chars().nth(column)
}

fn matching_close_for_open(ch: char) -> Option<char> {
    match ch {
        '{' => Some('}'),
        '(' => Some(')'),
        '[' => Some(']'),
        _ => None,
    }
}

fn is_closing_delimiter(ch: char) -> bool {
    matches!(ch, '}' | ')' | ']')
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
    fn cursor_moving_below_viewport_advances_editor_viewport() {
        let mut editor = EditorBuffer::new();
        editor.set_text("one\ntwo\nthree\nfour\nfive");
        editor.set_cursor(CursorPosition { line: 0, column: 0 });
        editor.set_viewport_height(3);

        editor.move_down();
        editor.move_down();
        assert_eq!(editor.viewport().first_line, 0);

        editor.move_down();
        assert_eq!(editor.cursor(), CursorPosition { line: 3, column: 0 });
        assert_eq!(editor.viewport().first_line, 1);

        editor.move_down();
        assert_eq!(editor.viewport().first_line, 2);
    }

    #[test]
    fn cursor_moving_above_viewport_retreats_editor_viewport() {
        let mut editor = EditorBuffer::new();
        editor.set_text("one\ntwo\nthree\nfour\nfive");
        editor.set_viewport_height(3);
        editor.set_cursor(CursorPosition { line: 4, column: 0 });
        assert_eq!(editor.viewport().first_line, 2);

        editor.move_up();
        editor.move_up();
        assert_eq!(editor.viewport().first_line, 2);

        editor.move_up();
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 0 });
        assert_eq!(editor.viewport().first_line, 1);
    }

    #[test]
    fn visible_editor_lines_are_sliced_from_buffer_viewport() {
        let mut editor = EditorBuffer::new();
        editor.set_text("one\ntwo\nthree\nfour\nfive");
        editor.set_viewport_height(2);
        editor.set_cursor(CursorPosition { line: 0, column: 0 });
        editor.move_down();
        editor.move_down();
        editor.move_down();

        let view = editor.view();

        assert_eq!(view.viewport.first_line, 2);
        assert_eq!(
            view.visible_lines(),
            &["three".to_string(), "four".to_string()]
        );
        assert_eq!(view.caret_row(), 1);
    }

    #[test]
    fn viewport_movement_does_not_change_buffer_content() {
        let mut editor = EditorBuffer::new();
        editor.set_text("one\ntwo\nthree\nfour\nfive");
        let before = editor.text();
        editor.set_viewport_height(2);

        editor.move_up();
        editor.move_down();
        editor.move_down();
        editor.move_down();
        editor.move_up();

        assert_eq!(editor.text(), before);
    }

    #[test]
    fn insert_char_updates_text_and_cursor() {
        let mut editor = EditorBuffer::new();

        editor.insert_char('a');

        assert_eq!(editor.text(), "a");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn typing_open_brace_creates_pair_with_cursor_between() {
        let mut editor = EditorBuffer::new();

        editor.insert_char('{');

        assert_eq!(editor.text(), "{}");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn typing_open_paren_creates_pair_with_cursor_between() {
        let mut editor = EditorBuffer::new();

        editor.insert_char('(');

        assert_eq!(editor.text(), "()");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn typing_open_bracket_creates_pair_with_cursor_between() {
        let mut editor = EditorBuffer::new();

        editor.insert_char('[');

        assert_eq!(editor.text(), "[]");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn smart_pair_insertion_preserves_surrounding_text() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main");
        editor.set_cursor(CursorPosition { line: 0, column: 3 });

        editor.insert_char('{');

        assert_eq!(editor.text(), "fn {}main");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 4 });
    }

    #[test]
    fn smart_pair_insertion_preserves_indentation_without_tabs() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("    ");

        editor.insert_char('{');

        assert_eq!(editor.text(), "    {}");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 5 });
        assert!(!editor.text().contains('\t'));
    }

    #[test]
    fn opening_delimiter_reuses_existing_matching_close() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() }");
        editor.set_cursor(CursorPosition {
            line: 0,
            column: "fn main() ".chars().count(),
        });

        editor.insert_char('{');

        assert_eq!(editor.text(), "fn main() {}");
        assert_eq!(
            editor.cursor(),
            CursorPosition {
                line: 0,
                column: "fn main() {".chars().count()
            }
        );
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
    fn closing_brace_skips_over_generated_pair_close() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('{');

        editor.insert_char('}');

        assert_eq!(editor.text(), "{}");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn closing_paren_skips_over_generated_pair_close() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('(');

        editor.insert_char(')');

        assert_eq!(editor.text(), "()");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn closing_bracket_skips_over_generated_pair_close() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('[');

        editor.insert_char(']');

        assert_eq!(editor.text(), "[]");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
    }

    #[test]
    fn closing_delimiter_inserts_normally_when_next_char_differs() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("abc");
        editor.set_cursor(CursorPosition { line: 0, column: 1 });

        editor.insert_char('}');

        assert_eq!(editor.text(), "a}bc");
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
    fn backspace_between_empty_braces_deletes_both_delimiters() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('{');

        editor.backspace();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn backspace_between_empty_parens_deletes_both_delimiters() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('(');

        editor.backspace();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn backspace_between_empty_brackets_deletes_both_delimiters() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('[');

        editor.backspace();

        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 0 });
    }

    #[test]
    fn backspace_inside_non_empty_pair_deletes_previous_character_only() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('{');
        editor.insert_char('x');

        editor.backspace();

        assert_eq!(editor.text(), "{}");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
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
    fn smart_newline_after_open_brace_indents_and_inserts_closing_brace() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("fn main() {");

        editor.insert_newline();

        assert_eq!(editor.text(), "fn main() {\n    \n}");
        assert_eq!(
            editor.lines(),
            &[
                "fn main() {".to_string(),
                "    ".to_string(),
                "}".to_string()
            ]
        );
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_after_raw_open_brace_inserts_closing_brace() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() {");

        editor.insert_newline();

        assert_eq!(editor.text(), "fn main() {\n    \n}");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_after_indented_open_brace_uses_nested_space_indent() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("    if true {");

        editor.insert_newline();

        assert_eq!(editor.text(), "    if true {\n        \n    }");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 8 });
        assert_eq!(editor.lines()[2], "    }");
    }

    #[test]
    fn smart_newline_between_generated_braces_expands_block() {
        let mut editor = EditorBuffer::new();
        editor.insert_char('{');

        editor.insert_newline();

        assert_eq!(editor.text(), "{\n    \n}");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_expands_nested_generated_brace_pair() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() {\n    if true {}\n}");
        editor.set_cursor(CursorPosition {
            line: 1,
            column: "    if true {".chars().count(),
        });

        editor.insert_newline();

        assert_eq!(
            editor.text(),
            "fn main() {\n    if true {\n        \n    }\n}"
        );
        assert_eq!(editor.cursor(), CursorPosition { line: 2, column: 8 });
        assert_eq!(editor.lines()[3], "    }");
        assert_eq!(editor.lines()[4], "}");
    }

    #[test]
    fn smart_newline_after_normal_line_copies_current_indentation() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("    let score = 0;");

        editor.insert_newline();

        assert_eq!(editor.text(), "    let score = 0;\n    ");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_in_middle_splits_right_text_with_current_indentation() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("    let score");
        editor.set_cursor(CursorPosition { line: 0, column: 8 });

        editor.insert_newline();

        assert_eq!(editor.text(), "    let \n    score");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_before_non_whitespace_right_text_does_not_insert_closing_brace() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() { body");
        editor.set_cursor(CursorPosition {
            line: 0,
            column: "fn main() {".chars().count(),
        });

        editor.insert_newline();

        assert_eq!(editor.text(), "fn main() {\n body");
        assert_eq!(editor.lines().len(), 2);
    }

    #[test]
    fn smart_newline_does_not_duplicate_existing_closing_brace() {
        let mut editor = EditorBuffer::new();
        editor.set_text("fn main() {\n}");
        editor.set_cursor(CursorPosition {
            line: 0,
            column: "fn main() {".chars().count(),
        });

        editor.insert_newline();

        assert_eq!(editor.text(), "fn main() {\n    \n}");
        assert_eq!(editor.lines().len(), 3);
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
    }

    #[test]
    fn smart_newline_preserves_blank_line_space_indent_without_tabs() {
        let mut editor = EditorBuffer::new();
        editor.insert_string("    ");

        editor.insert_newline();

        assert_eq!(editor.text(), "    \n    ");
        assert_eq!(editor.cursor(), CursorPosition { line: 1, column: 4 });
        assert!(!editor.text().contains('\t'));
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
    fn delete_forward_removes_character_after_cursor() {
        let mut editor = EditorBuffer::new();
        editor.set_text("abcd");
        editor.set_cursor(CursorPosition { line: 0, column: 1 });

        editor.delete_forward();

        assert_eq!(editor.text(), "acd");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 1 });
    }

    #[test]
    fn delete_forward_at_line_end_joins_next_line() {
        let mut editor = EditorBuffer::new();
        editor.set_text("let\nscore");
        editor.set_cursor(CursorPosition { line: 0, column: 3 });

        editor.delete_forward();

        assert_eq!(editor.text(), "letscore");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 3 });
    }

    #[test]
    fn delete_forward_at_final_line_end_is_safe() {
        let mut editor = EditorBuffer::new();
        editor.set_text("abc");
        editor.set_cursor(CursorPosition { line: 0, column: 3 });

        editor.delete_forward();

        assert_eq!(editor.text(), "abc");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 3 });
    }

    #[test]
    fn typing_past_visible_width_advances_horizontal_viewport() {
        let mut editor = EditorBuffer::new();
        editor.set_viewport_width(4);
        editor.insert_string("abcdef");

        assert_eq!(editor.text(), "abcdef");
        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 6 });
        assert_eq!(editor.viewport().first_column, 3);
    }

    #[test]
    fn moving_left_past_visible_start_retreats_horizontal_viewport() {
        let mut editor = EditorBuffer::new();
        editor.set_viewport_width(4);
        editor.insert_string("abcdef");

        editor.move_left();
        editor.move_left();
        editor.move_left();
        editor.move_left();

        assert_eq!(editor.cursor(), CursorPosition { line: 0, column: 2 });
        assert_eq!(editor.viewport().first_column, 2);
        editor.move_left();
        assert_eq!(editor.viewport().first_column, 1);
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
