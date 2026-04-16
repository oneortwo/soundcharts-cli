# Changelog

All notable changes to `sc` are listed here. Versions follow [CalVer](https://calver.org/) (`YYYY.MM.DD`).

## Unreleased

- **`--platform` flag on `get` commands** — look up a resource by a bare platform ID (e.g. `sc song get dQw4w9WgXcQ --platform youtube`, `sc artist get 4NRXx6U8ABQ --platform spotify`). Works on `song`, `artist`, `album`, `work`, `publisher`, `collaborator`. Previously you had to paste a full URL.
- **`sc tree`** — show all commands and subcommands in a tree view
- **Reorganized help** — `sc --help` now groups commands into Data and System sections

## 2026.04.16

- **Work, Publisher, Collaborator resources** — query the publishing/works side of the Soundcharts API
  - `sc work get` (by UUID, ISWC, or platform URL), `sc work identifiers`, `sc work recordings`
  - `sc publisher get` (by UUID, IPI, or platform URL), `sc publisher identifiers`
  - `sc collaborator get` (by UUID, IPI, or platform URL), `sc collaborator identifiers`
- **ISWC and IPI identifier detection** — auto-detects ISWC (e.g. `T9280410915` or `T-928.041.091-5`) and IPI (11-digit) codes
- **`--format csv` output** — `sc artist songs <uuid> --format csv` for spreadsheet-friendly output
- **Song and artist `identifiers` commands** — `sc song identifiers`, `sc artist identifiers` to list platform IDs
- Fix: platform URL identifier extraction now correctly passes the ID, not the full URL

## 2026.04.15

Initial release.

- **9 resource commands**: `auth`, `doctor`, `update`, `search`, `artist`, `song`, `album`, `chart`, `playlist`
- **Smart identifier detection** — pass a UUID, ISRC, UPC, or platform URL (Spotify, Apple Music, YouTube, Deezer, etc.) and `sc` figures out which API endpoint to call
- **Auto-pagination** — `--all` fetches every page, `--limit N` stops after N items
- **TTY-aware output** — human-readable tables in the terminal, JSON when piped
- **Shell completions** — auto-installed for fish, bash, zsh on `sc update`
- **Self-update** — `sc update` downloads the latest release from GitHub
- **Doctor** — `sc doctor` checks config, credentials, API connectivity, quota, and version
- **Config** — credentials stored at `~/.soundcharts/config.toml`, overridable via flags or env vars
