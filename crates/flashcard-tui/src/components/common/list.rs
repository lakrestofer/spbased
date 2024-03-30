#![allow(non_snake_case)]
use super::super::{Component, ComponentEventHandler, ComponentRenderer};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, Update},
};
use std::sync::Arc;

use crate::components::stub_component_event_handler;

pub fn List(title: String, is_focused: Memo<bool>, items: Memo<Vec<String>>) -> Component {
    // all the existing tags
    let widget_state = RwSignal::new(ListState::default());

    let handler: ComponentEventHandler = stub_component_event_handler();

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let list_widget = List::new(items.get())
            .block(
                Block::default()
                    .style(Style::default().fg(if is_focused.get() {
                        Color::LightBlue
                    } else {
                        Color::White
                    }))
                    .title(title.clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        widget_state.update(|state| frame.render_stateful_widget(list_widget, rect, state));
    });

    (renderer, handler)
}
