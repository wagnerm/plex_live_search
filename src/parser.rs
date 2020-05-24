extern crate xml;

use std::env;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use super::search_result::SearchResult;

pub fn parse(content: Cursor<String>, query: &str) -> Result<Vec::<SearchResult>, Box<dyn Error>> {
  let mut search_results = Vec::<SearchResult>::new();
  let parser = EventReader::new(content);

  let mut in_video_block = false;
  let mut in_media_block = false;
  let mut is_match = false;

  let mut search_result = SearchResult::new();
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, attributes, .. }) => {
        if name.local_name == "Video" {
          for attr in &attributes {
            if attr.value.contains(query) {
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
            search_result = SearchResult::new();
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