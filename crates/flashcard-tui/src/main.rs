use crossterm::event::{KeyCode, KeyModifiers};
use flashcard_tui::components::root::Root;
use flashcard_tui::constants::{log_dir_path, log_env, log_file_path};
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
use tracing::Level;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, prelude::*, util::SubscriberInitExt, Layer};

use any_spawner::Executor;

#[tokio::main]
async fn main() -> AppResult<()> {
    setup_error_hooks()?;
    setup_logging()?;

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
            let _span = tracing::span!(Level::TRACE, "Render effect");
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
        let _span = tracing::span!(Level::TRACE, "Event loop");
        if let Ok(event) = events.next().await {
            let event: Option<ApplicationEvent> = match event {
                Event::Key(key_event) => {
                    tracing::event!(Level::INFO, "Got key event: {key_event:?}");

                    match key_event.code {
                        // on C-c, always exit the application
                        KeyCode::Char('c') | KeyCode::Char('C')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            Some(ApplicationEvent::Shutdown)
                        }
                        _ => root_event_handler(key_event),
                    }
                }
                Event::Tick => {
                    // tick_counter.update(|TickCounterContext(count)| *count += 1);
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
                tracing::event!(Level::INFO, "Got application event: {event:?}");
                match event {
                    ApplicationEvent::Shutdown => {
                        tracing::event!(Level::INFO, "Shutting down...");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn setup_error_hooks() -> AppResult<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();
    eyre_hook.install()?;
    std::panic::set_hook(Box::new(move |panic_info| {
        let msg = format!("{}", panic_hook.panic_report(panic_info));
        log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));
        std::process::exit(1);
    }));
    Ok(())
}

fn setup_logging() -> AppResult<()> {
    let log_dir = log_dir_path();
    let log_file_path = log_file_path();
    std::fs::create_dir_all(log_dir)?;
    let log_file = std::fs::File::create(log_file_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(log_env()))
            .unwrap_or("trace".into()),
    );

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
