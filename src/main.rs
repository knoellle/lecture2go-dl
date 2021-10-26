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

    let mut downloaded_count = 0;
    println!("Found videos:");
    for item in &rss.channel.items {
        println!("{}\n\t{}", item.title, item.link);
        let filename = item.title.clone() + ".mp4";
        let filepath = std::path::Path::new(&filename);
        if filepath.exists() {
            println!(
                "File {} exists, skipping download",
                filepath.to_str().unwrap()
            );
        } else {
            println!("Downloading...");
            youtube_dl::YoutubeDl::new(item.link.as_str())
                .download_video(true)
                .extra_arg("--no-check-certificate")
                .extra_arg("--output".to_owned())
                .extra_arg(filepath.to_str().unwrap())
                .run()?;
            println!("Done, filesize: {} bytes", filepath.metadata()?.len());
            downloaded_count += 1;
        }
        println!();
    }

    println!("Done, processed {} videos ({} new)", rss.channel.items.len(), downloaded_count);

    Ok(())
}
