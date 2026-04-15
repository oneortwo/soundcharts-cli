use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::song::Song;
use crate::output;
use crate::paginator;

pub async fn get(client: &SoundchartsClient, identifier_str: &str, json_mode: bool) {
    let id = match identifier::detect(identifier_str) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let response = match id {
        Identifier::Uuid(uuid) => client.get(&format!("/api/v2.25/song/{uuid}"), &[]).await,
        Identifier::Isrc(isrc) => {
            client
                .get(&format!("/api/v2.25/song/by-isrc/{isrc}"), &[])
                .await
        }
        Identifier::PlatformUrl { platform, id } => {
            client
                .get(
                    &format!(
                        "/api/v2.25/song/by-platform/{}/{}",
                        platform,
                        urlencoding::encode(&id)
                    ),
                    &[],
                )
                .await
        }
        _ => {
            eprintln!("error: Songs can be looked up by UUID, ISRC, or platform URL.");
            std::process::exit(1);
        }
    };

    let object = &response.body["object"];

    if json_mode {
        output::print_json(object);
        return;
    }

    match Song::from_value(object) {
        Some(song) => {
            println!("{}", song.name);
            output::print_kv(&song.to_kv());
        }
        None => {
            output::print_json(object);
        }
    }
}

pub async fn audience(client: &SoundchartsClient, uuid: &str, platform: &str, json_mode: bool) {
    let path = format!("/api/v2/song/{uuid}/audience/{platform}");
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

pub async fn playlists(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/song/{uuid}/playlist/current/{platform}");
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
                item.pointer("/playlist/name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.pointer("/playlist/uuid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.pointer("/position")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ]
        })
        .collect();

    if rows.is_empty() {
        eprintln!("No playlist entries found.");
        return;
    }

    output::print_table(&["Playlist", "UUID", "Position"], rows);
}

pub async fn charts(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/song/{uuid}/charts/ranks/{platform}");
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
                item.get("chartName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("rank")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                item.get("date")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            ]
        })
        .collect();

    if rows.is_empty() {
        eprintln!("No chart entries found.");
        return;
    }

    output::print_table(&["Chart", "Rank", "Date"], rows);
}
