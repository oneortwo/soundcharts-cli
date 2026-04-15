use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub curator_name: Option<String>,
}

impl Playlist {
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "Platform", "Curator"]
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.platform.clone().unwrap_or_default(),
            self.curator_name.clone().unwrap_or_default(),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref platform) = self.platform {
            pairs.push(("Platform", platform.clone()));
        }
        if let Some(ref curator) = self.curator_name {
            pairs.push(("Curator", curator.clone()));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
