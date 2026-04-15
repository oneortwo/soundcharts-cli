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

## Configuration

Credentials are stored at `~/.config/soundcharts/config.toml` on your local machine only. They are never sent anywhere other than the Soundcharts API. No telemetry, no analytics, no third-party services.

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
