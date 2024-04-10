#![allow(non_snake_case)]
use std::sync::Arc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::Block,
    Frame,
};
use reactive_graph::{
    computed::Memo,
    effect::Effect,
    owner::{provide_context, use_context},
    signal::RwSignal,
    traits::{Get, GetUntracked, Update},
};
use tracing::{info, instrument};

use crate::{components::common::text_area::TextArea, contexts::help::HelpContext};

use super::{Component, ComponentEventHandler, ComponentRenderer};

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
        focused_field
            .try_update(FocusedField::next)
            .expect("could not update focused field");
        info!("Successfully updated the field to the next one!");
    };
    let help_text = RwSignal::new(HelpContext::new());
    provide_context(help_text);

    let a_focused = Memo::new(move |_| focused_field.get() == FocusedField::Answer);
    let q_focused = Memo::new(move |_| focused_field.get() == FocusedField::Question);
    let t_focused = Memo::new(move |_| focused_field.get() == FocusedField::Tag);

    // context
    let help_text = use_context::<RwSignal<HelpContext>>().unwrap();

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

    // ===== Components =======
    let ((q_renderer, q_handler), _, _) =
        TextArea(Memo::new(|_| "Question".into()), q_focused, None, None);
    let ((a_renderer, a_handler), _, _) =
        TextArea(Memo::new(|_| "Answer".into()), a_focused, None, None);
    let s_title: Memo<String> = Memo::new(move |_| {
        if t_focused.get() {
            "Search: [Press enter to add new tag]".into()
        } else {
            "Search:".into()
        }
    });
    let ((t_renderer, t_handler), _, _) = TextArea(s_title, t_focused, None, None);

    // ====== Event handler ======

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!(
            "AddCard: handling key event: {key_event:?}, focused field: {:?}",
            focused_field.get_untracked(),
        );
        match (
            key_event.code,
            key_event.modifiers,
            focused_field.get_untracked(),
        ) {
            (KeyCode::Tab, _, _) => focus_next_field(),
            (_, _, FocusedField::Question) => return q_handler(key_event),
            (_, _, FocusedField::Answer) => return a_handler(key_event),
            (_, _, FocusedField::Tag) => return t_handler(key_event),
            (_, _, _) => {}
        }
        info!("AddCard: returning from event handler");
        None
    });

    // ====== Renderer ======
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        info!("rendering add_card");
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
        q_renderer(frame, upper_left);

        // answer field
        a_renderer(frame, lower_left);

        // tag
        t_renderer(frame, right);
    });

    (renderer, handler)
}
