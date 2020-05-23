extern crate reqwest;

use std::env;
use std::error::Error;

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
            return Err("No enought arguments!");
        }
        let query = args[1].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let plex_token = env::var("PLEX_TOKEN").expect("You must set PLEX_TOKEN");
        let plex_hostname = env::var("PLEX_HOSTNAME").expect("You must set PLEX_HOSTNAME");
        let plex_port = env::var("PLEX_PORT").unwrap_or(String::from("32400"));

        return Ok(Config{ query, case_sensitive, plex_token, plex_hostname, plex_port })
    }
}

pub fn search(config: Config) -> Result<(), Box<dyn Error>> {
  let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";
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

  println!("{:?}", response.text());
  Ok(())
}
