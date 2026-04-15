use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub isrc: Option<String>,
    #[serde(default)]
    pub artist_name: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
}

impl Song {
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "Artist", "UUID", "ISRC", "Released"]
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.artist_name.clone().unwrap_or_default(),
            self.uuid.clone(),
            self.isrc.clone().unwrap_or_default(),
            self.release_date.clone().unwrap_or_default(),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref artist) = self.artist_name {
            pairs.push(("Artist", artist.clone()));
        }
        if let Some(ref isrc) = self.isrc {
            pairs.push(("ISRC", isrc.clone()));
        }
        if let Some(ref date) = self.release_date {
            pairs.push(("Released", date.clone()));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
