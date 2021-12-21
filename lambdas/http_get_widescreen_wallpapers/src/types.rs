use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HttpGetWidescreenWallpapers {}

#[derive(Serialize)]
pub struct Image {
    pub name: String,
    pub url: String,
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
}

impl Response {
    pub fn success(images: Vec<Image>) -> Response {
        Response {
            statusCode: 200,
            body: serde_json::to_string(&images).unwrap(),
        }
    }
    pub fn not_found() -> Response {
        Response {
            statusCode: 404,
            body: String::new(),
        }
    }
    pub fn internal_error() -> Response {
        Response {
            statusCode: 500,
            body: String::new(),
        }
    }
}
