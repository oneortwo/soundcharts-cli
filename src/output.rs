use comfy_table::{ContentArrangement, Table};
use console::Term;
use serde_json::Value;

pub fn is_json_mode(json_flag: bool) -> bool {
    json_flag || !Term::stdout().is_term()
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
    table.set_header(
        headers
            .iter()
            .map(|h| h.to_uppercase())
            .collect::<Vec<_>>(),
    );

    for row in rows {
        table.add_row(row);
    }

    println!("{table}");
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
