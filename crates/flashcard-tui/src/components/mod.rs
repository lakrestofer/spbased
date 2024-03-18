use std::sync::{Arc, RwLock};

use crate::preamble::{ApplicationEvent, CrosstermTerminal};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub type ComponentRenderer = Arc<dyn Fn(&mut Frame, Rect) -> () + Send + Sync + 'static>;
// a component renderer that renderes nothing
pub fn stub_component_renderer() -> ComponentRenderer {
    Arc::new(|_, _| {})
}

pub type ComponentEventHandler =
    Arc<dyn Fn(KeyEvent) -> Option<ApplicationEvent> + Send + Sync + 'static>;
/// an event handler that does nothing
pub fn stub_component_event_handler() -> ComponentEventHandler {
    Arc::new(|_| None)
}

pub type Component = (ComponentRenderer, ComponentEventHandler);

pub type DynamicRect = Arc<dyn Fn(Rect) -> Rect + Send + Sync>;
pub type ComponentDef = Arc<dyn Fn(Arc<RwLock<CrosstermTerminal>>, DynamicRect) -> Component>;

pub mod add_card;
pub mod browser;
pub mod edit_card;
pub mod help_bar;
pub mod home;
pub mod review;
pub mod root;
