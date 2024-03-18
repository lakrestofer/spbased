#![allow(non_snake_case)]

use std::sync::{Arc, RwLock};

use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use reactive_graph::effect::Effect;

use crate::preamble::CrosstermTerminal;

use super::{stub_component_event_handler, Component, ComponentRenderer, DynamicRect};

pub fn HelpBar(terminal: Arc<RwLock<CrosstermTerminal>>, compute_rect: DynamicRect) -> Component {
    let handler = stub_component_event_handler();

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let mut help_text = String::new();
        help_text.push_str("C-c: exit, Up: increment counter, Down: decrement counter");
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, rect);
    });

    // NOTE: This doesn't quite work
    // Effect::new_sync({
    //     let terminal = terminal.clone();
    //     let renderer = renderer.clone();
    //     move |_| {
    //         _ = terminal.write().unwrap().draw(|frame| {
    //             let rect = compute_rect(frame.size());
    //             renderer(frame, rect);
    //         });
    //     }
    // });

    (renderer, handler)
}
