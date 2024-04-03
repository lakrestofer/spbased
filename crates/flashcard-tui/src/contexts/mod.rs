pub mod events {
    use crossterm::event::KeyEvent;
    #[derive(Clone)]
    pub struct EventsContext(pub Option<KeyEvent>);
}
pub mod help;
pub mod stats;
