use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::playlist::Playlist;
use crate::output;
use crate::paginator;

pub async fn get(client: &SoundchartsClient, uuid: &str, json_mode: bool) {
    let response = client.get(&format!("/api/v2/playlist/{uuid}"), &[]).await;
    let object = &response.body["object"];

    if json_mode {
        output::print_json(object);
        return;
    }

    match Playlist::from_value(object) {
        Some(playlist) => {
            println!("{}", playlist.name);
            output::print_kv(&playlist.to_kv());
        }
        None => {
            output::print_json(object);
        }
    }
}

pub async fn tracks(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/playlist/{uuid}/tracklisting/latest");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

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

    output::print_table(&["#", "Song", "Artist", "Song UUID"], rows);
}

pub async fn audience(client: &SoundchartsClient, uuid: &str, platform: &str, json_mode: bool) {
    let path = format!("/api/v2/playlist/{uuid}/audience/{platform}");
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
                    item.get("date")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    item.get("value").map(|v| v.to_string()).unwrap_or_default(),
                ]
            })
            .collect();
        output::print_table(&["Date", "Value"], rows);
    } else {
        output::print_json(&response.body);
    }
}
