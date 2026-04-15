use crate::cli::PaginationArgs;
use crate::client::SoundchartsClient;
use crate::models::artist::Artist;
use crate::models::playlist::Playlist;
use crate::models::song::Song;
use crate::output;
use crate::paginator;

pub async fn artist(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/artist/search/{}", urlencoding::encode(query));
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
        eprintln!("No artists found for '{query}'.");
        return;
    }

    output::print_table(Artist::table_headers(), rows);
}

pub async fn song(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/song/search/{}", urlencoding::encode(query));
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
        eprintln!("No songs found for '{query}'.");
        return;
    }

    output::print_table(Song::table_headers(), rows);
}

pub async fn playlist(
    client: &SoundchartsClient,
    query: &str,
    pagination: &PaginationArgs,
    json_mode: bool,
) {
    let path = format!("/api/v2/playlist/search/{}", urlencoding::encode(query));
    let result = paginator::paginate(client, &path, &[], pagination).await;

    if json_mode {
        output::print_json_array(&result.items);
        return;
    }

    let rows: Vec<Vec<String>> = result
        .items
        .iter()
        .filter_map(|item| Playlist::from_value(item).map(|p| p.to_row()))
        .collect();

    if rows.is_empty() {
        eprintln!("No playlists found for '{query}'.");
        return;
    }

    output::print_table(Playlist::table_headers(), rows);
}
