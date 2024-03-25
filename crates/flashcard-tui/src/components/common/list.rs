#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListState, Padding, Paragraph},
    Frame,
};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, GetUntracked, Set, Update},
};
use std::sync::Arc;

use super::super::{Component, ComponentEventHandler, ComponentRenderer};

pub fn List(
    title: String,
    is_focused: Arc<dyn Fn() -> bool + Send + Sync>,
    items: Arc<dyn Fn() -> Vec<String> + Send + Sync>,
) -> Component {
    // all the existing tags
    let widget_state = RwSignal::new(ListState::default());

    let handler: ComponentEventHandler =
        Arc::new(move |key_event: crossterm::event::KeyEvent| None);

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let list_widget = List::new(items())
            .block(
                Block::default()
                    .style(Style::default().fg(if is_focused() {
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
