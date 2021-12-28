mod types;

use crate::types::{SqsEvent, SqsImage};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Region};
use lambda_runtime::{Context, Error};

async fn handler(event: SqsEvent, __: Context) -> Result<(), Error> {
	let messages = match event.Records {
		Some(b) => b,
		None => std::process::exit(1)
	};

	let message: SqsImage = match &messages[0].body {
		Some(v) => match serde_json::from_str(v) {
			Ok(s) => s,
			Err(e) => {
				println!("Error parsing event string into SqsImage: {}", e);
				std::process::exit(1)
			}
		},
		None => {
			println!("No stringified event was provided");
			std::process::exit(1)
		}
	};

	let receipt_handle = &messages[0].receiptHandle;
	let name = match &message.name {
		Some(v) => v,
		None => std::process::exit(1)
	};
	let url = match &message.url {
		Some(v) => v,
		None => std::process::exit(1)
	};

	let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let s3_client = Client::new(&shared_config);
	let sqs_client = aws_sdk_sqs::Client::new(&shared_config);

	let queue = match sqs_client
		.get_queue_url()
		.queue_name("media-sqs-images")
		.send()
		.await {
			Ok(v) => match v.queue_url() {
				Some(v) => String::from(v),
				None => panic!("No queue_url for media-sqs-images")
			},
			Err(e) => panic!("Error getting media-sqs-images: {}", e)
		};

	let stream = reqwest::get(url)
        .await?
        .bytes()
		.await?;

	match s3_client
		.put_object()
		.bucket("media-s3-patrick")
		.body(ByteStream::from(bytes::Bytes::from(stream)))
		.key(name)
		.send()
		.await {
			Ok(_) => {
				match sqs_client
					.delete_message()
					.queue_url(queue)
					.receipt_handle(receipt_handle)
					.send()
					.await {
						Ok(_) => return Ok(()),
						Err(e) => panic!("Could not delete_message: {}", e)
					};
			},
			Err(e) => panic!("Could not write image to S3: {}", e)
		}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
