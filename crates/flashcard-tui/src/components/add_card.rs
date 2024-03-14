use ratatui::{
    layout::{Constraint, Flex, Layout},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};
use tui_textarea::TextArea;

use super::Component;
use crate::preamble::*;

#[derive(Debug)]
pub struct AddCard<'a> {
    answer_area: TextArea<'a>,
    question_area: TextArea<'a>,
}

impl<'a> Default for AddCard<'a> {
    fn default() -> Self {
        let answer_area = TextArea::default();
        let question_area = TextArea::default();
        Self {
            answer_area,
            question_area,
        }
    }
}

impl<'a> Boxed for AddCard<'a> {}
impl<'a> Component for AddCard<'a> {
    fn render(
        &self,
        state: &crate::state::State,
        frame: &mut ratatui::prelude::Frame,
        rect: ratatui::prelude::Rect,
    ) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
                .areas(rect);

        let divider = Block::default()
            .padding(Padding::horizontal(4))
            .borders(Borders::RIGHT)
            .border_type(BorderType::Plain);

        frame.render_widget(&divider, left);

        let left = divider.inner(left);

        let [upper_left, lower_left] = Layout::vertical([Constraint::Percentage(40); 2])
            .flex(Flex::SpaceAround)
            .areas(left);

        // help text
        frame.render_widget(Paragraph::new("Border"), right);

        // question block
        let upper_left_block = Block::default()
            .title_top(Line::from("Question").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        frame.render_widget(&upper_left_block, upper_left);
        let upper_left = upper_left_block.inner(upper_left);
        frame.render_widget(self.question_area.widget(), upper_left);

        // answer block
        let lower_left_block = Block::default()
            .title_top(Line::from("Answer").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        frame.render_widget(&lower_left_block, lower_left);
        let lower_left = lower_left_block.inner(lower_left);
        frame.render_widget(self.answer_area.widget(), lower_left);
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
