#![allow(non_snake_case)]

use std::sync::{Arc, RwLock};

use ratatui::{backend::Backend, layout::Rect, widgets::Paragraph, Frame};
use reactive_graph::effect::Effect;

use crate::preamble::CrosstermTerminal;

use super::{stub_component_event_handler, Component, ComponentRenderer, DynamicRect};

pub fn HelpBar(terminal: Arc<RwLock<CrosstermTerminal>>, compute_rect: DynamicRect) -> Component {
    let handler = stub_component_event_handler();

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, view_port: Rect| {
        let rect = compute_rect(view_port);
        let mut help_text = String::new();
        help_text.push_str("C-c: exit, Up: increment counter, Down: decrement counter");
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, rect);
    });

    Effect::new_sync({
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        move |_| {
            let mut terminal = terminal.write().unwrap();
            terminal.autoresize().unwrap();
            let mut frame = terminal.get_frame();
            let view_port = frame.size();

            renderer(&mut frame, view_port);

            terminal.flush().unwrap();
            terminal.backend_mut().flush().unwrap();
        }
    });

    (renderer, handler)
}
