#![allow(dead_code)]
use super::super::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct InputField {
    pub label: String,
    pub value: String,
    pub focused: bool,
}

impl InputField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            focused: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn handle_char(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn handle_backspace(&mut self) {
        self.value.pop();
    }
}

pub struct InputDialog {
    pub title: String,
    pub fields: Vec<InputField>,
    pub current_field: usize,
    pub mode: DialogMode,
    pub message: Option<(String, bool)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogMode {
    Add,
    Edit,
    Delete,
    Run,
    Input,
}

impl InputDialog {
    pub fn new(title: impl Into<String>, mode: DialogMode) -> Self {
        Self {
            title: title.into(),
            fields: Vec::new(),
            current_field: 0,
            mode,
            message: None,
        }
    }

    pub fn add_field(mut self, label: impl Into<String>) -> Self {
        self.fields.push(InputField::new(label));
        self.update_focus();
        self
    }

    pub fn add_field_with_value(
        mut self,
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.fields.push(InputField::new(label).with_value(value));
        self.update_focus();
        self
    }

    pub fn next_field(&mut self) {
        if !self.fields.is_empty() {
            self.current_field = (self.current_field + 1) % self.fields.len();
            self.update_focus();
        }
    }

    pub fn prev_field(&mut self) {
        if !self.fields.is_empty() {
            self.current_field = if self.current_field == 0 {
                self.fields.len() - 1
            } else {
                self.current_field - 1
            };
            self.update_focus();
        }
    }

    pub fn update_focus(&mut self) {
        for (i, field) in self.fields.iter_mut().enumerate() {
            field.focused = i == self.current_field;
        }
    }

    pub fn current_field_mut(&mut self) -> Option<&mut InputField> {
        self.fields.get_mut(self.current_field)
    }

    pub fn get_values(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|f| (f.label.clone(), f.value.clone()))
            .collect()
    }

    pub fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

pub fn render_input_dialog(f: &mut Frame, dialog: &InputDialog, theme: &Theme) {
    let area = f.area();
    let popup_area = dialog.centered_rect(60, 60, area);

    f.render_widget(Clear, popup_area);

    let title_str = format!(" {} ", dialog.title);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style(true))
        .border_type(theme.border_type())
        .title(Line::from(theme.gradient_text(&title_str)))
        .style(Style::default().bg(theme.surface));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let constraints: Vec<Constraint> = dialog
        .fields
        .iter()
        .map(|_| Constraint::Length(3))
        .chain(std::iter::once(Constraint::Min(1)))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(inner_area);

    for (i, (field, chunk)) in dialog.fields.iter().zip(chunks.iter()).enumerate() {
        let is_focused = i == dialog.current_field;

        let color = theme.gradient_color(i as f32 / dialog.fields.len().max(1) as f32);

        let field_block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style(is_focused))
            .border_type(theme.border_type())
            .title(ratatui::text::Span::styled(
                format!(" {} ", field.label),
                if is_focused {
                    Style::default()
                        .fg(color)
                        .add_modifier(ratatui::style::Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_secondary)
                },
            ));

        let style = if is_focused {
            Style::default().fg(color)
        } else {
            Style::default().fg(theme.text_secondary)
        };

        let cursor = if is_focused { "|" } else { "" };
        let display_text = format!("{}{}", field.value, cursor);

        let paragraph = Paragraph::new(display_text).style(style).block(field_block);

        f.render_widget(paragraph, *chunk);
    }

    if let Some((msg, is_error)) = &dialog.message {
        let msg_style = if *is_error {
            theme.error_style()
        } else {
            theme.success_style()
        };

        let msg_chunk = chunks.last().unwrap();
        let msg_paragraph = Paragraph::new(msg.as_str())
            .style(msg_style)
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(msg_paragraph, *msg_chunk);
    }
}
