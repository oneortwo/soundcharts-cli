use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    name = "sc",
    about = "Soundcharts CLI",
    version = env!("SC_VERSION"),
    help_template = "\
{about}

{usage-heading} {usage}

Data:
  search        Search for artists, songs, or playlists
  artist        Query artist data
  song          Query song data
  album         Query album data
  chart         Query chart data
  playlist      Query playlist data
  work          Query work data (musical compositions)
  publisher     Query publisher data
  collaborator  Query collaborator data (songwriters, composers, producers)

System:
  auth          Manage authentication credentials
  doctor        Run health checks
  update        Update sc to the latest version
  completions   Generate shell completions
  tree          Show all commands in tree form

Options:
{options}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force JSON output regardless of terminal
    #[arg(long, global = true)]
    pub json: bool,

    /// Output format: table, json, csv (default: table in terminal, json when piped)
    #[arg(long, global = true)]
    pub format: Option<String>,

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
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
    /// Query playlist data
    Playlist {
        #[command(subcommand)]
        command: PlaylistCommands,
    },
    /// Query publisher data
    Publisher {
        #[command(subcommand)]
        command: PublisherCommands,
    },
    /// Query collaborator data (songwriters, composers, producers)
    Collaborator {
        #[command(subcommand)]
        command: CollaboratorCommands,
    },
    /// Query work data (musical compositions)
    Work {
        #[command(subcommand)]
        command: WorkCommands,
    },
    /// Show all commands and subcommands in tree form
    Tree,
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
        /// Artist UUID, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID (e.g. spotify, youtube, apple-music)
        #[arg(long)]
        platform: Option<String>,
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
    /// Get platform identifiers (Spotify, YouTube, Apple Music, etc.)
    Identifiers {
        /// Artist UUID
        uuid: String,
    },
}

#[derive(Subcommand)]
pub enum SongCommands {
    /// Get song metadata (accepts UUID, ISRC, or platform URL)
    Get {
        /// Song UUID, ISRC, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID (e.g. youtube, spotify, apple-music)
        #[arg(long)]
        platform: Option<String>,
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
    /// Get platform identifiers (Spotify, YouTube, Apple Music, etc.)
    Identifiers {
        /// Song UUID
        uuid: String,
    },
}

#[derive(Subcommand)]
pub enum AlbumCommands {
    /// Get album metadata (accepts UUID, UPC, or platform URL)
    Get {
        /// Album UUID, UPC, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID (e.g. spotify, apple-music)
        #[arg(long)]
        platform: Option<String>,
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

#[derive(Subcommand)]
pub enum WorkCommands {
    /// Get work metadata (accepts UUID, ISWC, or platform URL)
    Get {
        /// Work UUID, ISWC, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID
        #[arg(long)]
        platform: Option<String>,
    },
    /// Get platform identifiers
    Identifiers {
        /// Work UUID
        uuid: String,
    },
    /// List recordings of this work
    Recordings {
        /// Work UUID
        uuid: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

#[derive(Subcommand)]
pub enum CollaboratorCommands {
    /// Get collaborator metadata (accepts UUID, IPI, or platform URL)
    Get {
        /// Collaborator UUID, IPI, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID
        #[arg(long)]
        platform: Option<String>,
    },
    /// Get platform identifiers
    Identifiers {
        /// Collaborator UUID
        uuid: String,
    },
}

#[derive(Subcommand)]
pub enum PublisherCommands {
    /// Get publisher metadata (accepts UUID, IPI, or platform URL)
    Get {
        /// Publisher UUID, IPI, platform URL, or bare platform ID (with --platform)
        identifier: String,
        /// Treat the identifier as a bare platform ID
        #[arg(long)]
        platform: Option<String>,
    },
    /// Get platform identifiers
    Identifiers {
        /// Publisher UUID
        uuid: String,
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
