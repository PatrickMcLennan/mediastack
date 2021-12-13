use serde;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SAttribute {
    pub S: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct BOOLAttribute {
    pub BOOL: Option<bool>,
}

#[derive(Deserialize)]
pub struct NewImage {
    pub created_at: Option<SAttribute>,
    pub name: Option<SAttribute>,
    pub pk: Option<SAttribute>,
    pub sk: Option<SAttribute>,
    pub updated_at: Option<SAttribute>,
    pub url: Option<SAttribute>,
}

#[derive(Deserialize, Serialize)]
pub struct QueueImage {
    pub name: String,
    pub url: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct DynamoDb {
    pub NewImage: Option<NewImage>,
}

#[derive(Deserialize)]
pub struct Record {
    pub dynamodb: Option<DynamoDb>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Request {
    pub Records: Vec<Record>,
}
