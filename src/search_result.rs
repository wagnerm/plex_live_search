extern crate chrono;
use chrono::prelude::*;

use std::fmt;

pub struct SearchResult {
  pub media_type: String,
  pub title: String,
  pub parent_title: String,
  pub grand_parent_title: String,
  pub summary: String,
  pub duration: i32,
  pub channel_id: String,
  pub channel_human_id: String,
  pub channel_callsign: String,
  pub channel_title: String,
  pub begins_at: String,
  pub ends_at: String,
  pub genre: Vec<String>,
}

impl SearchResult {
  pub fn new () -> SearchResult {
    SearchResult{
      media_type: String::from(""),
      title: String::from(""),
      parent_title: String::from(""),
      grand_parent_title: String::from(""),
      summary: String::from(""),
      duration: 0,
      channel_id: String::from(""),
      channel_human_id: String::from(""),
      channel_callsign: String::from(""),
      channel_title: String::from(""),
      begins_at: String::from(""),
      ends_at: String::from(""),
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
      Summary: {}
      Channel: {}
      Begins At: {}
      Ends At: {}",
      self.title,
      self.parent_title,
      self.grand_parent_title,
      self.summary,
      self.channel_title,
      Local.timestamp(self.begins_at.parse().unwrap(), 0),
      Local.timestamp(self.ends_at.parse().unwrap(), 0),
    )
  }
}
