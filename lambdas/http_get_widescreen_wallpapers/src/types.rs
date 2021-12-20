use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HttpGetWidescreenWallpapers {}

#[derive(Serialize)]
pub struct Image {
    pub name: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct Output {
    pub status_code: u64,
    pub images: Vec<Image>,
    pub message: String,
}
