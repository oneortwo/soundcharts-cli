use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::publisher::Publisher;
use crate::output;
use crate::output::OutputFormat;

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
                    "/api/v2/publisher/by-platform/{}/{}",
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
            Identifier::Uuid(uuid) => client.get(&format!("/api/v2/publisher/{uuid}"), &[]).await,
            Identifier::Ipi(ipi) => {
                client
                    .get(&format!("/api/v2/publisher/by-ipi/{ipi}"), &[])
                    .await
            }
            Identifier::PlatformUrl { platform, id } => {
                client
                    .get(
                        &format!(
                            "/api/v2/publisher/by-platform/{}/{}",
                            platform,
                            urlencoding::encode(&id)
                        ),
                        &[],
                    )
                    .await
            }
            _ => {
                eprintln!("error: Publishers can be looked up by UUID, IPI, or platform URL.");
                std::process::exit(1);
            }
        }
    };

    let object = &response.body["object"];

    match format {
        OutputFormat::Table => match Publisher::from_value(object) {
            Some(publisher) => {
                println!("{}", publisher.name);
                output::print_kv(&publisher.to_kv());
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
    let path = format!(
        "/api/v2/publisher/{}/identifiers",
        urlencoding::encode(uuid)
    );
    let result = crate::paginator::paginate(client, &path, &[], pagination).await;
    output::print_records(
        &result.items,
        &["platformName", "platformCode", "identifier", "url"],
        format,
    );
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
        assert_eq!(requests[0].path(), "/api/v2/publisher/id/identifiers");

        assert!(requests[1]
            .query_pairs()
            .any(|(k, v)| k == "offset" && v == "100"));
    }
}
