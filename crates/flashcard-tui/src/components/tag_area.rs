#![allow(non_snake_case)]
use color_eyre::eyre::{eyre, Result};
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
    AllTags = 0,
    CardTags = 1,
    Search = 2,
}

impl TryFrom<u8> for ActiveField {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => ActiveField::AllTags,
            1 => ActiveField::CardTags,
            2 => ActiveField::Search,
            _ => return Err(eyre!("Could not convert from {} to enum", value)),
        };
        Ok(res)
    }
}

impl ActiveField {
    fn up(&mut self) {
        *self = (*self as u8)
            .checked_sub(1)
            .unwrap_or(2)
            .try_into()
            .unwrap();
    }
    fn down(&mut self) {
        *self = (((*self as u8) + 1) % 3).try_into().unwrap();
    }
}

pub fn TagArea(is_focused: Memo<bool>) -> Component {
    // ==== state and setters/getters ====
    // active field
    let active_field = RwSignal::new(ActiveField::Search);

    let up = move || active_field.update(|field| field.up());
    let down = move || active_field.update(|field| field.down());

    // tags
    let all_tags = RwSignal::new(Vec::new());
    let card_tags = RwSignal::new(Vec::new());

    // children
    // all tags
    let all_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::AllTags);
    let (all_tags_renderer, all_tags_handler) = List(
        "All Tags".into(),
        all_tags_is_focused,
        Memo::new(move |_| all_tags.get()),
    );
    // card tags
    let card_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::CardTags);
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
        match (
            key_event.code,
            key_event.modifiers,
            active_field.get_untracked(),
        ) {
            (KeyCode::Up, KeyModifiers::CONTROL, _) => up(),
            (KeyCode::Down, KeyModifiers::CONTROL, _) => down(),
            (KeyCode::Enter, _, ActiveField::Search) => s_submit.set(()),
            (_, _, ActiveField::AllTags) => return all_tags_handler(key_event),
            (_, _, ActiveField::CardTags) => return card_tags_handler(key_event),
            (_, _, ActiveField::Search) => return s_handler(key_event),
            // _, _, _ => {}
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
