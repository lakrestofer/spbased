#![allow(non_snake_case)]

use crate::preamble::*;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{
    computed::Memo,
    effect::Effect,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use std::{sync::Arc, time::Duration};

use tracing::{info, instrument};
use tui_textarea::TextArea as TuiTextArea;

use crate::{
    components::{Component, ComponentEventHandler, ComponentRenderer, Trigger},
    util::DebouncedFunction,
};

fn styled_text_area<'a>() -> TuiTextArea<'a> {
    let mut area = TuiTextArea::default();
    area.set_style(Style::default().bg(Color::Indexed(234)));
    area.set_cursor_line_style(Style::default());
    area
}

const ON_UPDATE_DURATION: Duration = Duration::from_millis(200);

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

    if let Some(on_update) = on_update {}

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("running event handler for text area");
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering text area");
        let [title_area, text_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(rect);

        let style = {
            if is_focused.get() {
                Style::default().bg(Color::Indexed(233))
            } else {
                Style::default().bg(Color::Indexed(235))
            }
        };
        frame.render_widget(Paragraph::new(title.get()).style(style), title_area);
    });

    ((renderer, handler), submit, clear)
}
