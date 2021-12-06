extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};
use scraper::{Html, Selector};
use lambda_runtime::{Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Event {}

#[derive(Serialize, Deserialize)]
struct Post {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Output {
    status_code: i32,
    body: Vec<Post>,
}

async fn handler(_: Event, __: Context) -> Result<Output, Error> {
    let resp = reqwest::get("https://old.reddit.com/r/widescreenwallpaper")
        .await?
        .text()
        .await?;

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

    // Pretty-print a table of results to stdout 
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Name")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Url")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));

    for post in &posts {
        table.add_row(Row::new(vec![
            Cell::new(&post.name).with_style(Attr::Italic(true)),
            Cell::new(&post.url).with_style(Attr::Italic(true)),
        ]));
    }

    table.printstd();
    Ok(Output {
      status_code: 200,
      body: posts
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let handler = lambda_runtime::handler_fn(handler);
  lambda_runtime::run(handler).await?; 
  Ok(())
}
