use yew::Properties;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Properties, Clone)]
pub struct WidescreenWallpaper {
	pub name: String,
	pub url: String,
	pub thumbnail_url: String,
}