# Contributing

Thanks for your interest in contributing to soundcharts-cli!

## Setup

```bash
git clone https://github.com/oneortwo/soundcharts-cli
cd soundcharts-cli
cargo build
```

To test against the API, you need Soundcharts credentials. The sandbox is free — use `"soundcharts"` for both App ID and API Key:

```bash
sc auth setup
```

## Development

Run lint and tests before submitting:

```bash
make check
```

See [AGENTS.md](AGENTS.md) for architecture, conventions, and coding standards.

## Pull Requests

- One feature or fix per PR
- `make check` must pass
- If you add a new API endpoint, update the coverage table in README.md
- Use conventional commits: `feat:`, `fix:`, `chore:`, `docs:`

## Adding New Endpoints

1. Add the command and args to `src/cli.rs`
2. Add the handler in `src/commands/<resource>.rs`
3. Add serde structs in `src/models/<resource>.rs` if needed
4. Wire it up in `src/main.rs`
5. Update the API coverage table in `README.md`
