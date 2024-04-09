#![allow(non_snake_case)]
use crate::contexts::help::HelpContext;

use super::common::text_area::TextArea;
use super::{
    root::ActiveView, tag_area::TagArea, Component, ComponentEventHandler, ComponentRenderer,
};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Margin;
use ratatui::style::{Color, Style};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Block,
    Frame,
};
use reactive_graph::effect::Effect;
use reactive_graph::owner::use_context;
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, GetUntracked, Set, Update},
};
use std::sync::Arc;
use tracing::{info, instrument};

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
    pub fn previous(&mut self) {
        *self = match self {
            FocusedField::Question => FocusedField::Tag,
            FocusedField::Answer => FocusedField::Question,
            FocusedField::Tag => FocusedField::Answer,
        }
    }
}

const ADD_CARD_HELP_TEXT: &str =
    "esc: go back, A-c: clear screen, A-enter: add card, tab/s-tab: navigate between sections";
const TAG_AREA_HELP_TEXT: &str = "C-up / C-down: Toggle search/list";

#[instrument]
pub fn AddCard(active_view: RwSignal<ActiveView>) -> Component {
    info!("Building AddCard component");
    // state
    let focused_field = RwSignal::new(FocusedField::Question);
    let focus_next_field = move || focused_field.update(FocusedField::next);
    let focus_previous_field = move || focused_field.update(FocusedField::previous);
    let a_focused = Memo::new(move |_| focused_field.get() == FocusedField::Answer);
    let a_text = RwSignal::new(String::new());
    let q_focused = Memo::new(move |_| focused_field.get() == FocusedField::Question);
    let q_text = RwSignal::new(String::new());
    let t_focused = Memo::new(move |_| focused_field.get() == FocusedField::Tag);

    // context
    let help_text = use_context::<RwSignal<HelpContext>>().unwrap();

    // effects
    Effect::new_sync(move |_| {
        if active_view.get() == ActiveView::AddCard {
            help_text.update(|help_text| {
                help_text.clear_below_level(1);
                help_text.update_desc_at_level(ADD_CARD_HELP_TEXT, 1);
                if t_focused.get() {
                    help_text.update_desc_at_level(TAG_AREA_HELP_TEXT, 2)
                }
            });
        }
    });

    // ===== Components =======
    let ((q_renderer, q_handler), _, q_clear) = TextArea(
        Memo::new(|_| "Question".into()),
        q_focused,
        None,
        Some(Arc::new(move |content| {
            q_text.update(|s| *s = content);
        })),
    );
    let ((a_renderer, a_handler), _, a_clear) = TextArea(
        Memo::new(|_| "Answer".into()),
        a_focused,
        None,
        Some(Arc::new(move |content| {
            a_text.update(|s| *s = content);
        })),
    );
    let ((t_renderer, t_handler), t_clear) = TagArea(t_focused);

    // ====== Event handler ======
    let clear = move || {
        q_clear();
        a_clear();
        t_clear();
    };

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
            (KeyCode::Esc, _, _) => active_view.set(ActiveView::Home),
            (KeyCode::Tab, _, _) => focus_next_field(),
            (KeyCode::BackTab, _, _) => focus_previous_field(),
            (KeyCode::Char('c'), KeyModifiers::ALT, _) => clear(),
            (_, _, FocusedField::Question) => return q_handler(key_event),
            (_, _, FocusedField::Answer) => return a_handler(key_event),
            (_, _, FocusedField::Tag) => return t_handler(key_event),
            // (_, _, _) => {}
        }
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

        // let [_upper_right, lower_right] = Layout::vertical([Constraint::Percentage(40); 2])
        //     .flex(Flex::SpaceAround)
        //     .areas(right);

        // question field
        q_renderer(frame, upper_left);

        // answer field
        a_renderer(frame, lower_left);

        // tag
        t_renderer(frame, right);

        // we take the upper right area and split it into multiple lines
        // let [upper, center, lower] = Layout::vertical([
        //     Constraint::Fill(1),
        //     Constraint::Fill(1),
        //     Constraint::Fill(1),
        // ])
        // .areas(upper_right);
        // stats
        // frame.render_widget(Paragraph::new(format!("counter: {}", counter.get())), upper);
        // frame.render_widget(Paragraph::new(format!("q_text: {}", q_text.get())), center);
    });

    (renderer, handler)
}
