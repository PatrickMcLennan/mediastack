use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Event {}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub status_code: i32,
    pub images: Vec<Post>,
    pub message: String,
}

pub async fn get_posts() -> Vec<Post> {
    let resp = reqwest::get("https://old.reddit.com/r/widescreenwallpaper")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&resp);
    let post_selector = Selector::parse("div.thing").unwrap(); 
    let mut posts: Vec<Post> = vec![];
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

        let name_split: Vec<&str> = url.split(".").collect();
        let name = name_split[name_split.len() - 2].replace("it/", "");
        let ext_split: Vec<&str> = url.split("/").collect();
        let ext = ext_split[ext_split.len() - 1];

        posts.push(Post { name: format!("{}.{}", name, ext), url: String::from(url) })
    }

    posts
}