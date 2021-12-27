use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
	html! {
		<header class="sticky-top bg-light py-2">
			<nav class="navbar px-2  container">
				{"MediaStack"}
			</nav>
		</header>
	}
}