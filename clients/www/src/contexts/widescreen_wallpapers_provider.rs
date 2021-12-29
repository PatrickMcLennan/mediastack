use yew::{function_component, Callback, use_state, Children, ContextProvider, Html, html, Properties, UseStateHandle};
use crate::types::WidescreenWallpaper;
use reqwasm::http::{Request};
use crate::hooks::{use_has_auth};
use web_sys::{console};
use wasm_bindgen::{JsValue};

#[derive(PartialEq, Properties, Clone)] 
pub struct Props {
	#[prop_or_default]
	pub children: Children
}

#[derive(PartialEq, Properties, Clone)]
pub struct WideScreenWallpaperContext {
	pub widescreen_wallpapers: UseStateHandle<Vec<WidescreenWallpaper>>,
	pub loading: UseStateHandle<bool>,
	pub get_widescreen_wallpapers: Callback<()>
}

#[function_component(WidescreenWallpapersProvider)]
pub fn widescreen_wallpapers_provider(props: &Props) -> Html {
	let (_has_auth, key) =  use_has_auth();
	let widescreen_wallpapers: UseStateHandle<Vec<WidescreenWallpaper>> = use_state(|| vec![]);
	let loading: UseStateHandle<bool> = use_state(|| true);

	let widescreen_wallpapers_clone = widescreen_wallpapers.clone();
	let loading_clone = loading.clone();
	let key_clone = key.clone();
	
	let get_widescreen_wallpapers = move |_| {
		let widescreen_wallpapers_clone_again = widescreen_wallpapers_clone.clone();
		let loading_clone_again = loading_clone.clone();
		let key_clone_again = key_clone.clone();

		wasm_bindgen_futures::spawn_local(async move {
			let url = format!("{}widescreen_wallpapers", dotenv!("API_GATEWAY_ENDPOINT").to_string());
			match Request::get(&url)
				.header("x-api-key", &key_clone_again.unwrap())
				.send()
				.await {
					Ok(v) => {
						let new_images = v.json::<Vec<WidescreenWallpaper>>().await.unwrap();
						widescreen_wallpapers_clone_again.set(new_images);
						loading_clone_again.set(false);
						()
					},
					Err(e) => console::log_1(&JsValue::from(e.to_string())),
				}
		})
	};

	let new_state = WideScreenWallpaperContext {
		widescreen_wallpapers,
		loading,
		get_widescreen_wallpapers: Callback::from(get_widescreen_wallpapers)
	};

	html! {
		<ContextProvider<WideScreenWallpaperContext> context={(*&new_state).clone()}>
			{ for props.children.iter() }
		</ContextProvider<WideScreenWallpaperContext>>
	}
}