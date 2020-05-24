use std::error::Error;

pub mod config;
pub mod parser;
pub mod plex;
pub mod search_result;

use config::Config;
use parser::parse;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    search(config)?;
    Ok(())
}

fn search(config: Config) -> Result<(), Box<dyn Error>> {
    let plex = plex::Plex::new();
    let data = plex.retrieve_guide_data().unwrap();
    parse(data, &config)?;
    Ok(())
}
