use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::album::Album;
use crate::output;
use crate::output::OutputFormat;
use crate::paginator;

pub async fn get(
    client: &SoundchartsClient,
    identifier_str: &str,
    platform: Option<&str>,
    format: &OutputFormat,
) {
    let response = if let Some(platform) = platform {
        client
            .get(
                &format!(
                    "/api/v2.36/album/by-platform/{}/{}",
                    platform,
                    urlencoding::encode(identifier_str)
                ),
                &[],
            )
            .await
    } else {
        let id = match identifier::detect(identifier_str) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        };
        match id {
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
        }
    };

    let object = &response.body["object"];

    match format {
        OutputFormat::Table => match Album::from_value(object) {
            Some(album) => {
                println!("{}", album.name);
                output::print_kv(&album.to_kv());
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
    let path = format!("/api/v2.26/album/{uuid}/tracks");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    let headers = &["#", "Name", "ISRC", "UUID"];

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

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(headers, rows),
        OutputFormat::Table => output::print_table(headers, rows),
    }
}

pub async fn charts(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2.26/album/{uuid}/charts/ranks/{platform}");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    let headers = &["Chart", "Rank", "Date"];

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

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(headers, rows),
        OutputFormat::Table => output::print_table(headers, rows),
    }
}
