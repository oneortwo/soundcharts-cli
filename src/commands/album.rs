use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::album::Album;
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
        Identifier::Uuid(uuid) => {
            client
                .get(&format!("/api/v2.36/album/by-uuid/{uuid}"), &[])
                .await
        }
        Identifier::Upc(upc) => {
            client
                .get(&format!("/api/v2.36/album/by-upc/{upc}"), &[])
                .await
        }
        Identifier::PlatformUrl { platform, id } => {
            client
                .get(
                    &format!(
                        "/api/v2.36/album/by-platform/{}/{}",
                        platform,
                        urlencoding::encode(&id)
                    ),
                    &[],
                )
                .await
        }
        _ => {
            eprintln!("error: Albums can be looked up by UUID, UPC, or platform URL.");
            std::process::exit(1);
        }
    };

    let object = &response.body["object"];

    if json_mode {
        output::print_json(object);
        return;
    }

    match Album::from_value(object) {
        Some(album) => {
            println!("{}", album.name);
            output::print_kv(&album.to_kv());
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
    let path = format!("/api/v2.26/album/{uuid}/tracks");
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
                item.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("isrc")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("uuid")
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

    output::print_table(&["#", "Name", "ISRC", "UUID"], rows);
}

pub async fn charts(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2.26/album/{uuid}/charts/ranks/{platform}");
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
