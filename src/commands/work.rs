use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::song::Song;
use crate::models::work::Work;
use crate::output;
use crate::output::OutputFormat;
use crate::paginator;

pub async fn get(client: &SoundchartsClient, identifier_str: &str, format: &OutputFormat) {
    let id = match identifier::detect(identifier_str) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let response = match id {
        Identifier::Uuid(uuid) => client.get(&format!("/api/v2/work/{uuid}"), &[]).await,
        Identifier::Iswc(iswc) => {
            client
                .get(&format!("/api/v2/work/by-iswc/{iswc}"), &[])
                .await
        }
        Identifier::PlatformUrl { platform, id } => {
            client
                .get(
                    &format!(
                        "/api/v2/work/by-platform/{}/{}",
                        platform,
                        urlencoding::encode(&id)
                    ),
                    &[],
                )
                .await
        }
        _ => {
            eprintln!("error: Works can be looked up by UUID, ISWC, or platform URL.");
            std::process::exit(1);
        }
    };

    let object = &response.body["object"];

    match format {
        OutputFormat::Table => match Work::from_value(object) {
            Some(work) => {
                println!("{}", work.name);
                output::print_kv(&work.to_kv());
            }
            None => {
                output::print_json(object);
            }
        },
        _ => output::print_json(object),
    }
}

pub async fn identifiers(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    let path = format!("/api/v2/work/{uuid}/identifiers");
    let response = client.get(&path, &[]).await;

    let items = response
        .body
        .get("items")
        .and_then(|i| i.as_array())
        .cloned()
        .unwrap_or_default();

    if items.is_empty() {
        eprintln!("No identifiers found.");
        return;
    }

    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            vec![
                item.get("platformName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("platformCode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("identifier")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item.get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            ]
        })
        .collect();

    let headers = &["Platform", "Code", "ID", "URL"];
    match format {
        OutputFormat::Json => output::print_json_array(&items),
        OutputFormat::Csv => output::print_csv(headers, rows),
        OutputFormat::Table => output::print_table(headers, rows),
    }
}

pub async fn recordings(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/work/{uuid}/recordings");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Song::from_value(item).map(|s| s.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No recordings found.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(Song::table_headers(), rows),
        OutputFormat::Table => output::print_table(Song::table_headers(), rows),
    }
}
