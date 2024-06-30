use futures::channel::mpsc::Sender;
use futures::prelude::*;
use std::sync::Weak;
use tokio::runtime::{Handle, Runtime};
use tracing::{debug, info, trace, warn};

pub use tokio::task::JoinHandle;

mod metrics;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShutdownReason {
    Success(&'static str),
    Failure(&'static str),
}

impl ShutdownReason {
    pub fn message(&self) -> &'static str {
        match self {
            ShutdownReason::Success(msg) | ShutdownReason::Failure(msg) => msg,
        }
    }
}

#[derive(Clone)]
pub enum HandleProvider {
    Runtime(Weak<Runtime>),
    Handle(Handle),
}

impl From<Handle> for HandleProvider {
    fn from(handle: Handle) -> Self {
        HandleProvider::Handle(handle)
    }
}

impl From<Weak<Runtime>> for HandleProvider {
    fn from(weak_runtime: Weak<Runtime>) -> Self {
        HandleProvider::Runtime(weak_runtime)
    }
}

impl HandleProvider {
    pub fn handle(&self) -> Option<Handle> {
        match self {
            HandleProvider::Runtime(weak_runtime) => weak_runtime
                .upgrade()
                .map(|runtime| runtime.handle().clone()),
            HandleProvider::Handle(handle) => Some(handle.clone()),
        }
    }
}

#[derive(Clone)]
pub struct TaskExecutor {
    handle_provider: HandleProvider,
    exit: async_channel::Receiver<()>,
    signal_tx: Sender<ShutdownReason>,
}

impl TaskExecutor {
    pub fn new<T: Into<HandleProvider>>(
        handle: T,
        exit: async_channel::Receiver<()>,
        signal_tx: Sender<ShutdownReason>,
    ) -> Self {
        Self {
            handle_provider: handle.into(),
            exit,
            signal_tx,
        }
    }

    pub fn clone_with_name(&self, service_name: String) -> Self {
        TaskExecutor {
            handle_provider: self.handle_provider.clone(),
            exit: self.exit.clone(),
            signal_tx: self.signal_tx.clone(),
        }
    }

    pub fn spawn_ignoring_error(
        &self,
        task: impl Future<Output = Result<(), ()>> + Send + 'static,
        name: &'static str,
    ) {
        self.spawn(task.map(|_| ()), name)
    }

    pub fn spawn(&self, task: impl Future<Output = ()> + Send + 'static, name: &'static str) {
        if let Some(task_handle) = self.spawn_handle(task, name) {
            self.spawn_monitor(task_handle, name)
        }
    }

    fn spawn_monitor<R: Send>(
        &self,
        task_handle: impl Future<Output = Result<R, tokio::task::JoinError>> + Send + 'static,
        name: &'static str,
    ) {
        let mut shutdown_sender = self.shutdown_sender();
        if let Some(handle) = self.handle() {
            handle.spawn(async move {
                let timer = metrics::start_timer_vec(&metrics::TASKS_HISTOGRAM, &[name]);
                if let Err(join_error) = task_handle.await {
                    if let Ok(_panic) = join_error.try_into_panic() {
                        warn!("Task panicked: {}", name);
                        let _ = shutdown_sender
                            .try_send(ShutdownReason::Failure("Panic (fatal error)"));
                    }
                }
                drop(timer);
            });
        } else {
            debug!("Couldn't spawn monitor task. Runtime shutting down");
        }
    }

    pub fn spawn_handle<R: Send + 'static>(
        &self,
        task: impl Future<Output = R> + Send + 'static,
        name: &'static str,
    ) -> Option<tokio::task::JoinHandle<Option<R>>> {
        let exit = self.exit();

        if let Some(int_gauge) = metrics::get_int_gauge(&metrics::ASYNC_TASKS_COUNT, &[name]) {
            int_gauge.inc();
            if let Some(handle) = self.handle() {
                Some(handle.spawn(async move {
                    futures::pin_mut!(exit);
                    let result = match future::select(Box::pin(task), exit).await {
                        future::Either::Left((value, _)) => {
                            trace!("Async task completed: {}", name);
                            Some(value)
                        }
                        future::Either::Right(_) => {
                            debug!("Async task shutdown, exit received: {}", name);
                            None
                        }
                    };
                    int_gauge.dec();
                    result
                }))
            } else {
                debug!("Couldn't spawn task. Runtime shutting down");
                None
            }
        } else {
            None
        }
    }

    pub fn handle(&self) -> Option<Handle> {
        self.handle_provider.handle()
    }

    pub fn exit(&self) -> impl Future<Output = ()> {
        let exit = self.exit.clone();
        async move {
            let _ = exit.recv().await;
        }
    }

    pub fn shutdown_sender(&self) -> Sender<ShutdownReason> {
        self.signal_tx.clone()
    }
}
