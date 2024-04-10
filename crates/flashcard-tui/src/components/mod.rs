use std::sync::Arc;

use crate::preamble::ApplicationEvent;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub type ComponentRenderer = Arc<dyn Fn(&mut Frame, Rect) + Send + Sync + 'static>;

pub type ComponentEventHandler =
    Arc<dyn Fn(KeyEvent) -> Option<ApplicationEvent> + Send + Sync + 'static>;

pub type Component = (ComponentRenderer, ComponentEventHandler);
// function that has some sideeffect
pub type Trigger = Arc<dyn Fn() + Send + Sync>;

pub mod root;
