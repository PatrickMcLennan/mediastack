use yew::{function_component, html, Html, use_context, use_effect_with_deps};
use yew_router::prelude::*;

use crate::hooks::{use_has_auth};
use crate::routes::{ImagesRoute, Route};
use crate::components::{WidescreenWallpaperModal};
use crate::contexts::{WideScreenWallpaperContext};

fn wallpapers_switch(routes: &ImagesRoute) -> Html {
	match routes {
		ImagesRoute::Home => html! { <></> },
		ImagesRoute::Widescreen_Wallpaper { name } => html! { <WidescreenWallpaperModal wallpaper_name={name.clone()} /> },
		ImagesRoute::ImageNotFound => html! { <WidescreenWallpaperModal /> },
	}
}

#[function_component(Home)]
pub fn home() -> Html {
	let (has_auth, _key) =  use_has_auth();
	if !has_auth { return html! { <Redirect<Route> to={Route::Login} /> } }
	let WideScreenWallpaperContext { loading, widescreen_wallpapers, get_widescreen_wallpapers } = use_context::<WideScreenWallpaperContext>().expect("something is wrong");
	use_effect_with_deps(move |_| { get_widescreen_wallpapers.emit(()); || () }, ());
	let wallpapers_ref = &*widescreen_wallpapers;

    html! {
		<>
			<Switch <ImagesRoute> render={Switch::render(wallpapers_switch)} />
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
								wallpapers_ref
									.into_iter()
									.map(|image| {
										html! {
											<div class="col-xl-3 col-lg-4 col-sm-6 mb-4">
												<div class="card">
													<Link<ImagesRoute> to={ImagesRoute::Widescreen_Wallpaper { name: image.name.to_string() }}>
														<img alt={image.name.to_string()} class="card-img-top" src={image.thumbnail_url.to_string()} style="min-height: 100px; aspect-ratio: 21 / 9;" />
													</Link<ImagesRoute>>
													<div class="card-body">
														<Link<ImagesRoute> to={ImagesRoute::Widescreen_Wallpaper { name: image.name.to_string() }}>
															<h5 class="card-title">{image.name.to_string()}</h5>
														</Link<ImagesRoute>>
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
		</>
    }
}