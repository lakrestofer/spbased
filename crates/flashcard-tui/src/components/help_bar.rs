use ratatui::{layout::Rect, prelude::Frame, widgets::Paragraph};

use crate::preamble::*;
use crate::state::State;

use super::Component;

#[derive(Default)]
pub struct HelpBar;

impl Boxed for HelpBar {}
impl Component for HelpBar {
    fn render(&self, state: &State, frame: &mut Frame, rect: Rect) {
        let mut help_text = String::new();
        match state.active_view {
            crate::state::ActiveView::Root => {
                help_text.push_str("C-c / q / esc: exit program, ");
            }
            crate::state::ActiveView::AddCard => {
                help_text.push_str("esc: go back");
            }
            crate::state::ActiveView::EditCard => {
                help_text.push_str("esc: go back");
            }
            crate::state::ActiveView::Browser => {
                help_text.push_str("esc: go back");
            }
            crate::state::ActiveView::Review => {
                help_text.push_str("esc: go back");
            }
        };
        let paragraph = Paragraph::new(help_text).centered();
        frame.render_widget(paragraph, rect);
    }

    fn handle_key_events(
        &self,
        app: &mut crate::app::App,
        key_event: crossterm::event::KeyEvent,
    ) -> crate::preamble::AppResult<()> {
        match key_event.code {
            crossterm::event::KeyCode::Esc => app.navigate_to(crate::state::ActiveView::Root),
            _ => {}
        }
        Ok(())
    }
}
