#![allow(non_snake_case)]
use std::sync::Arc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use reactive_graph::{
    owner::provide_context,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};

use crate::contexts::{events::EventsContext, help::HelpContext};

use super::{
    add_card::AddCard, browser::Browser, edit_card::EditCard, help_bar::HelpBar, home::Home,
    review::Review, Component, ComponentEventHandler, ComponentRenderer,
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
    // view state
    let active_view = RwSignal::new(ActiveView::Home);

    // state for contexts
    let help_context: RwSignal<HelpContext> = RwSignal::new(HelpContext::new());
    let event_context = RwSignal::new(EventsContext(None));
    // ==== define state end ====

    // ==== define context begin ====
    help_context.update(|help_context| help_context.update_desc_at_level("C-c: exit program", 0));
    provide_context(help_context);
    provide_context(event_context);

    // ==== define context end ====

    // ==== init child components begin ====
    let (home_renderer, home_event_handler) = Home(active_view);
    let (add_card_renderer, add_card_event_handler) = AddCard(active_view);
    let (edit_card_renderer, edit_card_event_handler) = EditCard(active_view);
    let (browser_renderer, browser_event_handler) = Browser(active_view);
    let (review_renderer, review_event_handler) = Review(active_view);
    let (help_bar_renderer, help_bar_handler) = HelpBar();
    // ==== init child components end ====

    // ==== Event handler begin ====
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        let res = match active_view.get_untracked() {
            ActiveView::Home => home_event_handler(key_event),
            ActiveView::AddCard => add_card_event_handler(key_event),
            ActiveView::EditCard => edit_card_event_handler(key_event),
            ActiveView::Browser => browser_event_handler(key_event),
            ActiveView::Review => review_event_handler(key_event),
        };
        if res.is_some() {
            return res;
        }
        // save away the event such that we can read it anywhere
        event_context.update(|EventsContext(event)| *event = Some(key_event));
        res
    });
    // ==== Event handler begin ====

    // ==== Renderer begin ====
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, view_port: Rect| {
        let [center, help_rect] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(2)])
            .areas(view_port);

        match active_view.get() {
            ActiveView::Home => home_renderer(frame, center),
            ActiveView::AddCard => add_card_renderer(frame, center),
            ActiveView::Browser => browser_renderer(frame, center),
            ActiveView::EditCard => edit_card_renderer(frame, center),
            ActiveView::Review => review_renderer(frame, center),
        }
        help_bar_renderer(frame, help_rect);
    });
    // ==== Renderer end ====

    (renderer, handler)
}
