use std::fmt;

pub struct SearchResult {
  pub media_type: String,
  pub title: String,
  pub parent_title: String,
  pub grand_parent_title: String,
  pub summary: String,
  pub duration: i32,
  pub channel_id: i32,
  pub channel_human_id: i32,
  pub channel_callsign: String,
  pub channel_title: String,
  pub begins_at: i32,
  pub ends_at: i32,
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
