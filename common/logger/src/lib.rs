use slog::Level;

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
