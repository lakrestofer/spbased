use crate::preamble::*;

use std::{sync::Arc, time::Duration};

use tokio::{select, sync::RwLock};
use tokio_util::sync::CancellationToken;

pub trait Boxed: Default {
    fn boxed() -> Box<Self> {
        Box::<Self>::default()
    }
}

pub struct DebouncedFunction<T: Clone + Send + Sync + 'static> {
    duration: Duration,
    previous_token: Arc<RwLock<Option<CancellationToken>>>,
    fun: ExtendedFn<T>,
}

impl<T: Clone + Send + Sync + 'static> DebouncedFunction<T> {
    pub fn new(duration: Duration, fun: ExtendedFn<T>) -> Self {
        let previous_token = Arc::new(RwLock::new(None));
        Self {
            duration,
            previous_token,
            fun,
        }
    }

    /// schedules a task that will call the wrapped function after self.duration
    /// consecutive calls to `call` will ancel the previously scheduled
    /// call task and schedule a new one.
    /// Returns whether a previous call task was canceled
    pub fn call(&self, arg: T) {
        let previous_token = self.previous_token.clone();
        let fun = self.fun.clone();
        let duration = self.duration;
        let arg = arg.clone();
        tokio::spawn(async move {
            {
                if let Some(previous_token) = previous_token.read().await.as_ref() {
                    previous_token.cancel();
                }
            }
            let new_token = CancellationToken::new();
            *previous_token.write().await = Some(new_token.clone());
            select! {
                _ = new_token.cancelled() => {},
                _ = tokio::time::sleep(duration) => {
                    fun(arg)
                }
            };
        });
    }
}
