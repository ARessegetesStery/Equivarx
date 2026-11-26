use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Core async I/O runtime owned by [`crate::prelude::App`].
///
/// Wraps Tokio behind an engine-native interface so callers never depend on
/// Tokio's runtime types or `#[tokio::main]`.
pub struct IoRuntime {
    inner: tokio::runtime::Runtime,
}

impl IoRuntime {
    /// Builds and starts the multi-threaded async I/O runtime.
    pub fn init() -> Self {
        let inner = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to initialize async I/O runtime");
        Self { inner }
    }

    /// Runs an async future to completion on the current thread inside this
    /// runtime's context.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.inner.block_on(future)
    }

    /// Spawns an async task onto the runtime's worker pool.
    pub fn spawn<F>(&self, future: F) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        TaskHandle {
            inner: self.inner.spawn(future),
        }
    }

    /// Offloads a synchronous blocking closure onto the runtime's blocking pool.
    pub fn spawn_blocking<F, R>(&self, func: F) -> TaskHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        TaskHandle {
            inner: self.inner.spawn_blocking(func),
        }
    }
}

/// Handle to a task spawned on [`IoRuntime`].
pub struct TaskHandle<T> {
    inner: tokio::task::JoinHandle<T>,
}

impl<T> TaskHandle<T> {
    /// Requests cancellation of the spawned task.
    pub fn abort(&self) {
        self.inner.abort();
    }
}

impl<T> Future for TaskHandle<T> {
    type Output = Result<T, TaskJoinError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map_err(|inner| TaskJoinError { inner })
    }
}

/// Error returned when a spawned task fails to run to completion (cancelled or panicked).
#[derive(Debug)]
pub struct TaskJoinError {
    inner: tokio::task::JoinError,
}

impl fmt::Display for TaskJoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl std::error::Error for TaskJoinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

#[cfg(test)]
mod tests {
    use super::IoRuntime;

    #[test]
    fn block_on_spawn_and_spawn_blocking_run_tasks() {
        let runtime = IoRuntime::init();
        let sum = runtime.block_on(async {
            let async_val = runtime.spawn(async { 20 }).await.unwrap();
            let blocking_val = runtime.spawn_blocking(|| 22).await.unwrap();
            async_val + blocking_val
        });
        assert_eq!(sum, 42);
    }
}
