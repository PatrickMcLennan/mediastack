use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
	#[at("/widescreen_wallpaper/:name")]
    Widescreen_Wallpaper { name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum ImagesRoute {
	#[at("/")]
    Home,
	#[at("/widescreen_wallpaper/:name")]
    Widescreen_Wallpaper { name: String },
	#[not_found]
	#[at("/widescreen_wallpaper/404")]
    ImageNotFound,
}