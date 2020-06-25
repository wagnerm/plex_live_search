use std::io::Cursor;
use std::error::Error;

pub mod config;
pub mod parser;
pub mod plex;
pub mod search_result;

use config::{Config, ContentCategory};
use parser::parse;
use search_result::SearchResult;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    search(config)?;
    Ok(())
}

fn content_category_ids(category: ContentCategory) -> Vec<i32> {
    match category {
        ContentCategory::all => vec![2, 3, 4],
        ContentCategory::shows => vec![2],
        ContentCategory::sports => vec![3],
        ContentCategory::news => vec![4],
    }
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

    let categories = content_category_ids(config.category);
    let mut search_results = Vec::new();

    for c in categories {
        let result = search_guide_data_for_category(&plex, c, &config.query, &config.ignore_case)?;
        search_results.push(result);
    }

    Ok(())
}

fn search_guide_data_for_category(
    plex: &plex::Plex<plex::PlexRequester>,
    category: i32,
    query: &String,
    ignore_case: &bool,
) -> Result<Vec<SearchResult>, Box<dyn Error>>{
    let data_str = plex.get_guide_data(category)?;
    let data = Cursor::new(data_str);
    parse(data, query.clone(), *ignore_case)
}
