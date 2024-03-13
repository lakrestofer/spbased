pub trait Boxed: Default {
    fn boxed() -> Box<Self> {
        Box::new(Self::default())
    }
}
