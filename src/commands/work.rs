use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::song::Song;
use crate::models::work::Work;
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
                    "/api/v2/work/by-platform/{}/{}",
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

pub async fn identifiers(
    client: &SoundchartsClient,
    uuid: &str,
    pagination: &crate::cli::PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/work/{}/identifiers", urlencoding::encode(uuid));
    let result = crate::paginator::paginate(client, &path, &[], pagination).await;
    output::print_records(
        &result.items,
        &["platformName", "platformCode", "identifier", "url"],
        format,
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{pagination, server};
    use serde_json::json;

    #[tokio::test]
    async fn identifiers_follow_pages() {
        let (client, requests) = server(vec![
            json!({"items":[{"identifier":"one"}],"page":{"next":"next"}}),
            json!({"items":[],"page":{"next":null}}),
        ])
        .await;
        identifiers(&client, "id", &pagination(), &OutputFormat::Json).await;
        let requests = requests.await.unwrap();
        assert_eq!(requests[0].path(), "/api/v2/work/id/identifiers");

        assert!(requests[1]
            .query_pairs()
            .any(|(k, v)| k == "offset" && v == "100"));
    }
}
