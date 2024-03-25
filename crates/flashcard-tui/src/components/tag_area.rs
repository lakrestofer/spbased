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

use super::{common::text_area::TextArea, Component, ComponentRenderer};

pub fn TagArea(is_focused: Arc<dyn Fn() -> bool + Send + Sync>) -> Component {
    let tags: RwSignal<Vec<String>> =
        RwSignal::new(vec!["example".into(), "here we go again".into()]);
    let tag_widget_state = RwSignal::new(ListState::default());

    let (tag_renderer, tag_event_handler) = TextArea("Tags", is_focused);

    // let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
    //     _ = tag_event_handler(key_event);
    //     None
    // });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let list = List::new(tags.get())
            .block(Block::default().borders(Borders::ALL.difference(Borders::TOP)))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        let [upper, lower] = Layout::new(
            Direction::Vertical,
            [Constraint::Length(3), Constraint::Fill(1)],
        )
        .areas(rect);

        // tag field
        tag_renderer(frame, upper);
        tag_widget_state.update(|state| frame.render_stateful_widget(list, lower, state));
    });

    (renderer, tag_event_handler)
}
