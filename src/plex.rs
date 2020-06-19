extern crate reqwest;

use std::error::Error;
use std::io::Cursor;

pub struct Plex {
    plex_token: String,
    plex_hostname: String,
    plex_port: String,
    guide_data_cache: String,
    enable_guide_data_cache: bool,
}

trait Requester {
    fn get(&self, url: String) -> Result<reqwest::blocking::Response, reqwest::Error>;
}

impl Requester for Plex {
    fn get(&self, url: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let result = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(&url)
            .send();

        result
    }
}

impl Plex {
    pub fn new(plex_token: String, plex_hostname: String, plex_port: String, guide_data_cache: String, enable_guide_data_cache: bool) -> Plex {
        Plex {
            plex_token,
            plex_hostname,
            plex_port,
            guide_data_cache,
            enable_guide_data_cache,
        }
    }

    fn guide_request_url(&self) -> String {
        // TODO more sections
        // section 3 == sports
        // section 2 == shows
        let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";

        // Describes the metadata we will get back from Plex.
        // 4 for video metadata, media metadata, and genre.
        let media_type = "type=4";

        format!(
            "https://{plex_hostname}:{plex_port}/{request_path}?{media_type}&X-Plex-Token={plex_token}",
            plex_hostname = self.plex_hostname,
            plex_port = self.plex_port,
            plex_token = self.plex_token,
            request_path = request_path,
            media_type = media_type,
        )
    }

    fn get_guide_data(&self) -> Result<Cursor<String>, Box<dyn Error>> {
        let request_url = &self.guide_request_url();
        println!("Requesting...");
        let response = &self.get(request_url).unwrap();

        let content = Cursor::new(response.text().unwrap());

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plex_url() {
        let plex = Plex::new(
            String::from("1234"),
            String::from("plexbox"),
            String::from("5678"),
            String::from("/fake_path"),
            false,
        );
        assert_eq!(
            "https://plexbox:5678/tv.plex.providers.epg.cloud:2/sections/3/all?type=4&X-Plex-Token=1234",
            plex.guide_request_url()
        );
    }
}
