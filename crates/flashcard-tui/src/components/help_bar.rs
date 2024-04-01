#![allow(non_snake_case)]
use super::{
    common::throbber::Throbber, root::ActiveView, stub_component_event_handler, Component,
    ComponentRenderer,
};
use ratatui::{layout::Rect, widgets::Paragraph, Frame};
use reactive_graph::{signal::RwSignal, traits::Get};
use std::sync::Arc;

pub fn HelpBar(active_view: RwSignal<ActiveView>) -> Component {
    let (throbber_renderer, _) = Throbber();

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        let mut help_text = String::new();
        help_text.push_str("C-c: exit program, ");
        match active_view.get() {
            ActiveView::Home => help_text.push_str("q / esc: exit program, "),
            ActiveView::AddCard => {
                help_text.push_str("esc: go back, ");
                help_text.push_str("tab/s-tab: navigate between sections");
            }
            ActiveView::EditCard => help_text.push_str("esc: go back"),
            ActiveView::Browser => help_text.push_str("esc: go back"),
            ActiveView::Review => help_text.push_str("esc: go back"),
        };
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, help_rect);
        throbber_renderer(frame, help_rect);
    });

    (renderer, stub_component_event_handler())
}
