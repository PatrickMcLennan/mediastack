use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize)]
pub struct Headers {
	#[serde(rename(serialize = "Access-Control-Allow-Origin"))]
	pub Access_Control_Allow_Origin: String,
}

impl Default for Headers {
	fn default() -> Self {
		Self {
			Access_Control_Allow_Origin: String::from("*")
		}
	}
}

#[derive(Deserialize)]
pub struct HttpGetWidescreenWallpapers {}

#[derive(Serialize)]
pub struct Image {
    pub name: String,
    pub url: String,
    pub thumbnail_url: String,
}

#[derive(Serialize)]
pub struct Body {
    pub images: Vec<Image>,
}


#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Response {
    pub statusCode: u64,
    pub body: String,
	pub headers: Headers
}

impl Response {
    pub fn success(images: Vec<Image>) -> Response {
        Response {
            statusCode: 200,
            body: serde_json::to_string(&images).unwrap(),
			headers: Headers::default()
        }
    }
    pub fn not_found() -> Response {
        Response {
            statusCode: 404,
            body: String::from("Not found"),
			headers: Headers::default()
        }
    }
    pub fn internal_error() -> Response {
        Response {
            statusCode: 500,
            body: String::from("Internal error"),
			headers: Headers::default()
        }
    }
}
