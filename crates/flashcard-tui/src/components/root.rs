#![allow(non_snake_case)]
use std::sync::{Arc, RwLock};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};

use super::{
    add_card::AddCard, browser::Browser, edit_card::EditCard, help_bar::HelpBar, home::Home,
    review::Review, Component, ComponentEventHandler, ComponentRenderer, DynamicRect,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Home,
    AddCard,
    EditCard,
    Browser,
    Review,
}

pub fn Root() -> Component {
    // ==== define state begin ====
    let active_view = RwSignal::new(ActiveView::Home);
    // ==== define state end ====

    // ==== init child components begin ====
    let (home_renderer, home_event_handler) = Home(active_view);
    let (add_card_renderer, add_card_event_handler) = AddCard(active_view);
    let (edit_card_renderer, edit_card_event_handler) = EditCard(active_view);
    let (browser_renderer, browser_event_handler) = Browser(active_view);
    let (review_renderer, review_event_handler) = Review(active_view);
    let (help_bar_renderer, _) = HelpBar(active_view);
    // ==== init child components end ====

    // ==== Event handler begin ====
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match active_view.get_untracked() {
            ActiveView::Home => return home_event_handler(key_event),
            ActiveView::AddCard => return add_card_event_handler(key_event),
            ActiveView::EditCard => return edit_card_event_handler(key_event),
            ActiveView::Browser => return browser_event_handler(key_event),
            ActiveView::Review => return review_event_handler(key_event),
        }
        // None
    });
    // ==== Event handler begin ====

    // ==== Renderer begin ====
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, view_port: Rect| {
        let [center, help] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .areas(view_port);

        match active_view.get() {
            ActiveView::Home => home_renderer(frame, center),
            ActiveView::AddCard => add_card_renderer(frame, center),
            ActiveView::Browser => browser_renderer(frame, center),
            ActiveView::EditCard => edit_card_renderer(frame, center),
            ActiveView::Review => review_renderer(frame, center),
        }
        help_bar_renderer(frame, help);
    });
    // ==== Renderer end ====

    (renderer, handler)
}
