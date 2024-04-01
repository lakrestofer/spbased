#![allow(non_snake_case)]
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    effect::Effect,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use std::{sync::Arc, time::Duration};

use tui_textarea::TextArea;

use crate::{
    components::{Component, ComponentEventHandler, ComponentRenderer, Trigger},
    util::DebouncedFunction,
};

use super::super::utils::set_focused_block;

/// modify the styling of a textarea to reflect it being
/// focused or not
fn set_focused(textarea: &mut TextArea, title: String, focused: bool) {
    let mut style = Style::default().add_modifier(Modifier::REVERSED);

    if !focused {
        style = style.add_modifier(Modifier::DIM);
    }

    let mut block = Block::default()
        .title_top(Line::from(title))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    set_focused_block(&mut block, focused);

    textarea.set_cursor_style(style);
    textarea.set_block(block);
}

fn styled_text_area<'a>(title: String) -> TextArea<'a> {
    let mut area = TextArea::default();
    area.set_cursor_line_style(Style::default());
    area.set_block(
        Block::default()
            .title_top(Line::from(title))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain),
    );
    area
}

const ON_UPDATE_DURATION: Duration = Duration::from_millis(200);

/// A full textarea component with emacs keybindings
pub fn TextArea(
    title: Memo<String>,
    is_focused: Memo<bool>,
    on_submit: Option<Arc<dyn Fn(String) -> () + Send + Sync>>,
    on_update: Option<Arc<dyn Fn(String) -> () + Send + Sync>>,
) -> (Component, Trigger, Trigger) {
    // local state and derived setters
    let area = styled_text_area(title.get());
    let area = RwSignal::new(area);

    // update the styling of the area when is_focused changes
    Effect::new_sync(move |_| area.update(|area| set_focused(area, title.get(), is_focused.get())));

    // we define functions that can modify local state and return them together with the renderer/handler
    let submit: Trigger = Arc::new({
        let area = area.clone();
        move || {
            let new_content: String = area.get_untracked().lines().join("\n").trim().into();
            if let Some(on_submit) = on_submit.clone() {
                on_submit(new_content);
            }
        }
    });
    let clear: Trigger = Arc::new(move || {
        area.update(|area| {
            let title = title.get();
            *area = styled_text_area(title.clone());
            set_focused(area, title, is_focused.get_untracked())
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
        area.update(|area| _ = area.input(key_event));
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        frame.render_widget(area.get().widget(), rect);
    });

    ((renderer, handler), submit, clear)
}
