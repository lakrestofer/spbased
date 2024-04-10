pub mod events {
    use crossterm::event::KeyEvent;
    #[derive(Clone, Default)]
    pub struct EventsContext(pub Option<KeyEvent>);
}
pub mod help;
