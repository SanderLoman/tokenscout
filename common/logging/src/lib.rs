#![deny(unsafe_code)]

use slog::{o, Drain, Level, Logger, Record};

// A custom Drain filter that allows logging only for specific log levels.
struct RangeLevelFilter<D> {
    levels: Vec<Level>, // List of log levels to allow
    drain: D,           // The underlying Drain
}

// Implement the Drain trait for RangeLevelFilter
impl<D> Drain for RangeLevelFilter<D>
where
    D: Drain<Ok = ()>,
{
    type Ok = ();
    type Err = D::Err;

    // Log the record if its level is in the allowed list; otherwise, do nothing.
    fn log(&self, record: &Record, values: &slog::OwnedKVList) -> Result<Self::Ok, Self::Err> {
        if self.levels.contains(&record.level()) {
            self.drain.log(record, values)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

// Parse the verbosity level from command line arguments and return a vector of allowed log levels.
pub fn parse_verbosity(verbosity: u8) -> Vec<Level> {
    match verbosity {
        1 => vec![Level::Info],
        2 => vec![Level::Info, Level::Warning],
        3 => vec![Level::Info, Level::Warning, Level::Error],
        4 => vec![Level::Info, Level::Warning, Level::Error, Level::Critical],
        5 => vec![
            Level::Info,
            Level::Warning,
            Level::Error,
            Level::Critical,
            Level::Debug,
        ],
        6 => vec![
            Level::Info,
            Level::Warning,
            Level::Error,
            Level::Critical,
            Level::Debug,
            Level::Trace,
        ],
        _ => vec![Level::Info, Level::Warning, Level::Error, Level::Critical],
    }
}

// Create and return a Logger configured with the given log levels.
pub fn init_logger(levels: Vec<Level>) -> Logger {
    // Create a Drain for stdout
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // Wrap the Drain in a RangeLevelFilter
    let drain = RangeLevelFilter { levels, drain };

    // Create and return the root Logger
    Logger::root(drain.fuse(), o!())
}
