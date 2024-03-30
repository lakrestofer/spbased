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
    traits::{Get, GetUntracked, Update},
};
use std::sync::Arc;

use crate::components::stub_component_event_handler;

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

pub fn List(title: String, is_focused: Memo<bool>, items: Memo<Vec<String>>) -> Component {
    // all the existing tags
    let list_state = RwSignal::new(ListState::default());
    let list: RwSignal<List> = RwSignal::new(List::new::<Vec<String>>(vec![]));
    let selected: RwSignal<Option<usize>> = RwSignal::new(None);
    let up = move || {
        let len = items.get_untracked().len();
        if len == 0 || selected.get_untracked().is_none() {
            return;
        }
        selected.update(|selected| {
            *selected = selected.map(|selected| selected.saturating_add(1) % len)
        })
    };
    let down = move || {
        let len = items.get_untracked().len();
        if len == 0 || selected.get_untracked().is_none() {
            return;
        }
        selected.update(|selected| {
            *selected = selected.map(|selected| selected.saturating_sub(1) % len)
        })
    };
    Effect::new_sync(move |_| {
        list.update(|list| {
            *list = styled_list(items.get_untracked(), is_focused.get(), title.clone())
        })
    });
    Effect::new_sync(move |_| {
        let items = items.get();
        let len = items.len();
        list_state.update(|state| {
            if len == 0 {
                state.select(None);
            }
            if state.selected().is_none() {
                state.select(Some(0));
            }
        });
        list.update(|list| *list = list.clone().items(items));
    });
    Effect::new_sync(move |_| list_state.update(|state| state.select(selected.get())));

    let handler: ComponentEventHandler = Arc::new(move |key_event| {
        match key_event.code {
            KeyCode::Up => up(),
            KeyCode::Down => down(),
            _ => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        list_state.update(|state| frame.render_stateful_widget(list.get(), rect, state));
    });

    (renderer, handler)
}
