extern crate xml;

use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::config::Config;
use super::search_result::SearchResult;

pub fn parse(
    content: Cursor<String>,
    config: &Config,
) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let search_results = Vec::<SearchResult>::new();
    let parser = EventReader::new(content);

    let mut in_video_block = false;
    let mut is_match = false;

    let mut search_result = SearchResult::new();

    let query = match config.ignore_case {
        true => config.query.to_lowercase(),
        false => config.query.clone(),
    };

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "Video" {
                    for attr in &attributes {
                        if config.ignore_case && attr.value.to_lowercase().contains(&query) {
                            is_match = true;
                        } else if attr.value.contains(&query) {
                            is_match = true;
                        }

                        if is_match {
                            in_video_block = true;
                            extract_attrs(&attributes, &mut search_result);
                        }
                    }
                } else if name.local_name == "Media" && in_video_block {
                    extract_media_attrs(&attributes, &mut search_result);
                // TODO Add media metadata to the result
                } else if name.local_name == "Genre" && in_video_block {
                    // TODO Add genre metadata to the result
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "Video" {
                    in_video_block = false;
                    if is_match {
                        is_match = false;
                        println!("{}", search_result);
                        search_result = SearchResult::new();
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(search_results)
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

fn extract_media_attrs(attributes: &Vec<OwnedAttribute>, search_result: &mut SearchResult) {
    let mut all_attrs = HashMap::new();
    for attr in attributes {
        all_attrs.insert(attr.name.local_name.clone(), attr.value.clone());
    }

    // TODO handle unwraps better
    search_result.channel_human_id = all_attrs.get("channelIdentifier").unwrap().to_string();
    search_result.channel_title = all_attrs.get("channelTitle").unwrap().to_string();
    search_result.begins_at = all_attrs.get("beginsAt").unwrap().to_string();
    search_result.ends_at = all_attrs.get("endsAt").unwrap().to_string();
}
