use eyre::{Ok, Result};
use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_logfmt::builder;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, EnvFilter, Layer};

#[derive(Clone, Debug)]
pub struct LoggerConfig {
    pub verbosity: u8,
    pub output_file: Option<PathBuf>,
}

impl LoggerConfig {
    pub fn new(verbosity: u8, output_file: Option<PathBuf>, use_json: bool) -> Self {
        Self {
            verbosity,
            output_file,
        }
    }
}

pub fn init_logger(config: &LoggerConfig) -> Result<()> {
    Ok(())
}

// Define a macro for critical logging
#[macro_export]
macro_rules! crit {
    ($($arg:tt)+) => {
        tracing::error!(critical = true, $($arg)+)
    };
}

pub fn log_test_messages() {
    crit!("This is a critical message");
    error!("This is an error");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
