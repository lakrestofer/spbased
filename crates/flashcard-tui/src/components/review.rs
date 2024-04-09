#![allow(non_snake_case)]
use crate::contexts::help::HelpContext;

use super::{root::ActiveView, Component, ComponentEventHandler, ComponentRenderer};
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, style::Stylize, widgets::Paragraph, Frame};
use reactive_graph::{
    effect::Effect,
    owner::use_context,
    signal::RwSignal,
    traits::{Get, Set, Update},
};
use std::sync::Arc;
use tracing::{info, instrument};

const REVIEW_HELP_TEXT: &str = "esc: go back";

#[instrument]
pub fn Review(active_view: RwSignal<ActiveView>) -> Component {
    info!("Building Review component");
    // context
    let help_text = use_context::<RwSignal<HelpContext>>().unwrap();

    // effects
    Effect::new_sync(move |_| {
        if active_view.get() == ActiveView::EditCard {
            help_text.update(|help_text| {
                help_text.clear_below_level(1);
                help_text.update_desc_at_level(REVIEW_HELP_TEXT, 1);
            });
        }
    });

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("running event handler for Review");
        if key_event.code == KeyCode::Esc {
            active_view.set(ActiveView::Home)
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering Review");
        let title = Paragraph::new("AddCard View").blue().centered();
        frame.render_widget(title, rect);
    });

    (renderer, handler)
}
