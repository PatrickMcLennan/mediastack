mod types;
use crate::types::{HttpGetWidescreenWallpapers, Image, Response};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Region};
use lambda_runtime::{Context, Error};
use aws_sdk_dynamodb::model::{AttributeValue};

async fn handler(_: HttpGetWidescreenWallpapers, __: Context) -> Result<Response, Error> {
	const TABLE_NAME: &str = "media-dynamo";
	let region_provider = RegionProviderChain::first_try("us-east-1")
		.or_default_provider()
		.or_else(Region::new("us-east-1"));
	let shared_config = aws_config::from_env().region(region_provider).load().await;
	let dynamo_client = Client::new(&shared_config);

	let mut failed_query: bool = false;
	let mut no_images: bool = false;

	let mut images: Vec<Image> = vec![];
	match dynamo_client
		.query()
		.table_name(TABLE_NAME)
		.index_name("media-dynamo-index")
		.key_condition_expression("#media_type = :widescreen_wallpaper")
		.projection_expression("#name, #url, #thumbnail_url")
		.expression_attribute_names("#media_type", "media_type")
		.expression_attribute_names("#name", "name")
		.expression_attribute_names("#url", "url")
		.expression_attribute_names("#thumbnail_url", "thumbnail_url")
		.expression_attribute_values(":widescreen_wallpaper", AttributeValue::S(String::from("widescreen_wallpaper")))
		.send()	
		.await {
			Ok(r) => {
				let count = r.count;
				if count == 0 {
					no_images = true
				} else {
					let values = match r.items {
						Some(v) => v,
						None => vec![]
					};
					if values.len() == 0 {
						no_images = true
					} else {
						for value in values {
							let (name_option, url_option, thumbnail_url_option) = (value.get("name"), value.get("url"), value.get("thumbnail_url"));							
							let name = match name_option {
								Some(n) => match n.as_s() {
									Ok(s) => String::from(s),
									Err(e) => {
										println!("Error unwrapping specific query name: {:?}", e);
										continue
									}
								},
								None => continue
							};

							let url = match url_option {
								Some(u) => match u.as_s() {
									Ok(s) => String::from(s),
									Err(e) => {
										println!("Error unwrapping specific query url: {:?}", e);
										continue
									}
								},
								None => continue
							};

							let thumbnail_url = match thumbnail_url_option {
								Some(u) => match u.as_s() {
									Ok(s) => String::from(s),
									Err(e) => {
										println!("Error getthing thumbnail_url: {:?}", e);
										continue
									}
								},
								None => continue
							};
							images.push(Image { name, url, thumbnail_url })
						}
					}
				}
			},
			Err(e) => {
				println!("Error querying the table for media_type = widescreen_wallpaper: {}", e);
				failed_query = true
			}
		};

	let total_images = images.len();
	if failed_query == true {
		return Ok(Response::internal_error())
	} else if no_images == true || total_images == 0 {
		return Ok(Response::not_found())
	};

	Ok(Response::success(images))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
