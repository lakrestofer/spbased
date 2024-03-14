use ratatui::widgets::Paragraph;

use super::Component;
use crate::preamble::*;

#[derive(Default, Debug)]
pub struct Review;

impl Boxed for Review {}
impl Component for Review {
    fn render(
        &self,
        state: &crate::state::State,
        frame: &mut ratatui::prelude::Frame,
        rect: ratatui::prelude::Rect,
    ) {
        frame.render_widget(Paragraph::new(format!("{:?}", self)), rect);
    }

    fn handle_key_events(
        &mut self,
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
