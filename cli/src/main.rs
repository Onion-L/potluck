use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{
        self as crossterm_event, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod api;
mod app;
mod config;
mod event;
mod ui;

use app::App;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let result = run_app(&mut terminal, config).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    config: Config,
) -> Result<()> {
    let mut app = App::new(&config.api_url, config.limit);

    // Initial load
    app.load_initial().await?;

    // Main event loop
    loop {
        // Render
        terminal.draw(|f| ui::render(f, &mut app))?;

        // Check if should quit
        if app.should_quit {
            break;
        }

        // Check refresh
        if app.needs_refresh {
            app.needs_refresh = false;
            app.refresh().await?;
        }

        // Handle events
        if crossterm_event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm_event::read()? {
                if key.kind == KeyEventKind::Press {
                    event::handle_key_event(&mut app, key);
                }
            }
        }
    }

    Ok(())
}
