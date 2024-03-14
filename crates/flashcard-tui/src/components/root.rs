use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use leptos_reactive::{create_signal, SignalGet, SignalGetUntracked, SignalUpdate};
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
        let (count, set_count) = create_signal(0);

        let renderer = move |frame: &mut Frame, rect: Rect| {
            frame.render_widget(
                Paragraph::new(format!("Counter: {}", count.get_untracked())).centered(),
                rect,
            );
        };

        let handler = move |app: &mut App, key_event: crossterm::event::KeyEvent| {
            match key_event.code {
                // independend of state, if we pres C-c we quit
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    };
                }
                KeyCode::Up => {
                    set_count.update(|old| *old += 1);
                }
                KeyCode::Down => {
                    set_count.update(|old| *old -= 1);
                }
                _ => {}
            };
        };

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
