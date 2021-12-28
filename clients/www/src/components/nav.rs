use yew::prelude::*;
use crate::routes::Route;
use crate::hooks::{use_has_auth};
use yew_router::components::Link;

#[function_component(Nav)]
pub fn nav() -> Html {
	let has_auth = use_has_auth();
	html! {
		<header class="sticky-top bg-light py-2">
			<nav class="navbar px-2 container d-flex justify-content-space-between">
				{"MediaStack"}
				<ul class="nav">
					{if !has_auth {
						html! {
							<li class="nav-item active">
								// <Link<Route> disabled to={Route::Login}>
									<span class="nav-link disabled">
										{"Login"}
									</span>
								// </Link<Route>>
							</li>
						}
					} else {
						html! {
							<>
								<li class="nav-item">
									<Link<Route> to={Route::Home}>
										{"Home"}
									</Link<Route>>
								</li>
								<li class="nav-item">
									<Link<Route> to={Route::Home}>
										{"Logout"}
									</Link<Route>>
								</li>
							</>
						}
					}} 
				</ul>
			</nav>
		</header>
	}
}