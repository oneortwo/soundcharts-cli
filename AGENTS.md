# soundcharts-cli

Rust CLI for the Soundcharts API. Binary name: `sc`.

## Architecture

```
src/
  main.rs              # Entry point, tokio::main, top-level command dispatch
  cli.rs               # Clap derive structs — all commands, args, and flags defined here
  client.rs            # SoundchartsClient — HTTP GET with auth headers, error handling, quota tracking
  config.rs            # Config file (~/.config/soundcharts/config.toml), credential resolution chain
  identifier.rs        # Auto-detect identifier type: UUID, ISRC, UPC, platform URL
  output.rs            # TTY detection, JSON/table/key-value output helpers
  paginator.rs         # Multi-page API fetching with --limit/--all/--no-paginate support
  commands/
    mod.rs             # Module re-exports
    auth.rs            # auth setup (interactive + non-interactive), auth status
    doctor.rs          # Health checks: config, creds, connectivity, quota, version
    update.rs          # Self-update via GitHub Releases (self_update crate)
    search.rs          # Search artists, songs, playlists (20-item page cap)
    artist.rs          # get, songs, albums, stats, audience, playlists, charts, similar
    song.rs            # get, audience, playlists, charts
    album.rs           # get, tracks, charts
    chart.rs           # list, ranking (latest or by date)
    playlist.rs        # get, tracks, audience
  models/
    mod.rs             # Module re-exports
    common.rs          # SingleResponse, CollectionResponse, PageInfo structs
    artist.rs          # Artist + Genre serde structs, table/kv display
    song.rs            # Song serde struct, table/kv display
    album.rs           # Album serde struct, table/kv display
    chart.rs           # ChartEntry table display (rank, name, artist, change)
    playlist.rs        # Playlist serde struct, table/kv display
```

## Development

- Build: `make build`
- Test: `make test`
- Lint: `make lint`
- Both: `make check`
- Format: `cargo fmt`
- Release build: `make release`
- Install locally: `make install`

## Quality

- Run `make check` (lint + tests) before committing and periodically during development
- Every new function or behavior change should have tests
- Tests go in `#[cfg(test)] mod tests` at the bottom of the file they test
- Clippy warnings must be clean — fix them, don't suppress
- Commit often — after each meaningful change. Use conventional commits (`feat:`, `fix:`, `chore:`)
- When adding or removing endpoint support, update the API Endpoint Coverage tables in README.md:
  - Set the endpoint row's "Supported" to "Yes" and fill in the "CLI Command" column
  - If adding a new resource group, move it from the "Not Yet Supported" section into its own table
  - The tables are grouped by resource (Search, Artist, Song, Album, Chart, Playlist) with individual rows per endpoint

## Conventions

- Use clap derive macros, not builder API
- All API calls go through `SoundchartsClient` in `client.rs`
- New API endpoints: add to `cli.rs` (args), `commands/<resource>.rs` (handler), `models/<resource>.rs` (serde struct if needed)
- Nullable API fields: use `Option<T>` with `#[serde(default)]`
- Identifier auto-detection: extend `identifier.rs` when adding new identifier types
- Error handling: `eprintln!("error: ...")` to stderr, then `std::process::exit(N)`
- stdout for data, stderr for diagnostics and progress (per clig.dev)
- TTY: human-readable tables/kv. Piped: JSON. `--json` flag forces JSON.
- Pagination: use `paginator::paginate()` for collection endpoints. Support `--limit`/`--all`/`--page-size`/`--no-paginate`.
- Search endpoints have a 20-item page size cap from the API

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error (missing or invalid credentials) |
| 3 | Not found |
| 4 | Rate limit / quota exceeded |

## API

- Base URL: `https://customer.api.soundcharts.com`
- Auth headers: `x-app-id`, `x-api-key`
- Sandbox: both values are `"soundcharts"` (limited dataset, restricted search terms)
- Quota tracked via `x-quota-remaining` response header
- Credential precedence: CLI flags > env vars (`SOUNDCHARTS_APP_ID`, `SOUNDCHARTS_API_KEY`) > config file
