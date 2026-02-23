use crate::models::{Alias, Snippet};
use crate::ui::DialogMode;
use crossterm::event::KeyCode;

use super::state::{AppState, Focus};

pub fn handle_key(
    app: &mut AppState,
    key: crossterm::event::KeyEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    if app.help_visible {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc => app.toggle_help(),
            _ => {}
        }
        return Ok(());
    }

    match app.mode {
        super::state::AppMode::Normal => handle_normal_mode(app, key),
        super::state::AppMode::Search => handle_search_mode(app, key),
        super::state::AppMode::Dialog => handle_dialog_mode(app, key),
    }

    Ok(())
}

fn handle_normal_mode(app: &mut AppState, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('?') => app.toggle_help(),
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Left => app.prev_tab(),
        KeyCode::Right => app.next_tab(),
        KeyCode::Up => app.prev_item(),
        KeyCode::Down => app.next_item(),
        KeyCode::Char('j') => app.next_item(),
        KeyCode::Char('k') => app.prev_item(),
        KeyCode::Char('/') => app.focus_search(),
        KeyCode::Enter => handle_enter(app),
        KeyCode::Char('a') => handle_add(app),
        KeyCode::Char('e') => handle_edit(app),
        KeyCode::Char('d') => handle_delete(app),
        KeyCode::Esc => {
            if app.focus == Focus::Search {
                app.unfocus_search();
            }
        }
        _ => {}
    }
}

fn handle_search_mode(app: &mut AppState, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.unfocus_search(),
        KeyCode::Enter => app.unfocus_search(),
        KeyCode::Char(c) => {
            app.search.handle_char(c);
        }
        KeyCode::Backspace => {
            app.search.handle_backspace();
        }
        _ => {}
    }
}

fn handle_dialog_mode(app: &mut AppState, key: crossterm::event::KeyEvent) {
    if let Some(dialog) = &mut app.dialog {
        match key.code {
            KeyCode::Esc => {
                app.close_dialog();
                return;
            }
            KeyCode::Tab => {
                dialog.next_field();
            }
            KeyCode::BackTab => {
                dialog.prev_field();
            }
            KeyCode::Up => {
                dialog.prev_field();
            }
            KeyCode::Down => {
                dialog.next_field();
            }
            KeyCode::Enter => {
                let values = dialog.get_values();
                let mode = dialog.mode;
                if let Err(e) = handle_dialog_submit(app, mode, &values) {
                    app.error_message = Some(e.to_string());
                }
                return;
            }
            KeyCode::Char(c) => {
                if let Some(field) = dialog.current_field_mut() {
                    field.handle_char(c);
                }
            }
            KeyCode::Backspace => {
                if let Some(field) = dialog.current_field_mut() {
                    field.handle_backspace();
                }
            }
            _ => {}
        }
    }
}

fn handle_enter(app: &mut AppState) {
    match app.current_tab {
        crate::ui::Tab::Aliases => {}
        crate::ui::Tab::Snippets => { app.try_run_snippet(); }
    }
}

fn handle_add(app: &mut AppState) {
    match app.current_tab {
        crate::ui::Tab::Aliases => app.show_add_alias_dialog(),
        crate::ui::Tab::Snippets => app.show_add_snippet_dialog(),
    }
}

fn handle_edit(app: &mut AppState) {
    match app.current_tab {
        crate::ui::Tab::Aliases => app.show_edit_alias_dialog(),
        crate::ui::Tab::Snippets => app.show_edit_snippet_dialog(),
    }
}

fn handle_delete(app: &mut AppState) {
    match app.current_tab {
        crate::ui::Tab::Aliases => app.show_delete_alias_dialog(),
        crate::ui::Tab::Snippets => app.show_delete_snippet_dialog(),
    }
}

fn handle_dialog_submit(
    app: &mut AppState,
    mode: DialogMode,
    values: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let values_map: std::collections::HashMap<String, String> = values.iter().cloned().collect();

    match mode {
        DialogMode::Add => match app.current_tab {
            crate::ui::Tab::Aliases => add_alias(app, &values_map)?,
            crate::ui::Tab::Snippets => add_snippet(app, &values_map)?,
        },
        DialogMode::Edit => match app.current_tab {
            crate::ui::Tab::Aliases => edit_alias(app, &values_map)?,
            crate::ui::Tab::Snippets => edit_snippet(app, &values_map)?,
        },
        DialogMode::Delete => match app.current_tab {
            crate::ui::Tab::Aliases => delete_alias(app)?,
            crate::ui::Tab::Snippets => delete_snippet(app)?,
        },
        DialogMode::Run => {
            run_snippet(app, &values_map)?;
        }
        _ => {}
    }

    Ok(())
}

