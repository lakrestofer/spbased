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
    let area = RwSignal::new(styled_text_area());

    // we define functions that can modify local state and return them together with the renderer/handler
    let submit: Trigger = Arc::new({
        move || {
            let new_content: String = area.get_untracked().lines().join("\n").trim().into();
            if let Some(on_submit) = on_submit.clone() {
                on_submit(new_content);
            }
        }
    });
    let clear: Trigger = Arc::new(move || {
        area.update(|area| {
            *area = styled_text_area();
        });
    });

    if let Some(on_update) = on_update {
        let on_update = DebouncedFunction::new(ON_UPDATE_DURATION, on_update);
        Effect::new_sync(move |_| {
            let new_content: String = area.get().lines().join("\n").trim().into();
            on_update.call(new_content);
        });
    }

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("running event handler for text area");
        area.update(|area| _ = area.input(key_event));
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
        frame.render_widget(area.get().widget(), text_area);
    });

    ((renderer, handler), submit, clear)
}
