use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use reqwest::blocking::get;

fn generate_rss(data: &[serde_json::Value]) -> String {
    let mut rss = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<rss version=\"2.0\">\n\n<channel>\n<title>ComicK - English RSS Feed</title>\n<link>https://github.com/ld3z/manga-rss</link>\n<description>A simple RSS feed for ComicK!</description>\n");

    for i in data {
        let c = i["md_comics"].as_object().unwrap();
        let title = format!("{} - Chapter {}", c["title"].as_str().unwrap().replace("&", "and"), i["chap"].as_str().unwrap());
        let link = format!("https://comick.io/comic/{}", c["slug"].as_str().unwrap());
        let description = format!("Chapter {} of {} is now available on ComicK!", i["chap"].as_str().unwrap(), c["title"].as_str().unwrap().replace("&", "and"));

        rss.push_str(&format!("\n<item>\n    <title>{}</title>\n    <link>{}</link>\n    <description>{}</description>\n</item>\n", title, link, description));
    }

    rss.push_str("\n</channel>\n</rss>");
    rss
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url_nsfw = "https://api.comick.fun/chapter/?lang=en&page=1&order=new&accept_mature_content=true";
    let url_sfw = "https://api.comick.fun/chapter/?lang=en&page=1&order=new&accept_mature_content=false";

    let data_nsfw = get(url_nsfw)?.json::<Vec<serde_json::Value>>()?;
    let data_sfw = get(url_sfw)?.json::<Vec<serde_json::Value>>()?;

    let filename_nsfw = "./comick/comick-rss-en-nsfw.xml";
    let filename_sfw = "./comick/comick-rss-en-sfw.xml";

    if let Ok(dir) = std::fs::create_dir_all(Path::new(filename_nsfw).parent().unwrap()) {
        if let Err(_) = dir {
            println!("Failed to create directory!");
            return Ok(());
        }
    }

    if let Ok(dir) = std::fs::create_dir_all(Path::new(filename_sfw).parent().unwrap()) {
        if let Err(_) = dir {
            println!("Failed to create directory!");
            return Ok(());
        }
    }

    let rss_nsfw = generate_rss(&data_nsfw);
    let rss_sfw = generate_rss(&data_sfw);

    let mut file_nsfw = File::create(filename_nsfw)?;
    file_nsfw.write_all(rss_nsfw.as_bytes())?;

    let mut file_sfw = File::create(filename_sfw)?;
    file_sfw.write_all(rss_sfw.as_bytes())?;

    Ok(())
}
