use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
		<h1>{"this is the home page h1"}</h1>
    }
}