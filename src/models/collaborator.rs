use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collaborator {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub isni: Option<String>,
    #[serde(default)]
    pub ipi: Option<String>,
    #[serde(default)]
    pub roles: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub web_url: Option<String>,
    #[serde(default)]
    pub biography: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub birth_date: Option<String>,
    #[serde(default)]
    pub city_name: Option<String>,
}

impl Collaborator {
    #[allow(dead_code)]
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "IPI", "Roles", "Country", "Type"]
    }

    #[allow(dead_code)]
    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.ipi.clone().unwrap_or_default(),
            self.roles.clone().unwrap_or_default(),
            self.country_code.clone().unwrap_or_default(),
            self.r#type.clone().unwrap_or_default(),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref ipi) = self.ipi {
            pairs.push(("IPI", ipi.clone()));
        }
        if let Some(ref isni) = self.isni {
            pairs.push(("ISNI", isni.clone()));
        }
        if let Some(ref roles) = self.roles {
            pairs.push(("Roles", roles.clone()));
        }
        if let Some(ref t) = self.r#type {
            pairs.push(("Type", t.clone()));
        }
        if let Some(ref gender) = self.gender {
            pairs.push(("Gender", gender.clone()));
        }
        if let Some(ref country) = self.country_code {
            pairs.push(("Country", country.clone()));
        }
        if let Some(ref city) = self.city_name {
            pairs.push(("City", city.clone()));
        }
        if let Some(ref date) = self.birth_date {
            pairs.push(("Born", date.clone()));
        }
        if let Some(ref url) = self.web_url {
            pairs.push(("Website", url.clone()));
        }
        if let Some(ref bio) = self.biography {
            pairs.push(("Bio", bio.clone()));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
