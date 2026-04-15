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
