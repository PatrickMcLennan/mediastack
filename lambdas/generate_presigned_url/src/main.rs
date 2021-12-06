use lambda_runtime::{Context, Error};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::{Client, Region};
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
struct Image {
    image_name: Option<String>,
    bucket_name: Option<String>,
}

#[derive(Serialize)]
struct Response {
    status_code: u16,
    message: String,
    url: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Request {
    body: Image,
}

async fn handler(event: Request, __: Context) -> Result<Response, Error> {
    let image = event.body;
    let name = image.image_name.unwrap_or(String::from(""));
    let bucket = image.bucket_name.unwrap_or(String::from(""));

    if name.len() == 0 || bucket.len() == 0 {
        return Ok(Response {
            status_code: 400,
            message: String::from("The request is missing 1 or more required fields"),
            url: String::from(""),
        })
    };

    let region_provider = RegionProviderChain::first_try("us-east-1")
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let time_limit = Duration::from_secs(180);
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let url = client.put_object().bucket(&bucket).key(name).presigned(PresigningConfig::expires_in(time_limit)?).await?;

    Ok(Response {
        status_code: 201,
        message: String::from("Created"),
        url: url.uri().to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}