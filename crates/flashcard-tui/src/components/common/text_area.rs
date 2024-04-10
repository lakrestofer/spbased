#![allow(non_snake_case)]

use crate::preamble::*;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{computed::Memo, traits::Get};
use std::{sync::Arc, time::Duration};

use tracing::{info, instrument};
use tui_textarea::TextArea as TuiTextArea;

use crate::components::{Component, ComponentEventHandler, ComponentRenderer, Trigger};

/// A full textarea component with emacs keybindings
#[instrument]
pub fn TextArea(
    title: Memo<String>,
    is_focused: Memo<bool>,
    on_submit: Option<ExtendedFn<String>>,
    on_update: Option<ExtendedFn<String>>,
) -> (Component, Trigger, Trigger) {
    info!("Building TextArea component");
    // local state and derived setters
    // let area = RwSignal::new(styled_text_area());

    // we define functions that can modify local state and return them together with the renderer/handler
    let submit: Trigger = Arc::new(move || {});
    let clear: Trigger = Arc::new(move || {});

    let handler: ComponentEventHandler = Arc::new(move |_key_event: crossterm::event::KeyEvent| {
        info!("running event handler for text area");
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering text area");

        let style = {
            if is_focused.get() {
                Style::default().bg(Color::Indexed(233))
            } else {
                Style::default().bg(Color::Indexed(235))
            }
        };
        frame.render_widget(Paragraph::new(title.get()).style(style), rect);
    });

    ((renderer, handler), submit, clear)
}
