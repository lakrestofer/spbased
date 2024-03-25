#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, Update},
};
use std::sync::Arc;

use super::{
    common::{list::List, text_area::TextArea},
    Component, ComponentEventHandler, ComponentRenderer,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveField {
    // list of tags that are one this card
    CardTags,
    // list of all tags in db
    AllTags,
    // the search field
    Search,
}

impl ActiveField {
    fn up(&mut self) {
        *self = match self {
            ActiveField::CardTags => ActiveField::Search,
            ActiveField::AllTags => ActiveField::CardTags,
            ActiveField::Search => ActiveField::AllTags,
        }
    }
    fn down(&mut self) {
        *self = match self {
            ActiveField::CardTags => ActiveField::AllTags,
            ActiveField::AllTags => ActiveField::Search,
            ActiveField::Search => ActiveField::CardTags,
        }
    }
}

pub fn TagArea(is_focused: Arc<dyn Fn() -> bool + Send + Sync>) -> Component {
    // state and setters/getters
    let active_field = RwSignal::new(ActiveField::Search);
    let all_tags_is_focused = Arc::new({
        let is_focused = is_focused.clone();
        move || is_focused() && active_field.get() == ActiveField::CardTags
    });
    let card_tags_is_focused = Arc::new({
        let is_focused = is_focused.clone();
        move || is_focused() && active_field.get() == ActiveField::AllTags
    });
    let search_is_focused =
        Arc::new(move || is_focused() && active_field.get() == ActiveField::Search);
    let up = move || active_field.update(|field| field.up());
    let down = move || active_field.update(|field| field.down());

    // children
    let (all_tags_renderer, all_tags_handler) =
        List("All Tags".into(), all_tags_is_focused, Arc::new(|| vec![]));
    let (card_tags_renderer, card_tags_handler) = List(
        "Tags on this card".into(),
        card_tags_is_focused,
        Arc::new(|| vec![]),
    );
    let (tag_renderer, tag_event_handler) = TextArea("Search/Add tag", search_is_focused);

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match key_event.code {
            KeyCode::Up if key_event.modifiers.contains(KeyModifiers::CONTROL) => up(),
            KeyCode::Down if key_event.modifiers.contains(KeyModifiers::CONTROL) => down(),
            _ => return tag_event_handler(key_event),
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let [upper, center, lower] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Length(3),
            ],
        )
        .areas(rect);

        all_tags_renderer(frame, upper);
        card_tags_renderer(frame, center);
        tag_renderer(frame, lower);
    });

    (renderer, handler)
}
