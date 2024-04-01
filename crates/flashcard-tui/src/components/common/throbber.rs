#![allow(non_snake_case)]
use std::sync::Arc;

use ratatui::{layout::Rect, Frame};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, UpdateUntracked},
};
use throbber_widgets_tui::widgets::{Throbber as ThrobberWidget, ThrobberState};

use crate::components::{stub_component_event_handler, Component, ComponentRenderer};

use throbber_widgets_tui::BRAILLE_EIGHT;

pub fn Throbber() -> Component {
    let state = RwSignal::new(ThrobberState::default());
    let widget = RwSignal::new(ThrobberWidget::default().throbber_set(BRAILLE_EIGHT));
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        // each time we render this component, we also update it's state
        state.update_untracked(|state| {
            frame.render_stateful_widget(widget.get(), rect, state);
            state.calc_next();
        });
    });

    (renderer, stub_component_event_handler())
}
