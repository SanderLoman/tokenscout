use clap::{Arg, ArgAction, Command};
use logging::{init_logger, parse_verbosity};
use slog::{crit, debug, error, info, trace, warn};
use tokenscout_version::VERSION;

#[tokio::main]
async fn main() {
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
        .arg(
            Arg::new("chains")
                .long("chain")
                .short('c')
                .help("Tracks desired chains")
                .action(ArgAction::Set)
                .use_value_delimiter(true)
                .global(true)
                .display_order(0),
        )
        .arg(
            Arg::new("custom-url")
                .long("custom-url")
                .short('u')
                .help("Specifies a custom node URL for specific chains")
                .action(ArgAction::Set)
                .use_value_delimiter(true)
                .value_names(&["CHAIN", "URL"])
                .display_order(0),
        )
        .arg(
            Arg::new("telegram-channels")
                .long("telegram-channels")
                .short('t')
                .help("Outputs to specified Telegram channels")
                .action(ArgAction::Set)
                .use_value_delimiter(true)
                .global(true)
                .display_order(0),
        )
        .get_matches();

    let verbosity = matches.get_count("verbosity");
    let levels = parse_verbosity(verbosity);

    let logger = init_logger(levels);

    info!(logger, "Starting tokenscout"; "version" => VERSION);
    warn!(logger, "This is a warning message");
    error!(logger, "This is an error message");
    crit!(logger, "This is a critical message");
    debug!(logger, "This is a debug message");
    trace!(logger, "This is a trace message");
}
