extern crate quick_xml;
extern crate serde;

use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    title: String,
    link: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Channel {
    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct RSS {
    channel: Channel,
}

fn get_feed(url: &str) -> anyhow::Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn main() -> anyhow::Result<()> {
    let xml = get_feed("")?;
    let rss: RSS = from_str(xml.as_str())?;
    println!("{:?}", rss.channel);
    Ok(())
}
