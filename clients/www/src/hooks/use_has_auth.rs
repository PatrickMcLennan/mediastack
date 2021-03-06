
pub fn use_has_auth() -> (bool, Option<String>) {
	let session_storage = web_sys::window().unwrap().session_storage().unwrap().unwrap();
	match session_storage.get_item("media-stack") {
		Ok(value) => {
			match value {
				Some(v) => (v.to_string() == dotenv!("API_GATEWAY_API_KEY").to_string(), Some(v.to_string())),
				None => (false, None)
			}
		},
		Err(_) => (false, None)
	}
}