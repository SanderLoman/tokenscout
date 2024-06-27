pub mod cli;
pub mod config;
pub mod importer;
pub mod networking;

use clap::{Arg, ArgAction, Command};

impl TokenscoutConfig {
    pub fn new() {}
}

#[derive(Debug, Clone)]
pub struct TokenscoutConfig {}
