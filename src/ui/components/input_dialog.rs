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
    pub cursor_pos: usize,
    pub focused: bool,
}

impl InputField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            cursor_pos: 0,
            focused: false,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_pos = self.value.len();
        self
    }

    pub fn handle_char(&mut self, c: char) {
        self.value.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn handle_delete(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn handle_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn handle_right(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.cursor_pos += 1;
        }
    }
}

pub struct InputDialog {
    pub title: String,
    pub fields: Vec<InputField>,
    pub current_field: usize,
    pub mode: DialogMode,
    pub message: Option<(String, bool)>,
    pub update_info: Option<(String, String)>, // (local_version, remote_version)
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
            update_info: None,
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

    pub fn with_update_info(mut self, local: &str, remote: &str, _install_method: &str) -> Self {
        self.update_info = Some((local.to_string(), remote.to_string()));
        // Add the confirmation field
        self.fields.push(InputField::new("Update? (y/n)"));
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
    let popup_area = dialog.centered_rect(
        60,
        if dialog.mode == DialogMode::Update {
            40
        } else {
            60
        },
        area,
    );

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

    // Special for Update rendering dialog
    if dialog.mode == DialogMode::Update {
        render_update_dialog(f, dialog, theme, inner_area);
        return;
    }

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

        let display_text = if is_focused {
            let prefix = &field.value[..field.cursor_pos];
            let suffix = &field.value[field.cursor_pos..];
            format!("{}|{}", prefix, suffix)
        } else {
            field.value.clone()
        };

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

fn render_update_dialog(f: &mut Frame, dialog: &InputDialog, theme: &Theme, area: Rect) {
    use ratatui::layout::Alignment;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Version info
            Constraint::Length(3), // Install method info
            Constraint::Min(3),    // Confirm prompt
        ])
        .split(area);

    // Show version info
    if let Some((local, remote)) = &dialog.update_info {
        let version_text = Paragraph::new(format!("Version {} -> {} available!", local, remote))
            .style(Style::default().fg(theme.success))
            .alignment(Alignment::Center);
        f.render_widget(version_text, chunks[0]);
    }

    // Show install method message
    let method_text = if let Some(_field) = dialog.fields.first() {
        format!("Installed manually. Run update? Press y to confirm.")
    } else {
        "".to_string()
    };

    let method_paragraph = Paragraph::new(method_text)
        .style(Style::default().fg(theme.text_secondary))
        .alignment(Alignment::Center);
    f.render_widget(method_paragraph, chunks[1]);

    // Show confirmation field
    if let Some(field) = dialog.fields.first() {
        let is_focused = field.focused;
        let field_block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style(is_focused))
            .border_type(theme.border_type())
            .title(ratatui::text::Span::styled(
                format!(" {} ", field.label),
                Style::default()
                    .fg(theme.success)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ));

        let color = theme.success;
        let style = Style::default().fg(color);

        let display_text = if is_focused {
            let prefix = &field.value[..field.cursor_pos];
            let suffix = &field.value[field.cursor_pos..];
            format!("{}|{}", prefix, suffix)
        } else {
            field.value.clone()
        };

        let paragraph = Paragraph::new(display_text)
            .style(style)
            .block(field_block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[2]);
    }
}
