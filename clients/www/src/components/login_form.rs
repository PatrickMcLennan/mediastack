use yew::{function_component, html, Html, Callback, Event, FocusEvent, Properties, TargetCast, Reducible, use_reducer, UseReducerHandle};
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement};
use validator::{Validate};
use std::rc::Rc;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref HAS_NUMBER: Regex = Regex::new(r"[0-9]").unwrap();
    static ref HAS_LOWERCASE_LETTER: Regex = Regex::new(r"[a-z]").unwrap();
    static ref HAS_UPPERCASE_LETTER: Regex = Regex::new(r"[A-Z]").unwrap();
}

enum FormAction {
	Email(String),
	Password(String),
	SetErrors(HashMap<String, Vec<String>>)
}

#[derive(PartialEq, Properties)]
pub struct Props {
	pub onsubmit: Callback<LoginFormDTO>,
	pub loading: bool,
}
#[derive(Serialize)]
pub struct LoginFormDTO {
	pub email: String,
	pub password: String
}

#[derive(Validate, Deserialize)]
struct LoginFormState {
	#[validate(email(message = "A valid email must be provided"))]
	email: String,
	email_errors: Vec<String>,
	#[validate(length(min = 8, message = "8 characters long"), regex(path = "HAS_NUMBER", message = "1 number"), regex(path = "HAS_LOWERCASE_LETTER", message = "1 lowercase letter"), regex(path = "HAS_UPPERCASE_LETTER", message = "1 uppercase letter"))]
	password: String,
	password_errors: Vec<String>,
}

impl Default for LoginFormState {
	fn default() -> Self {
		LoginFormState {
			email: String::new(),
			email_errors: vec![],
			password: String::new(),
			password_errors: vec![]
		}
	}
}

impl Reducible for LoginFormState {
	type Action = FormAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
		let email = match &action {
			FormAction::Email(s) => s.to_string(),
			_ => self.email.to_string()
		};

		let password = match &action {
			FormAction::Password(s) => s.to_string(),
			_ => self.password.to_string()
		};

		let mut email_errors = self.email_errors.clone();
		let mut password_errors = self.password_errors.clone();
		match action {
			FormAction::SetErrors(e) => {
				if e.contains_key("email") { email_errors = e.get("email").unwrap().clone(); } else { email_errors = vec![]; };
				if e.contains_key("password") { password_errors = e.get("password").unwrap().clone(); } else { password_errors = vec![]; };
				()
			},
			_ => ()
		};

