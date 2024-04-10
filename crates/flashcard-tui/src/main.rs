use any_spawner::Executor;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use flashcard_tui::{
    constants::{log_dir_path, log_env, log_file_path},
    event::TerminalEventHandler,
    preamble::{ApplicationEvent, *},
    tui::{exit_terminal, init_terminal},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use reactive_graph::{
    computed::{Memo, ScopedFuture},
    effect::Effect,
    owner::Owner,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use std::{
    io,
    sync::{Arc, RwLock},
};
use tracing::{info, instrument, Level};
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, prelude::*, util::SubscriberInitExt, Layer};

#[tokio::main]
async fn main() -> AppResult<()> {
    setup_error_hooks()?;
    setup_logging()?;

    // first we setup some terminal abstraction layers
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let events = TerminalEventHandler::new(50);

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
        .await;

    exit_terminal(&mut terminal.write().unwrap()).expect("could not restore terminal");

    println!("Goodbye!");

    Ok(())
}

async fn run(terminal: Arc<RwLock<CrosstermTerminal>>, mut events: TerminalEventHandler) -> ! {
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
            terminal
                .write()
                .unwrap()
                .draw(|frame| {
                    let view_port = frame.size();
                    renderer(frame, view_port);
                })
                .expect("Could not render view!");
        }
    });

    // start event loop
    loop {
        let _span = tracing::span!(Level::TRACE, "Event loop");
        if let Ok(_event) = events.next().await {
            root_event_handler(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        }
    }
}

pub type ComponentRenderer = Arc<dyn Fn(&mut Frame, Rect) + Send + Sync + 'static>;

pub type ComponentEventHandler =
    Arc<dyn Fn(KeyEvent) -> Option<ApplicationEvent> + Send + Sync + 'static>;

pub type Component = (ComponentRenderer, ComponentEventHandler);
// function that has some sideeffect
pub type Trigger = Arc<dyn Fn() + Send + Sync>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FocusedField {
    Question,
    Answer,
    Tag,
}

impl FocusedField {
    pub fn next(&mut self) {
        *self = match self {
            FocusedField::Question => FocusedField::Answer,
            FocusedField::Answer => FocusedField::Tag,
            FocusedField::Tag => FocusedField::Question,
        }
    }
}
const ADD_CARD_HELP_TEXT: &str = "this is some help text";
const TAG_AREA_HELP_TEXT: &str = "this is some other help text given some state";

#[instrument]
#[allow(non_snake_case)]
pub fn Root() -> Component {
    info!("Building AddCard component");
    // state
    let focused_field = RwSignal::new(FocusedField::Question);
    let focus_next_field = move || {
        info!("Updating the focused field to the next one");
        focused_field.update(FocusedField::next);
        info!("Successfully updated the field to the next one!");
    };
    let help_text = RwSignal::new(HelpContext::new());

    let a_focused = Memo::new(move |_| focused_field.get() == FocusedField::Answer);
    let q_focused = Memo::new(move |_| focused_field.get() == FocusedField::Question);
    let t_focused = Memo::new(move |_| focused_field.get() == FocusedField::Tag);
    // memos
    let s_title: Memo<String> = Memo::new(move |_| {
        if t_focused.get() {
            "Lower (but with some added text since I'm focused)".into()
        } else {
            "Lower:".into()
        }
    });

    // effects
    Effect::new_sync(move |_| {
        help_text.update(|help_text| {
            help_text.clear_below_level(1);
            help_text.update_desc_at_level(ADD_CARD_HELP_TEXT, 1);
            if t_focused.get() {
                help_text.update_desc_at_level(TAG_AREA_HELP_TEXT, 2)
            }
        });
    });

    // ====== Event handler ======

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!(
            "root: handling key event: {key_event:?}, focused field: {:?}",
            focused_field.get_untracked(),
        );
        if let KeyCode::Tab = key_event.code {
            focus_next_field()
        }
        info!("AddCard: returning from event handler");
        None
    });

    // ====== Renderer ======
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering root ");
        let [upper, center, lower] = Layout::vertical([Constraint::Fill(1); 3]).areas(rect);

        // question field
        frame.render_widget(
            Paragraph::new("First")
                .style(if q_focused.get() {
                    Style::default().bg(Color::Indexed(233))
                } else {
                    Style::default().bg(Color::Indexed(235))
                })
                .centered(),
            upper,
        );
        // answer field
        frame.render_widget(
            Paragraph::new("Second")
                .style(if a_focused.get() {
                    Style::default().bg(Color::Indexed(233))
                } else {
                    Style::default().bg(Color::Indexed(235))
                })
                .centered(),
            center,
        );
        // tag
        frame.render_widget(
            Paragraph::new(s_title.get())
                .style(if t_focused.get() {
                    Style::default().bg(Color::Indexed(233))
                } else {
                    Style::default().bg(Color::Indexed(235))
                })
                .centered(),
            lower,
        );
    });

    (renderer, handler)
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

#[derive(Clone, Default)]
pub struct HelpContext {
    help_descs: Vec<Option<String>>,
}

// the maximum depth on which a new component
const MAX_COMPONENT_DEPTH: usize = 8;

impl HelpContext {
    pub fn new() -> Self {
        Self {
            help_descs: vec![None; MAX_COMPONENT_DEPTH],
        }
    }

    pub fn into_help_string(&self) -> String {
        let mut help_descs = Vec::new();
        for help_desc in self.help_descs.iter().flatten() {
            help_descs.push(help_desc.clone());
        }
        help_descs.join(", ")
    }

    pub fn update_desc_at_level(&mut self, desc: &str, level: usize) {
        if level >= MAX_COMPONENT_DEPTH {
            return;
        }
        self.help_descs[level] = Some(desc.into());
    }

    // removes all help comments below this level
    // usefull to remove help messages when moving uppwards in the component
    // tree
    pub fn clear_below_level(&mut self, level: usize) {
        for i in (level + 1)..MAX_COMPONENT_DEPTH {
            self.help_descs[i] = None;
        }
    }
}
