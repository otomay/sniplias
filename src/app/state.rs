#![allow(dead_code)]
use crate::models::{Alias, Snippet};
use crate::storage::{AliasStore, SnippetStore};
use crate::ui::{InputDialog, SearchBar, Tab};
use crate::utils::UpdateInfo;
use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tabs,
    Search,
    List,
    Dialog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Search,
    Dialog,
}

pub struct AppState {
    pub running: bool,
    pub current_tab: Tab,
    pub focus: Focus,
    pub mode: AppMode,
    pub search: SearchBar,
    pub alias_store: AliasStore,
    pub snippet_store: SnippetStore,
    pub alias_list_state: ListState,
    pub snippet_list_state: ListState,
    pub dialog: Option<InputDialog>,
    pub help_visible: bool,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub pending_command: Option<String>,
    pub aliases_modified: bool,
    pub source_command: Option<String>,
    pub update_info: UpdateInfo,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let alias_store = AliasStore::new()?;
        let snippet_store = SnippetStore::new()?;

        let source_command = alias_store.source_command();

        let mut update_info = UpdateInfo::new();
        update_info.check_update();

        Ok(Self {
            running: true,
            current_tab: Tab::Snippets,
            focus: Focus::List,
            mode: AppMode::Normal,
            search: SearchBar::new(),
            alias_store,
            snippet_store,
            alias_list_state: ListState::default(),
            snippet_list_state: ListState::default(),
            dialog: None,
            help_visible: false,
            error_message: None,
            success_message: None,
            pending_command: None,
            aliases_modified: false,
            source_command,
            update_info,
        })
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = self.current_tab.prev();
    }

    pub fn focus_search(&mut self) {
        self.focus = Focus::Search;
        self.mode = AppMode::Search;
        self.search.focused = true;
    }

    pub fn unfocus_search(&mut self) {
        self.focus = Focus::List;
        self.mode = AppMode::Normal;
        self.search.focused = false;
    }

    pub fn next_item(&mut self) {
        let len = match self.current_tab {
            Tab::Aliases => self.filtered_aliases().len(),
            Tab::Snippets => self.filtered_snippets().len(),
        };

        let state = match self.current_tab {
            Tab::Aliases => &mut self.alias_list_state,
            Tab::Snippets => &mut self.snippet_list_state,
        };

        if len > 0 {
            let i = state
                .selected()
                .map_or(0, |i| if i >= len - 1 { 0 } else { i + 1 });
            state.select(Some(i));
        }
    }

    pub fn prev_item(&mut self) {
        let len = match self.current_tab {
            Tab::Aliases => self.filtered_aliases().len(),
            Tab::Snippets => self.filtered_snippets().len(),
        };

        let state = match self.current_tab {
            Tab::Aliases => &mut self.alias_list_state,
            Tab::Snippets => &mut self.snippet_list_state,
        };

        if len > 0 {
            let i = state
                .selected()
                .map_or(0, |i| if i == 0 { len - 1 } else { i - 1 });
            state.select(Some(i));
        }
    }

    pub fn filtered_aliases(&self) -> Vec<&Alias> {
        if self.search.query.is_empty() {
            self.alias_store.list()
        } else {
            self.alias_store.list_filtered(&self.search.query)
        }
    }

    pub fn filtered_snippets(&self) -> Vec<&Snippet> {
        if self.search.query.is_empty() {
            self.snippet_store.list()
        } else {
            self.snippet_store.list_filtered(&self.search.query)
        }
    }

    pub fn selected_alias(&self) -> Option<&Alias> {
        let aliases = self.filtered_aliases();
        self.alias_list_state
            .selected()
            .and_then(|i| aliases.get(i).copied())
    }

    pub fn selected_snippet(&self) -> Option<&Snippet> {
        let snippets = self.filtered_snippets();
        self.snippet_list_state
            .selected()
            .and_then(|i| snippets.get(i).copied())
    }

    pub fn show_add_alias_dialog(&mut self) {
        self.dialog = Some(
            InputDialog::new("Add Alias", crate::ui::DialogMode::Add)
                .add_field("Name")
                .add_field("Command")
                .add_field("Description (optional)"),
        );
        self.focus = Focus::Dialog;
        self.mode = AppMode::Dialog;
    }

    pub fn show_edit_alias_dialog(&mut self) {
        if let Some(alias) = self.selected_alias() {
            self.dialog = Some(
                InputDialog::new("Edit Alias", crate::ui::DialogMode::Edit)
                    .add_field_with_value("Name", &alias.name)
                    .add_field_with_value("Command", &alias.command)
                    .add_field_with_value(
                        "Description (optional)",
                        alias.description.as_deref().unwrap_or(""),
                    ),
            );
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
        }
    }

    pub fn show_delete_alias_dialog(&mut self) {
        if let Some(alias) = self.selected_alias() {
            self.dialog = Some(
                InputDialog::new("Delete Alias", crate::ui::DialogMode::Delete)
                    .add_field_with_value("Confirm", &format!("Delete '{}'?", alias.name)),
            );
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
        }
    }

    pub fn show_add_snippet_dialog(&mut self) {
        self.dialog = Some(
            InputDialog::new("Add Snippet", crate::ui::DialogMode::Add)
                .add_field("Title")
                .add_field("Command")
                .add_field("Description (optional)"),
        );
        self.focus = Focus::Dialog;
        self.mode = AppMode::Dialog;
    }

    pub fn show_edit_snippet_dialog(&mut self) {
        if let Some(snippet) = self.selected_snippet() {
            self.dialog = Some(
                InputDialog::new("Edit Snippet", crate::ui::DialogMode::Edit)
                    .add_field_with_value("Title", &snippet.title)
                    .add_field_with_value("Command", &snippet.command)
                    .add_field_with_value(
                        "Description (optional)",
                        snippet.description.as_deref().unwrap_or(""),
                    ),
            );
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
        }
    }

    pub fn show_delete_snippet_dialog(&mut self) {
        if let Some(snippet) = self.selected_snippet() {
            self.dialog = Some(
                InputDialog::new("Delete Snippet", crate::ui::DialogMode::Delete)
                    .add_field_with_value("Confirm", &format!("Delete '{}'?", snippet.title)),
            );
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
        }
    }

    pub fn try_run_snippet(&mut self) -> bool {
        if let Some(snippet) = self.selected_snippet() {
            let variables = snippet.extract_variables();

            if variables.is_empty() {
                let command = snippet.command.clone();
                self.pending_command = Some(command);
                self.running = false;
                return true;
            }

            let mut dialog = InputDialog::new("Run Snippet", crate::ui::DialogMode::Run);

            for var in &variables {
                let field_label = format!(
                    "{} {}",
                    var.name,
                    var.default_value
                        .as_deref()
                        .map(|d| format!("(default: {})", d))
                        .unwrap_or_default()
                );
                dialog = dialog.add_field(field_label);
            }

            self.dialog = Some(dialog);
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
            return false;
        }
        false
    }

    pub fn close_dialog(&mut self) {
        self.dialog = None;
        self.focus = Focus::List;
        self.mode = AppMode::Normal;
    }

    pub fn mark_aliases_modified(&mut self) {
        self.aliases_modified = true;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    pub fn show_update_dialog(&mut self) {
        if self.update_info.update_available {
            let local = self.update_info.local_version.clone();
            let remote = self.update_info.remote_version.clone().unwrap_or_default();
            self.dialog = Some(
                crate::ui::InputDialog::new("Update Available", crate::ui::DialogMode::Update)
                    .with_update_info(&local, &remote, "manual"),
            );
            self.focus = Focus::Dialog;
            self.mode = AppMode::Dialog;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().expect("Failed to initialize AppState")
    }
}
