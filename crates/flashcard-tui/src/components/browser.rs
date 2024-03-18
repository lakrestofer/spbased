#![allow(non_snake_case)]
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, style::Stylize, widgets::Paragraph, Frame};
use reactive_graph::{signal::RwSignal, traits::Set};
use std::sync::Arc;

use super::{root::ActiveView, Component, ComponentEventHandler, ComponentRenderer};

pub fn Browser(active_view: RwSignal<ActiveView>) -> Component {
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match key_event.code {
            KeyCode::Esc => active_view.set(ActiveView::Home),
            _ => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let title = Paragraph::new("Browser View").blue().centered();
        frame.render_widget(title, rect);
    });

    (renderer, handler)
}
