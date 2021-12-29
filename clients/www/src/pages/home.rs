use yew::{function_component, html, Html, use_state, use_effect_with_deps, UseStateHandle};
use yew_router::components::{Link, Redirect};
use crate::hooks::{use_has_auth};
use crate::routes::Route;
use crate::types::WidescreenWallpaper;
use reqwasm::http::{Request};
use web_sys::{console};
use wasm_bindgen::{JsValue};

#[function_component(Home)]
pub fn home() -> Html {
	let (has_auth, key) =  use_has_auth();
	if !has_auth { return html! { <Redirect<Route> to={Route::Login} /> } }
	let loading = use_state(|| true);
	let images: UseStateHandle<Vec<WidescreenWallpaper>> = use_state(|| vec![]);
	
	let images_clone = images.clone();
	let loading_clone = loading.clone();

	use_effect_with_deps(move |_| {
		wasm_bindgen_futures::spawn_local(async move {
			let url = format!("{}widescreen_wallpapers", dotenv!("API_GATEWAY_ENDPOINT").to_string());
			match Request::get(&url)
				.header("x-api-key", &key.unwrap())
				.send()
				.await {
					Ok(v) => {
						let new_images = v.json::<Vec<WidescreenWallpaper>>().await.unwrap();
						images_clone.set(new_images);
						loading_clone.set(false);
					},
					Err(e) => console::log_1(&JsValue::from(e.to_string())),
				}
		});
		|| ()
	}, ());

	let image_ref = &*images;

    html! {
		<div class="container">
			{if *loading {
				html! {
					<div class="d-flex justify-content-center">
						<div class="spinner-border text-primary mx-auto mt-5" role="status">
							<span class="sr-only">{"Loading..."}</span>
						</div>
					</div>
				}
			} else {
				html!{
					<div class="row g-2">
						{
							image_ref
								.into_iter()
								.map(|image| {
									html! {
										<div class="col-xl-3 col-lg-4 col-sm-6 mb-4">
											<div class="card">
												<Link<Route> to={Route::Home}>
													<img alt={image.name.to_string()} class="card-img-top" src={image.thumbnail_url.to_string()} style="min-height: 100px; aspect-ratio: 21 / 9;" />
												</Link<Route>>
												<div class="card-body">
													<Link<Route> to={Route::Home}>
														<h5 class="card-title">{image.name.to_string()}</h5>
													</Link<Route>>
													<h6 class="card-subtitle">{"widescreen_wallpaper"}</h6>
												</div>
											</div>
										</div>
									}
								})
								.collect::<Html>()
						}
					</div>
				}
			}}
		</div>
    }
}