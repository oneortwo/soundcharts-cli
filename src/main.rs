mod cli;
mod client;
mod commands;
mod config;
mod identifier;
mod models;
mod output;
mod paginator;

use clap::{CommandFactory, Parser};
use cli::{
    AlbumCommands, ArtistCommands, AuthCommands, ChartCommands, Cli, CollaboratorCommands,
    Commands, PlaylistCommands, PublisherCommands, SearchCommands, SongCommands, WorkCommands,
};
use client::SoundchartsClient;
use config::resolve_credentials;
use output::resolve_format;

fn require_client(cli: &Cli) -> SoundchartsClient {
    let creds = match resolve_credentials(cli.app_id.as_deref(), cli.api_key.as_deref()) {
        Some(c) => c,
        None => {
            eprintln!("error: No credentials configured. Run 'sc auth setup'.");
            std::process::exit(2);
        }
    };
    SoundchartsClient::new(&creds.app_id, &creds.api_key)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let format = resolve_format(cli.json, cli.format.as_deref());

    match &cli.command {
        Commands::Auth { command } => match command {
            AuthCommands::Setup => commands::auth::setup(cli.no_input).await,
            AuthCommands::Status => {
                commands::auth::status(cli.app_id.as_deref(), cli.api_key.as_deref()).await
            }
        },
        Commands::Doctor => {
            commands::doctor::run(cli.app_id.as_deref(), cli.api_key.as_deref()).await
        }
        Commands::Update => commands::update::run().await,
        Commands::Completions { shell } => {
            clap_complete::generate(*shell, &mut Cli::command(), "sc", &mut std::io::stdout());
        }
        Commands::Search { command } => {
            let client = require_client(&cli);
            match command {
                SearchCommands::Artist { query, pagination } => {
                    commands::search::artist(&client, query, pagination, &format).await
                }
                SearchCommands::Song { query, pagination } => {
                    commands::search::song(&client, query, pagination, &format).await
                }
                SearchCommands::Playlist { query, pagination } => {
                    commands::search::playlist(&client, query, pagination, &format).await
                }
            }
        }
        Commands::Artist { command } => {
            let client = require_client(&cli);
            match command {
                ArtistCommands::Get { identifier } => {
                    commands::artist::get(&client, identifier, &format).await
                }
                ArtistCommands::Songs { uuid, pagination } => {
                    commands::artist::songs(&client, uuid, pagination, &format).await
                }
                ArtistCommands::Albums { uuid, pagination } => {
                    commands::artist::albums(&client, uuid, pagination, &format).await
                }
                ArtistCommands::Stats { uuid } => {
                    commands::artist::stats(&client, uuid, &format).await
                }
                ArtistCommands::Audience { uuid, platform } => {
                    commands::artist::audience(&client, uuid, platform, &format).await
                }
                ArtistCommands::Playlists {
                    uuid,
                    platform,
                    pagination,
                } => {
                    commands::artist::playlists(&client, uuid, platform, pagination, &format).await
                }
                ArtistCommands::Charts {
                    uuid,
                    platform,
                    r#type,
                    pagination,
                } => {
                    commands::artist::charts(&client, uuid, platform, r#type, pagination, &format)
                        .await
                }
                ArtistCommands::Similar { uuid, pagination } => {
                    commands::artist::similar(&client, uuid, pagination, &format).await
                }
                ArtistCommands::Identifiers { uuid } => {
                    commands::artist::identifiers(&client, uuid, &format).await
                }
            }
        }
        Commands::Song { command } => {
            let client = require_client(&cli);
            match command {
                SongCommands::Get { identifier } => {
                    commands::song::get(&client, identifier, &format).await
                }
                SongCommands::Audience { uuid, platform } => {
                    commands::song::audience(&client, uuid, platform, &format).await
                }
                SongCommands::Playlists {
                    uuid,
                    platform,
                    pagination,
                } => commands::song::playlists(&client, uuid, platform, pagination, &format).await,
                SongCommands::Charts {
                    uuid,
                    platform,
                    pagination,
                } => commands::song::charts(&client, uuid, platform, pagination, &format).await,
                SongCommands::Identifiers { uuid } => {
                    commands::song::identifiers(&client, uuid, &format).await
                }
            }
        }
        Commands::Album { command } => {
            let client = require_client(&cli);
            match command {
                AlbumCommands::Get { identifier } => {
                    commands::album::get(&client, identifier, &format).await
                }
                AlbumCommands::Tracks { uuid, pagination } => {
                    commands::album::tracks(&client, uuid, pagination, &format).await
                }
                AlbumCommands::Charts {
                    uuid,
                    platform,
                    pagination,
                } => commands::album::charts(&client, uuid, platform, pagination, &format).await,
            }
        }
        Commands::Chart { command } => {
            let client = require_client(&cli);
            match command {
                ChartCommands::List { platform, r#type } => {
                    commands::chart::list(&client, platform, r#type, &format).await
                }
                ChartCommands::Ranking {
                    slug,
                    r#type,
                    date,
                    latest: _,
                    pagination,
                } => {
                    commands::chart::ranking(
                        &client,
                        slug,
                        r#type,
                        date.as_deref(),
                        pagination,
                        &format,
                    )
                    .await
                }
            }
        }
        Commands::Playlist { command } => {
            let client = require_client(&cli);
            match command {
                PlaylistCommands::Get { uuid } => {
                    commands::playlist::get(&client, uuid, &format).await
                }
                PlaylistCommands::Tracks { uuid, pagination } => {
                    commands::playlist::tracks(&client, uuid, pagination, &format).await
                }
                PlaylistCommands::Audience { uuid, platform } => {
                    commands::playlist::audience(&client, uuid, platform, &format).await
                }
            }
        }
        Commands::Publisher { command } => {
            let client = require_client(&cli);
            match command {
                PublisherCommands::Get { identifier } => {
                    commands::publisher::get(&client, identifier, &format).await
                }
                PublisherCommands::Identifiers { uuid } => {
                    commands::publisher::identifiers(&client, uuid, &format).await
                }
            }
        }
        Commands::Collaborator { command } => {
            let client = require_client(&cli);
            match command {
                CollaboratorCommands::Get { identifier } => {
                    commands::collaborator::get(&client, identifier, &format).await
                }
                CollaboratorCommands::Identifiers { uuid } => {
                    commands::collaborator::identifiers(&client, uuid, &format).await
                }
            }
        }
        Commands::Work { command } => {
            let client = require_client(&cli);
            match command {
                WorkCommands::Get { identifier } => {
                    commands::work::get(&client, identifier, &format).await
                }
                WorkCommands::Identifiers { uuid } => {
                    commands::work::identifiers(&client, uuid, &format).await
                }
                WorkCommands::Recordings { uuid, pagination } => {
                    commands::work::recordings(&client, uuid, pagination, &format).await
                }
            }
        }
        Commands::Tree => {
            print_tree(&Cli::command(), "", true);
        }
    }
}

fn print_tree(cmd: &clap::Command, prefix: &str, is_root: bool) {
    if is_root {
        println!("{}", cmd.get_name());
    }

    let subs: Vec<_> = cmd
        .get_subcommands()
        .filter(|s| !s.is_hide_set() && s.get_name() != "help")
        .collect();

    for (i, sub) in subs.iter().enumerate() {
        let is_last = i == subs.len() - 1;
        let connector = if is_last { "└──" } else { "├──" };
        let about = sub
            .get_about()
            .map(|a| format!("  {a}"))
            .unwrap_or_default();

        println!("{prefix}{connector} {}{about}", sub.get_name());

        let child_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };
        print_tree(sub, &child_prefix, false);
    }
}
