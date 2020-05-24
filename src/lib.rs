extern crate reqwest;
extern crate xml;

use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

pub mod search_result;
pub mod parser;
pub mod plex;

use search_result::SearchResult;
use parser::parse;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  search(config);
  Ok(())
}

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("No enough arguments!");
        }
        let query = args[1].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config{ query, case_sensitive })
    }
}

fn search(config: Config) -> Result<(), Box<dyn Error>> {
  let plex = plex::Plex::new();
  let data = plex.retrieve_guide_data().unwrap();
  parse(data, &config.query);
  Ok(())
}
