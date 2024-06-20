pub mod cli;
pub mod config;
pub mod importer;
pub mod networking;

use clap::{Arg, ArgAction, Command};
use slog::{crit, debug, error, info, trace, warn};

impl TokenscoutConfig {
    pub fn new() {}
}

#[derive(Debug, Clone)]
pub struct TokenscoutConfig {}
