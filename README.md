# sc — Soundcharts CLI

An unofficial command-line interface for the [Soundcharts API](https://developers.soundcharts.com/).

> **Disclaimer:** This project is not affiliated with, endorsed by, or in any way officially connected to Soundcharts. It is an independent wrapper around their public API. "Soundcharts" is a trademark of its respective owner. You need your own Soundcharts API credentials to use this tool.

## Install

### From source

```bash
cargo install --git https://github.com/oneortwo/soundcharts-cli
```

### Prebuilt binaries

Download from [GitHub Releases](https://github.com/oneortwo/soundcharts-cli/releases).

### Install script

```bash
curl -sSL https://raw.githubusercontent.com/oneortwo/soundcharts-cli/main/install.sh | sh
```

## Update

If you installed via the install script or prebuilt binary:

```bash
sc update
```

If you installed via cargo:

```bash
cargo install --git https://github.com/oneortwo/soundcharts-cli --force
```

## Shell Completions

Tab-completion for commands, subcommands, and flags. Completions are installed automatically by the install script and kept up to date by `sc update`.

To install manually:

```bash
# Fish
sc completions fish > ~/.config/fish/completions/sc.fish

# Bash
sc completions bash > ~/.local/share/bash-completion/completions/sc

# Zsh
sc completions zsh > ~/.zfunc/_sc
```

## Quick Start

```bash
# Set up credentials (sandbox uses "soundcharts" for both values)
sc auth setup

# Check everything works
sc doctor

# Search for an artist
sc search artist "Drake"

# Get artist metadata
sc artist get <uuid>

# List an artist's songs (first page)
sc artist songs <uuid>

# Get all songs (auto-paginate)
sc artist songs <uuid> --all

# Get up to 50 songs
sc artist songs <uuid> --limit 50

# Force JSON output
sc artist get <uuid> --json

# Pipe-friendly (auto-JSON when not a terminal)
sc search artist "Drake" | jq '.[0].uuid'
```

## Commands

| Command | Description |
|---------|-------------|
| `sc auth setup` | Configure API credentials |
| `sc auth status` | Show auth state and quota |
| `sc doctor` | Run health checks |
| `sc update` | Self-update to latest version |
| `sc search artist <query>` | Search artists by name |
| `sc search song <query>` | Search songs by name |
| `sc search playlist <query>` | Search playlists by name |
| `sc artist get <id>` | Get artist (UUID or platform URL) |
| `sc artist songs <uuid>` | List artist's songs |
| `sc artist albums <uuid>` | List artist's albums |
| `sc artist stats <uuid>` | Get current stats |
| `sc artist audience <uuid> --platform spotify` | Get audience data |
| `sc artist playlists <uuid>` | List playlist placements |
| `sc artist charts <uuid>` | List chart entries |
| `sc artist similar <uuid>` | List similar artists |
| `sc song get <id>` | Get song (UUID or ISRC) |
| `sc song audience <uuid> --platform spotify` | Get audience data |
| `sc song playlists <uuid>` | List playlist placements |
| `sc song charts <uuid>` | List chart entries |
| `sc album get <id>` | Get album (UUID or UPC) |
| `sc album tracks <uuid>` | List album tracks |
| `sc album charts <uuid>` | List chart entries |
| `sc chart list --platform spotify` | List available charts |
| `sc chart ranking <slug>` | Get chart ranking |
| `sc playlist get <uuid>` | Get playlist metadata |
| `sc playlist tracks <uuid>` | Get playlist tracks |
| `sc playlist audience <uuid> --platform spotify` | Get audience data |

## API Endpoint Coverage

The [Soundcharts API](https://developers.soundcharts.com/) has ~130 endpoints across 16 resource groups. The table below lists every known endpoint and whether this CLI supports it.

### Search

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Search artist by name | ✅ | `sc search artist <query>` |
| Search song by name | ✅ | `sc search song <query>` |
| Search playlist by name | ✅ | `sc search playlist <query>` |
| Search radio by name | ❌ | |
| Search festival by name | ❌ | |
| Search venue by name | ❌ | |
| Get Soundcharts URL from platform URL | ❌ | |

### Artist

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get artist metadata | ✅ | `sc artist get <uuid>` |
| Get artist by platform ID | ✅ | `sc artist get <url>` |
| Get artist songs | ✅ | `sc artist songs <uuid>` |
| Get artist albums | ✅ | `sc artist albums <uuid>` |
| Get similar artists | ✅ | `sc artist similar <uuid>` |
| Get current stats | ✅ | `sc artist stats <uuid>` |
| Get audience | ✅ | `sc artist audience <uuid>` |
| Get playlist entries | ✅ | `sc artist playlists <uuid>` |
| Get chart song entries | ✅ | `sc artist charts <uuid>` |
| Get chart album entries | ✅ | `sc artist charts <uuid> --type album` |
| Get ranked artists | ❌ | |
| Get IDs / platform identifiers | ❌ | |
| Get Soundcharts score | ❌ | |
| Get local audience | ❌ | |
| Get streaming audience | ❌ | |
| Get local streaming audience | ❌ | |
| Get retention | ❌ | |
| Get popularity | ❌ | |
| Get audience report (latest) | ❌ | |
| Get audience report dates | ❌ | |
| Get audience report (by date) | ❌ | |
| Get short videos | ❌ | |
| Get short video audience | ❌ | |
| Get playlist reach | ❌ | |
| Get radio spins | ❌ | |
| Get radio spin count | ❌ | |
| Get events | ❌ | |
| Add links | ❌ | |
| Get contacts | ❌ | |

### Song

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get song metadata | ✅ | `sc song get <uuid>` |
| Get song by ISRC | ✅ | `sc song get <isrc>` |
| Get song by platform ID | ✅ | `sc song get <url>` |
| Get audience | ✅ | `sc song audience <uuid>` |
| Get playlist entries | ✅ | `sc song playlists <uuid>` |
| Get chart entries | ✅ | `sc song charts <uuid>` |
| Get ranked songs | ❌ | |
| Get IDs / platform identifiers | ❌ | |
| Get lyrics analysis | ❌ | |
| Get albums containing song | ❌ | |
| Get popularity | ❌ | |
| Get playlist reach | ❌ | |
| Get radio spins | ❌ | |
| Add links | ❌ | |

### Album

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get album by UUID | ✅ | `sc album get <uuid>` |
| Get album by UPC | ✅ | `sc album get <upc>` |
| Get album by platform ID | ✅ | `sc album get <url>` |
| Get tracklisting | ✅ | `sc album tracks <uuid>` |
| Get chart entries | ✅ | `sc album charts <uuid>` |
| Get IDs / platform identifiers | ❌ | |
| Get audience | ❌ | |
| Get popularity | ❌ | |

### Chart

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| List song charts by platform | ✅ | `sc chart list --platform spotify` |
| List album charts by platform | ✅ | `sc chart list --platform spotify --type album` |
| Get song chart ranking (latest) | ✅ | `sc chart ranking <slug>` |
| Get song chart ranking (by date) | ✅ | `sc chart ranking <slug> --date 2025-01-01` |
| Get album chart ranking (latest) | ✅ | `sc chart ranking <slug> --type album` |
| Get album chart ranking (by date) | ✅ | `sc chart ranking <slug> --type album --date 2025-01-01` |
| Get song chart available dates | ❌ | |
| Get album chart available dates | ❌ | |
| TikTok music ranking (latest) | ❌ | |
| TikTok music ranking dates | ❌ | |
| TikTok music ranking (by date) | ❌ | |

### Playlist

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get playlist metadata | ✅ | `sc playlist get <uuid>` |
| Get tracklisting (latest) | ✅ | `sc playlist tracks <uuid>` |
| Get audience | ✅ | `sc playlist audience <uuid>` |
| List playlists | ❌ | |
| Get playlist by platform ID | ❌ | |
| Get curators by platform | ❌ | |
| Get playlists by curator | ❌ | |
| Get playlists by type | ❌ | |
| Get tracklisting dates | ❌ | |
| Get tracklisting (by date) | ❌ | |

### Not Yet Supported

These resource groups have no CLI support yet:

| Resource | Endpoints |
|----------|-----------|
| Radio | Get radios, live feed, IDs (~4) |
| Festival | Get festivals, metadata, by platform, editions (~7) |
| Venue | Get venues, metadata, by platform, concerts (~7) |
| TikTok | Get music, video count (~3) |
| User | Get blocklists for artists/songs/labels (~4) |
| My Library | Get/add/delete artist and song lists (~7) |
| Referential | Platforms, genres, cities, distributors, etc. (~13) |
| Collaborator | Metadata, by IPI, by platform, IDs (~5) |
| Work | Metadata, by ISWC, by platform, recordings (~6) |
| Publisher | Metadata, by IPI, by platform, IDs (~5) |

PRs welcome for new endpoint support! See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add commands.

## Configuration

Credentials are stored at `~/.soundcharts/config.toml` on your local machine only. They are never sent anywhere other than the Soundcharts API. No telemetry, no analytics, no third-party services.

Credential precedence (highest to lowest):
1. CLI flags (`--app-id`, `--api-key`)
2. Environment variables (`SOUNDCHARTS_APP_ID`, `SOUNDCHARTS_API_KEY`)
3. Config file

## Output

- **Terminal**: human-readable tables
- **Piped**: JSON (auto-detected)
- `--json`: force JSON output

## Building from source

```bash
git clone https://github.com/oneortwo/soundcharts-cli
cd soundcharts-cli
cargo build --release
# Binary at target/release/sc
```

### Cross-compilation

```bash
# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```
