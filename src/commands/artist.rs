use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::album::Album;
use crate::models::artist::Artist;
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
        Identifier::Uuid(uuid) => client.get(&format!("/api/v2.9/artist/{uuid}"), &[]).await,
        Identifier::PlatformUrl { platform, id } => {
            client
                .get(
                    &format!(
                        "/api/v2/artist/by-platform/{}/{}",
                        platform,
                        urlencoding::encode(&id)
                    ),
                    &[],
                )
                .await
        }
        _ => {
            eprintln!("error: Artists can be looked up by UUID or platform URL.");
            std::process::exit(1);
        }
    };

    let object = &response.body["object"];

    if json_mode {
        output::print_json(object);
        return;
    }

    match Artist::from_value(object) {
        Some(artist) => {
            println!("{}", artist.name);
            output::print_kv(&artist.to_kv());
        }
        None => {
            output::print_json(object);
        }
    }
}

pub async fn songs(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2.21/artist/{uuid}/songs");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Song::from_value(item).map(|s| s.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No songs found.");
        return;
    }

    output::print_table(Song::table_headers(), rows);
}

pub async fn albums(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2.34/artist/{uuid}/albums");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Album::from_value(item).map(|a| a.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No albums found.");
        return;
    }

    output::print_table(Album::table_headers(), rows);
}

pub async fn stats(client: &SoundchartsClient, uuid: &str, json_mode: bool) {
    let response = client
        .get(&format!("/api/v2/artist/{uuid}/current/stats"), &[])
        .await;

    if json_mode {
        output::print_json(&response.body);
        return;
    }

    let object = &response.body["object"];
    if let Some(obj) = object.as_object() {
        for (platform, data) in obj {
            println!("{}:", platform);
            if let Some(inner) = data.as_object() {
                for (key, value) in inner {
                    println!("  {}: {}", key, value);
                }
            }
            println!();
        }
    } else {
        output::print_json(&response.body);
    }
}

pub async fn audience(client: &SoundchartsClient, uuid: &str, platform: &str, json_mode: bool) {
    let path = format!("/api/v2/artist/{uuid}/audience/{platform}");
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
    let path = format!("/api/v2.20/artist/{uuid}/playlist/current/{platform}");
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
    chart_type: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = match chart_type {
        "album" => format!("/api/v2/artist/{uuid}/charts/album/ranks/{platform}"),
        _ => format!("/api/v2/artist/{uuid}/charts/song/ranks/{platform}"),
    };
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

pub async fn similar(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/artist/{uuid}/related");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Artist::from_value(item).map(|a| a.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No similar artists found.");
        return;
    }

    output::print_table(Artist::table_headers(), rows);
}
