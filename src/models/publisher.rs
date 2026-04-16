use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub ipi: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
}

impl Publisher {
    #[allow(dead_code)]
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "IPI", "Email", "Website"]
    }

    #[allow(dead_code)]
    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.ipi.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            self.website.clone().unwrap_or_default(),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref ipi) = self.ipi {
            pairs.push(("IPI", ipi.clone()));
        }
        if let Some(ref phone) = self.phone {
            pairs.push(("Phone", phone.clone()));
        }
        if let Some(ref email) = self.email {
            pairs.push(("Email", email.clone()));
        }
        if let Some(ref website) = self.website {
            pairs.push(("Website", website.clone()));
        }
        if let Some(ref address) = self.address {
            pairs.push(("Address", address.clone()));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}
