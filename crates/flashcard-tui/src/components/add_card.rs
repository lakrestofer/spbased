#![allow(non_snake_case)]
use super::common::text_area::TextArea;
use super::{
    root::ActiveView, tag_area::TagArea, Component, ComponentEventHandler, ComponentRenderer,
};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    signal::RwSignal,
    traits::{Get, GetUntracked, Set, Update},
};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub fn AddCard(active_view: RwSignal<ActiveView>) -> Component {
    // state
    let focused_field = RwSignal::new(FocusedField::Question);
    let focus_next_field = move || focused_field.update(FocusedField::next);
    let focus_previous_field = move || focused_field.update(FocusedField::previous);
    let a_focused = Memo::new(move |_| focused_field.get() == FocusedField::Answer);
    let a_clear = RwSignal::new(());
    let a_text = RwSignal::new(String::new());
    let q_focused = Memo::new(move |_| focused_field.get() == FocusedField::Question);
    let q_clear = RwSignal::new(());
    let q_text = RwSignal::new(String::new());
    let t_focused = Memo::new(move |_| focused_field.get() == FocusedField::Tag);

    // children
    let ((a_renderer, a_handler), a_submit, a_clear) = TextArea(
        "Answer".into(),
        a_focused,
        None,
        Some(Arc::new(move |content| {
            a_text.update(|s| *s = content);
        })),
    );
    let ((q_renderer, q_handler), q_submit, q_clear) = TextArea(
        "Question".into(),
        q_focused,
        None,
        Some(Arc::new(move |content| {
            q_text.update(|s| *s = content);
        })),
    );

    let (t_renderer, t_handler) = TagArea(t_focused);
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match (
            key_event.code,
            key_event.modifiers,
            focused_field.get_untracked(),
        ) {
            (KeyCode::Esc, _, _) => active_view.set(ActiveView::Home),
            (KeyCode::Tab, _, _) => focus_next_field(),
            (KeyCode::BackTab, _, _) => focus_previous_field(),
            (KeyCode::Enter, KeyModifiers::CONTROL, _) => {}
            (_, _, FocusedField::Question) => return q_handler(key_event),
            (_, _, FocusedField::Answer) => return a_handler(key_event),
            (_, _, FocusedField::Tag) => return t_handler(key_event),
            // (_, _, _) => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let [left, center, right] = Layout::horizontal([
            Constraint::Percentage(75),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(rect);

        let divider = Block::default()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Plain);

        frame.render_widget(&divider, center);

        let [upper_left, lower_left] = Layout::vertical([Constraint::Percentage(40); 2])
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
