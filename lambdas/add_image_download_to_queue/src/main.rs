mod types;

use crate::types::{Request, QueueImage};
use lambda_runtime::{Context, Error};
use aws_sdk_sqs::{Client, Region};
use aws_config::meta::region::RegionProviderChain;

async fn handler(event: Request, __: Context) -> Result<(), Error> {
	let mut valid_images: Vec<QueueImage> = vec![];
    for record in event.Records {
		let dynamo_item = match record.dynamodb {
			Some(v) => v,
			None => continue
		};
		
		let image = match dynamo_item.NewImage {
			Some(v) => v,
			None => continue
		};
		
		let name = match image.name {
			Some(v) => {
				match v.S {
					Some(s) => s,
					None => continue
				}
			},
			None => continue
		};
		
		let url = match image.url {
			Some(v) => {
				match v.S {
					Some(s) => s,
					None => continue
				}
			},
			None => continue
		};

		valid_images.push(QueueImage { name: format!("widescreenwallpapers/{}", name), url })
	}

	let region_provider = RegionProviderChain::first_try("us-east-1")
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
	let client = Client::new(&shared_config);

	let queue = match client
		.get_queue_url()
		.queue_name("media-sqs-images")
		.send()
		.await {
			Ok(v) => match v.queue_url() {
				Some(v) => String::from(v),
				None => {
					println!("No queue_url for media-sqs-images");
					std::process::exit(1)	
				}
			},
			Err(e) => {
				println!("Error getting media-sqs-images: {}", e);
				std::process::exit(1)
			}
		};

	for image in valid_images {
		let string = match serde_json::to_string(&image) {
			Ok(v) => v,
			Err(e) => { 
				println!("Can't create string for this: {:?}", e);
				continue
			}
		};

		client
		  .send_message()
		  .queue_url(&queue)
		  .message_body(string)
		  .send()
		  .await?;
	}
	
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}