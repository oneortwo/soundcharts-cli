use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sc", about = "Soundcharts CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output regardless of terminal
    #[arg(long, global = true)]
    pub json: bool,

    /// Override App ID
    #[arg(long, global = true)]
    pub app_id: Option<String>,

    /// Override API Key
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Disable interactive prompts
    #[arg(long, global = true)]
    pub no_input: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Increase output detail
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage authentication credentials
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// Run health checks
    Doctor,
    /// Update sc to the latest version
    Update,
    /// Search for artists, songs, or playlists
    Search {
        #[command(subcommand)]
        command: SearchCommands,
    },
    /// Query artist data
    Artist {
        #[command(subcommand)]
        command: ArtistCommands,
    },
    /// Query song data
    Song {
        #[command(subcommand)]
        command: SongCommands,
    },
    /// Query album data
    Album {
        #[command(subcommand)]
        command: AlbumCommands,
    },
    /// Query chart data
    Chart {
        #[command(subcommand)]
        command: ChartCommands,
    },
    /// Query playlist data
    Playlist {
        #[command(subcommand)]
        command: PlaylistCommands,
    },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Set up API credentials interactively
    Setup,
    /// Show current auth status and quota
    Status,
}

#[derive(Subcommand)]
pub enum SearchCommands {
    /// Search artists by name
    Artist {
        /// Artist name to search for
        query: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Search songs by name
    Song {
        /// Song name to search for
        query: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Search playlists by name
    Playlist {
        /// Playlist name to search for
        query: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum ArtistCommands {
    /// Get artist metadata (accepts UUID or platform URL)
    Get {
        /// Artist UUID or platform URL
        identifier: String,
    },
    /// List artist's songs
    Songs {
        /// Artist UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// List artist's albums
    Albums {
        /// Artist UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get current stats (followers, listeners)
    Stats {
        /// Artist UUID
        uuid: String,
    },
    /// Get audience data
    Audience {
        /// Artist UUID
        uuid: String,
        /// Platform (spotify, instagram, youtube, etc.)
        #[arg(long)]
        platform: String,
    },
    /// List playlist placements
    Playlists {
        /// Artist UUID
        uuid: String,
        /// Platform (spotify, apple-music, deezer, amazon)
        #[arg(long, default_value = "spotify")]
        platform: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// List chart entries
    Charts {
        /// Artist UUID
        uuid: String,
        /// Platform (spotify, apple-music, etc.)
        #[arg(long, default_value = "spotify")]
        platform: String,
        /// Type: song or album
        #[arg(long, default_value = "song")]
        r#type: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// List similar artists
    Similar {
        /// Artist UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum SongCommands {
    /// Get song metadata (accepts UUID or ISRC)
    Get {
        /// Song UUID or ISRC
        identifier: String,
    },
    /// Get audience data
    Audience {
        /// Song UUID
        uuid: String,
        /// Platform (spotify, apple-music, etc.)
        #[arg(long)]
        platform: String,
    },
    /// List playlist placements
    Playlists {
        /// Song UUID
        uuid: String,
        /// Platform (spotify, apple-music, deezer, amazon)
        #[arg(long, default_value = "spotify")]
        platform: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// List chart entries
    Charts {
        /// Song UUID
        uuid: String,
        /// Platform (spotify, apple-music, etc.)
        #[arg(long, default_value = "spotify")]
        platform: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum AlbumCommands {
    /// Get album metadata (accepts UUID or UPC)
    Get {
        /// Album UUID or UPC
        identifier: String,
    },
    /// List album tracks
    Tracks {
        /// Album UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// List chart entries
    Charts {
        /// Album UUID
        uuid: String,
        /// Platform (spotify, apple-music, etc.)
        #[arg(long, default_value = "spotify")]
        platform: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum ChartCommands {
    /// List available charts for a platform
    List {
        /// Platform (spotify, apple-music, etc.)
        #[arg(long)]
        platform: String,
        /// Type: song or album
        #[arg(long, default_value = "song")]
        r#type: String,
    },
    /// Get chart ranking
    Ranking {
        /// Chart slug (from chart list)
        slug: String,
        /// Type: song or album
        #[arg(long, default_value = "song")]
        r#type: String,
        /// Get ranking for a specific date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Get the latest ranking (default)
        #[arg(long, default_value_t = true)]
        latest: bool,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum PlaylistCommands {
    /// Get playlist metadata
    Get {
        /// Playlist UUID
        uuid: String,
    },
    /// Get current tracklisting
    Tracks {
        /// Playlist UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get audience data
    Audience {
        /// Playlist UUID
        uuid: String,
        /// Platform (spotify, apple-music, deezer, amazon)
        #[arg(long)]
        platform: String,
    },
}

#[derive(clap::Args, Clone)]
pub struct PaginationArgs {
    /// Maximum number of items to return (auto-paginates)
    #[arg(long)]
    pub limit: Option<usize>,

    /// Fetch all pages
    #[arg(long)]
    pub all: bool,

    /// Items per API request
    #[arg(long, default_value = "100")]
    pub page_size: usize,

    /// Return only the first page (no pagination)
    #[arg(long)]
    pub no_paginate: bool,
}