        Self { email, password, email_errors, password_errors }.into()
    }
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
	let form = use_reducer(LoginFormState::default);

	let onchange = |field: String| {
		let form = form.clone();
		Callback::from(move |e: Event| {
			let input = e.target_unchecked_into::<HtmlInputElement>().value();
            if field == "email" {
				form.dispatch(FormAction::Email(input))
			} else {
				form.dispatch(FormAction::Password(input))
			}
        })
	};

	let onsubmit = |current_form: UseReducerHandle<LoginFormState>| {
		let form = current_form.clone();
		let callback = props.onsubmit.clone();
		Callback::from(move |e: FocusEvent| {
			e.prevent_default();
			match form.validate() {
				Ok(_) => form.dispatch(FormAction::SetErrors(HashMap::new())),
				Err(e) => {
					let mut errors = HashMap::new();
					for error in e.field_errors() {
						let key = String::from(error.0);
						let mut value = vec![];
						for message in error.1 {
							let message = match &message.message {
								Some(v) => v,
								None => continue
							};
							value.push(message.to_string())
						}
						errors.insert(key, value);
					};
					return form.dispatch(FormAction::SetErrors(errors));
				}
			};
			callback.emit(LoginFormDTO { email: String::from(form.email.clone()), password: String::from(form.password.clone()) })
		})
	};

	let email_error_messages = &form.email_errors;
	let password_error_messages = &form.password_errors;

    html! {
		<form class="container" onsubmit={onsubmit(form.clone())}>
			<div class="form-group mb-2">
				<label for="email">{"Email address"}</label>
				<div class="input-group">
					<div class="input-group-prepend">
						<span class="input-group-text" id="basic-addon1">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-circle" viewBox="0 0 16 16">
								<path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z"/>
								<path fill-rule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"/>
							</svg>
						</span>
					</div>
					<input class="form-control" id="email" onchange={onchange("email".to_string())} type="text" value={form.email.to_string()} />
				</div>
				<div style="min-height: 50px;">
					{
						email_error_messages
							.into_iter()
							.map(|message| {
								html! { 
									<small class="d-inline-flex align-items-center badge bg-danger mt-3 ml-4 p-2 pr-3" key={String::from(message)} style="color: white;">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x d-block mr-1" viewBox="0 0 16 16">
											<path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
										</svg>
										{message}
									</small>  
								}
							})
							.collect::<Html>()
					}
				</div>
			</div>
			<div class="form-group">
				<label for="password">{"Password"}</label>
				<div class="input-group">
					<div class="input-group-prepend">
						<span class="input-group-text" id="basic-addon1">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-fingerprint" viewBox="0 0 16 16">
								<path d="M8.06 6.5a.5.5 0 0 1 .5.5v.776a11.5 11.5 0 0 1-.552 3.519l-1.331 4.14a.5.5 0 0 1-.952-.305l1.33-4.141a10.5 10.5 0 0 0 .504-3.213V7a.5.5 0 0 1 .5-.5Z"/>
								<path d="M6.06 7a2 2 0 1 1 4 0 .5.5 0 1 1-1 0 1 1 0 1 0-2 0v.332c0 .409-.022.816-.066 1.221A.5.5 0 0 1 6 8.447c.04-.37.06-.742.06-1.115V7Zm3.509 1a.5.5 0 0 1 .487.513 11.5 11.5 0 0 1-.587 3.339l-1.266 3.8a.5.5 0 0 1-.949-.317l1.267-3.8a10.5 10.5 0 0 0 .535-3.048A.5.5 0 0 1 9.569 8Zm-3.356 2.115a.5.5 0 0 1 .33.626L5.24 14.939a.5.5 0 1 1-.955-.296l1.303-4.199a.5.5 0 0 1 .625-.329Z"/>
								<path d="M4.759 5.833A3.501 3.501 0 0 1 11.559 7a.5.5 0 0 1-1 0 2.5 2.5 0 0 0-4.857-.833.5.5 0 1 1-.943-.334Zm.3 1.67a.5.5 0 0 1 .449.546 10.72 10.72 0 0 1-.4 2.031l-1.222 4.072a.5.5 0 1 1-.958-.287L4.15 9.793a9.72 9.72 0 0 0 .363-1.842.5.5 0 0 1 .546-.449Zm6 .647a.5.5 0 0 1 .5.5c0 1.28-.213 2.552-.632 3.762l-1.09 3.145a.5.5 0 0 1-.944-.327l1.089-3.145c.382-1.105.578-2.266.578-3.435a.5.5 0 0 1 .5-.5Z"/>
								<path d="M3.902 4.222a4.996 4.996 0 0 1 5.202-2.113.5.5 0 0 1-.208.979 3.996 3.996 0 0 0-4.163 1.69.5.5 0 0 1-.831-.556Zm6.72-.955a.5.5 0 0 1 .705-.052A4.99 4.99 0 0 1 13.059 7v1.5a.5.5 0 1 1-1 0V7a3.99 3.99 0 0 0-1.386-3.028.5.5 0 0 1-.051-.705ZM3.68 5.842a.5.5 0 0 1 .422.568c-.029.192-.044.39-.044.59 0 .71-.1 1.417-.298 2.1l-1.14 3.923a.5.5 0 1 1-.96-.279L2.8 8.821A6.531 6.531 0 0 0 3.058 7c0-.25.019-.496.054-.736a.5.5 0 0 1 .568-.422Zm8.882 3.66a.5.5 0 0 1 .456.54c-.084 1-.298 1.986-.64 2.934l-.744 2.068a.5.5 0 0 1-.941-.338l.745-2.07a10.51 10.51 0 0 0 .584-2.678.5.5 0 0 1 .54-.456Z"/>
								<path d="M4.81 1.37A6.5 6.5 0 0 1 14.56 7a.5.5 0 1 1-1 0 5.5 5.5 0 0 0-8.25-4.765.5.5 0 0 1-.5-.865Zm-.89 1.257a.5.5 0 0 1 .04.706A5.478 5.478 0 0 0 2.56 7a.5.5 0 0 1-1 0c0-1.664.626-3.184 1.655-4.333a.5.5 0 0 1 .706-.04ZM1.915 8.02a.5.5 0 0 1 .346.616l-.779 2.767a.5.5 0 1 1-.962-.27l.778-2.767a.5.5 0 0 1 .617-.346Zm12.15.481a.5.5 0 0 1 .49.51c-.03 1.499-.161 3.025-.727 4.533l-.07.187a.5.5 0 0 1-.936-.351l.07-.187c.506-1.35.634-2.74.663-4.202a.5.5 0 0 1 .51-.49Z"/>
							</svg>
						</span>
					</div>
					<input class="form-control" id="password" onchange={onchange("password".to_string())} type="password" value={form.password.to_string()} />
				</div>
				<div style="min-height: 50px;">
					{
						password_error_messages
							.into_iter()
							.map(|message| {
								html! { 
									<small class="d-inline-flex align-items-center badge bg-danger mt-3 ml-4 p-2 pr-3" key={String::from(message)} style="color: white;">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x mr-1" viewBox="0 0 16 16">
											<path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
										</svg>
										{message}
									</small>  
								}
							})
							.collect::<Html>()
					}
				</div>
			</div>
			<button disabled={props.loading} type="submit" class="btn btn-primary btn-large btn-block mt-3">{"Submit"}</button>
		</form>
    }
}
