use flashcard_tui::app::App;
use flashcard_tui::components::root::Root;
use flashcard_tui::components::Component;
use flashcard_tui::event::{Event, TerminalEventHandler};
use flashcard_tui::preamble::*;
use flashcard_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    // abstract away terminal and application loop
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = TerminalEventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // we make sure to start our state runtime
    let runtime = leptos_reactive::create_runtime();

    // all app state
    let mut app = App::default();

    let mut root: Box<dyn Component> = Box::new(Root::new());

    // Start the main loop.
    while app.running() {
        // Render the user interface using supplied renderer
        tui.draw(&root)?;

        // Handle events. Waits for "tickrate"
        match tui.events.next().await? {
            Event::Tick => {}
            Event::Key(key_event) => root.handle_key_events(&mut app, key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    runtime.dispose();

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
