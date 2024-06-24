use slog::*;

use clap::{Arg, ArgAction, Command};
use logger::parse_verbosity;
use slog::{crit, debug, error, info, trace, warn};
use tokenscout_version::VERSION;

pub struct TokenscoutCli {
    pub verbosity: u8,
    pub chains: Vec<String>,
    pub telegram_channels: Vec<String>,
    pub custom_endpoint: Option<Vec<String>>,
    pub logger: Logger,
}

impl TokenscoutCli {
    pub fn new() -> Self {
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

        TokenscoutCli {
            chains: vec![],
            telegram_channels: vec![],
            custom_endpoint: None,
            verbosity: 4,
            logger: Logger::root(slog::Discard, o!()),
        }
    }

    fn get_verbosity(&self) -> Vec<Level> {
        parse_verbosity(self.verbosity)
    }

    fn get_chains(&self) -> Vec<String> {
        self.chains.clone()
    }

    fn get_endpoints(&self) -> Option<Vec<String>> {
        self.custom_endpoint.clone()
    }

    fn get_telegram_channels(&self) -> Vec<String> {
        self.telegram_channels.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // Make tests that check if the CLI is working as expected
}
