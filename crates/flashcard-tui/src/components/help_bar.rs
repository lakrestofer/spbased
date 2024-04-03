#![allow(non_snake_case)]
use crate::contexts::{events::EventsContext, help::HelpContext};

use super::{
    common::throbber::Throbber, root::ActiveView, stub_component_event_handler, Component,
    ComponentEventHandler, ComponentRenderer,
};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use reactive_graph::{
    owner::use_context,
    signal::RwSignal,
    traits::{Get, Update},
};
use std::sync::Arc;

pub fn HelpBar() -> Component {
    // == Context ===
    let help_text_context = use_context::<RwSignal<HelpContext>>().unwrap();
    let event_context = use_context::<RwSignal<EventsContext>>().unwrap();
    // === Components ===

    let (throbber_renderer, _) = Throbber();

    // === Renderer ===

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        let [bar, help] = Layout::vertical([Constraint::Ratio(1, 2); 2]).areas(help_rect);
        let EventsContext(key_event) = event_context.get();
        let mut event_str = String::new();
        if let Some(key_event) = key_event {
            event_str = format!(
                "Event {{ key: {:?}, modifier: {:?} }}",
                key_event.code, key_event.modifiers
            );
        }
        frame.render_widget(
            Paragraph::new(event_str)
                .centered()
                .style(Style::default().bg(Color::Indexed(233))),
            bar,
        );
        let bar = bar.inner(&Margin::new(2, 0));
        throbber_renderer(frame, bar);

        let help_text = help_text_context.get().into_help_string();
        frame.render_widget(
            Paragraph::new(help_text)
                .style(Style::default().fg(Color::Yellow))
                .centered(),
            help,
        );
    });

    (renderer, stub_component_event_handler())
}
