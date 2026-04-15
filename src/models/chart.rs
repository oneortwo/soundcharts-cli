use serde_json::Value;

pub struct ChartEntry;

impl ChartEntry {
    pub fn table_headers() -> &'static [&'static str] {
        &["Rank", "Name", "Artist", "Change"]
    }

    pub fn to_row(value: &Value) -> Vec<String> {
        vec![
            value
                .pointer("/rank")
                .and_then(|v| v.as_u64())
                .map(|v| v.to_string())
                .unwrap_or_default(),
            value
                .pointer("/name")
                .or_else(|| value.pointer("/song/name"))
                .or_else(|| value.pointer("/album/name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            value
                .pointer("/artist/name")
                .or_else(|| value.pointer("/song/artist/name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            value
                .pointer("/rankChange")
                .and_then(|v| v.as_i64())
                .map(|v| {
                    if v > 0 {
                        format!("+{v}")
                    } else {
                        v.to_string()
                    }
                })
                .unwrap_or_default(),
        ]
    }
}
