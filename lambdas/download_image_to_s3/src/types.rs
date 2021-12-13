use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SqsImage {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Body {
    pub receiptHandle: String,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SqsEvent {
    pub Records: Option<Vec<Body>>,
}
