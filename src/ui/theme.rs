#![allow(dead_code)]
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::BorderType;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub gradient_start: Color,
    pub gradient_end: Color,
    pub background: Color,
    pub surface: Color,
    pub error: Color,
    pub success: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub border: Color,
    pub border_active: Color,
    pub highlight: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            gradient_start: Color::Rgb(0, 200, 255),
            gradient_end: Color::Rgb(255, 100, 100),
            background: Color::Rgb(22, 22, 30),
            surface: Color::Rgb(30, 30, 42),
            error: Color::Rgb(255, 100, 100),
            success: Color::Rgb(100, 255, 150),
            text_primary: Color::Rgb(245, 245, 250),
            text_secondary: Color::Rgb(140, 140, 160),
            border: Color::Rgb(55, 55, 75),
            border_active: Color::Rgb(0, 100, 128),
            highlight: Color::Rgb(50, 50, 70),
        }
    }

    pub fn gradient_color(&self, ratio: f32) -> Color {
        let start_r = 0;
        let start_g = 200;
        let start_b = 255;

        let end_r = 255;
        let end_g = 100;
        let end_b = 100;

        let r = (start_r as f32 + (end_r as f32 - start_r as f32) * ratio) as u8;
        let g = (start_g as f32 + (end_g as f32 - start_g as f32) * ratio) as u8;
        let b = (start_b as f32 + (end_b as f32 - start_b as f32) * ratio) as u8;

        Color::Rgb(r, g, b)
    }

    pub fn gradient_text<'a>(&self, text: &'a str) -> Vec<Span<'a>> {
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                let ratio = if text.len() > 1 {
                    i as f32 / (text.len() - 1) as f32
                } else {
                    0.0
                };
                let color = self.gradient_color(ratio);
                Span::styled(c.to_string(), Style::default().fg(color))
            })
            .collect()
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.gradient_start)
            .bg(self.background)
            .add_modifier(Modifier::BOLD)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.text_primary)
            .bg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.gradient_start)
            .add_modifier(Modifier::BOLD)
    }

    pub fn inactive_tab_style(&self) -> Style {
        Style::default().fg(self.text_secondary).bg(self.surface)
    }

    pub fn active_tab_style(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.gradient_start)
            .add_modifier(Modifier::BOLD)
    }

    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).bg(self.background)
    }

    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success).bg(self.background)
    }

    pub fn border_type(&self) -> BorderType {
        BorderType::Rounded
    }

    pub fn border_style(&self, active: bool) -> Style {
        if active {
            Style::default().fg(self.border_active)
        } else {
            Style::default().fg(self.border)
        }
    }
}
