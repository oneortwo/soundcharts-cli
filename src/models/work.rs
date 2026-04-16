use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub iswc: Option<String>,
    #[serde(default)]
    pub writers: Vec<Writer>,
    #[serde(default)]
    pub publishers: Vec<WorkPublisher>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Writer {
    #[serde(default)]
    #[allow(dead_code)]
    pub uuid: Option<String>,
    pub name: String,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkPublisher {
    #[serde(default)]
    #[allow(dead_code)]
    pub uuid: Option<String>,
    pub name: String,
    #[serde(default)]
    pub share: Option<f64>,
    #[serde(default)]
    #[allow(dead_code)]
    pub admin_publisher: Option<AdminPublisher>,
    #[serde(default)]
    #[allow(dead_code)]
    pub related_writers: Vec<Writer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AdminPublisher {
    #[serde(default)]
    pub uuid: Option<String>,
    pub name: String,
    #[serde(default)]
    pub share: Option<f64>,
}

impl Work {
    #[allow(dead_code)]
    pub fn table_headers() -> &'static [&'static str] {
        &["Name", "UUID", "ISWC", "Writers", "Publishers"]
    }

    #[allow(dead_code)]
    pub fn to_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.uuid.clone(),
            self.iswc.clone().unwrap_or_default(),
            format_writers(&self.writers),
            format_publishers(&self.publishers),
        ]
    }

    pub fn to_kv(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("UUID", self.uuid.clone()), ("Name", self.name.clone())];
        if let Some(ref iswc) = self.iswc {
            pairs.push(("ISWC", iswc.clone()));
        }
        if !self.writers.is_empty() {
            pairs.push(("Writers", format_writers(&self.writers)));
        }
        if !self.publishers.is_empty() {
            pairs.push(("Publishers", format_publishers(&self.publishers)));
        }
        pairs
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
    }
}

fn format_writers(writers: &[Writer]) -> String {
    writers
        .iter()
        .map(|w| match &w.role {
            Some(role) if !role.is_empty() => format!("{} ({})", w.name, role),
            _ => w.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_publishers(publishers: &[WorkPublisher]) -> String {
    publishers
        .iter()
        .map(|p| match p.share {
            Some(share) => format!("{} ({:.1}%)", p.name, share),
            None => p.name.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}
