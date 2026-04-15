use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub app_url: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub genres: Vec<Genre>,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub name: String,
}

impl Artist {
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "Genres"]
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.genres
                .iter()
                .map(|g| g.name.clone())
                .collect::<Vec<_>>()
                .join(", "),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        vec![
            ("UUID", self.uuid.clone()),
            ("Name", self.name.clone()),
            (
                "Genres",
                self.genres
                    .iter()
                    .map(|g| g.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        ]
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
