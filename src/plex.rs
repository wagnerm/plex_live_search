extern crate reqwest;

use std::error::Error;
use std::io::Cursor;

use super::config::ContentCategory;

pub struct Plex<'a, R: Requester> {
    requester: &'a R,
    plex_token: String,
    plex_hostname: String,
    plex_port: String,
    guide_data_cache: String,
    enable_guide_data_cache: bool,
    category: ContentCategory,
}

pub struct PlexRequester {}

pub trait Requester {
    fn get(&self, url: &String) -> Result<String, Box<dyn Error>>;
}

impl Requester for PlexRequester {
    fn get(&self, url: &String) -> Result<String, Box<dyn Error>> {
        let result = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .get(url)
            .send();

        match result {
            Ok(r) => match r.text() {
                Ok(t) => return Ok(t),
                Err(e) => return Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl<'a, R> Plex<'a, R>
where
    R: Requester,
{
    pub fn new(
        requester: &R,
        plex_token: String,
        plex_hostname: String,
        plex_port: String,
        guide_data_cache: String,
        enable_guide_data_cache: bool,
        category: ContentCategory,
    ) -> Plex<R> {
        Plex {
            requester,
            plex_token,
            plex_hostname,
            plex_port,
            guide_data_cache,
            enable_guide_data_cache,
            category,
        }
    }

    fn content_category_ids(&self) -> Vec<i32> {
        match &self.category {
            ContentCategory::all => vec![2, 3, 4],
            ContentCategory::shows => vec![2],
            ContentCategory::sports => vec![3],
            ContentCategory::news => vec![4],
        }
    }

    fn guide_request_url(&self) -> String {
        let ids = self.content_category_ids();

        // TODO more sections
        // section 3 == sports
        // section 2 == shows
        let request_path = "tv.plex.providers.epg.cloud:2/sections/4/all";

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

    pub fn get_guide_data(&self) -> Result<Cursor<String>, Box<dyn Error>> {
        println!("Requesting...");

        let request_url = self.guide_request_url();
        println!("{}", request_url);
        let text = &self.requester.get(&request_url)?;
        let content = Cursor::new(text.clone());

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct MockPlexRequester {
        response_text: String,
    }

    impl MockPlexRequester {
        fn new(response_text: String) -> MockPlexRequester {
            MockPlexRequester {
                response_text: response_text,
            }
        }
    }

    impl Requester for MockPlexRequester {
        fn get(&self, url: &String) -> Result<String, Box<dyn Error>> {
            Ok(self.response_text.clone())
        }
    }

    impl fmt::Display for PlexError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Some genereic request error")
        }
    }

    #[derive(Debug)]
    struct PlexError {
        message: String,
    }

    impl Error for PlexError {}

    trait DoStuff {
        fn get_message() -> String;
    }

    struct MockPlexRequesterWithError {
        error: String,
    }

    impl<'a> MockPlexRequesterWithError {
        fn new(error: String) -> MockPlexRequesterWithError {
            MockPlexRequesterWithError { error: error }
        }
    }

    impl Requester for MockPlexRequesterWithError {
        fn get(&self, url: &String) -> Result<String, Box<dyn Error>> {
            Err(Box::new(PlexError {
                message: self.error.clone(),
            }))
        }
    }

    #[test]
    fn test_plex_url() {
        let plex_requester = PlexRequester {};
        let plex = Plex::new(
            &plex_requester,
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

    #[test]
    fn test_get_returns_response() {
        let mock_plex_requester = MockPlexRequester::new(String::from("Hello World!"));

        let result = mock_plex_requester.get(&String::from("http://plexbox.fake.invalid"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_get_guide_data_creates_cursor() {
        let mock_plex_requester =
            MockPlexRequester::new(String::from("Hello World! This is a fake response!"));

        let plex = Plex::new(
            &mock_plex_requester,
            String::from("1234"),
            String::from("plexbox"),
            String::from("5678"),
            String::from("/fake_path"),
            false,
        );

        let result = plex.get_guide_data();

        assert!(result.is_ok());

        let content = result.unwrap();
        let text_ref = content.get_ref();
        assert_eq!(text_ref, "Hello World! This is a fake response!");
    }

    #[test]
    fn test_get_guide_data_error() {
        let mock_plex_requester_error =
            MockPlexRequesterWithError::new(String::from("Shit broke yo"));
        let plex = Plex::new(
            &mock_plex_requester_error,
            String::from("1234"),
            String::from("plexbox"),
            String::from("5678"),
            String::from("/fake_path"),
            false,
        );

        let result = plex.get_guide_data();
        assert!(result.is_err());
    }
}
