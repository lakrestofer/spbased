use crate::{
    components::{root::Root, Component},
    preamble::AppResult,
    state::{ActiveView, State},
};

pub struct App {
    pub state: State,
    pub root: Box<dyn Component>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::default(),
            root: Box::new(Root::default()),
        }
    }
}

impl App {
    pub fn running(&self) -> bool {
        self.state.running
    }

    pub fn quit(&mut self) {
        self.state.running = false;
    }

    pub fn navigate_to(&mut self, view: ActiveView) {
        self.state.active_view = view;
    }
}
