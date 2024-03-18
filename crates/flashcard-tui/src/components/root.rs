#![allow(non_snake_case)]
use std::sync::{Arc, RwLock};

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use reactive_graph::{
    effect::Effect,
    signal::RwSignal,
    traits::{Get, Update},
};

use crate::tui::CrosstermTerminal;

use super::{help_bar::HelpBar, Component, ComponentEventHandler, ComponentRenderer, DynamicRect};

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

pub fn Root(terminal: Arc<RwLock<CrosstermTerminal>>, compute_rect: DynamicRect) -> Component {
    // ==== define state begin ====
    let counter = RwSignal::new(0);
    // ==== define state end ====

    // ==== define layout begin ====
    // the `compute_rect` function tells us what portion of the screen
    // our parent has
    let center_rect: DynamicRect = Arc::new({
        let compute_rect = compute_rect.clone();
        move |view_port: Rect| {
            let filtered_rect = compute_rect(view_port);
            let [upper, _] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .areas(filtered_rect);
            upper
        }
    });
    let help_rect: DynamicRect = Arc::new({
        let compute_rect = compute_rect.clone();
        move |view_port: Rect| {
            let filtered_rect = compute_rect(view_port);
            let [_, lower] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .areas(filtered_rect);
            lower
        }
    });

    // ==== define layout end ====

    // ==== init child components begin ====
    let (help_bar_renderer, _help_bar_event_handler) = HelpBar(terminal.clone(), help_rect.clone());
    // ==== init child components end ====

    // ==== Event handler begin ====
    let handler: ComponentEventHandler =
        Arc::new(
            move |key_event: crossterm::event::KeyEvent| match key_event.code {
                KeyCode::Up => counter.update(|c| *c += 1),
                KeyCode::Down => counter.update(|c| *c -= 1),
                _ => {}
            },
        );
    // ==== Event handler begin ====

    // ==== Renderer begin ====
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, view_port: Rect| {
        let center_rect = center_rect(view_port);
        let help_rect = help_rect(view_port);

        render_root(frame, center_rect, counter);
        help_bar_renderer(frame, help_rect);
    });
    // ==== Renderer end ====

    // Registering rendering side effect
    Effect::new_sync({
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        move |_| {
            _ = terminal.write().unwrap().draw(|frame| {
                let view_port = frame.size();
                renderer(frame, view_port);
            });
        }
    });

    (renderer, handler)
}

fn render_root(frame: &mut Frame, rect: Rect, counter: RwSignal<i32>) {
    let ver: [Rect; 5] = Layout::vertical([Constraint::Ratio(1, 5); 5]).areas(rect);

    let title = Paragraph::new(
        TITLE
            .iter()
            .map(|line| Line::from(*line))
            .collect::<Vec<Line>>(),
    )
    .blue()
    .centered();

    let description_stats = vec![
        DESCRIPTION.into(),
        format!("To review: {} cards", counter.get()).into(),
    ];

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
}
