use std::process;

use plex_live_search::config::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();

    if let Err(e) = plex_live_search::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
