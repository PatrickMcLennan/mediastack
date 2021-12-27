use yew::prelude::{function_component, html, Callback, use_state};
use web_sys::{console};
use wasm_bindgen::{JsValue};
use crate::components::{LoginForm, LoginFormDTO};

#[function_component(Login)]
pub fn login() -> Html {
	let loading = use_state(|| false);
	let loading_clone = loading.clone();

	
	let callback = move |form: LoginFormDTO| {
		loading_clone.set(true);
		console::log_1(&JsValue::from(form.email.to_string()));
		console::log_1(&JsValue::from(form.password.to_string()));
	};

    html! {
		<LoginForm onsubmit={Callback::from(callback)} loading={*loading} />
    }
}