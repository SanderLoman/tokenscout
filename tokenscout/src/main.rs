pub mod importer;
pub mod networking;

use clap::{Arg, ArgAction, Command};
use slog::{crit, debug, error, info, trace, warn};
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
        )
        .get_matches();

    let verbosity = matches.get_count("verbosity");

    Ok(())
}
