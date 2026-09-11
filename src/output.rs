use comfy_table::{ContentArrangement, Table};
use console::Term;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

pub fn resolve_format(json_flag: bool, format_flag: Option<&str>) -> OutputFormat {
    // Explicit --format takes priority
    if let Some(fmt) = format_flag {
        return match fmt {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            "table" => OutputFormat::Table,
            _ => {
                eprintln!("error: Unknown format '{}'. Options: table, json, csv", fmt);
                std::process::exit(1);
            }
        };
    }
    // --json flag
    if json_flag {
        return OutputFormat::Json;
    }
    // TTY detection
    if Term::stdout().is_term() {
        OutputFormat::Table
    } else {
        OutputFormat::Json
    }
}

pub fn print_json(value: &Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("Failed to serialize JSON")
    );
}

pub fn print_json_array(items: &[Value]) {
    let arr = Value::Array(items.to_vec());
    print_json(&arr);
}

pub fn print_table(headers: &[&str], rows: Vec<Vec<String>>) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.load_preset(comfy_table::presets::NOTHING);
    table.set_header(headers.iter().map(|h| h.to_uppercase()).collect::<Vec<_>>());

    for row in rows {
        table.add_row(row);
    }

    println!("{table}");
}

pub fn print_csv(headers: &[&str], rows: Vec<Vec<String>>) {
    // Header row
    println!("{}", headers.join(","));
    // Data rows - quote fields that contain commas, quotes, or newlines
    for row in rows {
        let fields: Vec<String> = row
            .iter()
            .map(|field| {
                if field.contains(',') || field.contains('"') || field.contains('\n') {
                    format!("\"{}\"", field.replace('"', "\"\""))
                } else {
                    field.clone()
                }
            })
            .collect();
        println!("{}", fields.join(","));
    }
}

pub fn print_kv(pairs: &[(&str, String)]) {
    let max_key_len = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in pairs {
        println!(
            "{:width$}  {}",
            format!("{}:", key),
            value,
            width = max_key_len + 1
        );
    }
}

/// Keep raw records in JSON; render selected fields for terminal and CSV output.
pub fn print_records(items: &[Value], fields: &[&str], format: &OutputFormat) {
    match format {
        OutputFormat::Json => print_json_array(items),
        OutputFormat::Csv => print_csv(fields, record_rows(items, fields)),
        OutputFormat::Table => print_table(fields, record_rows(items, fields)),
    }
}

fn record_rows(items: &[Value], fields: &[&str]) -> Vec<Vec<String>> {
    items
        .iter()
        .map(|item| {
            fields
                .iter()
                .map(|field| match item.get(field) {
                    None | Some(Value::Null) => String::new(),
                    Some(Value::String(text)) => text.clone(),
                    Some(value) => value.to_string(),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn record_rows_preserve_false_zero_nested_data_and_missing_fields() {
        let items = [
            json!({"name":"A, B", "verified":false,"value":0,"roles":["writer"],"countryCode":null}),
        ];
        assert_eq!(
            record_rows(
                &items,
                &[
                    "name",
                    "verified",
                    "value",
                    "roles",
                    "countryCode",
                    "missing"
                ]
            ),
            vec![vec!["A, B", "false", "0", "[\"writer\"]", "", ""]]
        );
        assert!(record_rows(&[], &["name"]).is_empty());
    }
}
