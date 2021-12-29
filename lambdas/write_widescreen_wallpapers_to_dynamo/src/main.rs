mod lib;

use crate::lib::{WidescreenWallpaperInvocation, WidescreenWallpaperPost, get_widescreen_wallpapers};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Region};
use aws_sdk_dynamodb::model::{AttributeValue, KeysAndAttributes, PutRequest, WriteRequest};
use lambda_runtime::{Context, Error};
use std::time::{SystemTime};
use std::collections::HashMap;

async fn handler(_: WidescreenWallpaperInvocation, __: Context) -> Result<(), Error> {
    const TABLE_NAME: &str = "media-dynamo";
    let region_provider = RegionProviderChain::first_try("us-east-1")
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let dynamo_client = Client::new(&shared_config);
	
    let fetched_posts: Vec<WidescreenWallpaperPost> = get_widescreen_wallpapers().await;
	let mut get_keys: Vec<HashMap<String, AttributeValue>> = vec![];
	for post in &fetched_posts {
		let mut keys: HashMap<String, AttributeValue> = HashMap::with_capacity(2);
		let name = &post.name;
		keys.insert(String::from("pk"), AttributeValue::S(name.to_string()));
		keys.insert(String::from("sk"), AttributeValue::S(format!("widescreen_wallpaper|{}", name)));
		get_keys.push(keys)
	}

	let get_params = KeysAndAttributes::builder()
		.set_keys(Some(get_keys))
		.projection_expression("pk")
		.build();
		
	let mut dynamo_hashmap: HashMap<String, bool> = HashMap::new();
	match dynamo_client
		.batch_get_item()
		.request_items(TABLE_NAME, get_params)
		.send()
		.await {
			Ok(s) => {
				let responses = s.responses.unwrap();
				match responses.get(TABLE_NAME) {
					Some(r) => {
						const MIN_RECORDS: usize = 1;
						let has_records = &r.len() >= &MIN_RECORDS;
						if has_records {
							for record in r {
								for pk in record.values() {
									let value = match pk.as_s() {
										Ok(v) => String::from(v),
										Err(e) => panic!("Error getting the results from the batch_get_item: {:?}", e)
									};
									dynamo_hashmap.insert(value, true);
								}
							}
						}
					},
					None => println!("No existing records were found today")
				};
			},
			Err(e) => {
				println!("Could not batch_get_item: {:?}", e);
				std::process::exit(1)
			}
		};
	
		let time_stamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
			Ok(v) => v.as_secs(),
			Err(_) => 0
		};
	
		let mut put_params = vec![];
		for post in fetched_posts {
			if dynamo_hashmap.contains_key(&post.name) {
				continue
			} else {
				let new_request = PutRequest::builder()
					.item("created_at", AttributeValue::N(time_stamp.to_string()))
					.item("media_type", AttributeValue::S("widescreen_wallpaper".to_string()))
					.item("name", AttributeValue::S(post.name.to_string()))
					.item("pk", AttributeValue::S(post.name.to_string()))
					.item("sk", AttributeValue::S(format!("widescreen_wallpaper|{}", post.name)))
					.item("thumbnail_url", AttributeValue::S(post.thumbnail_url.to_string()))
					.item("updated_at", AttributeValue::N(time_stamp.to_string()))
					.item("url", AttributeValue::S(post.url.to_string()))
					.build();
				put_params.push(WriteRequest::builder().put_request(new_request).build());
				continue
			}
		}
		let total_params = put_params.len();

		if total_params == 0 {
			println!("No new images found");
			Ok(())
		} else {
			match dynamo_client
				.batch_write_item()	
				.request_items(TABLE_NAME, put_params)
				.send()
				.await {
					Ok(_) => {
						println!("{} records added to Dynamo", total_params);
						Ok(())
					},
					Err(e) => {
						println!("Error in the batch_write_item: {:?}", e);
						std::process::exit(1)
					},
				}
		}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
