#![allow(non_snake_case)]
use crate::components::ComponentRenderer;
use ratatui::{layout::Rect, Frame};
use reactive_graph::{
    signal::RwSignal,
    traits::{Get, UpdateUntracked},
};
use std::sync::Arc;
use throbber_widgets_tui::widgets::{Throbber as ThrobberWidget, ThrobberState};
use throbber_widgets_tui::BRAILLE_EIGHT;
use tracing::{info, instrument};

#[instrument]
pub fn Throbber() -> ComponentRenderer {
    info!("Building Throbber component");
    let state = RwSignal::new(ThrobberState::default());
    let widget = RwSignal::new(ThrobberWidget::default().throbber_set(BRAILLE_EIGHT));
    let renderer: ComponentRenderer = Arc::new(move |frame: &mut Frame, rect: Rect| {
        // each time we render this component, we also update it's state
        state.update_untracked(|state| {
            frame.render_stateful_widget(widget.get(), rect, state);
            state.calc_next();
        });
    });
    renderer
}
