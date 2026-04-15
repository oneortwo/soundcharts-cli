use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SingleResponse {
    pub object: Value,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CollectionResponse {
    pub items: Vec<Value>,
    pub page: Option<PageInfo>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PageInfo {
    pub offset: u64,
    pub total: Option<u64>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub limit: u64,
}
