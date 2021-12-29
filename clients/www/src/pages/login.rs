use yew::prelude::{function_component, html, Callback, use_state};
use yew_router::prelude::*;
use web_sys::{console};
use wasm_bindgen::{JsValue};
use crate::components::{LoginForm, LoginFormDTO};
use crate::routes::{Route};
use reqwasm::http::{Request};

#[function_component(Login)]
pub fn login() -> Html {
	let loading = use_state(|| false);
	let api_error = use_state(|| String::new());
	let history = use_history().unwrap();
	let loading_clone = loading.clone();

	let callback = move |form: LoginFormDTO| {
		loading_clone.set(true);

		let new_clone = loading_clone.clone();
		let history_clone = history.clone();

		wasm_bindgen_futures::spawn_local(async move {
			let url = format!("{}auth", dotenv!("API_GATEWAY_ENDPOINT").to_string());
			let body_string = serde_json::to_string(&form).unwrap();
			match Request::post(&url)
				.body(&JsValue::from(&body_string))
				.send()
				.await {
					Ok(v) => {
						let text = match v.text().await {
							Ok(k) => k,
							Err(e) => return console::log_1(&JsValue::from(format!("No body in the request: {}", e)))
						};
						let session_storage = web_sys::window().unwrap().session_storage().unwrap().unwrap();
						session_storage.set_item("media-stack", &text).unwrap();
						history_clone.push(Route::Home)
					},
					Err(e) => console::log_1(&JsValue::from(e.to_string()))
				};
			new_clone.set(false);
			()
		})
	};

    html! {
		<>
			<div aria_hidden={ if *&api_error.len() >= 1  { "false".to_string() } else { "true".to_string() } } class="alert alert-danger container" role="alert" style={format!("min-height: 50px; opacity: {}", if *&api_error.len() >= 1 { 1 } else { 0 })}>
				{format!("{}", *api_error)}
			</div>
			<LoginForm onsubmit={Callback::from(callback)} loading={*loading} />
		</>
    }
}