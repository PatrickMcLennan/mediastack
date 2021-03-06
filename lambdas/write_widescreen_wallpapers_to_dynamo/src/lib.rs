use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WidescreenWallpaperInvocation {}

#[derive(Serialize, Deserialize)]
pub struct WidescreenWallpaperPost {
    pub name: String,
    pub url: String,
	pub thumbnail_url: String,
}

pub async fn get_widescreen_wallpapers() -> Vec<WidescreenWallpaperPost> {
    let resp = reqwest::get("https://old.reddit.com/r/widescreenwallpaper")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&resp);
    let post_selector = Selector::parse("div.thing").unwrap(); 
	let thumbnail_selector = Selector::parse("a.thumbnail[data-event-action='thumbnail'] > img").unwrap();
    let mut posts: Vec<WidescreenWallpaperPost> = vec![];

    for post in document.select(&post_selector) {
        match post.value().attr("data-nsfw") {
            Some(nsfw) => {
                if nsfw == "true" {
                    continue;
                }
            }
            None => continue,
        };

        match post.value().attr("data-promoted") {
            Some(promoted) => {
                if promoted == "true" {
                    continue;
                }
            }
            None => continue,
        };

        let url = match post.value().attr("data-url") {
            Some(url) => {
                if !url.contains("jpg") && !url.contains("jpeg") && !url.contains("png") {
                    continue;
                } else {
                    url
                }
            },
            None => continue,
        };

		let thumbnail_img = post.select(&thumbnail_selector);
		let mut thumbnail_url = String::new();
		for thumbnail in thumbnail_img {
			thumbnail_url = match thumbnail.value().attr("src") {
				Some(s) => format!("https://{}", s),
				None => continue
			}
		}

		if thumbnail_url.len() == 0 {
			continue
		}

        let name_split: Vec<&str> = url.split(".").collect();
        let name = name_split[name_split.len() - 2].replace("it/", "");
        let ext_split: Vec<&str> = url.split("/").collect();
        let ext = ext_split[ext_split.len() - 1];

        posts.push(WidescreenWallpaperPost { name: format!("{}.{}", name, ext), url: String::from(url), thumbnail_url })
    }
    posts
} 