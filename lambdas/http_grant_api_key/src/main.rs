#[macro_use]
extern crate dotenv_codegen;
use serde_json;

mod types;
use lambda_runtime::{Context, Error};
use crate::types::{Event, Response, Body};

async fn handler(event: Event, __: Context) -> Result<Response, Error> {
	let body: Body = match event.body {
		Some(b) => {
			println!("event body: {:?}", b);
			match serde_json::from_str(&b) {
				Ok(v) => {
					println!("from_str result: {:?}", v);
					v.body
				},
				Err(_) => return Ok(Response::invalid())
			}
		},
		None => return Ok(Response::invalid())
	};

	let correct_email = dotenv!("ADMIN_EMAIL").to_string();
	let correct_password = dotenv!("ADMIN_PASSWORD").to_string();
	let key = dotenv!("API_GATEWAY_API_KEY").to_string();

	println!("body: {:?}", body);
	println!("correct_email: {}", correct_email);
	println!("correct_password: {}", correct_password);
	println!("key: {}", key);
	
	match body.email {
		Some(e) => {
			println!("body.email: {}", e);
			if e != correct_email {
				return Ok(Response::invalid());
			}
		},
		None => return Ok(Response::invalid())
	};
	
	match body.password {
		Some(p) => {
			println!("body.password: {}", p);
			if p != correct_password {
				return Ok(Response::invalid())
			}
		},
		None => return Ok(Response::invalid())
	};

	Ok(Response::success(key))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}
