#![allow(non_snake_case)]
use color_eyre::owo_colors::styles;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Styled},
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

/// A full textarea component with emacs keybindings
/// To retrieve the contents of the contents of the textarea,
/// the submit signal needs to be fired.
pub fn TextArea(
    title: String,
    is_focused: Memo<bool>,
    clear: RwSignal<()>,
    submit: RwSignal<()>,
    on_submit: Arc<dyn Fn(String) -> () + Send + Sync>,
) -> Component {
    // local state and derived setters
    let area = styled_text_area(title.to_string());
    let area = RwSignal::new(area);

    // HACK We have no good way to make the TextArea component
    // a controlled one. We will have to simulate it through the use of
    // side effects
    Effect::new_sync(move |_| {
        area.update(|area| {
            set_focused(area, is_focused.get());
        });
    });
    Effect::new_sync(move |_| {
        let title = title.to_string();
        _ = clear.get(); // subscribe to clear signal
        area.update(|area| *area = styled_text_area(title));
    });
    Effect::new_sync(move |_| {
        _ = submit.get();
        let new_content: String = area.get_untracked().lines().join("\n");
        on_submit(new_content);
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
