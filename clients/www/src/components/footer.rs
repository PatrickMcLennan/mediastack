use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
	html! {
		<footer class="mt-auto bg-light text-align-center">
			<p class="container my-3 text-center">{"Copyright Patrick McLennan"}</p>
		</footer>
	}
}