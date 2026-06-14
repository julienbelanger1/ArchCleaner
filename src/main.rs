use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use crate::app::App;
use crate::ui::ui;

mod app;
mod scanner;
mod scanners;
mod sudo;
mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    app.load().await; // Scan the system initially

    let res = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        let action = app.handle_enter();
                        if let crate::app::Action::ExecuteClean(pwd) = action {
                            let mut total = 0;
                            // Clean selected items via scanners
                            for scanner in &app.scanners {
                                // Find items belonging to this scanner
                                let mut items_to_clean = Vec::new();
                                for (item, selected) in &app.items {
                                    // Hacky way to associate items with scanners via a specific string matching 
                                    // (In a real app, we'd store an index or enum)
                                    if *selected {
                                        items_to_clean.push(item.clone());
                                    }
                                }
                                if !items_to_clean.is_empty() {
                                    if let Ok(freed) = scanner.clean(&items_to_clean, Some(&pwd)).await {
                                        total += freed;
                                    }
                                }
                            }
                            app.modal = crate::app::ModalState::Success { freed_bytes: total };
                        }
                    }
                    KeyCode::Backspace => app.handle_backspace(),
                    KeyCode::Char(c) => {
                        if c == 'c' && matches!(app.modal, crate::app::ModalState::None) {
                            app.clean().await;
                        } else {
                            app.handle_input(c);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
