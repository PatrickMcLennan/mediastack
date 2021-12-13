mod types;

use crate::types::{SqsEvent, SqsImage};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Region};
use futures_util::StreamExt;
use lambda_runtime::{Context, Error};
use aws_sdk_s3::model::{CompletedMultipartUpload, CompletedPart};
use bytes::{BytesMut, BufMut};

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

	let upload = match s3_client
		.create_multipart_upload()
		.bucket("media-s3-patrick")
		.key(name)
		.send()
		.await {
			Ok(s) => s,
			Err(e) => {
				println!("Error creating the multipart upload: {}", e);
				std::process::exit(1)
			}
		};

	let upload_id = match upload.upload_id {
		Some(v) => String::from(v),
		None => {
			println!("Can't get an upload_id");
			std::process::exit(1)
		}
	};

	let queue = match sqs_client
		.get_queue_url()
		.queue_name("media-sqs")
		.send()
		.await {
			Ok(v) => match v.queue_url() {
				Some(v) => String::from(v),
				None => {
					println!("No queue_url for media-sqs");
					std::process::exit(1)	
				}
			},
			Err(e) => {
				println!("Error getting media-sqs: {}", e);
				std::process::exit(1)
			}
		};

	let mut stream = reqwest::get(url)
        .await?
        .bytes_stream();

	let mut part_number: i32 = 1;
	let mut parts: Vec<CompletedPart> = vec![];
	let mut buffer = BytesMut::with_capacity(5000000);

	// TODO

	// Return to this.  MultiPartUpload has a 5mb minimum, 
	// need to store the stream in a buffer and only trigger 
	// the upload_part() method when the buffer is larger than that.

	while let Some(item) = stream.next().await {
		match item {
			Ok(v) => {
				part_number += 1;
				match s3_client
					.upload_part()
					.bucket("media-s3-patrick")
					.key(name)
					.upload_id(&upload_id)
					.part_number(part_number)
					.body(ByteStream::from(v))
					.send()
					.await {
						Ok(v) => {
							let e_tag = match v.e_tag {
								Some(e) => e,
								None => {
									println!("No e_tag for {}", part_number);
									std::process::exit(1)		
								}
							};
							let new_part = CompletedPart::builder().e_tag(e_tag).part_number(part_number).build();
							parts.push(new_part)
						},
						Err(e) => {
							println!("Could not upload_part: {}", e);
							std::process::exit(1)
						}
					};
			},
			Err(e) => {
				println!("Error adding to the multipart upload: {}", e);
				match s3_client
					.abort_multipart_upload()
					.bucket("media-s3-patrick")
					.key(name)
					.upload_id(&upload_id)
					.send()
					.await {
						Ok(v) => v,
						Err(e) => {
							println!("Could not abort_multipart_upload: {}", e);
							std::process::exit(1)
						}
					};
				std::process::exit(1)
			}
		}
    }

	let completed_upload = CompletedMultipartUpload::builder().set_parts(Some(parts)).build();
	match s3_client
		.complete_multipart_upload()
		.bucket("media-s3-patrick")
		.key(name)
		.upload_id(&upload_id)
		.multipart_upload(completed_upload)
		.send()
		.await {
			Ok(v) => v,
			Err(e) => {
				println!("Could not complete_multipart_upload: {}", e);
				std::process::exit(1)
			}
		};
	
	match sqs_client
		.delete_message()
		.queue_url(queue)
		.receipt_handle(receipt_handle)
		.send()
		.await {
			Ok(v) => v,
			Err(e) => {
				println!("Could not delete_message: {}", e);
				std::process::exit(1)
			}
		};

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
