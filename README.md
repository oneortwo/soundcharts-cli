# sc — Soundcharts CLI

An unofficial command-line interface for the [Soundcharts API](https://developers.soundcharts.com/), **designed with agent UX in mind**.

> **Disclaimer:** This project is not affiliated with, endorsed by, or in any way officially connected to Soundcharts. It is an independent wrapper around their public API. "Soundcharts" is a trademark of its respective owner. You need your own Soundcharts API credentials to use this tool.

## Built for agents (and humans)

`sc` is friendly to both shell-wielding humans and LLM agents driving a terminal:

- **Auto-JSON when piped** — tables for humans on a TTY, clean JSON the moment you pipe into `jq`, `xargs`, or a file. No `--format` juggling.
- **Stable, documented exit codes** — `0` ok, `2` auth, `3` not found, `4` rate-limited. Agents can branch on failure without scraping stderr.
- **stdout/stderr separation** — data on stdout, diagnostics on stderr. `sc foo > data.json` never mixes logs into the payload.
- **Identifier auto-detection** — pass a UUID, ISRC, ISWC, IPI, UPC, or a Spotify/Apple/Deezer URL. The CLI figures out what you meant.
- **Predictable pagination** — `--limit N`, `--all`, `--page-size`, `--no-paginate`. No hidden caps, no "why did I only get 20 rows."
- **Quota-aware** — every call surfaces `x-quota-remaining`. `sc auth status` tells you how much you have left before you burn through it.
- **Tab completion** — for every command, subcommand, and flag. Works in bash, zsh, and fish.
- **Discoverable** — `sc <cmd> --help` is exhaustive and machine-readable. `sc doctor` tells you exactly what's wrong.
- **No telemetry** — credentials stay on your machine, requests only go to Soundcharts.

## Quickstart

```bash
# 1. Install
curl -sSL https://raw.githubusercontent.com/oneortwo/soundcharts-cli/main/install.sh | sh

# 2. Authenticate (use "soundcharts" / "soundcharts" for the sandbox)
sc auth setup

# 3. Confirm it works
sc doctor
```

That's it. Everything below is examples.

## Examples

### The basics

```bash
# Search
sc search artist "Drake"
sc search song "One Dance"
sc search playlist "RapCaviar"

# Fetch by any identifier — sc figures out the type
sc artist get 11e81bcc-9c1c-ce38-b96b-a0369fe50396
sc song get USUM71703861                                  # ISRC
sc album get 00602557775471                               # UPC
sc work get T-010.140.236-1                               # ISWC
sc artist get "https://open.spotify.com/artist/3TVXtAsR1Inumwj472S9r4"

# Force JSON even on a TTY
sc artist stats <uuid> --json

# Paginate a full catalog
sc artist songs <uuid> --all
```

### One-liners: search → pipe → dig deeper

The fun part. Because every `sc` command emits JSON when piped and accepts identifiers
on stdin via `xargs`, you can chain searches into detail lookups without ever copy-pasting
a UUID.

```bash
# Search for an artist → grab their UUID → list every song they've ever released
sc search artist "Phoebe Bridgers" \
  | jq -r '.[0].uuid' \
  | xargs sc artist songs --all

# Top song from a search → list every playlist it's currently on
sc search song "Espresso" \
  | jq -r '.[0].uuid' \
  | xargs sc song playlists --all

# From a search term, pull the artist's current Spotify monthly listeners
sc search artist "Fred again.." \
  | jq -r '.[0].uuid' \
  | xargs -I{} sc artist audience {} --platform spotify --json \
  | jq '.[-1].value'

# ISRC → song → every chart it ever hit
sc song get USUM72309438 --json \
  | jq -r '.uuid' \
  | xargs sc song charts --all

# Artist name → their 5 most similar artists → each one's current stats
sc search artist "Caroline Polachek" \
  | jq -r '.[0].uuid' \
  | xargs sc artist similar --limit 5 \
  | jq -r '.[].uuid' \
  | xargs -n1 sc artist stats
```

### CSV and table shaping with jq/awk

```bash
# Turn an artist's catalog into a clean CSV (ISRC, title, release date)
sc search artist "Charli xcx" \
  | jq -r '.[0].uuid' \
  | xargs sc artist songs --all \
  | jq -r '["isrc","title","release"], (.[] | [.isrc, .name, .releaseDate]) | @csv' \
  > charli.csv

# Rank an artist's songs by playlist reach
sc artist songs <uuid> --all --json \
  | jq -r '.[] | "\(.playlistCount)\t\(.name)"' \
  | sort -rn \
  | head -20 \
  | column -t -s $'\t'
```

### Fan-out with GNU parallel

```bash
# Enrich 200 ISRCs → one NDJSON record per song, 8 concurrent, quota-aware
parallel -j8 'sc song get {} --json' :::: isrcs.txt \
  | jq -c '{isrc, title: .name, artist: .artists[0].name, spotify: .platformIds.spotify}' \
  > enriched.ndjson

# For an artist's whole catalog, fetch audience per song in parallel
sc artist songs <uuid> --all \
  | jq -r '.[].uuid' \
  | parallel -j6 'sc song audience {} --platform spotify --json' \
  | jq -s 'map({uuid: .uuid, streams: (.audience | last | .value)}) | sort_by(-.streams)'

# Every Top-10 hit across every Spotify chart today
sc chart list --platform spotify \
  | jq -r '.[].slug' \
  | parallel -j4 'sc chart ranking {} --limit 10 --json' \
  | jq -s 'flatten | group_by(.song.uuid) | map({song: .[0].song.name, appearances: length}) | sort_by(-.appearances)'
```

### Multi-step recipes

```bash
# "What playlists is Drake's new single on, and who owns each one?"
sc search artist "Drake" \
  | jq -r '.[0].uuid' \
  | xargs sc artist songs --limit 1 \
  | jq -r '.[0].uuid' \
  | xargs sc song playlists --all \
  | jq -r '.[] | "\(.playlist.audience)\t\(.playlist.curator.name)\t\(.playlist.name)"' \
  | sort -rn | head

# Diff a chart week-over-week — who entered, who fell off
comm -3 \
  <(sc chart ranking spotify-top-200-global --json | jq -r '.[].song.name' | sort) \
  <(sc chart ranking spotify-top-200-global --date $(date -v-7d +%F) --json | jq -r '.[].song.name' | sort)

# Compare an artist's reach across platforms in a single table
sc search artist "SZA" | jq -r '.[0].uuid' | read -r UUID
for p in spotify instagram tiktok youtube; do
  echo -e "$p\t$(sc artist audience "$UUID" --platform "$p" --json | jq '.[-1].value')"
done | column -t

# All of an artist's songs that charted on Billboard in the last 12 months
sc artist songs <uuid> --all \
  | jq -r '.[].uuid' \
  | parallel -j6 'sc song charts {} --all --json' \
  | jq -s --arg cutoff "$(date -v-1y +%F)" \
      'flatten | map(select(.chart.platform=="billboard" and .entryDate > $cutoff))'

# Quota-aware batch job: stop if you drop below 500 calls remaining
while read -r isrc; do
  remaining=$(sc auth status --json | jq '.quotaRemaining')
  [ "$remaining" -lt 500 ] && { echo "quota low, stopping" >&2; break; }
  sc song get "$isrc" --json
done < isrcs.txt >> out.ndjson
```

### Exit codes in scripts (and agents)

```bash
sc artist get "$uuid" > /dev/null
case $? in
  0) echo "found" ;;
  2) echo "check your credentials (sc auth status)" ;;
  3) echo "no such artist" ;;
  4) echo "rate limited — back off" ;;
  *) echo "other failure" ;;
esac
```

## Install

### Install script (recommended)

```bash
curl -sSL https://raw.githubusercontent.com/oneortwo/soundcharts-cli/main/install.sh | sh
```

### From source

```bash
cargo install --git https://github.com/oneortwo/soundcharts-cli
```

### Prebuilt binaries

Download from [GitHub Releases](https://github.com/oneortwo/soundcharts-cli/releases).

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
| `sc artist identifiers <uuid>` | Get platform identifiers |
| `sc song get <id>` | Get song (UUID or ISRC) |
| `sc song audience <uuid> --platform spotify` | Get audience data |
| `sc song playlists <uuid>` | List playlist placements |
| `sc song charts <uuid>` | List chart entries |
| `sc song identifiers <uuid>` | Get platform identifiers |
| `sc album get <id>` | Get album (UUID or UPC) |
| `sc album tracks <uuid>` | List album tracks |
| `sc album charts <uuid>` | List chart entries |
| `sc chart list --platform spotify` | List available charts |
| `sc chart ranking <slug>` | Get chart ranking |
| `sc playlist get <uuid>` | Get playlist metadata |
| `sc playlist tracks <uuid>` | Get playlist tracks |
| `sc playlist audience <uuid> --platform spotify` | Get audience data |
| `sc work get <id>` | Get work (UUID, ISWC, or platform URL) |
| `sc work identifiers <uuid>` | Get platform identifiers |
| `sc work recordings <uuid>` | List recordings of this work |
| `sc publisher get <id>` | Get publisher (UUID, IPI, or platform URL) |
| `sc publisher identifiers <uuid>` | Get platform identifiers |
| `sc collaborator get <id>` | Get collaborator (UUID, IPI, or platform URL) |
| `sc collaborator identifiers <uuid>` | Get platform identifiers |

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
| Get IDs / platform identifiers | ✅ | `sc artist identifiers <uuid>` |
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
| Get IDs / platform identifiers | ✅ | `sc song identifiers <uuid>` |
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

### Work

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get work by UUID | ✅ | `sc work get <uuid>` |
| Get work by ISWC | ✅ | `sc work get <iswc>` |
| Get work by platform ID | ✅ | `sc work get <url>` |
| Get IDs / platform identifiers | ✅ | `sc work identifiers <uuid>` |
| Get recordings | ✅ | `sc work recordings <uuid>` |

### Publisher

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get publisher by UUID | ✅ | `sc publisher get <uuid>` |
| Get publisher by IPI | ✅ | `sc publisher get <ipi>` |
| Get publisher by platform ID | ✅ | `sc publisher get <url>` |
| Get IDs / platform identifiers | ✅ | `sc publisher identifiers <uuid>` |

### Collaborator

| Endpoint | Supported | CLI Command |
|----------|-----------|-------------|
| Get collaborator by UUID | ✅ | `sc collaborator get <uuid>` |
| Get collaborator by IPI | ✅ | `sc collaborator get <ipi>` |
| Get collaborator by platform ID | ✅ | `sc collaborator get <url>` |
| Get IDs / platform identifiers | ✅ | `sc collaborator identifiers <uuid>` |

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
