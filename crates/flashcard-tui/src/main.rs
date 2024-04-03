use crossterm::event::{KeyCode, KeyModifiers};
use flashcard_tui::components::root::Root;
use flashcard_tui::event::{Event, TerminalEventHandler};
use flashcard_tui::preamble::*;
use flashcard_tui::tui::{exit_terminal, init_terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reactive_graph::effect::Effect;
use std::io;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

use any_spawner::Executor;

#[tokio::main]
async fn main() -> AppResult<()> {
    // abstract away terminal and application loop
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut events = TerminalEventHandler::new(250);
    init_terminal(&mut terminal)?;

    let terminal = Arc::new(RwLock::new(terminal));

    _ = Executor::init_tokio();

    let (root_renderer, root_event_handler) = Root();

    // Registering rendering side effect
    Effect::new_sync({
        // since the effect might be run on another thread
        // we have to pass both the renderer and terminal
        // in Arcs
        let terminal = terminal.clone();
        let renderer = root_renderer.clone();
        move |_| {
            _ = terminal.write().unwrap().draw(|frame| {
                let view_port = frame.size();
                renderer(frame, view_port);
            });
        }
    });

    // start event loop
    loop {
        if let Ok(event) = events.next().await {
            let event: Option<ApplicationEvent> = match event {
                Event::Key(key_event) => match key_event.code {
                    // on C-c, always exit the application
                    KeyCode::Char('c') | KeyCode::Char('C')
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        Some(ApplicationEvent::Shutdown)
                    }
                    _ => root_event_handler(key_event),
                },
                Event::Tick => None,
                Event::Mouse(_) => None,
                Event::Resize(_, _) => {
                    _ = terminal.write().unwrap().draw(|frame| {
                        let view_port = frame.size();
                        root_renderer(frame, view_port);
                    });
                    None
                }
            };

            match event {
                Some(event) => match event {
                    ApplicationEvent::Shutdown => {
                        break;
                    }
                },
                None => {}
            }
        }
    }

    exit_terminal(&mut terminal.write().unwrap()).expect("could not restore terminal");

    println!("Goodbye!");

    Ok(())
}
