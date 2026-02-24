use ratatui::{layout::Rect, style::Style, text::Line, widgets::Paragraph, Frame};

use super::super::Theme;
use crate::ui::Tab;
use crate::utils::UpdateInfo;

pub fn render_status_bar(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
    current_tab: Tab,
    is_searching: bool,
    is_dialog_open: bool,
    is_help_visible: bool,
    update_info: &UpdateInfo,
) {
    let shortcuts = if is_help_visible {
        vec![("Esc", "Close")]
    } else if is_dialog_open {
        vec![("Tab", "Next"), ("Enter", "Confirm"), ("Esc", "Cancel")]
    } else if is_searching {
        vec![("Enter/Esc", "Exit"), ("q", "Quit")]
    } else {
        match current_tab {
            Tab::Aliases => {
                let mut base = vec![
                    ("←/→  j/k", "Navigate"),
                    ("/", "Search"),
                    ("a", "Add"),
                    ("e", "Edit"),
                    ("d", "Delete"),
                    ("?", "Help"),
                    ("q", "Quit"),
                ];
                if update_info.update_available {
                    base.insert(base.len() - 1, ("u", "Update"));
                }
                base
            }
            Tab::Snippets => {
                let mut base = vec![
                    ("←/→  j/k", "Navigate"),
                    ("/", "Search"),
                    ("Enter", "Run"),
                    ("a", "Add"),
                    ("e", "Edit"),
                    ("d", "Delete"),
                    ("?", "Help"),
                    ("q", "Quit"),
                ];
                if update_info.update_available {
                    base.insert(base.len() - 1, ("u", "Update"));
                }
                base
            }
        }
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

    // Add version info at the end
    spans.push(ratatui::text::Span::styled(
        " | ",
        Style::default().fg(theme.border),
    ));
    let version_text = if update_info.update_available {
        format!(
            "v{} -> v{}",
            update_info.local_version,
            update_info.remote_version.as_deref().unwrap_or("unknown")
        )
    } else {
        format!("v{} (latest)", update_info.local_version)
    };
    spans.push(ratatui::text::Span::styled(
        version_text,
        if update_info.update_available {
            Style::default().fg(theme.success)
        } else {
            Style::default().fg(theme.text_secondary)
        },
    ));

    let paragraph = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(theme.surface))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}
