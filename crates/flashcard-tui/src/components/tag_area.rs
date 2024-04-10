#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use std::sync::Arc;
use tracing::{info, instrument};

use super::{
    common::text_area::TextArea, Component, ComponentEventHandler, ComponentRenderer, Trigger,
};

#[instrument]
pub fn TagArea(is_focused: Memo<bool>) -> (Component, Trigger) {
    info!("Building TagArea component");
    let search_bar_focused = Memo::new(move |_| is_focused.get());
    let search_bar_title: Memo<String> = Memo::new(move |_| {
        if search_bar_focused.get() {
            "Search: [Press enter to add new tag]".into()
        } else {
            "Search:".into()
        }
    });

    // ======= Components =======
    let ((s_renderer, s_handler), _, s_clear) =
        TextArea(search_bar_title, search_bar_focused, None, None);

    // ======= Event handler ========

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("run event handler for tag area");
        s_handler(key_event)
    });

    // ======= Renderer ========
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("render tag area");
        s_renderer(frame, rect);
    });

    ((renderer, handler), s_clear)
}
