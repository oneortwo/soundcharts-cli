use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::chart::ChartEntry;
use crate::output;
use crate::paginator;

pub async fn list(client: &SoundchartsClient, platform: &str, chart_type: &str, json_mode: bool) {
    let path = match chart_type {
        "album" => format!("/api/v2/chart/album/by-platform/{platform}"),
        _ => format!("/api/v2/chart/song/by-platform/{platform}"),
    };

    let response = client.get(&path, &[]).await;

    if json_mode {
        output::print_json(&response.body);
        return;
    }

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

        output::print_table(&["Slug", "Name", "Country"], rows);
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
    json_mode: bool,
) {
    let path = match (chart_type, date) {
        ("album", Some(d)) => format!("/api/v2.26/chart/album/{slug}/ranking/{d}"),
        ("album", None) => format!("/api/v2.26/chart/album/{slug}/ranking/latest"),
        (_, Some(d)) => format!("/api/v2.14/chart/song/{slug}/ranking/{d}"),
        (_, None) => format!("/api/v2.14/chart/song/{slug}/ranking/latest"),
    };

    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

    let rows: Vec<Vec<String>> = result.items.iter().map(ChartEntry::to_row).collect();

    if rows.is_empty() {
        eprintln!("No ranking data found.");
        return;
    }

    output::print_table(ChartEntry::table_headers(), rows);
}
