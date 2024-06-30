use std::sync::Arc;

use clap::{Arg, ArgAction, Command};
use logging::crit;
use logging::{init_logger, LoggerConfig};
use tokenscout::cli::TokenscoutCli;
use tokenscout_version::VERSION;

use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {}
