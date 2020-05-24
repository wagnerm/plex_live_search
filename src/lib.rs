extern crate reqwest;
extern crate xml;

use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  search(config);
  Ok(())
}

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
    plex_token: String,
    plex_hostname: String,
    plex_port: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("No enough arguments!");
        }
        let query = args[1].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let plex_token = env::var("PLEX_TOKEN").expect("You must set PLEX_TOKEN");
        let plex_hostname = env::var("PLEX_HOSTNAME").expect("You must set PLEX_HOSTNAME");
        let plex_port = env::var("PLEX_PORT").unwrap_or(String::from("32400"));

        return Ok(Config{ query, case_sensitive, plex_token, plex_hostname, plex_port })
    }
}

struct SearchResult {
  media_type: String,
  title: String,
  parent_title: String,
  grand_parent_title: String,
  summary: String,
  duration: i32,
  channel_id: i32,
  channel_human_id: i32,
  channel_callsign: String,
  channel_title: String,
  begins_at: i32,
  ends_at: i32,
  genre: Vec<String>,
}

impl Default for SearchResult {
  fn default () -> SearchResult {
    SearchResult{
      media_type: String::from(""),
      title: String::from(""),
      parent_title: String::from(""),
      grand_parent_title: String::from(""),
      summary: String::from(""),
      duration: 0,
      channel_id: 0,
      channel_human_id: 0,
      channel_callsign: String::from(""),
      channel_title: String::from(""),
      begins_at: 0,
      ends_at: 0,
      genre: Vec::new(),
    }
  }
}

impl fmt::Display for SearchResult {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(
      formatter,
      "Title: {}
      Parent Title: {}
      Grand Parent Title: {}
      Summary: {}",
      self.title,
      self.parent_title,
      self.grand_parent_title,
      self.summary
    )
  }
}

pub fn search(config: Config) -> Result<(), Box<dyn Error>> {
  let mut search_results = Vec::<SearchResult>::new();

  // TODO more sections
  // section 3 == sports
  // section 2 == shows
  let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";

  // Describes the metadata we will get back from Plex.
  // 4 for video metadata, media metadata, and genre.
  let media_type = "type=4";

  let request_url = format!(
      "https://{plex_hostname}:{plex_port}/{request_path}?{media_type}&X-Plex-Token={plex_token}",
      plex_hostname = config.plex_hostname,
      plex_port = config.plex_port,
      plex_token = config.plex_token,
      request_path = request_path,
      media_type = media_type,
  );

  let response = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build()
      .unwrap()
      .get(&request_url)
      .send()
      .unwrap();

  let content = io::Cursor::new(response.text().unwrap());
  let parser = EventReader::new(content);

  let mut in_video_block = false;
  let mut in_media_block = false;
  let mut is_match = false;

  let mut search_result = SearchResult::default();
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, attributes, .. }) => {
        if name.local_name == "Video" {
          for attr in &attributes {
            if attr.value.contains(&config.query) {
              in_video_block = true;
              is_match = true;
              extract_attrs(&attributes, &mut search_result);
            }
          }
        } else if name.local_name == "Media" && in_video_block {
          // TODO Add media metadata to the result
        } else if name.local_name == "Genre" && in_video_block {
          // TODO Add genre metadata to the result
        }
      },
      Ok(XmlEvent::EndElement { name }) => {
        if name.local_name == "Video" {
          in_video_block = false;
          if is_match {
            is_match = false;
            println!("{}", search_result);
            search_result = SearchResult::default();
          }
        } else if name.local_name == "Media" {
          in_media_block = false;
        }
      }
      Err(e) => {
        println!("Error: {}", e);
        break;
      },
      _ => {}
    }
  }
  Ok(())
}

fn extract_attrs(attributes: &Vec<OwnedAttribute>, search_result: &mut SearchResult) {
  let mut all_attrs = HashMap::new();
  for attr in attributes {
    all_attrs.insert(attr.name.local_name.clone(), attr.value.clone());
  }

  // TODO handle unwraps better
  search_result.media_type = all_attrs.get("type").unwrap().to_string();
  search_result.title = all_attrs.get("title").unwrap().to_string();
  search_result.parent_title = all_attrs.get("parentTitle").unwrap().to_string();
  search_result.grand_parent_title = all_attrs.get("grandparentTitle").unwrap().to_string();
  search_result.summary = all_attrs.get("summary").unwrap().to_string();
}
