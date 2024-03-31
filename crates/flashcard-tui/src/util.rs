use std::{ops::Deref, sync::Arc, time::Duration};

use tokio::{select, sync::RwLock};
use tokio_util::sync::CancellationToken;

pub trait Boxed: Default {
    fn boxed() -> Box<Self> {
        Box::new(Self::default())
    }
}

pub struct DebouncedFunction<F: Fn() -> () + Sync + Send + 'static> {
    duration: Duration,
    previous_token: Arc<RwLock<Option<CancellationToken>>>,
    fun: Arc<F>,
}

impl<F: Fn() -> () + Sync + Send + 'static> DebouncedFunction<F> {
    pub fn new(duration: Duration, fun: F) -> Self {
        let previous_token = Arc::new(RwLock::new(None));
        Self {
            duration,
            previous_token,
            fun: Arc::new(fun),
        }
    }

    /// schedules a task that will call the wrapped function after self.duration
    /// consecutive calls to `call` will ancel the previously scheduled
    /// call task and schedule a new one.
    /// Returns whether a previous call task was canceled
    pub fn call(&self) {
        let previous_token = self.previous_token.clone();
        let fun = self.fun.clone();
        let duration = self.duration;
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
                    fun()
                }
            };
        });
    }
}
