mod app;
mod models;
mod storage;
mod ui;
mod update;
mod utils;

use app::{AppState, EventHandler};
use clap::Parser;
use std::sync::mpsc;
use std::thread;
use ui::{
    render_help_dialog, render_input_dialog, render_list, render_search_bar, render_status_bar,
    render_tabs, Theme,
};
use utils::Terminal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    // Start update check in background
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let update_info = update::check_for_update();
        let _ = tx.send(update_info);
    });

    let mut terminal = Terminal::new()?;
    let mut app = AppState::new()?;
    let events = EventHandler::default();
    let theme = Theme::default();

    // Check for update in the first iteration
    let mut update_checked = false;

    while app.running {
        terminal.draw(|f| {
            let area = f.area();
            render_ui(f, &app, &theme, area);
        })?;

        // Process update result if available (non-blocking)
        if !update_checked {
            if let Ok(info) = rx.try_recv() {
                if let Some(update_info) = info {
                    app.set_update_info(update_info);
                }
                update_checked = true;
            }
        }

        match events.next()? {
            app::Event::Key(key) => {
                app::handlers::handle_key(&mut app, key)?;
            }
            app::Event::Tick => {}
        }
    }

    drop(terminal);

    if let Some(cmd) = app.pending_command.take() {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .status()?;

        if !status.success() {
            eprintln!("Command exited with code: {:?}", status.code());
        }
    }

    if app.aliases_modified {
        if let Some(source_cmd) = &app.source_command {
            eprintln!("\nAliases modified! Run to reload:");
            eprintln!("  {}\n", source_cmd);
        }
    }

    // Show update notification after app closes
    if let Some(update_info) = &app.update_info {
        if update_info.has_update {
            println!();
            println!(
                "Update available: v{} -> v{}",
                update_info.current_version, update_info.latest_version
            );

            // If Manual install and user confirmed, run the update script
            if app.update_confirmed {
                if update_info.install_method == update::InstallMethod::Manual {
                    println!("\nRunning update script...");
                    let status = std::process::Command::new("sh")
                        .arg("-c")
                        .arg("curl -sL https://raw.githubusercontent.com/otomay/sniplias/master/scripts/install.sh | sh")
                        .status();

                    match status {
                        Ok(s) if s.success() => {
                            println!("Update completed successfully!");
                        }
                        Ok(s) => {
                            eprintln!("Update failed with exit code: {:?}", s.code());
                        }
                        Err(e) => {
                            eprintln!("Failed to run update script: {}", e);
                        }
                    }
                }
            }
            println!();
        }
    }

    Ok(())
}

fn render_ui(f: &mut ratatui::Frame, app: &AppState, theme: &Theme, area: ratatui::layout::Rect) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_tabs(f, chunks[0], app.current_tab, theme);
    render_search_bar(f, chunks[1], &app.search, theme);

    match app.current_tab {
        ui::Tab::Aliases => {
            let aliases = app.filtered_aliases();
            let mut state = app.alias_list_state.clone();
            render_list(
                f,
                chunks[2],
                &aliases.iter().map(|a| *a).collect::<Vec<_>>(),
                &mut state,
                theme,
                "ALIASES",
            );
        }
        ui::Tab::Snippets => {
            let snippets = app.filtered_snippets();
            let mut state = app.snippet_list_state.clone();
            render_list(
                f,
                chunks[2],
                &snippets.iter().map(|s| *s).collect::<Vec<_>>(),
                &mut state,
                theme,
                "SNIPPETS",
            );
        }
    }

    render_status_bar(
        f,
        chunks[3],
        theme,
        app.current_tab,
        app.search.focused,
        app.dialog.is_some(),
        app.help_visible,
        &app.update_info,
    );

    if app.help_visible {
        render_help_dialog(f, theme);
    }

    if let Some(dialog) = &app.dialog {
        render_input_dialog(f, dialog, theme);
    }
}

impl ui::Listable for models::Alias {
    fn title(&self) -> &str {
        &self.name
    }

    fn subtitle(&self) -> Option<&str> {
        Some(&self.command)
    }
}

impl ui::Listable for models::Snippet {
    fn title(&self) -> &str {
        &self.title
    }

    fn subtitle(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
