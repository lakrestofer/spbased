#![allow(non_snake_case)]
use crate::contexts::help::HelpContext;

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
    // === State ===
    let event_text = RwSignal::new(String::new());

    // == Context ===
    let help_text = use_context::<RwSignal<HelpContext>>().unwrap();

    // Effect::new_sync(move |_| {
    //     let mut new_help_text = String::new();
    //     new_help_text.push_str("C-c: exit program, ");
    //     match active_view.get() {
    //         ActiveView::AddCard => {
    //             new_help_text.push_str("esc: go back, ");
    //             new_help_text.push_str("A-c: clear screen, ");
    //             new_help_text.push_str("A-enter: add card, ");
    //             new_help_text.push_str("tab/s-tab: navigate between sections");
    //         }
    //         ActiveView::EditCard => new_help_text.push_str("esc: go back"),
    //         ActiveView::Browser => new_help_text.push_str("esc: go back"),
    //         ActiveView::Review => new_help_text.push_str("esc: go back"),
    //     };
    //     help_text.update(|help_text| *help_text = new_help_text);
    // });

    // === Components ===

    let (throbber_renderer, _) = Throbber();

    // === Event handler ===
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        event_text.update(|event| {
            *event = format!(
                "Event {{ key: {:?}, modifier: {:?} }}",
                key_event.code, key_event.modifiers
            )
        });
        None
    });

    // === Renderer ===

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        let [bar, help] = Layout::vertical([Constraint::Ratio(1, 2); 2]).areas(help_rect);
        frame.render_widget(
            Paragraph::new(event_text.get())
                .centered()
                .style(Style::default().bg(Color::Indexed(233))),
            bar,
        );
        let bar = bar.inner(&Margin::new(2, 0));
        throbber_renderer(frame, bar);

        let help_text = help_text.get().into_help_string();
        frame.render_widget(
            Paragraph::new(help_text)
                .style(Style::default().fg(Color::Yellow))
                .centered(),
            help,
        );
    });

    (renderer, handler)
}
