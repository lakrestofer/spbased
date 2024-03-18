#![allow(non_snake_case)]

use super::{stub_component_event_handler, Component, ComponentRenderer};
use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use reactive_graph::{signal::RwSignal, traits::Get};
use std::sync::Arc;

pub fn HelpBar() -> Component {
    let counter = RwSignal::new(0);

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        let mut help_text = String::new();
        help_text.push_str("C-c: exit");
        help_text.push_str(&format!("Counter: {}", counter.get()));
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, help_rect);
    });

    (renderer, stub_component_event_handler())
}
