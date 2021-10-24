extern crate quick_xml;
extern crate serde;

use quick_xml::de::from_str;
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
    let url = std::fs::read_to_string("feed.url")
        .expect("Could not read \"feed.url\". Make sure it exists");
    let xml = get_feed(url.as_str())?;
    let rss: RSS = from_str(xml.as_str())?;
    println!("Found videos:");
    for item in &rss.channel.items {
        println!("{}\n\t{}", item.title, item.link);
        let filename = item.title.clone() + ".mp4";
        if std::path::Path::new(filename.as_str()).exists() {
            println!("File {} exists, skipping download", filename);
        } else {
            println!("\tDownloading...");
            let output = youtube_dl::YoutubeDl::new(item.link.as_str())
                .extra_arg("--no-check-certificate")
                .download_video(true)
                .extra_arg("-o".to_owned() + filename.as_str())
                .run()?;
            println!("{:?}", output);
        }
        println!();
    }

    Ok(())
}
