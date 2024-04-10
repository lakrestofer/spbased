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
use tracing::{info, instrument};

use crate::contexts::{events::EventsContext, help::HelpContext};

use super::{
    add_card::AddCard, bottom_bar::BottomBar, browser::Browser, edit_card::EditCard, home::Home,
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

#[instrument]
pub fn Root() -> Component {
    info!("Building Root component");
    // ==== define state begin ====
    // view state
    let active_view = RwSignal::new(ActiveView::AddCard);

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
    let (add_card_renderer, add_card_event_handler) = AddCard(active_view);
    // ==== init child components end ====

    // ==== Event handler begin ====
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("run event handler for Root");
        // save away the event such that we can read it anywhere
        event_context.update(|EventsContext(event)| *event = Some(key_event));
        add_card_event_handler(key_event)
    });
    // ==== Event handler begin ====

    // ==== Renderer begin ====
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, view_port: Rect| {
        info!("render Root");
        add_card_renderer(frame, view_port);
    });
    // ==== Renderer end ====

    (renderer, handler)
}
