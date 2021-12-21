use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub name: String,
    pub url: String,
}
