#![allow(non_snake_case)]
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, GetUntracked, Set, Update},
};
use std::sync::Arc;

use super::{
    root::ActiveView, tag_area::TagArea, Component, ComponentEventHandler, ComponentRenderer,
};

use super::common::text_area::TextArea;

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
    // local state and derived setters
    let focused_field = RwSignal::new(FocusedField::Question);
    let focus_next_field = move || focused_field.update(FocusedField::next);
    let focus_previous_field = move || focused_field.update(FocusedField::previous);

    // children
    let (question_field_renderer, question_field_handler) = TextArea(
        "Question",
        Arc::new(move || focused_field.get() == FocusedField::Question),
    );

    let (answer_field_renderer, answer_field_handler) = TextArea(
        "Answer",
        Arc::new(move || focused_field.get() == FocusedField::Answer),
    );
    let (tag_renderer, tag_event_handler) =
        TagArea(Arc::new(move || focused_field.get() == FocusedField::Tag));

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match key_event.code {
            KeyCode::Esc => active_view.set(ActiveView::Home),
            KeyCode::Tab => focus_next_field(),
            KeyCode::BackTab => focus_previous_field(),
            KeyCode::Enter if key_event.modifiers.contains(KeyModifiers::CONTROL) => {}
            _ => match focused_field.get_untracked() {
                FocusedField::Question => return question_field_handler(key_event),
                FocusedField::Answer => return answer_field_handler(key_event),
                FocusedField::Tag => return tag_event_handler(key_event),
            },
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

        let [upper_right, lower_right] = Layout::vertical([Constraint::Percentage(40); 2])
            .flex(Flex::SpaceAround)
            .areas(right);

        // question field
        question_field_renderer(frame, upper_left);

        // answer field
        answer_field_renderer(frame, lower_left);

        // stats
        frame.render_widget(Paragraph::new("Stats here"), upper_right);

        // tag
        tag_renderer(frame, lower_right);
    });

    (renderer, handler)
}
