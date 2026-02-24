use ratatui::{layout::Rect, style::Style, text::Line, widgets::Paragraph, Frame};

use super::super::Theme;
use crate::ui::Tab;
use crate::update::{get_current_version, UpdateInfo};

pub fn render_status_bar(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
    current_tab: Tab,
    is_searching: bool,
    is_dialog_open: bool,
    is_help_visible: bool,
    update_info: &Option<UpdateInfo>,
) {
    // Check if update is available
    let has_update = update_info.as_ref().map(|i| i.has_update).unwrap_or(false);

    let shortcuts = if is_help_visible {
        vec![("Esc", "Close")]
    } else if is_dialog_open {
        vec![("Tab", "Next"), ("Enter", "Confirm"), ("Esc", "Cancel")]
    } else if is_searching {
        let mut shortcuts = vec![("Enter/Esc", "Exit"), ("q", "Quit")];
        if has_update {
            shortcuts.insert(0, ("U", "Update"));
        }
        shortcuts
    } else {
        let mut base_shortcuts = match current_tab {
            Tab::Aliases => vec![
                ("←/→  j/k", "Navigate"),
                ("/", "Search"),
                ("a", "Add"),
                ("e", "Edit"),
                ("d", "Delete"),
                ("?", "Help"),
                ("q", "Quit"),
            ],
            Tab::Snippets => vec![
                ("←/→  j/k", "Navigate"),
                ("/", "Search"),
                ("Enter", "Run"),
                ("a", "Add"),
                ("e", "Edit"),
                ("d", "Delete"),
                ("?", "Help"),
                ("q", "Quit"),
            ],
        };

        // Add U for update if available
        if has_update {
            base_shortcuts.insert(0, ("U", "Update"));
        }

        base_shortcuts
    };

    let mut spans = Vec::new();
    for (i, (key, action)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(ratatui::text::Span::styled(
                " | ",
                Style::default().fg(theme.border),
            ));
        }
        let color = theme.gradient_color(i as f32 / shortcuts.len().max(1) as f32);
        spans.push(ratatui::text::Span::styled(
            *key,
            Style::default()
                .fg(color)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ));
        spans.push(ratatui::text::Span::styled(
            format!(" {}", action),
            Style::default().fg(theme.text_secondary),
        ));
    }

    // Version text on the right
    let version_text = if let Some(info) = update_info {
        if info.has_update {
            format!("v{} → v{}", info.current_version, info.latest_version)
        } else {
            format!("v{} (latest)", info.current_version)
        }
    } else {
        format!("v{}", get_current_version())
    };

    // Render shortcuts centered
    let paragraph = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(theme.surface))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);

    // Render version on the right side
    let version_width = version_text.len() as u16 + 4;
    let version_area = Rect {
        x: area.width.saturating_sub(version_width),
        width: version_width,
        ..area
    };

    let version_paragraph = Paragraph::new(Line::from(vec![ratatui::text::Span::styled(
        format!(" {} ", version_text),
        Style::default().fg(theme.text_secondary),
    )]))
    .style(Style::default().bg(theme.surface))
    .alignment(ratatui::layout::Alignment::Right);

    f.render_widget(version_paragraph, version_area);
}
