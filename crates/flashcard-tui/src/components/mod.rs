use std::sync::Arc;

use crate::preamble::ApplicationEvent;
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

// function that has some sideeffect
pub type Trigger = Arc<dyn Fn() -> () + Send + Sync>;

pub mod add_card;
pub mod bottom_bar;
pub mod browser;
pub mod common;
pub mod edit_card;
pub mod home;
pub mod review;
pub mod root;
pub mod tag_area;
pub mod utils;
