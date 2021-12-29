#![recursion_limit = "256"]
#[macro_use]
extern crate dotenv_codegen;
mod components;
mod pages;
mod routes;
mod hooks;

use crate::pages::*;
use crate::components::*;
use crate::routes::Route;

use yew::{Callback, function_component, html};

use yew_router::prelude::*;
use yew::prelude::*;

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
	
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
	match routes {
		Route::Home => html! { <Home /> },
		Route::Login => html! { <Login /> },
		Route::NotFound => html! { <FourOhFour /> },
	}
}

#[function_component(App)]
fn app() -> Html {
    html! {
		<BrowserRouter>
			<div class="d-flex justify-content-flex-start align-items-stretch flex-column" style="min-height: 100vh; overflow-x: hidden; overflow-y: auto;">
				<Nav />
				<Banner />
				<main class="main">
					<Switch <Route> render={Switch::render(switch)} />
				</main>
				<Footer />
			</div>
		</BrowserRouter>
    }
}

// #[tokio::main]
fn main() {
    yew::start_app::<App>();
}