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
    pub focused: bool,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            focused: false,
        }
    }

    pub fn handle_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn handle_backspace(&mut self) {
        self.query.pop();
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_search_bar(f: &mut Frame, area: Rect, search: &SearchBar, theme: &Theme) {
    let cursor = if search.focused { "|" } else { "" };
    let display_text = format!("{}{}", search.query, cursor);

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
