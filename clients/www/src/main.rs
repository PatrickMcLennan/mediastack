#![recursion_limit = "256"]
#[macro_use]
extern crate dotenv_codegen;
mod components;
mod contexts;
mod pages;
mod routes;
mod hooks;
mod types;

use yew::{function_component, html};

use crate::pages::*;
use crate::components::*;
use crate::contexts::*;
use crate::routes::Route;


use yew_router::prelude::*;
use yew::prelude::*;

fn switch(routes: &Route) -> Html {
	match routes {
		Route::Home => html! { <Home /> },
		Route::Widescreen_Wallpaper { name } => html! { <Home /> },
		Route::Login => html! { <Login /> },
		Route::NotFound => html! { <FourOhFour /> },
	}
}

#[function_component(App)]
fn app() -> Html {
    html! {
		<BrowserRouter>
			<WidescreenWallpapersProvider>
				<div class="d-flex justify-content-flex-start align-items-stretch flex-column" style="min-height: 100vh; overflow-x: hidden; overflow-y: auto;">
					<Nav />
					<Banner />
					<main class="main">
						<Switch <Route> render={Switch::render(switch)} />
					</main>
					<Footer />
				</div>
			</WidescreenWallpapersProvider>
		</BrowserRouter>
    }
}

// #[tokio::main]
fn main() {
    yew::start_app::<App>();
}