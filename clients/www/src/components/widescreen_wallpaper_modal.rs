use yew::{function_component, Callback, html, Properties, use_context, use_state};
use yew_router::components::Link;
use crate::routes::{Route, ImagesRoute};
use crate::contexts::{WideScreenWallpaperContext};
use crate::types::{WidescreenWallpaper};

#[derive(PartialEq, Properties)]
pub struct Props {
	pub wallpaper_name: Option<String>,
}

#[function_component(WidescreenWallpaperModal)]
pub fn widescreen_wallpaper_modal(props: &Props) -> Html {
	let widescreen_wallpaper_context = use_context::<WideScreenWallpaperContext>().expect("something is wrong");
	let loading = use_state(|| true);
	let name = match &props.wallpaper_name {
		Some(n) => String::from(n),
		None => String::new()
	};

	let wallpapers_clone = widescreen_wallpaper_context.widescreen_wallpapers.clone(); 

	let image: WidescreenWallpaper = match &wallpapers_clone.iter().find(|x| x.name == name) {
		Some(v) => v.clone().clone(),
		None => WidescreenWallpaper { name: "404!".to_string(), url: String::new(), thumbnail_url: String::new() }
	};

	let url_clone = image.url.clone();
	let thumbnail_url_clone = image.thumbnail_url.clone();

	let onload = {
		let loading_clone = loading.clone();
		Callback::from(move |_| loading_clone.set(false))
	};

	html! {
		<div class="modal fade show" data-show="true" tabindex="-1" role="dialog" style="display: block; opacity: 1; background-color: rgba(0,0,0,.2); padding-right: 17px;"> 
			<div class="modal-dialog modal-xl modal-dialog-centered" role="document">
				<div class="modal-content">
					<div class="modal-header">
						<h5 class="modal-title">{if name.len() >= 1 { name.to_string() } else { "404!".to_string() }}</h5>
					</div>
					<div class="modal-body" style="overflow: hidden; position: relative;">
						<a href={url_clone.to_string()} target="_blank">
							<img aria-hidden="true" src={thumbnail_url_clone.to_string()} style={format!("
								display: block; 
								position: absolute;
								top: 14px;
								right: 0;
								bottom: 0;
								left: 0;
								aspect-ratio: 21 / 9; 
								height: 200px; 
								width: 100%; 
								filter: blur(10px); 
								transition: opacity ease-in-out .2s;
								opacity: {};", if *loading { "1" } else { "0" })}
							/>
							<img 
								src={url_clone.to_string()} 
								style={format!("
									display: block; 
									aspect-ratio: 21 / 9; 
									height: 200px; 
									width: 100%; 
									transition: opacity ease-in-out .2s;
									opacity: {};", if *loading { "0" } else { "1" })}
								{onload}
							/>
						</a>
					</div>
					<div class="modal-footer">
						<Link<Route> to={Route::Home}>
							<button type="button" class="btn btn-primary">
								{"Close"}
							</button>
						</Link<Route>>
					</div>
				</div>
			</div>
		</div>
	}
}