# cli-soundcharts

Rust CLI for the Soundcharts API. Binary name: `sc`.

## Build & Run

- `cargo build` — build debug binary
- `cargo run -- <args>` — run with arguments
- `cargo test` — run tests
- `cargo clippy` — lint
- `cargo fmt` — format

## Project Structure

- `src/cli.rs` — Clap command tree (all args/flags defined here)
- `src/client.rs` — HTTP client for Soundcharts API
- `src/paginator.rs` — Multi-page API response handling
- `src/config.rs` — Config file (~/.config/soundcharts/config.toml)
- `src/output.rs` — TTY detection, JSON vs table output
- `src/identifier.rs` — Auto-detect UUID/ISRC/UPC/URL
- `src/commands/` — One file per command group (auth, artist, song, etc.)
- `src/models/` — Serde structs matching API responses

## API

Base URL: `https://customer.api.soundcharts.com`
Auth headers: `x-app-id`, `x-api-key`
Sandbox: both values are `"soundcharts"`

## Style

- Use clap derive macros, not builder API
- All API calls go through SoundchartsClient in client.rs
- Errors to stderr, data to stdout
- Exit codes: 0=ok, 1=general, 2=auth, 3=not found, 4=rate limit
