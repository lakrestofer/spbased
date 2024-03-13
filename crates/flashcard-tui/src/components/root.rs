use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::preamble::*;

use crate::{
    app::App,
    state::{ActiveView, State},
};

use super::{
    add_card::AddCard, browser::Browser, edit_card::EditCard, help_bar::HelpBar,
    review_card::Review, Component, DynamicComponent,
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

pub struct Root {
    add_card: DynamicComponent,
    edit_card: DynamicComponent,
    browser: DynamicComponent,
    help_bar: DynamicComponent,
    review_card: DynamicComponent,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            add_card: AddCard::boxed(),
            edit_card: EditCard::boxed(),
            browser: Browser::boxed(),
            help_bar: HelpBar::boxed(),
            review_card: Review::boxed(),
        }
    }
}

impl Boxed for Root {}

impl Root {
    fn render_root(&self, _state: &State, frame: &mut Frame, rect: Rect) {
        // vertical division of space
        let ver: [Rect; 5] = Layout::vertical([Constraint::Ratio(1, 5); 5]).areas(rect);

        let title = Paragraph::new(
            TITLE
                .iter()
                .map(|line| Line::from(*line))
                .collect::<Vec<Line>>(),
        )
        .blue()
        .centered();

        let description_stats = vec![DESCRIPTION.into(), "To review: 12 cards".into()];

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
}

impl Component for Root {
    fn render(&self, state: &State, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(rect);

        // the "Root" container
        // screen 1 or screen2
        match state.active_view {
            ActiveView::Root => self.render_root(state, frame, layout[1]),
            ActiveView::AddCard => self.add_card.render(state, frame, layout[1]),
            ActiveView::Browser => self.browser.render(state, frame, layout[1]),
            ActiveView::EditCard => self.edit_card.render(state, frame, layout[1]),
            ActiveView::Review => self.review_card.render(state, frame, layout[1]),
        }

        // help bar
        self.help_bar.render(state, frame, layout[2]);
    }

    fn handle_key_events(
        &self,
        app: &mut App,
        key_event: crossterm::event::KeyEvent,
    ) -> crate::preamble::AppResult<()> {
        match (&app.state.active_view, key_event.code) {
            // independend of state, if we pres C-c we quit
            (_, KeyCode::Char('c') | KeyCode::Char('C')) => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                };
            }
            // if we are in the root state, pressing ESC exits
            (ActiveView::Root, code) => match code {
                // Exit application on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                KeyCode::Char('a') => app.navigate_to(ActiveView::AddCard),
                KeyCode::Char('b') => app.navigate_to(ActiveView::Browser),
                KeyCode::Char('r') => app.navigate_to(ActiveView::Review),
                // otherwise do nothing
                _ => {}
            },
            (ActiveView::AddCard, _) => self.add_card.handle_key_events(app, key_event)?,
            (ActiveView::EditCard, _) => self.edit_card.handle_key_events(app, key_event)?,
            (ActiveView::Browser, _) => self.browser.handle_key_events(app, key_event)?,
            (ActiveView::Review, _) => self.browser.handle_key_events(app, key_event)?,
        }
        Ok(())
    }
}
