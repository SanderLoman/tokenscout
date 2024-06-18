use clap::{Arg, ArgAction, ArgMatches, Command};
use tokenscout_version::VERSION;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let matches = Command::new("tokenscout")
        .version(VERSION)
        .about("A tool for monitoring Ethereum tokens")
        .get_matches();
}
