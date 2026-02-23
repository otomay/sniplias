use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem as RatatuiListItem, ListState},
    Frame,
};

use super::super::Theme;

pub trait Listable {
    fn title(&self) -> &str;
    fn subtitle(&self) -> Option<&str>;
}

pub fn render_list<T: Listable>(
    f: &mut Frame,
    area: Rect,
    items: &[&T],
    state: &mut ListState,
    theme: &Theme,
    title: &str,
) {
    let list_items: Vec<RatatuiListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = state.selected() == Some(i);

            let style = if is_selected {
                theme.selected_style()
            } else {
                Style::default().fg(theme.text_primary).bg(theme.background)
            };

            let prefix = if is_selected { "> " } else { "  " };
            let title_text = format!("{}{}", prefix, item.title());

            let mut lines = vec![Line::styled(title_text, style)];

            if let Some(subtitle) = item.subtitle() {
                let subtitle_style = if is_selected {
                    Style::default()
                        .fg(theme.background)
                        .bg(theme.gradient_start)
                } else {
                    Style::default()
                        .fg(theme.text_secondary)
                        .bg(theme.background)
                };
                let subtitle_text = format!("    {}", subtitle);
                lines.push(Line::styled(subtitle_text, subtitle_style));
            }

            RatatuiListItem::new(lines).style(Style::default().bg(theme.background))
        })
        .collect();

    let title_str = format!(" {} ", title);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_style(true))
        .border_type(theme.border_type())
        .title(Line::from(theme.gradient_text(&title_str)));

    let list = List::new(list_items)
        .block(block)
        .highlight_style(theme.highlight_style());

    f.render_stateful_widget(list, area, state);
}
