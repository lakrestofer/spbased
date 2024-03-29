#![allow(non_snake_case)]
use std::sync::{Arc, RwLock};

use color_eyre::config::PanicHook;
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{
    effect::Effect,
    signal::RwSignal,
    traits::{Get, Set, Update},
};

use crate::{preamble::ApplicationEvent, tui::CrosstermTerminal};

use super::{
    help_bar::HelpBar, root::ActiveView, stub_component_event_handler, Component,
    ComponentEventHandler, ComponentRenderer, DynamicRect,
};

const TITLE: [&str; 10] = [
    " ██████  ██▓███   ▄▄▄▄    ▄▄▄        ██████ ▓█████ ▓█████▄  ",
    "▒██    ▒ ▓██░  ██▒▓█████▄ ▒████▄    ▒██    ▒ ▓█   ▀ ▒██▀ ██▌",
    "░ ▓██▄   ▓██░ ██▓▒▒██▒ ▄██▒██  ▀█▄  ░ ▓██▄   ▒███   ░██   █▌",
    "  ▒   ██▒▒██▄█▓▒ ▒▒██░█▀  ░██▄▄▄▄██   ▒   ██▒▒▓█  ▄ ░▓█▄   ▌",
    "▒██████▒▒▒██▒ ░  ░░▓█  ▀█▓ ▓█   ▓██▒▒██████▒▒░▒████▒░▒████▓ ",
    "▒ ▒▓▒ ▒ ░▒▓▒░ ░  ░░▒▓███▀▒ ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░░░ ▒░ ░ ▒▒▓  ▒ ",
    "░ ░▒  ░ ░░▒ ░     ▒░▒   ░   ▒   ▒▒ ░░ ░▒  ░ ░ ░ ░  ░ ░ ▒  ▒ ",
    "░  ░  ░  ░░        ░    ░   ░   ▒   ░  ░  ░     ░    ░ ░  ░ ",
    "      ░            ░            ░  ░      ░     ░  ░   ░    ",
    "                        ░                            ░      ",
];

const DESCRIPTION: &str = "Flashcard frontend for the spbased framework.";

pub fn Home(active_view: RwSignal<ActiveView>) -> Component {
    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        match key_event.code {
            KeyCode::Char('q') => return Some(ApplicationEvent::Shutdown),
            KeyCode::Char('a') => active_view.set(ActiveView::AddCard),
            KeyCode::Char('b') => active_view.set(ActiveView::Browser),
            KeyCode::Char('r') => active_view.set(ActiveView::Review),
            _ => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, central_area: Rect| {
        let ver: [Rect; 5] = Layout::vertical([Constraint::Ratio(1, 5); 5]).areas(central_area);

        let title = Paragraph::new(
            TITLE
                .iter()
                .map(|line| Line::from(*line))
                .collect::<Vec<Line>>(),
        )
        .blue()
        .centered();

        let description_stats = vec![DESCRIPTION.into(), "To review: 14 cards".into()];

        let nav_hint = vec![
            Line::from(vec![
                Span::styled("a", Style::new().blue()),
                Span::raw(": Add card"),
            ]),
            Line::from(vec![
                Span::styled("b", Style::new().blue()),
                Span::raw(": Browser"),
            ]),
            Line::from(vec![
                Span::styled("r", Style::new().blue()),
                Span::raw(": Review"),
            ]),
        ];

        frame.render_widget(title, ver[1]);
        frame.render_widget(Paragraph::new(description_stats).centered(), ver[3]);
        frame.render_widget(Paragraph::new(nav_hint).centered(), ver[4]);
    });

    (renderer, handler)
}
