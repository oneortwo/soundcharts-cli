use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub upc: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}

impl Album {
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "UPC", "Type", "Released", "Label"]
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.upc.clone().unwrap_or_default(),
            self.r#type.clone().unwrap_or_default(),
            self.release_date.clone().unwrap_or_default(),
            self.label.clone().unwrap_or_default(),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref upc) = self.upc {
            pairs.push(("UPC", upc.clone()));
        }
        if let Some(ref t) = self.r#type {
            pairs.push(("Type", t.clone()));
        }
        if let Some(ref date) = self.release_date {
            pairs.push(("Released", date.clone()));
        }
        if let Some(ref label) = self.label {
            pairs.push(("Label", label.clone()));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