fn add_alias(
    app: &mut AppState,
    values: &std::collections::HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = values.get("Name").map(|s| s.as_str()).unwrap_or("");
    let command = values.get("Command").map(|s| s.as_str()).unwrap_or("");
    let description = values.get("Description (optional)").and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(s.clone())
        }
    });

    if name.is_empty() || command.is_empty() {
        return Err("Name and Command are required".into());
    }

    let alias = Alias::new(
        name.to_string(),
        command.to_string(),
        app.alias_store.source().clone(),
    )
    .with_description(description.unwrap_or_default());

    app.alias_store.add(alias)?;
    app.mark_aliases_modified();
    app.success_message = Some(format!("Alias '{}' added", name));
    app.close_dialog();
    Ok(())
}

fn edit_alias(
    app: &mut AppState,
    values: &std::collections::HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(old_alias) = app.selected_alias() {
        let old_name = old_alias.name.clone();
        let name = values.get("Name").map(|s| s.as_str()).unwrap_or("");
        let command = values.get("Command").map(|s| s.as_str()).unwrap_or("");
        let description = values.get("Description (optional)").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.clone())
            }
        });

        if name.is_empty() || command.is_empty() {
            return Err("Name and Command are required".into());
        }

        let new_alias = Alias::new(
            name.to_string(),
            command.to_string(),
            app.alias_store.source().clone(),
        )
        .with_description(description.unwrap_or_default());

        app.alias_store.update(&old_name, new_alias)?;
        app.mark_aliases_modified();
        app.success_message = Some(format!("Alias '{}' updated", name));
    }
    app.close_dialog();
    Ok(())
}

fn delete_alias(app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(alias) = app.selected_alias() {
        let name = alias.name.clone();
        app.alias_store.delete(&name)?;
        app.mark_aliases_modified();
        app.success_message = Some(format!("Alias '{}' deleted", name));
    }
    app.close_dialog();
    Ok(())
}

fn add_snippet(
    app: &mut AppState,
    values: &std::collections::HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = values.get("Title").map(|s| s.as_str()).unwrap_or("");
    let command = values.get("Command").map(|s| s.as_str()).unwrap_or("");
    let description = values.get("Description (optional)").and_then(|s| {
        if s.is_empty() {
            None
        } else {
            Some(s.clone())
        }
    });

    if title.is_empty() || command.is_empty() {
        return Err("Title and Command are required".into());
    }

    let snippet = Snippet::new(title.to_string(), command.to_string())
        .with_description(description.unwrap_or_default());

    app.snippet_store.add(snippet)?;
    app.success_message = Some(format!("Snippet '{}' added", title));
    app.close_dialog();
    Ok(())
}

fn edit_snippet(
    app: &mut AppState,
    values: &std::collections::HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(old_snippet) = app.selected_snippet() {
        let id = old_snippet.id;
        let title = values.get("Title").map(|s| s.as_str()).unwrap_or("");
        let command = values.get("Command").map(|s| s.as_str()).unwrap_or("");
        let description = values.get("Description (optional)").and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.clone())
            }
        });

        if title.is_empty() || command.is_empty() {
            return Err("Title and Command are required".into());
        }

        let new_snippet = Snippet::new(title.to_string(), command.to_string())
            .with_id(id)
            .with_description(description.unwrap_or_default());

        app.snippet_store.update(id, new_snippet)?;
        app.success_message = Some(format!("Snippet '{}' updated", title));
    }
    app.close_dialog();
    Ok(())
}

fn delete_snippet(app: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(snippet) = app.selected_snippet() {
        let id = snippet.id;
        let title = snippet.title.clone();
        app.snippet_store.delete(&id)?;
        app.success_message = Some(format!("Snippet '{}' deleted", title));
    }
    app.close_dialog();
    Ok(())
}

fn run_snippet(
    app: &mut AppState,
    values: &std::collections::HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(snippet) = app.selected_snippet() {
        let variables = snippet.extract_variables();
        let mut input_values = std::collections::HashMap::new();

        for var in variables.iter() {
            let field_name = format!(
                "{} {}",
                var.name,
                var.default_value
                    .as_deref()
                    .map(|d| format!("(default: {})", d))
                    .unwrap_or_default()
            );
            if let Some(value) = values.get(&field_name) {
                if !value.is_empty() {
                    input_values.insert(var.name.clone(), value.clone());
                } else if let Some(default) = &var.default_value {
                    input_values.insert(var.name.clone(), default.clone());
                }
            } else if let Some(default) = &var.default_value {
                input_values.insert(var.name.clone(), default.clone());
            }
        }

        let command = snippet.render_command(&input_values);
        app.pending_command = Some(command);
        app.running = false;
    }
    Ok(())
}
