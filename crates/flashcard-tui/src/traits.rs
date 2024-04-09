use std::sync::Arc;

// extension trait
pub trait ExtendedFnT<T>: Fn(T) -> () + Send + Sync {}
impl<F, T> ExtendedFnT<T> for F where F: Fn(T) -> () + Send + Sync {}

impl<T> std::fmt::Debug for dyn ExtendedFnT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fn({}) -> ()", std::any::type_name::<T>())
    }
}
// Any function taking a
pub type ExtendedFn<T> = Arc<dyn ExtendedFnT<T>>;
