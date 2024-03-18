#![allow(non_snake_case)]
use std::sync::{Arc, RwLock};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use reactive_graph::{
    effect::Effect,
    signal::RwSignal,
    traits::{Get, Update},
};

use crate::tui::CrosstermTerminal;

use super::{Component, ComponentEventHandler, ComponentRenderer, DynamicRect};

pub fn Root(terminal: Arc<RwLock<CrosstermTerminal>>, compute_rect: DynamicRect) -> Component {
    let counter = RwSignal::new(0);

    let handler: ComponentEventHandler =
        Arc::new(
            move |key_event: crossterm::event::KeyEvent| match key_event.code {
                KeyCode::Up => counter.update(|c| *c += 1),
                KeyCode::Down => counter.update(|c| *c -= 1),
                _ => {}
            },
        );

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        frame.render_widget(Paragraph::new(format!("Counter: {}", counter.get())), rect);
    });

    Effect::new_sync({
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        move |_| {
            _ = terminal.write().unwrap().draw(|frame| {
                let rect = compute_rect(frame.size());
                renderer(frame, rect);
            });
        }
    });

    (renderer, handler)
}
