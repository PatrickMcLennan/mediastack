use serde::{Deserialize, Serialize};
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
	pub headers: Headers
}

impl Response {
    pub fn success(key: String) -> Response {
        Response {
            statusCode: 200,
            body: key,
			headers: Headers::default()
        }
    }
    pub fn invalid() -> Response {
        Response {
            statusCode: 400,
            body: String::from("Invalid parameters"),
			headers: Headers::default()
        }
    }
}
