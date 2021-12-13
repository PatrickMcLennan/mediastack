mod lib;

use crate::lib::{Output, Event, Post, get_posts};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Region};
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{Context, Error};
use std::time::{SystemTime};

async fn handler(_: Event, __: Context) -> Result<Output, Error> {
    const TABLE_NAME: &str = "media-dynamo";
    let region_provider = RegionProviderChain::first_try("us-east-1")
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    let posts = get_posts().await;

    let mut new_posts: Vec<Post> = vec![];
    for pk in posts {
        match client
            .query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#pk = :pk and #sk = :sk")
            .expression_attribute_names("#sk", "sk")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(":sk", AttributeValue::S(format!("WidescreenWallpaper|{}", pk.name.to_string())))
            .expression_attribute_values(":pk", AttributeValue::S(pk.name.to_string()))
            .send()
            .await {
				Ok(s) => {
					if s.count >= 1 {
						continue
					} else {
						new_posts.push(pk)
					}
				},
				Err(_) => {
					println!("There's an error reading from DynamoDB right now: {:?}", pk.name);
				}
			}
    }

    let time_stamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(v) => v.as_secs(),
        Err(_) => 0
    };

    for post in &new_posts {
        match client
            .put_item()
            .table_name(TABLE_NAME)
            .item("created_at", AttributeValue::N(time_stamp.to_string()))
            .item("name", AttributeValue::S(post.name.clone()))
            .item("pk", AttributeValue::S(post.name.clone()))
            .item("sk", AttributeValue::S(format!("WidescreenWallpaper|{}", post.name)))
            .item("updated_at", AttributeValue::N(time_stamp.to_string()))
            .item("media_type", AttributeValue::S("image".to_string()))
            .item("url", AttributeValue::S(post.url.clone()))
            .send()
            .await {
				Ok(_) => continue,
				Err(e) => {
					println!("Unable to write this item: {:?}, {:?}", post.name, post.url);
					println!("{}", e);
					continue
				}
			};
    }

    Ok(Output {
        status_code: 200,
        images: new_posts,
        message: format!("New images found")
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
