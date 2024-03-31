#![allow(non_snake_case)]
use super::super::{Component, ComponentEventHandler, ComponentRenderer};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState},
    Frame,
};
use reactive_graph::{
    computed::Memo,
    effect::Effect,
    signal::RwSignal,
    traits::{Get, GetUntracked, Update, UpdateUntracked},
};
use std::sync::Arc;

fn styled_list<'a>(items: Vec<String>, is_focused: bool, title: String) -> List<'a> {
    List::new::<Vec<String>>(items)
        .block(
            Block::default()
                .style(Style::default().fg(if is_focused {
                    Color::LightBlue
                } else {
                    Color::White
                }))
                .title(title)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

trait SelectionExtension {
    fn up(&mut self, max: usize);
    fn down(&mut self, max: usize);
}

impl SelectionExtension for ListState {
    fn up(&mut self, max: usize) {
        if max == 0 {
            self.select(None);
        }
        let new = self
            .selected()
            .map(|selected| selected.checked_sub(1).unwrap_or(max - 1));
        self.select(new);
    }

    fn down(&mut self, max: usize) {
        if max == 0 {
            self.select(None);
        }
        self.select(self.selected().map(|selected| (selected + 1) % max));
    }
}

pub fn List(title: String, is_focused: Memo<bool>, items: Memo<Vec<String>>) -> Component {
    let items_len = Memo::new(move |_| items.get().len());
    let state = RwSignal::new(ListState::default());
    let up = move || state.update(|state| state.up(items_len.get_untracked()));
    let down = move || state.update(|state| state.down(items_len.get_untracked()));
    let event = RwSignal::new(None);

    // select first element when items changes
    Effect::new_sync(move |_| {
        state.update(|state| {
            state.select(if items_len.get() == 0 {
                None
            } else {
                Some(items_len.get() - 1)
            })
        });
    });

    let handler: ComponentEventHandler = Arc::new(move |key_event| {
        event.update(|event| *event = Some(key_event));
        match key_event.code {
            KeyCode::Up => up(),
            KeyCode::Down => down(),
            _ => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        let mut new_state = state.get();
        let title = format!(
            "selected: {:?}, event: {:?}",
            new_state.selected(),
            event.get()
        );
        let widget = styled_list(items.get(), is_focused.get(), title);
        frame.render_stateful_widget(widget, rect, &mut new_state);
        state.update_untracked(|state| *state = new_state);
    });

    (renderer, handler)
}
