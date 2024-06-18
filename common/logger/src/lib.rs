use slog::{o, Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};
use sloggers::types::Severity;

pub fn setup_logger(verbosity: u64) -> Logger {
    let severity = match verbosity {
        0 => Severity::Info,
        1 => Severity::Warning,
        2 => Severity::Error,
        3 => Severity::Critical,
        4 => Severity::Debug,
        5 => Severity::Trace,
        _ => Severity::Info,
    };

    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let logger = Logger::root(drain, o!("severity" => format!("{:?}", severity)));

    logger
}
