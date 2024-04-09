use crossterm::event::{KeyCode, KeyModifiers};
use flashcard_tui::components::root::Root;
use flashcard_tui::contexts::stats::FrameTimeContext;
use flashcard_tui::contexts::tick::TickCounterContext;
use flashcard_tui::event::{Event, TerminalEventHandler};
use flashcard_tui::preamble::*;
use flashcard_tui::tui::{exit_terminal, init_terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reactive_graph::computed::ScopedFuture;
use reactive_graph::effect::Effect;
use reactive_graph::owner::{provide_context, Owner};
use reactive_graph::signal::RwSignal;
use reactive_graph::traits::{Update, UpdateUntracked};
use std::io;
use std::sync::{Arc, RwLock};

use any_spawner::Executor;

#[tokio::main]
async fn main() -> AppResult<()> {
    // first we setup some terminal abstraction layers
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let events = TerminalEventHandler::new(250);
    // we send some initial byte sequences to stdout, signaling
    // to the terminal that we want to enter a specific state.
    // (enter alternate mode, disable cursor, set color options etc)
    init_terminal(&mut terminal)?;

    let terminal = Arc::new(RwLock::new(terminal));

    // we then init the reactive runtime for the reactive_graph
    _ = Executor::init_tokio();

    // we create an root owner
    Owner::new()
        .with({
            let terminal = terminal.clone();
            || ScopedFuture::new(run(terminal, events))
        })
        .await?;

    exit_terminal(&mut terminal.write().unwrap()).expect("could not restore terminal");

    println!("Goodbye!");

    Ok(())
}

async fn run(
    terminal: Arc<RwLock<CrosstermTerminal>>,
    mut events: TerminalEventHandler,
) -> AppResult<()> {
    let stats = RwSignal::new(FrameTimeContext::default());
    provide_context::<RwSignal<FrameTimeContext>>(stats);
    let tick_counter = RwSignal::new(TickCounterContext(0));
    provide_context(tick_counter);

    let (root_renderer, root_event_handler) = Root();

    // Registering rendering side effect
    Effect::new_sync({
        // since the effect might be run on another thread
        // we have to pass both the renderer and terminal
        // in Arcs
        let terminal = terminal.clone();
        let renderer = root_renderer.clone();
        move |_| {
            // we measure the time it takes to perform the draw call
            let before = std::time::Instant::now();
            terminal
                .write()
                .unwrap()
                .draw(|frame| {
                    let view_port = frame.size();
                    renderer(frame, view_port);
                })
                .expect("Could not render view!");
            let dur = std::time::Instant::now().duration_since(before);
            stats.update_untracked(|FrameTimeContext(old_dur)| *old_dur = dur);
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
                Event::Tick => {
                    tick_counter.update(|TickCounterContext(count)| *count += 1);
                    None
                }
                Event::Mouse(_) => None,
                Event::Resize(_, _) => {
                    _ = terminal.write().unwrap().draw(|frame| {
                        let view_port = frame.size();
                        root_renderer(frame, view_port);
                    });
                    None
                }
            };

            if let Some(event) = event {
                match event {
                    ApplicationEvent::Shutdown => {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
