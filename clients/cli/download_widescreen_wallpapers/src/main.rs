mod lib;
use crate::lib::{Image};
use dotenv::dotenv;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
	dotenv().unwrap();
	
    let endpoint = match dotenv::var("API_GATEWAY_ENDPOINT") {
        Ok(e) => String::from(e),
        Err(e) => panic!("Error getting endpoint from .env: {}", e)
    };

    let api_key = match dotenv::var("API_GATEWAY_API_KEY") {
        Ok(k) => String::from(k),
        Err(e) => panic!("Error loading api key from .env: {}", e)
    };

	let images_dir = match dotenv::var("WIDESCREEN_WALLPAPERS_DIR") {
        Ok(k) => String::from(k),
        Err(e) => panic!("Error loading local image dir from .env: {}", e)
    };

	let reqwest_client = reqwest::Client::new();
	let mut dynamo_images: HashMap<String, Image> = HashMap::new();
	match reqwest_client
		.get(&endpoint)
		.header("x-api-key", &api_key)
		.send()
		.await {
			Ok(v) => match v.json::<Vec<Image>>().await {
				Ok(i) => {
					for image in i {
						dynamo_images.insert(String::from(&image.name), Image { name: image.name, url: image.url });
						continue
					}
				},
				Err(e) => panic!("Error creating Image structs from response: {}", e)
			},
			Err(e) => panic!("Error making initial GET request: {}", e)
		};

	let local_image_path = Path::new(&images_dir);
	match fs::read_dir(local_image_path) {
		Ok(f) => {
			for file in f {
				let entry = file.unwrap();
				let path = entry.path();
				let file = Path::new(&path);
				let name = file.display();
				if path.is_dir() {
					continue
				};
				let in_dynamo = match dynamo_images.get(&name.to_string()) {
					Some(_) => true,
					None => false
				};
				if in_dynamo {
					dynamo_images.remove(&name.to_string());
				} 
				continue
			}
		},
		Err(e) => {
			panic!("Error reading local image path: {}", e)
		}
	}

	// TODO:
	// Refactor to download all using threads (tokio::spawn)

	for image in dynamo_images {
		let path_string = format!("{}/{}", &images_dir, &image.0);
		let path = Path::new(&path_string);
		let mut file = match File::create(&path) {
			Ok(f) => f,
			Err(e) => panic!("Error attempting to create {}: {}", image.0, e),
		};

		match reqwest::get(&image.1.url).await {
			Ok(r) => match r.bytes().await {
				Ok(b) => match file.write_all(&b) {
					Ok(_) => continue,
					Err(e) => panic!("Error downloading {}: {}", image.1.url, e)
				},
				Err(e) => panic!("Error attempting to read bytes from {} response: {}", image.1.url, e)
			},
			Err(e) => panic!("Error attempting GET for {}: {}", image.1.url, e)
		};
	}
	Ok(())
}
