use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{app::App, state::State};

use super::Component;

pub struct Root {
    renderer: Box<dyn Fn(&mut Frame, Rect) -> ()>,
    handler: Box<dyn Fn(&mut App, KeyEvent) -> ()>,
}

impl Root {
    pub fn new() -> Root {
        let renderer = move |frame: &mut Frame, rect: Rect| {};

        let handler = move |app: &mut App, key_event: crossterm::event::KeyEvent| {};

        Self {
            renderer: Box::new(renderer),
            handler: Box::new(handler),
        }
    }
}

impl Component for Root {
    fn render(&self, frame: &mut Frame, rect: Rect) {
        (self.renderer)(frame, rect);
    }

    fn handle_key_events(
        &mut self,
        app: &mut App,
        key_event: crossterm::event::KeyEvent,
    ) -> crate::preamble::AppResult<()> {
        (self.handler)(app, key_event);
        Ok(())
    }
}
