use crossterm::event::{KeyCode, KeyModifiers};
use flashcard_tui::components::root::Root;
use flashcard_tui::event::{Event, TerminalEventHandler};
use flashcard_tui::preamble::*;
use flashcard_tui::tui::{exit_terminal, init_terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use reactive_graph::effect::Effect;
use reactive_graph::signal::RwSignal;
use std::sync::{Arc, RwLock};
use std::{io, mem};
use tokio::sync::mpsc;

use any_spawner::Executor;
use reactive_graph::prelude::*;

pub struct ShutdownSignal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // abstract away terminal and application loop
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut events = TerminalEventHandler::new(250);
    init_terminal(&mut terminal)?;

    let terminal = Arc::new(RwLock::new(terminal));

    _ = Executor::init_tokio();

    let _terminal = terminal.clone();
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<ShutdownSignal>();

    Executor::spawn(async move {
        let terminal = _terminal;

        let (_root_renderer, root_event_handler) = Root(terminal, Arc::new(|x| x));

        loop {
            if let Ok(event) = events.next().await {
                match event {
                    Event::Tick => {}
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                shutdown_send.send(ShutdownSignal).unwrap();
                                break;
                            }
                        }
                        _ => root_event_handler(key_event),
                    },
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
        }
    });

    shutdown_recv
        .recv()
        .await
        .expect("Tried to wait for shutdown signal");

    exit_terminal(&mut terminal.write().unwrap()).expect("could not restore terminal");

    println!("Goodbye!");

    Ok(())
}
