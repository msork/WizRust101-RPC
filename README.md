![image](assets/icons/sizes/128.png)
# WizRust101-RPC

WizRust101-RPC is a fully vibe-coded project. Codex CLI drives research, specs, design, implementation, testing, and refactoring through the version-controlled spec-driven workflow. The human supplies product decisions and verification data when required; ordinary development does not require the human to program.

Rust Wizard101 log and Discord presence prototype. It discovers Wizard101 log files, tails them incrementally, parses zone and health records, maintains typed game state, and builds a conservative Discord presence. Short sanitized fixtures include records captured from client version `W.1.610.21`. The Discord IPC adapter is implemented; a user interface is not included yet.

## Build and checks

```sh
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Runtime

Run `cargo run --bin wizrust101-rpc` on Windows. It checks the standalone ProgramData path, Steam's Wizard101 app manifest and additional libraries, and the historical default Steam path. It selects the most recently modified valid log and tails new complete lines. The app starts at the end of an existing log, so presence appears after new supported records arrive.

For development, set `WIZRUST101_DISCORD_APP_ID` to a Discord application ID registered with the title `Wizard101`. If unset, the watcher continues without IPC. `WIZRUST101_DISPLAY_STAT=health` is the default; `none` omits the stat. Only locally attributed health observed within the last 60 seconds is shown, labeled as the last logged value. The IPC publisher retries after Discord disconnects. Public packaging will supply one project-owned application ID and registered world art so ordinary users do not configure them.

The runtime `data/zones.json` contains the single currently verified row `Zafaria/ZF_Z07_Stone_Town` → Stone Town → Zafaria. Other raw zone IDs remain unresolved. To import Bacon1661's legacy catalog as unverified candidates, download that project's `zones.json` and run:

```sh
cargo run --bin import-bacon-zones -- <legacy-zones.json> data/zones.json
```

Imported entries are labeled unverified legacy data and never displayed as verified location/world presence. Unknown zone IDs remain unresolved. `data/world-assets.json` maps Zafaria to the project-owner supplied `zafaria` Discord asset key; live client display of that uploaded image still needs smoke testing.

## Evidence limit

The original M1 fixture is reconstructed from literal patterns in the older Bacon1661 parser. M2 includes short sanitized excerpts from real [September 30](tests/fixtures/current-steam-2026-09-30-README.md) and [October 1](tests/fixtures/current-steam-2026-10-01-README.md) Linux Steam logs for client `W.1.610.21`. A controlled October 1 screenshot pair verifies one explicitly local combat-damage globe record; a later three-image sequence supports the exact `WizardClientMod MSG_UpdateHealth` source for out-of-combat recovery. Health observations carry per-record attribution, and only supported local observations update GameState. Other globe sources and general freshness remain unverified. Windows live discovery verification is deferred. See [milestone status](spec/status.md).

[Wizard101 Central's Locations reference](https://wiki.wizard101central.com/wiki/Basic:Locations) is required for reviewing readable location and world relationships. The checked-in [176-page export](research/wizard101central-locations.json) is an incomplete research snapshot. Automated wiki HTML/API crawling is prohibited without site-owner authorization after Cloudflare blocks and a reported IP ban. No raw zone mapping is promoted to current-verified without observed client logs.

## License

WizRust101-RPC is licensed under the [MIT License](LICENSE).
