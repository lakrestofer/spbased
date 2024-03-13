use crate::preamble::*;
use ratatui::widgets::Paragraph;

use super::Component;

#[derive(Debug, Default)]
pub struct EditCard;

impl Boxed for EditCard {}
impl Component for EditCard {
    fn render(
        &self,
        _state: &crate::state::State,
        frame: &mut ratatui::prelude::Frame,
        rect: ratatui::prelude::Rect,
    ) {
        frame.render_widget(Paragraph::new(format!("{:?}", self)), rect);
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
