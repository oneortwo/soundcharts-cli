use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::artist::Artist;
use crate::models::playlist::Playlist;
use crate::models::song::Song;
use crate::output;
use crate::output::OutputFormat;
use crate::paginator;

const SEARCH_MAX_PAGE_SIZE: usize = 20;

fn search_pagination(args: &PaginationArgs) -> PaginationArgs {
    let mut p = args.clone();
    p.page_size = p.page_size.min(SEARCH_MAX_PAGE_SIZE);
    p
}

pub async fn artist(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/artist/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Artist::from_value(item).map(|a| a.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No artists found for '{query}'.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(Artist::table_headers(), rows),
        OutputFormat::Table => output::print_table(Artist::table_headers(), rows),
    }
}

pub async fn song(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/song/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Song::from_value(item).map(|s| s.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No songs found for '{query}'.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(Song::table_headers(), rows),
        OutputFormat::Table => output::print_table(Song::table_headers(), rows),
    }
}

pub async fn playlist(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/playlist/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Playlist::from_value(item).map(|p| p.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No playlists found for '{query}'.");
        return;
    }

    match format {
        OutputFormat::Json => output::print_json_array(&result.items),
        OutputFormat::Csv => output::print_csv(Playlist::table_headers(), rows),
        OutputFormat::Table => output::print_table(Playlist::table_headers(), rows),
    }
}

pub async fn collaborator(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/collaborator/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;
    output::print_records(&result.items, &["name", "uuid", "roles"], format);
}

pub async fn album(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/album/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;
    output::print_records(
        &result.items,
        &[
            "name",
            "uuid",
            "creditName",
            "releaseDate",
            "type",
            "totalTracks",
        ],
        format,
    );
}

pub async fn label(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    format: &OutputFormat,
) {
    let path = format!("/api/v2/label/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], &search_pagination(pagination)).await;
    output::print_records(
        &result.items,
        &["name", "uuid", "type", "countryCode"],
        format,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{pagination, server};
    use serde_json::json;

    #[tokio::test]
    async fn new_searches_encode_terms_cap_pages_and_follow_next() {
        for resource in ["collaborator", "album", "label"] {
            let (client, requests) = server(vec![
                json!({"items":[{"uuid":"one","name":"Example"}],"page":{"next":"next"}}),
                json!({"items":[{"uuid":"two","name":"Other"}],"page":{"next":null}}),
            ])
            .await;
            match resource {
                "collaborator" => {
                    collaborator(&client, "A/B & C", &pagination(), &OutputFormat::Json).await
                }
                "album" => album(&client, "A/B & C", &pagination(), &OutputFormat::Json).await,
                _ => label(&client, "A/B & C", &pagination(), &OutputFormat::Json).await,
            }
            let requests = requests.await.unwrap();
            assert_eq!(requests.len(), 2);
            assert_eq!(
                requests[0].path(),
                format!("/api/v2/{resource}/search/A%2FB%20%26%20C")
            );
            assert!(requests[0]
                .query_pairs()
                .any(|(k, v)| k == "limit" && v == "20"));
            assert!(requests[1]
                .query_pairs()
                .any(|(k, v)| k == "offset" && v == "20"));
        }
    }
}
