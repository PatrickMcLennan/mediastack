// use yew::{function_component, };
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{console, HtmlDocument};

pub fn use_has_auth() -> bool {
	let window = web_sys::window().unwrap().document().unwrap();
	let document = window.dyn_into::<HtmlDocument>().unwrap();
	let auth = match document.cookie() {
		Ok(v) => v,
		Err(_e) => {
			// console::log_1(&JsValue::from(format!("{}", e.to_string())));
			return false;
		}
	};
	console::log_1(&JsValue::from("hello?"));
	console::log_1(&JsValue::from(format!("{}", auth)));
	if auth.len() == 0 {
		return false;
	}
	// let html_document = document.dyn_into::<Document>().unwrap();
	// let cookie = html_document.cookie().unwrap();
	true
}