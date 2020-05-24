extern crate reqwest;

use std::env;
use std::error::Error;
use std::io::Cursor;

pub struct Plex {
    plex_token: String,
    plex_hostname: String,
    plex_port: String,
}

impl Plex {
  pub fn new() -> Plex {
      let plex_token = env::var("PLEX_TOKEN").expect("You must set PLEX_TOKEN");
      let plex_hostname = env::var("PLEX_HOSTNAME").expect("You must set PLEX_HOSTNAME");
      let plex_port = env::var("PLEX_PORT").unwrap_or(String::from("32400"));

      Plex{ plex_token, plex_hostname, plex_port }
  }

  pub fn search_live_guide(&self) -> Result<Cursor<String>, Box<dyn Error>> {
    // TODO more sections
    // section 3 == sports
    // section 2 == shows
    let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";

    // Describes the metadata we will get back from Plex.
    // 4 for video metadata, media metadata, and genre.
    let media_type = "type=4";

    let request_url = format!(
        "https://{plex_hostname}:{plex_port}/{request_path}?{media_type}&X-Plex-Token={plex_token}",
        plex_hostname = self.plex_hostname,
        plex_port = self.plex_port,
        plex_token = self.plex_token,
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

    let content = Cursor::new(response.text().unwrap());

    Ok(content)
  }
}
