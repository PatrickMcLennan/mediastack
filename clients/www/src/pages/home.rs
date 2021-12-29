use yew::{function_component, html};
use yew_router::components::Redirect;
use crate::hooks::{use_has_auth};
use crate::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
	let has_auth =  use_has_auth();
	if !has_auth { return html! { <Redirect<Route> to={Route::Login} /> } }

    html! {
		<h1>{"this is the home page h1"}</h1>
    }
}