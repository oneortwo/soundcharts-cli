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

pub async fn identifiers(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    let path = format!("/api/v2/publisher/{uuid}/identifiers");
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
