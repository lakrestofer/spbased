#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use std::sync::Arc;

use super::{
    common::{list::List, text_area::TextArea},
    Component, ComponentEventHandler, ComponentRenderer, Trigger,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveField {
    AllTags,
    CardTags,
    Search,
}

impl ActiveField {
    fn up(&mut self) {
        *self = match self {
            ActiveField::AllTags => ActiveField::Search,
            ActiveField::CardTags => ActiveField::AllTags,
            ActiveField::Search => ActiveField::CardTags,
        }
    }
    fn down(&mut self) {
        *self = match self {
            ActiveField::AllTags => ActiveField::CardTags,
            ActiveField::CardTags => ActiveField::Search,
            ActiveField::Search => ActiveField::AllTags,
        }
    }
}

pub fn TagArea(is_focused: Memo<bool>) -> (Component, Trigger) {
    // ==== State and setters/getters ====
    // active field
    let active_field = RwSignal::new(ActiveField::Search);
    let up = move || active_field.update(|field| field.up());
    let down = move || active_field.update(|field| field.down());
    let search_bar_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::Search);
    let all_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::AllTags);
    let card_tags_is_focused =
        Memo::new(move |_| is_focused.get() && active_field.get() == ActiveField::CardTags);
    let search_bar_title: Memo<String> = Memo::new(move |_| {
        if search_bar_focused.get() {
            "Search: [Press enter to add new tag]".into()
        } else {
            "Search:".into()
        }
    });
    // tags and filter
    let filter = RwSignal::new(String::new());
    let all_tags = RwSignal::new(vec![
        "chemistry".into(),
        "physics".into(),
        "datastructures".into(),
        "algorithms".into(),
    ]);
    let card_tags = RwSignal::new(Vec::new());
    let filtered_all_tags: Memo<Vec<String>> = Memo::new(move |_| {
        let tags = all_tags.get();
        let filter = filter.get();
        tags.into_iter()
            .filter(|s: &String| s.contains(&filter))
            .collect()
    });
    let filtered_card_tags: Memo<Vec<String>> = Memo::new(move |_| {
        let tags = card_tags.get();
        let filter = filter.get();
        tags.into_iter()
            .filter(|s: &String| s.contains(&filter))
            .collect()
    });
    let search_bar_on_submit = move |content: String| {
        if !content.is_empty() {
            card_tags.update(|tags| tags.push(content))
        }
    };
    let search_bar_on_update = move |content: String| filter.update(|f| *f = content);

    // ======= Components =======
    let (all_tags_renderer, all_tags_handler) =
        List("All Tags".into(), all_tags_is_focused, filtered_all_tags);
    let (card_tags_renderer, card_tags_handler) = List(
        "Tags on this card".into(),
        card_tags_is_focused,
        filtered_card_tags,
    );
    let ((s_renderer, s_handler), s_submit, s_clear) = TextArea(
        search_bar_title,
        search_bar_focused,
        Some(Arc::new(search_bar_on_submit)),
        Some(Arc::new(search_bar_on_update)),
    );

    // ======= Event handler ========

    let add_tag = {
        let s_clear = s_clear.clone();
        move || {
            s_submit();
            s_clear();
        }
    };

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match (
            key_event.code,
            key_event.modifiers,
            active_field.get_untracked(),
        ) {
            (KeyCode::Up, KeyModifiers::CONTROL, _) => up(),
            (KeyCode::Down, KeyModifiers::CONTROL, _) => down(),
            (KeyCode::Enter, _, ActiveField::Search) => add_tag(),
            (_, _, ActiveField::AllTags) => return all_tags_handler(key_event),
            (_, _, ActiveField::CardTags) => return card_tags_handler(key_event),
            (_, _, ActiveField::Search) => return s_handler(key_event),
            // _, _, _ => {}
        }
        None
    });

    // ======= Renderer ========
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let [upper, _, center, _, lower] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(2),
            ],
        )
        // .flex(Flex::SpaceBetween)
        .areas(rect);

        all_tags_renderer(frame, upper);
        card_tags_renderer(frame, center);
        s_renderer(frame, lower);
    });

    ((renderer, handler), s_clear)
}
