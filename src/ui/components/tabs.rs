use super::super::Theme;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Tabs as RatatuiTabs},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Aliases,
    Snippets,
}

impl Tab {
    pub fn titles() -> Vec<&'static str> {
        vec!["Aliases", "Snippets"]
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Aliases => 0,
            Tab::Snippets => 1,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Tab::Aliases,
            _ => Tab::Snippets,
        }
    }

    pub fn next(self) -> Self {
        Self::from_index((self.index() + 1) % 2)
    }

    pub fn prev(self) -> Self {
        Self::from_index(if self.index() == 0 { 1 } else { 0 })
    }
}

pub fn render_tabs(f: &mut Frame, area: Rect, active_tab: Tab, theme: &Theme) {
    let titles: Vec<Line> = Tab::titles()
        .iter()
        .enumerate()
        .map(|(i, title)| {
            let is_active = i == active_tab.index();
            let style = if is_active {
                theme.active_tab_style()
            } else {
                theme.inactive_tab_style()
            };

            Line::styled(format!(" {} ", title), style)
        })
        .collect();

    let title_text = " SNIPLIAS ";

    let tabs = RatatuiTabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style(true))
                .border_type(theme.border_type())
                .title(Line::from(theme.gradient_text(title_text))),
        )
        .highlight_style(theme.selected_style())
        .select(active_tab.index());

    f.render_widget(tabs, area);
}
