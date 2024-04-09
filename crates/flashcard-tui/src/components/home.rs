#![allow(non_snake_case)]
use super::{root::ActiveView, Component, ComponentEventHandler, ComponentRenderer};
use crate::{contexts::help::HelpContext, preamble::ApplicationEvent};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{
    effect::Effect,
    owner::use_context,
    signal::RwSignal,
    traits::{Get, Set, Update},
};
use std::sync::Arc;
use tracing::{info, instrument};

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

#[instrument]
pub fn Home(active_view: RwSignal<ActiveView>) -> Component {
    info!("Building Home component");
    let help_text = use_context::<RwSignal<HelpContext>>().unwrap();

    Effect::new_sync(move |_| {
        if active_view.get() == ActiveView::Home {
            help_text.update(|help_text| {
                help_text.clear_below_level(1);
                help_text.update_desc_at_level("q / esc: exit program", 1)
            });
        }
    });

    let handler: ComponentEventHandler = Arc::new(move |key_event: crossterm::event::KeyEvent| {
        info!("running eventhandler for Home");
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => return Some(ApplicationEvent::Shutdown),
            KeyCode::Char('a') => active_view.set(ActiveView::AddCard),
            KeyCode::Char('b') => active_view.set(ActiveView::Browser),
            KeyCode::Char('r') => active_view.set(ActiveView::Review),
            _ => {}
        }
        None
    });

    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, central_area: Rect| {
        info!("rendering Home");
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
