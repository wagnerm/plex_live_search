extern crate reqwest;
extern crate xml;

use std::io;
use reqwest::Error;
use xml::reader::{EventReader, XmlEvent};
use std::env;

fn main() {
    match get() {
        Ok(()) => println!("It works!"),
        Err(e) => panic!("Something broke {}", e)
    }
    // let mut buff = io::Cursor::new("My stuff is here");
    // buff.set_position(0);
    // let reader = io::BufReader::new(buff);
    // let mut line = String::new();
    // let len = reader.read(&mut line)?;
    // println!("{}", buff.get_ref());

}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn get() -> Result<(), Error> {
    let request_path = "tv.plex.providers.epg.cloud:2/sections/3/all";
    let media_type = "type=4";
    let token = env::var("PLEX_TOKEN").expect("You must set PLEX_TOKEN");

    let request_url = format!(
        "https://maxbox:32400/{request_path}?{media_type}&X-Plex-Token={token}",
        request_path = request_path,
        media_type = media_type,
        token = token
    );
    println!("{}", request_url);
    // let mut response = reqwest::blocking::
    let response = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(&request_url)
        .send()
        .unwrap();

    // println!("{:?}", response.text());
    let mut content = io::Cursor::new(response.text().unwrap());
    let parser = EventReader::new(content);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace, }) => {
                println!("{}+{}", indent(depth), name);
                if ! attributes.is_empty() {
                    for attr in attributes {
                        let temp_name_local_name = attr.name.local_name.clone();
                        let temp_value = attr.value.clone();
                        println!("{}{}={}", indent(depth), temp_name_local_name, temp_value);
                    }
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
