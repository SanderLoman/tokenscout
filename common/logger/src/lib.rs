use slog::{o, Drain, FnValue, Level, Logger, Record};
use slog_async;
use slog_term::{self, FullFormat, TermDecorator};

struct RangeLevelFilter<D> {
    levels: Vec<Level>,
    drain: D,
}

impl<D> Drain for RangeLevelFilter<D>
where
    D: Drain<Ok = ()>,
{
    type Ok = ();
    type Err = D::Err;

    fn log(&self, record: &Record, values: &slog::OwnedKVList) -> Result<Self::Ok, Self::Err> {
        if self.levels.contains(&record.level()) {
            self.drain.log(record, values)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

pub fn parse_verbosity(verbosity: u64) -> Vec<Level> {
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

pub fn create_logger(levels: Vec<Level>) -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator)
        .use_custom_timestamp(slog_term::timestamp_local)
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = RangeLevelFilter { levels, drain }.fuse();

    Logger::root(
        drain,
        o!(
            "location" => FnValue(move |info| format!("{}::{}::{}", info.file(), info.module(), info.line()))
        ),
    )
}
