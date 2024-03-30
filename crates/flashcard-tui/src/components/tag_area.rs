#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, GetUntracked, Set, Update},
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

pub fn TagArea(is_focused: Memo<bool>) -> Component {
    // ==== state and setters/getters ====
    // active field
    let active_field = RwSignal::new(ActiveField::Search);
    let card_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::AllTags);

    let up = move || active_field.update(|field| field.up());
    let down = move || active_field.update(|field| field.down());

    // tags
    let all_tags = RwSignal::new(Vec::new());
    let card_tags = RwSignal::new(Vec::new());

    // children
    // all tags
    let all_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::CardTags);
    let (all_tags_renderer, all_tags_handler) = List(
        "All Tags".into(),
        all_tags_is_focused,
        Memo::new(move |_| all_tags.get()),
    );
    // card tags
    let (card_tags_renderer, card_tags_handler) = List(
        "Tags on this card".into(),
        card_tags_is_focused,
        Memo::new(move |_| card_tags.get()),
    );
    // search
    let s_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::Search);
    let s_clear = RwSignal::new(());
    let s_submit = RwSignal::new(());
    let s_on_submit = move |content| {
        card_tags.update(|tags| tags.push(content));
        s_clear.set(()); // clear
    };
    let (s_renderer, s_handler) = TextArea(
        "Search/Add tag".into(),
        s_focused,
        s_clear,
        s_submit,
        Arc::new(s_on_submit),
    );

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match key_event.code {
            KeyCode::Up if key_event.modifiers.contains(KeyModifiers::CONTROL) => up(),
            KeyCode::Down if key_event.modifiers.contains(KeyModifiers::CONTROL) => down(),
            KeyCode::Enter => match active_field.get_untracked() {
                ActiveField::CardTags => {}
                ActiveField::AllTags => {}
                ActiveField::Search => s_submit.set(()), // call the on_submit passed to the search field
            },
            _ => match active_field.get_untracked() {
                ActiveField::AllTags => return all_tags_handler(key_event),
                ActiveField::CardTags => return card_tags_handler(key_event),
                ActiveField::Search => return s_handler(key_event),
            },
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
        s_renderer(frame, lower);
    });

    (renderer, handler)
}
