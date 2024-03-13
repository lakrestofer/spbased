use std::fmt::Display;

#[derive(Debug)]
pub enum ActiveView {
    Root,
    AddCard,
    EditCard,
    Browser,
    Review,
}

impl Display for ActiveView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct State {
    pub running: bool,
    pub active_view: ActiveView,
}

impl Default for State {
    fn default() -> Self {
        Self {
            active_view: ActiveView::Root,
            running: true,
        }
    }
}

impl State {
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
