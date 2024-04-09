pub mod events {
    use crossterm::event::KeyEvent;
    #[derive(Clone, Default)]
    pub struct EventsContext(pub Option<KeyEvent>);
}
pub mod help;
pub mod stats;
pub mod tick {
    #[derive(Default, Clone, Copy)]
    pub struct TickCounterContext(pub u32);
}
