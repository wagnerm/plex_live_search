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
    let plex_requester = plex::PlexRequester{};
    let plex = plex::Plex::new(
        &plex_requester,
        config.plex_token,
        config.plex_hostname,
        config.plex_port,
        config.plex_guide_data_cache,
        config.plex_enable_guide_data_cache,
    );
    let data = plex.get_guide_data().unwrap();
    parse(data, config.query, config.ignore_case)?;
    Ok(())
}
