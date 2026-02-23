#![allow(dead_code)]
use super::super::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct HelpDialog {
    pub visible: bool,
}

impl HelpDialog {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

impl Default for HelpDialog {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_help_dialog(f: &mut Frame, theme: &Theme) {
    let area = f.area();

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1])[1];

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style(true))
        .border_type(theme.border_type())
        .title(Line::from(theme.gradient_text(" KEYBOARD SHORTCUTS ")))
        .style(Style::default().bg(theme.surface));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let shortcuts = [
        ("Tab", "Switch tabs"),
        ("j/k", "Navigate list"),
        ("/", "Search"),
        ("a", "Add new"),
        ("e", "Edit"),
        ("d", "Delete"),
        ("Enter", "Run snippet"),
        ("Esc", "Cancel"),
        ("?", "Toggle help"),
        ("q", "Quit"),
    ];

    let lines: Vec<Line> = shortcuts
        .iter()
        .enumerate()
        .map(|(i, (key, action))| {
            let color = theme.gradient_color(i as f32 / shortcuts.len() as f32);
            Line::from(vec![
                ratatui::text::Span::styled(
                    format!("  {:8}", key),
                    Style::default()
                        .fg(color)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                ratatui::text::Span::styled(*action, Style::default().fg(theme.text_secondary)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(Style::default().bg(theme.surface));

    f.render_widget(paragraph, inner_area);
}
