use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;
use tokenscout_version::VERSION;

pub struct TokenscoutCli {
    pub verbosity: u8,
    pub chains: Vec<String>,
    pub telegram_channels: Vec<String>,
    pub custom_endpoint: Option<Vec<String>>,
    pub output_file: Option<PathBuf>,
    pub use_json: bool,
}

impl TokenscoutCli {
    pub fn new() -> Self {
        let matches = Command::new("tokenscout")
            .version(VERSION)
            .author("SanderLoman <sanderfeitsma13@gmail.com>")
            .next_line_help(true)
            .term_width(80)
            .about("tokenscout is a tool for finding profitable tokens.")
            .arg(
                Arg::new("verbosity")
                    .long("verbosity")
                    .short('v')
                    .help("Sets the level of verbosity")
                    .action(ArgAction::Count)
                    .global(true),
            )
            .arg(
                Arg::new("chains")
                    .long("chain")
                    .short('c')
                    .help("Tracks desired chains")
                    .action(ArgAction::Append)
                    .global(true),
            )
            .arg(
                Arg::new("custom-url")
                    .long("custom-url")
                    .short('u')
                    .help("Specifies a custom node URL for specific chains")
                    .action(ArgAction::Append)
                    .number_of_values(2)
                    .value_names(&["CHAIN", "URL"]),
            )
            .arg(
                Arg::new("telegram-channels")
                    .long("telegram-channels")
                    .short('t')
                    .help("Outputs to specified Telegram channels")
                    .action(ArgAction::Append)
                    .global(true),
            )
            .arg(
                Arg::new("output-file")
                    .long("output-file")
                    .short('o')
                    .help("Specifies the output log file")
                    .action(ArgAction::Set)
                    .value_name("FILE"),
            )
            .arg(
                Arg::new("json")
                    .long("json")
                    .help("Use JSON format for logging")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        TokenscoutCli {
            verbosity: matches.get_count("verbosity"),
            chains: matches
                .get_many("chains")
                .map(|v| v.cloned().collect())
                .unwrap_or_default(),
            telegram_channels: matches
                .get_many("telegram-channels")
                .map(|v| v.cloned().collect())
                .unwrap_or_default(),
            custom_endpoint: matches.get_many("custom-url").map(|v| v.cloned().collect()),
            output_file: matches
                .get_one::<String>("output-file")
                .map(|s| PathBuf::from(s)),
            use_json: matches.get_flag("json"),
        }
    }

    pub fn get_chains(&self) -> &[String] {
        &self.chains
    }

    pub fn get_endpoints(&self) -> Option<&[String]> {
        self.custom_endpoint.as_deref()
    }

    pub fn get_telegram_channels(&self) -> &[String] {
        &self.telegram_channels
    }

    pub fn get_verbosity(&self) -> u8 {
        // Default to INFO level (2) if no verbosity is specified
        if self.verbosity == 0 {
            2
        } else {
            self.verbosity
        }
    }
}
