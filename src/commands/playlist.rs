use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::playlist::Playlist;
use crate::output;
use crate::output::OutputFormat;
use crate::paginator;

pub async fn get(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    let response = client.get(&format!("/api/v2/playlist/{uuid}"), &[]).await;
    let object = &response.body["object"];

    match format {
        OutputFormat::Table => match Playlist::from_value(object) {
            Some(playlist) => {
                println!("{}", playlist.name);
                output::print_kv(&playlist.to_kv());
            }
            None => {
                output::print_json(object);
            }
        },
        _ => output::print_json(object),
    }
}

pub async fn tracks(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/playlist/{uuid}/tracklisting/latest");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    let headers = &["#", "Song", "Artist", "Song UUID"];

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .map(|item| {
            vec![
                item.get("position")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                item.pointer("/song/name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.pointer("/song/artist/name")
                    .or_else(|| item.pointer("/artist/name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.pointer("/song/uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            ]
        })
        .collect();

    if rows.is_empty() {
        eprintln!("No tracks found.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(headers, rows),
        OutputFormat::Table => output::print_table(headers, rows),
    }
}

pub async fn audience(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/playlist/{uuid}/audience/{platform}");
    let response = client.get(&path, &[]).await;

    let headers = &["Date", "Value"];

    if let Some(items) = response.body.get("items").and_then(|i| i.as_array()) {
        let rows: Vec<Vec<String>> = items
            .iter()
            .map(|item| {
                vec![
                    item.get("date")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    item.get("value").map(|v| v.to_string()).unwrap_or_default(),
                ]
            })
            .collect();

        match format {
            OutputFormat::Json => output::print_json(&response.body),
            OutputFormat::Csv => output::print_csv(headers, rows),
            OutputFormat::Table => output::print_table(headers, rows),
        }
    } else {
        output::print_json(&response.body);
    }
}
