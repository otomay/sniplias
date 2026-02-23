use super::super::Theme;
use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct SearchBar {
    pub query: String,
    pub cursor_pos: usize,
    pub focused: bool,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            cursor_pos: 0,
            focused: false,
        }
    }

    pub fn handle_char(&mut self, c: char) {
        self.query.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.query.remove(self.cursor_pos);
        }
    }

    pub fn handle_delete(&mut self) {
        if self.cursor_pos < self.query.len() {
            self.query.remove(self.cursor_pos);
        }
    }

    pub fn handle_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn handle_right(&mut self) {
        if self.cursor_pos < self.query.len() {
            self.cursor_pos += 1;
        }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_search_bar(f: &mut Frame, area: Rect, search: &SearchBar, theme: &Theme) {
    let display_text = if search.focused {
        let prefix = &search.query[..search.cursor_pos];
        let suffix = &search.query[search.cursor_pos..];
        format!("{}|{}", prefix, suffix)
    } else {
        search.query.clone()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style(true))
        .border_type(theme.border_type())
        .title(Line::from(theme.gradient_text(" SEARCH ")));

    let style = if search.focused {
        Style::default().fg(theme.gradient_start)
    } else {
        Style::default().fg(theme.text_secondary)
    };

    let paragraph = Paragraph::new(display_text).style(style).block(block);

    f.render_widget(paragraph, area);
}
