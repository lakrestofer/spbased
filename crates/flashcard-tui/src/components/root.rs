#![allow(non_snake_case)]
use crate::preamble::*;
use std::sync::Arc;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{computed::Memo, traits::Get};
use tracing::{info, instrument};

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Flex, Margin},
    widgets::Block,
};

use reactive_graph::{
    effect::Effect,
    signal::RwSignal,
    traits::{GetUntracked, Update},
};

use crate::components::{Component, ComponentEventHandler, ComponentRenderer, Trigger};
use crate::contexts::help::HelpContext;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FocusedField {
    Question,
    Answer,
    Tag,
}

impl FocusedField {
    pub fn next(&mut self) {
        *self = match self {
            FocusedField::Question => FocusedField::Answer,
            FocusedField::Answer => FocusedField::Tag,
            FocusedField::Tag => FocusedField::Question,
        }
    }
}
const ADD_CARD_HELP_TEXT: &str =
    "esc: go back, A-c: clear screen, A-enter: add card, tab/s-tab: navigate between sections";
const TAG_AREA_HELP_TEXT: &str = "C-up / C-down: Toggle search/list";

#[instrument]
pub fn Root() -> Component {
    info!("Building AddCard component");
    // state
    let focused_field = RwSignal::new(FocusedField::Question);
    let focus_next_field = move || {
        info!("Updating the focused field to the next one");
        focused_field.update(FocusedField::next);
        info!("Successfully updated the field to the next one!");
    };
    let help_text = RwSignal::new(HelpContext::new());

    let a_focused = Memo::new(move |_| focused_field.get() == FocusedField::Answer);
    let q_focused = Memo::new(move |_| focused_field.get() == FocusedField::Question);
    let t_focused = Memo::new(move |_| focused_field.get() == FocusedField::Tag);
    // memos
    let s_title: Memo<String> = Memo::new(move |_| {
        if t_focused.get() {
            "Search: [Press enter to add new tag]".into()
        } else {
            "Search:".into()
        }
    });

    // effects
    Effect::new_sync(move |_| {
        help_text.update(|help_text| {
            help_text.clear_below_level(1);
            help_text.update_desc_at_level(ADD_CARD_HELP_TEXT, 1);
            if t_focused.get() {
                help_text.update_desc_at_level(TAG_AREA_HELP_TEXT, 2)
            }
        });
    });

    // ====== Event handler ======

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!(
            "root: handling key event: {key_event:?}, focused field: {:?}",
            focused_field.get_untracked(),
        );
        if let KeyCode::Tab = key_event.code {
            focus_next_field()
        }
        info!("AddCard: returning from event handler");
        None
    });

    // ====== Renderer ======
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering root ");
        let [left, center, right] = Layout::horizontal([
            Constraint::Percentage(75),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(rect);

        let divider = Block::default().style(Style::default().bg(Color::Indexed(236)));

        frame.render_widget(&divider, center);

        let left = left.inner(&Margin::new(1, 1));
        let right = right.inner(&Margin::new(1, 1));

        let [upper_left, _, lower_left] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .flex(Flex::SpaceAround)
        .areas(left);

        // question field
        frame.render_widget(
            Paragraph::new("Question").style(if q_focused.get() {
                Style::default().bg(Color::Indexed(233))
            } else {
                Style::default().bg(Color::Indexed(235))
            }),
            upper_left,
        );
        // answer field
        frame.render_widget(
            Paragraph::new("Answer").style(if a_focused.get() {
                Style::default().bg(Color::Indexed(233))
            } else {
                Style::default().bg(Color::Indexed(235))
            }),
            lower_left,
        );
        // tag
        frame.render_widget(
            Paragraph::new(s_title.get()).style(if t_focused.get() {
                Style::default().bg(Color::Indexed(233))
            } else {
                Style::default().bg(Color::Indexed(235))
            }),
            right,
        );
    });

    (renderer, handler)
}

/// A full textarea component with emacs keybindings
#[instrument]
pub fn TextArea(
    title: Memo<String>,
    is_focused: Memo<bool>,
    on_submit: Option<ExtendedFn<String>>,
    on_update: Option<ExtendedFn<String>>,
) -> ComponentRenderer {
    info!("Building TextArea component");
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

    renderer
}
