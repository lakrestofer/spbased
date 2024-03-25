#![allow(non_snake_case)]
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Styled},
    text::Line,
    widgets::{Block, BorderType, Borders},
    Frame,
};
use reactive_graph::{
    effect::Effect,
    signal::RwSignal,
    traits::{Get, Update},
};
use std::sync::Arc;

use tui_textarea::TextArea;

use crate::components::{Component, ComponentEventHandler, ComponentRenderer};

use super::super::utils::set_focused_block;

/// modify the styling of a textarea to reflect it being
/// focused or not
fn set_focused(textarea: &mut TextArea, focused: bool) {
    let mut style = Style::default().add_modifier(Modifier::REVERSED);

    if !focused {
        style = style.add_modifier(Modifier::DIM);
    }

    let mut block = textarea.block().unwrap().clone();

    set_focused_block(&mut block, focused);

    textarea.set_cursor_style(style);
    textarea.set_block(block);
}

pub fn TextArea(title: &str, is_focused: Arc<dyn Fn() -> bool + Send + Sync>) -> Component {
    // local state and derived setters
    let mut area = TextArea::default();
    area.set_cursor_line_style(Style::default());
    area.set_block(
        Block::default()
            .title_top(Line::from(title.to_string()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain),
    );
    let area = RwSignal::new(area);

    // effects
    Effect::new_sync(move |_| {
        area.update(|area| set_focused(area, is_focused()));
    });

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        area.update(|area| _ = area.input(key_event));
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        frame.render_widget(area.get().widget(), rect);
    });

    (renderer, handler)
}
