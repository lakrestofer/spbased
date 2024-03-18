use std::sync::{Arc, RwLock};

use crate::preamble::CrosstermTerminal;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub type ComponentRenderer = Arc<dyn Fn(&mut Frame, Rect) -> () + Send + Sync + 'static>;
// a component renderer that renderes nothing
pub fn stub_component_renderer() -> ComponentRenderer {
    Arc::new(|_, _| {})
}

pub type ComponentEventHandler = Arc<dyn Fn(KeyEvent) -> () + Send + Sync + 'static>;
/// an event handler that does nothing
pub fn stub_component_event_handler() -> ComponentEventHandler {
    Arc::new(|_| {})
}

pub type Component = (ComponentRenderer, ComponentEventHandler);

pub type DynamicRect = Arc<dyn Fn(Rect) -> Rect + Send + Sync>;
pub type ComponentDef = Arc<dyn Fn(Arc<RwLock<CrosstermTerminal>>, DynamicRect) -> Component>;

pub mod root;
