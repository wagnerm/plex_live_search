extern crate reqwest;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

pub struct Plex {
    plex_token: String,
    plex_hostname: String,
    plex_port: String,
    guide_data_cache: String,
    enable_guide_data_cache: bool,
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

    fn guide_cache_exists(&self) -> bool {
        Path::new(&self.guide_data_cache).exists()
    }

    fn read_guide_cache(&self) -> Result<String, Box<dyn Error>> {
        let path = Path::new(&self.guide_data_cache);
        let mut contents = String::new();

        if self.guide_cache_exists() {
            let mut file = File::open(&path)?;
            file.read_to_string(&mut contents)?;
        }

        Ok(contents)
    }

    fn write_guide_data_cache(&self, content: Cursor<String>) -> std::io::Result<()> {
        let path = Path::new(&self.guide_data_cache);

        let mut file = File::create(&path)?;

        file.write_all(content.get_ref().as_bytes())?;
        file.sync_all()?;

        Ok(())
    }

    pub fn retrieve_guide_data(&self) -> Result<Cursor<String>, Box<dyn Error>> {
        if self.enable_guide_data_cache {
            let content = self.read_guide_cache().unwrap();
            Ok(Cursor::new(content))
        } else {
            let content = self.get_guide_data()?;
            self.write_guide_data_cache(content.clone()).unwrap();
            Ok(content)
        }
    }

    fn get_guide_data(&self) -> Result<Cursor<String>, Box<dyn Error>> {
        // TODO more sections
        // section 3 == sports
        // section 2 == shows
        let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";

        // Describes the metadata we will get back from Plex.
        // 4 for video metadata, media metadata, and genre.
        let media_type = "type=4";

        println!("Requesting...");
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
