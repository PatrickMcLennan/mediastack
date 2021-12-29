use yew::{function_component, html};
use yew_router::hooks::use_route;
use crate::routes::Route;

#[function_component(Banner)]
pub fn banner() -> Html {
	let route: Route = use_route().unwrap();

	let (h1, h2) = match route {
		Route::Login => (String::from("MediaStack"), String::from("Please log in")),
		Route::NotFound => (String::from("404!"), String::from("you're lost")),
		Route::Home => (String::from("Welcome home"), String::from("Your media")),
	};

	html! {
		<section class="jumbotron jumbotron-fluid mb-4 pt-5 pb-5">
			<div class="container">
				<h1 class="my-1.5">{h1}</h1>
				<h2 class="mb-1.5 font-italic">{h2}</h2>
			</div>
		</section>
	}
}