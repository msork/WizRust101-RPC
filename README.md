# WizRust101-RPC

WizRust101-RPC is a fully vibe-coded project. Codex CLI drives research, specs, design, implementation, testing, and refactoring through the version-controlled spec-driven workflow. The human supplies product decisions and verification data when required; ordinary development does not require the human to program.

Rust Wizard101 log-ingestion prototype. This milestone discovers Wizard101 log files, tails them incrementally, parses legacy-reference zone and health records, and maintains typed game state. Discord RPC and a user interface are not included yet.

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

The checked-in log fixture is reconstructed from literal patterns in the older Bacon1661 parser; it is not a captured Wizard101 log. Current game-build compatibility and local-character health attribution require a sanitized current log sample and Windows smoke testing. See [the M1 research and plan](spec/m1-research.md) and [milestone status](spec/status.md).

[Wizard101 Central's Locations reference](https://wiki.wizard101central.com/wiki/Basic:Locations) is required for reviewing readable location and world names. The page currently returns HTTP 403 to Codex, and its MediaWiki API returns an access-block response. No mapping is promoted from that source until it can be reviewed alongside observed client logs.
