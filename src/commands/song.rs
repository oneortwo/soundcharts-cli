use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::identifier::{self, Identifier};
use crate::models::song::Song;
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
                    "/api/v2.25/song/by-platform/{}/{}",
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
        }
    };

    let object = &response.body["object"];

    match format {
        OutputFormat::Table => match Song::from_value(object) {
            Some(song) => {
                println!("{}", song.name);
                output::print_kv(&song.to_kv());
            }
            None => {
                output::print_json(object);
            }
        },
        _ => output::print_json(object),
    }
}

pub async fn audience(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/song/{uuid}/audience/{platform}");
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

pub async fn playlists(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/song/{uuid}/playlist/current/{platform}");
    let result = paginator::paginate(client, &path, &[], pagination).await;

    let headers = &["Playlist", "UUID", "Position"];

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
    let path = format!("/api/v2/song/{uuid}/charts/ranks/{platform}");
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

pub async fn identifiers(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    let path = format!("/api/v2/song/{uuid}/identifiers");
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

pub async fn stats(client: &SoundchartsClient, uuid: &str, period: u32, format: &OutputFormat) {
    let response = client
        .get(
            &format!("/api/v2/song/{}/current/stats", urlencoding::encode(uuid)),
            &[("period", &period.to_string())],
        )
        .await;
    if *format == OutputFormat::Json {
        output::print_json(&response.body);
    } else {
        let rows = stats_rows(&response.body);
        output::print_records(
            &rows,
            &[
                "metric",
                "platform",
                "value",
                "date",
                "evolution",
                "percentEvolution",
            ],
            format,
        );
    }
}

fn stats_rows(body: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut rows = Vec::new();
    for key in ["audience", "popularity", "score"] {
        if let Some(items) = body[key].as_array() {
            for item in items {
                if let Some(object) = item.as_object() {
                    let mut object = object.clone();
                    object.insert("metric".into(), serde_json::json!(key));
                    rows.push(serde_json::Value::Object(object));
                }
            }
        }
    }
    rows
}

pub async fn score(
    client: &SoundchartsClient,
    uuid: &str,
    history: &crate::cli::HistoryArgs,
    pagination: &crate::cli::PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!(
        "/api/v2/song/{}/soundcharts/score",
        urlencoding::encode(uuid)
    );
    let result = crate::paginator::paginate(client, &path, &history.params(), pagination).await;
    output::print_records(
        &result.items,
        &["date", "fanbaseScore", "trendingScore"],
        format,
    );
}

pub async fn streaming(
    client: &SoundchartsClient,
    uuid: &str,
    platform: &str,
    history: &crate::cli::HistoryArgs,
    pagination: &crate::cli::PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!(
        "/api/v2/song/{}/streaming/{}",
        urlencoding::encode(uuid),
        urlencoding::encode(platform)
    );
    let result = crate::paginator::paginate(client, &path, &history.params(), pagination).await;
    output::print_records(
        &result.items,
        &["date", "value", "countryPlots", "cityPlots"],
        format,
    );
}

pub async fn related(client: &SoundchartsClient, uuid: &str, format: &OutputFormat) {
    // This collection has no documented offset/limit parameters.
    let response = client
        .get(
            &format!("/api/v2/song/{}/related", urlencoding::encode(uuid)),
            &[],
        )
        .await;
    let items = response.body["items"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    output::print_records(&items, &["name", "uuid", "creditName"], format);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cli::HistoryArgs,
        test_support::{pagination, server},
    };
    use serde_json::json;

    #[test]
    fn stats_table_keeps_each_metric_family() {
        let rows = stats_rows(
            &json!({"audience":[{"platform":"spotify","value":0}],"popularity":null,"score":[{"platform":"soundcharts","value":50}]}),
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["metric"], "audience");
        assert_eq!(rows[0]["value"], 0);
        assert_eq!(rows[1]["metric"], "score");
    }

    #[tokio::test]
    async fn metrics_send_period_dates_and_platform_to_documented_routes() {
        let (client, requests) = server(vec![
            json!({"audience":[],"score":[]}),
            json!({"items":[]}),
            json!({"items":[]}),
            json!({"items":[]}),
        ])
        .await;
        let history = HistoryArgs {
            start_date: Some("2026-05-01".into()),
            end_date: Some("2026-06-01".into()),
            sort: "asc".into(),
        };
        stats(&client, "song-id", 28, &OutputFormat::Json).await;
        score(
            &client,
            "song-id",
            &history,
            &pagination(),
            &OutputFormat::Json,
        )
        .await;
        streaming(
            &client,
            "song-id",
            "youtube",
            &history,
            &pagination(),
            &OutputFormat::Json,
        )
        .await;
        related(&client, "song-id", &OutputFormat::Json).await;
        let requests = requests.await.unwrap();
        assert_eq!(requests[0].path(), "/api/v2/song/song-id/current/stats");
        assert_eq!(requests[0].query(), Some("period=28"));
        assert_eq!(requests[1].path(), "/api/v2/song/song-id/soundcharts/score");
        assert_eq!(requests[2].path(), "/api/v2/song/song-id/streaming/youtube");
        for request in &requests[1..3] {
            let pairs: std::collections::HashMap<_, _> = request.query_pairs().collect();
            assert_eq!(pairs["startDate"], "2026-05-01");
            assert_eq!(pairs["endDate"], "2026-06-01");
            assert_eq!(pairs["sort"], "asc");
        }
        assert_eq!(requests[3].path(), "/api/v2/song/song-id/related");
        assert_eq!(requests[3].query(), None);
    }
}
