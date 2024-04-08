#![allow(non_snake_case)]
use crate::contexts::{events::EventsContext, help::HelpContext, stats::FrameTimeContext};

use super::{common::throbber::Throbber, ComponentRenderer};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use reactive_graph::{owner::use_context, signal::RwSignal, traits::Get};
use std::sync::Arc;

pub fn BottomBar() -> ComponentRenderer {
    // == Context ===
    let help_text_context = use_context::<RwSignal<HelpContext>>().unwrap();
    let event_context = use_context::<RwSignal<EventsContext>>().unwrap();
    let stats_context = use_context::<RwSignal<FrameTimeContext>>().unwrap();
    // === Components ===
    let throbber_renderer = Throbber();
    // === Renderer ===
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, help_rect: Rect| {
        // first we retrieve data
        // key_event
        let EventsContext(key_event) = event_context.get();
        let mut event_str = String::new();
        if let Some(key_event) = key_event {
            event_str = format!(
                "Event {{ key: {:?}, modifier: {:?} }}",
                key_event.code, key_event.modifiers
            );
        }
        // help_text
        let help_text = help_text_context.get().into_help_string();
        // frame_time
        let FrameTimeContext(frame_time) = stats_context.get();
        let frame_time = format!("{} ms/render", frame_time.as_millis());

        // when we split the area into multiple bars
        let [bar_outer, help] = Layout::vertical([Constraint::Ratio(1, 2); 2]).areas(help_rect);
        let bar = bar_outer.inner(&Margin::new(2, 0));
        let [left, right] = Layout::horizontal([Constraint::Percentage(25); 2]).areas(bar);
        let left: [Rect; 4] = Layout::horizontal([Constraint::Fill(1); 4]).areas(left);
        let right: [Rect; 4] = Layout::horizontal([Constraint::Fill(1); 4]).areas(right);

        // color the bar black
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Indexed(233))),
            bar_outer,
        );
        // render the help text
        frame.render_widget(
            Paragraph::new(help_text)
                .style(Style::default().fg(Color::Yellow))
                .centered(),
            help,
        );
        // we render the key_event text
        frame.render_widget(Paragraph::new(event_str).centered(), bar);

        throbber_renderer(frame, left[0]);
        frame.render_widget(Paragraph::new(frame_time), left[1]);
    });

    renderer
}
