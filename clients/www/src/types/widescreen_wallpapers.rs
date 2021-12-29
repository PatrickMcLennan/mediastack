use serde::Deserialize;

#[derive(Deserialize)]
pub struct WidescreenWallpaper {
	pub name: String,
	pub url: String,
	pub thumbnail_url: String,
}