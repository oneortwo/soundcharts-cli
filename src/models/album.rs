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
    pub labels: Option<Vec<AlbumLabel>>,
    #[serde(default)]
    pub generated_with_ai: Option<bool>,
    #[serde(default)]
    pub genres: Option<Vec<crate::models::artist::Genre>>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumLabel {
    pub name: String,
}

impl Album {
    fn label_names(&self) -> String {
        self.labels
            .as_ref()
            .map(|labels| {
                labels
                    .iter()
                    .map(|label| label.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .filter(|names| !names.is_empty())
            .or_else(|| self.label.clone())
            .unwrap_or_default()
    }

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
            self.label_names(),
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
        let labels = self.label_names();
        if !labels.is_empty() {
            pairs.push(("Label", labels));
        }
        if let Some(generated) = self.generated_with_ai {
            pairs.push(("Generated with AI", generated.to_string()));
        }
        if let Some(genres) = &self.genres {
            pairs.push((
                "Genres",
                genres
                    .iter()
                    .map(|genre| genre.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn current_album_fields_and_nullable_metadata_render() {
        let album = Album::from_value(&json!({"uuid":"id","name":"Album","labels":[{"name":"First"},{"name":"Second"}],"generatedWithAi":false,"genres":null})).unwrap();
        assert_eq!(album.label_names(), "First, Second");
        assert!(album
            .to_kv()
            .contains(&("Generated with AI", "false".into())));
        let legacy = Album::from_value(&json!({"uuid":"id","name":"Album","label":"Legacy","labels":null,"generatedWithAi":null})).unwrap();
        assert_eq!(legacy.label_names(), "Legacy");
        assert!(!legacy
            .to_kv()
            .iter()
            .any(|(k, _)| *k == "Generated with AI"));
    }
}
