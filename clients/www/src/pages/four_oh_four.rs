use yew::{function_component, html};
use crate::hooks::{use_has_auth};

#[function_component(FourOhFour)]
pub fn four_oh_four() -> Html {
	let has_auth = use_has_auth();
    html! {
		<div class="container">
			<p>{"This page doesn't exist!"}</p>
		</div>
    }
}