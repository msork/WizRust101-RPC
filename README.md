# WizRust101-RPC

WizRust101-RPC is a fully vibe-coded project. Codex CLI drives research, specs, design, implementation, testing, and refactoring through the version-controlled spec-driven workflow. The human supplies product decisions and verification data when required; ordinary development does not require the human to program.

Rust Wizard101 log-ingestion prototype. It discovers Wizard101 log files, tails them incrementally, parses zone and health records, and maintains typed game state. Short sanitized fixtures now include records captured from client version `W.1.610.21`. Discord RPC and a user interface are not included yet.

## Build and checks

```sh
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Runtime

Run `cargo run --bin wizrust101-rpc` on Windows. It checks the standalone ProgramData path, Steam's Wizard101 app manifest and additional libraries, and the historical default Steam path. It selects the most recently modified valid log and tails new complete lines.

The default `data/zones.json` is empty. To import Bacon1661's legacy catalog, download that project's `zones.json` and run:

```sh
cargo run --bin import-bacon-zones -- <legacy-zones.json> data/zones.json
```

Imported entries are labeled unverified legacy data. Unknown zone IDs remain unresolved.

## Evidence limit

The original M1 fixture is reconstructed from literal patterns in the older Bacon1661 parser. M2 includes [short sanitized excerpts](tests/fixtures/current-steam-2026-09-30-README.md) from a real 2026-09-30 Linux Steam log, verifying those specific record forms. Local-character health attribution and Windows behavior still require a synchronized in-game observation and Windows smoke testing. See [milestone status](spec/status.md).

[Wizard101 Central's Locations reference](https://wiki.wizard101central.com/wiki/Basic:Locations) is required for reviewing readable location and world relationships. The checked-in [176-page export](research/wizard101central-locations.json) is an incomplete research snapshot. Automated wiki HTML/API crawling is prohibited without site-owner authorization after Cloudflare blocks and a reported IP ban. No raw zone mapping is promoted to current-verified without observed client logs.
