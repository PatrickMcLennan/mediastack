use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{console};
use crate::routes::Route;
use crate::hooks::{use_has_auth};
use yew_router::components::Link;
use wasm_bindgen::{JsValue};

#[function_component(Nav)]
pub fn nav() -> Html {
	let has_auth = use_has_auth();
	let history = use_history().unwrap();

	let logout = move |_: MouseEvent| {
		let session_storage = web_sys::window().unwrap().session_storage().unwrap().unwrap();
		match session_storage.remove_item("media-stack") {
			Ok(_) => history.push(Route::Login),
			Err(e) => console::log_1(&JsValue::from(format!("{:?}", e)))
		}
	};

	html! {
		<header class="sticky-top bg-light py-2">
			<nav class="navbar px-2 container d-flex justify-content-space-between enter">
				{"MediaStack"}
				<ul class="d-flex align-items-center nav">
					{if !has_auth {
						html! {
							<li class="nav-item active">
								<span class="nav-link disabled">
									{"Login"}
								</span>
							</li>
						}
					} else {
						html! {
							<>
								<li class="nav-item mr-4">
									<Link<Route> to={Route::Home}>
										{"Home"}
									</Link<Route>>
								</li>
								<li class="nav-item">
									<button class="btn btn-secondary" onclick={logout}>
										{"Logout"}
									</button>
								</li>
							</>
						}
					}} 
				</ul>
			</nav>
		</header>
	}
}