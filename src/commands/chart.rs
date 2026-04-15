use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::chart::ChartEntry;
use crate::output;
use crate::output::OutputFormat;
use crate::paginator;

pub async fn list(
    client: &SoundchartsClient,
    platform: &str,
    chart_type: &str,
    format: &OutputFormat,
) {
    let path = match chart_type {
        "album" => format!("/api/v2/chart/album/by-platform/{platform}"),
        _ => format!("/api/v2/chart/song/by-platform/{platform}"),
    };

    let response = client.get(&path, &[]).await;

    let headers = &["Slug", "Name", "Country"];

    if let Some(items) = response.body.get("items").and_then(|i| i.as_array()) {
        let rows: Vec<Vec<String>> = items
            .iter()
            .map(|item| {
                vec![
                    item.get("slug")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    item.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    item.get("countryCode")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                ]
            })
            .collect();

        if rows.is_empty() {
            eprintln!("No charts found for platform '{platform}'.");
            return;
        }

        match format {
            OutputFormat::Json => output::print_json(&response.body),
            OutputFormat::Csv => output::print_csv(headers, rows),
            OutputFormat::Table => output::print_table(headers, rows),
        }
    } else {
        output::print_json(&response.body);
    }
}

pub async fn ranking(
    client: &SoundchartsClient,
    slug: &str,
    chart_type: &str,
    date: Option<&str>,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = match (chart_type, date) {
        ("album", Some(d)) => format!("/api/v2.26/chart/album/{slug}/ranking/{d}"),
        ("album", None) => format!("/api/v2.26/chart/album/{slug}/ranking/latest"),
        (_, Some(d)) => format!("/api/v2.14/chart/song/{slug}/ranking/{d}"),
        (_, None) => format!("/api/v2.14/chart/song/{slug}/ranking/latest"),
    };

    let result = paginator::paginate(client, &path, &[], pagination).await;

    let rows: Vec<Vec<String>> = result.items.iter().map(ChartEntry::to_row).collect();

    if rows.is_empty() {
        eprintln!("No ranking data found.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(ChartEntry::table_headers(), rows),
        OutputFormat::Table => output::print_table(ChartEntry::table_headers(), rows),
    }
}
