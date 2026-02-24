use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::Theme;

#[derive(Debug, Clone)]
pub struct InputField {
    pub label: String,
    pub value: String,
    pub cursor_position: usize,
}

impl InputField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            cursor_position: 0,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_position = self.value.len();
        self
    }

    pub fn handle_char(&mut self, c: char) {
        if self.cursor_position <= self.value.len() {
            self.value.insert(self.cursor_position, c);
        } else {
            self.value.push(c);
        }
        self.cursor_position += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_position > 0 && !self.value.is_empty() {
            self.value.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn handle_delete(&mut self) {
        if self.cursor_position < self.value.len() {
            self.value.remove(self.cursor_position);
        }
    }

    pub fn handle_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn handle_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
        }
    }
}

#[derive(Debug, Clone)]
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
    Update,
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

    pub fn current_field(&self) -> Option<&InputField> {
        self.fields.get(self.current_field)
    }

    pub fn current_field_mut(&mut self) -> Option<&mut InputField> {
        self.fields.get_mut(self.current_field)
    }

    pub fn next_field(&mut self) {
        if !self.fields.is_empty() {
            self.current_field = (self.current_field + 1) % self.fields.len();
        }
    }

    pub fn prev_field(&mut self) {
        if !self.fields.is_empty() {
            self.current_field = if self.current_field == 0 {
                self.fields.len() - 1
            } else {
                self.current_field - 1
            };
        }
    }

    fn update_focus(&mut self) {
        if self.current_field >= self.fields.len() {
            self.current_field = self.fields.len().saturating_sub(1);
        }
    }

    pub fn get_values(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|f| (f.label.clone(), f.value.clone()))
            .collect()
    }
}

pub fn render_input_dialog(f: &mut Frame, dialog: &InputDialog, theme: &Theme) {
    let area = f.area();
    let width = std::cmp::min(60, area.width - 4);
    let height = std::cmp::min(dialog.fields.len() as u16 * 2 + 5, area.height - 2);

    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;

    let block = Block::default()
        .title(dialog.title.clone())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));

    let inner_area = block.inner(Rect {
        x,
        y,
        width,
        height,
    });

    f.render_widget(
        block,
        Rect {
            x,
            y,
            width,
            height,
        },
    );

    let mut spans = Vec::new();

    for (i, field) in dialog.fields.iter().enumerate() {
        let is_focused = i == dialog.current_field;
        let field_color = if is_focused {
            theme.gradient_color(0.0)
        } else {
            theme.text_primary
        };

        spans.push(Span::styled(
            format!("{}: ", field.label),
            Style::default().fg(theme.text_secondary),
        ));

        if is_focused {
            let before_cursor = &field.value[..field.cursor_position.min(field.value.len())];
            let after_cursor = &field.value[field.cursor_position.min(field.value.len())..];

            spans.push(Span::styled(
                before_cursor,
                Style::default().fg(field_color),
            ));
            spans.push(Span::styled(
                "_",
                Style::default()
                    .fg(field_color)
                    .add_modifier(ratatui::style::Modifier::UNDERLINED),
            ));
            spans.push(Span::styled(after_cursor, Style::default().fg(field_color)));
        } else {
            spans.push(Span::styled(&field.value, Style::default().fg(field_color)));
        }

        if i < dialog.fields.len() - 1 {
            spans.push(Span::raw("\n"));
        }
    }

    if let Some((msg, is_error)) = &dialog.message {
        spans.push(Span::raw("\n\n"));
        let color = if *is_error { Color::Red } else { Color::Green };
        spans.push(Span::styled(msg, Style::default().fg(color)));
    }

    let paragraph =
        Paragraph::new(Line::from(spans)).style(Style::default().fg(theme.text_primary));

    f.render_widget(paragraph, inner_area);
}
