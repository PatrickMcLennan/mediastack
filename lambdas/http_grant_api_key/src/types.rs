use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Body {
	pub email: Option<String>,
	pub password: Option<String>
}

#[derive(Deserialize)]
pub struct Event {
	pub body: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Response {
    pub statusCode: u64,
    pub body: String,
}

impl Response {
    pub fn success(key: String) -> Response {
        Response {
            statusCode: 200,
            body: key,
        }
    }
    pub fn invalid() -> Response {
        Response {
            statusCode: 400,
            body: String::from("Invalid parameters"),
        }
    }
}
