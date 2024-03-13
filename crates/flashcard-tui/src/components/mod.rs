use crate::{app::App, preamble::AppResult, state::State};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

// Trait describing operations needed by every component
// the component is not a container for state,
// but only a description of how to render
// itself given state, and how state should be changed
pub trait Component {
    fn render(&self, state: &State, frame: &mut Frame, rect: Rect);
    fn handle_key_events(&self, app: &mut App, key_event: KeyEvent) -> AppResult<()>;
}

/// A boxed component
pub type DynamicComponent = Box<dyn Component>;

pub mod add_card;
pub mod browser;
pub mod edit_card;
pub mod help_bar;
pub mod review_card;
pub mod root;
