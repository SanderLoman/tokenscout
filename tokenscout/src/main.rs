pub mod importer;
pub mod networking;

use clap::ArgAction;
use clap::{Arg, Command};
use logger::{create_logger, parse_verbosity};
use networking::initialize_network_manager;
use networking::EthNode;
use slog::{crit, debug, error, info, trace, warn};
use std::sync::Arc;
use tokenscout_version::VERSION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let matches = Command::new("tokenscout")
        .version(VERSION)
        .author("SanderLoman <sanderfeitsma13@gmail.com>")
        .next_line_help(true)
        .term_width(80)
        .disable_help_flag(true)
        .about("tokenscout is a tool for finding profitable tokens.")
        .display_order(0)
        .arg(
            Arg::new("verbosity")
                .long("verbosity")
                .short('v')
                .help("Sets the level of verbosity")
                .action(ArgAction::Count)
                .global(true)
                .display_order(0),
        );

    let matches = matches.get_matches();

    let verbosity = matches.get_count("verbosity");
    let levels = parse_verbosity(verbosity.into());

    let logger = create_logger(levels);
    let logger = Arc::new(logger);

    info!(logger, "Starting tokenscout"; "version" => VERSION);
    warn!(logger, "This is a warning message");
    error!(logger, "This is an error message");
    crit!(logger, "This is a critical message");
    debug!(logger, "This is a debug message");
    trace!(logger, "This is a trace message");

    Ok(())
}
