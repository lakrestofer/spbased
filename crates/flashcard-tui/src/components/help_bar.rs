#![allow(non_snake_case)]

use super::{root::ActiveView, stub_component_event_handler, Component, ComponentRenderer};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{signal::RwSignal, traits::Get};
use std::sync::Arc;

pub fn HelpBar(active_view: RwSignal<ActiveView>) -> Component {
    let counter = RwSignal::new(0);

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        let mut help_text = String::new();
        match active_view.get() {
            ActiveView::Home => {
                help_text.push_str("C-c / q / esc: exit program, ");
            }
            ActiveView::AddCard => {
                help_text.push_str("esc: go back");
            }
            ActiveView::EditCard => {
                help_text.push_str("esc: go back");
            }
            ActiveView::Browser => {
                help_text.push_str("esc: go back");
            }
            ActiveView::Review => {
                help_text.push_str("esc: go back");
            }
        };
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, help_rect);
    });

    (renderer, stub_component_event_handler())
}
