![image](assets/icons/sizes/128.png)
# WizRust101-RPC

WizRust101-RPC is a fully vibe-coded project. Codex CLI drives research, specs, design, implementation, testing, and refactoring through the version-controlled spec-driven workflow. The author supplies product decisions and verification data when required; ordinary development does not require the author to program.

Rust Wizard101 Steam log and Discord presence application. It tails logs incrementally, parses zone and health records, maintains typed game state, and builds a conservative Discord presence. Short sanitized fixtures include records captured from client version `W.1.610.21`. Linux native Steam discovery and Discord presence have been live-tested. M6 adds the Linux Flatpak tray package; live Flatpak/Proton validation remains pending.

Active Wizard101 support is Steam only. Official targets, in order, are Linux Flatpak tray app for Steam through Proton, Windows installer/setup tray app for native Steam, and macOS packaged menu-bar app for Steam through CrossOver. Standalone support is deferred until these three targets are complete and tested; each later platform requires its own research and tests. Do not assume compatibility-layer paths. See [the distribution roadmap](spec/future.md).

## Build and checks

```sh
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Runtime

For local development, run `cargo run --bin wizrust101-rpc`; this opens the Linux tray frontend:

```sh
WIZRUST101_DISCORD_APP_ID=<id> cargo run --bin wizrust101-rpc
```

Replace `<id>` with the project Discord Application ID for development/testing. The app selects the most recently modified valid Steam log and tails new complete lines. It starts at the end of an existing log, so presence appears after new supported records arrive. Windows runtime behavior remains deferred.

Active automatic discovery is Steam-only. The app selects the most recently modified valid log and tails new complete lines. It starts at the end of an existing log, so presence appears after new supported records arrive. The native Linux Steam path is verified; Steam-through-Proton inside the Flatpak still needs live validation. Windows runtime behavior remains deferred.

For development/testing, `WIZRUST101_DISCORD_APP_ID` may select a Discord application registered with the title `Wizard101`. If unset in an ordinary development build, the watcher continues without IPC. The release Flatpak embeds the project-owned Application ID, so users do not create a Discord application or set an ID. The ID is not part of normal user configuration. `WIZRUST101_DISPLAY_STAT=health` is the default; `none` omits the stat. Only locally attributed health observed within the last 60 seconds is shown, labeled as the last logged value. The IPC publisher retries after Discord disconnects.

## Linux Flatpak

The Flatpak tray starts log discovery when launched from the desktop. Its menu shows watcher/Discord status, **Add Steam library…** (a folder portal), and **Quit WizRust101-RPC**. No terminal or environment setup is needed for an installed release. It reads the known native Steam and Steam Flatpak directories with read-only access. If Wizard101 is installed in another Steam library, select that library through the tray menu; the app remembers the portal-granted folder separately from `config.json`.

Maintainers can build and install a release locally after installing `flatpak-builder`, the Freedesktop 26.08 SDK/runtime and Rust extension, then running `WIZRUST101_RELEASE_DISCORD_APP_ID=<project-id> scripts/build-flatpak.sh`. The script stages only runtime source files, injects the build-time ID into that temporary manifest, builds from locked offline Cargo sources, and installs the app. The ID is never written into the repository. This maintainer command is not part of normal user setup.

## User configuration

Configuration is optional. Without a file, the app uses automatic log discovery, Health as the Discord State value, and warning-level diagnostics. Create `config.json` in the platform config directory to customize it:

- Linux: `$XDG_CONFIG_HOME/wizrust101-rpc/config.json`, or `~/.config/wizrust101-rpc/config.json` by default.
- Windows: `%APPDATA%\msork\WizRust101-RPC\config\config.json`.

```json
{
  "schema_version": 1,
  "game_log_path": null,
  "display_stat": "health",
  "log_level": "warn"
}
```

Set `game_log_path` to an existing `WizardClient.log` to override discovery; relative paths are resolved from the process working directory. `display_stat` accepts `health` or `none`; `log_level` accepts `error`, `warn`, `info`, or `debug`. Valid `WIZRUST101_DISPLAY_STAT` and `WIZRUST101_LOG_LEVEL` environment values override their file settings; invalid values warn and fall back to the file/default. The `WIZRUST101_DISCORD_APP_ID` environment variable is a development/testing override only and is never stored in this file. Official releases will embed the project ID. Settings take effect on the next app start. Missing files use defaults. Malformed or unsupported-version files are left unchanged and ignored safely; schema version 1 has no automatic migration or rewrite. Unknown fields survive typed JSON round-trips.

The runtime `data/zones.json` contains the single currently verified row `Zafaria/ZF_Z07_Stone_Town` → Stone Town → Zafaria. Other raw zone IDs remain unresolved. To import Bacon1661's legacy catalog as unverified candidates, download that project's `zones.json` and run:

```sh
cargo run --bin import-bacon-zones -- <legacy-zones.json> data/zones.json
```

Imported entries are labeled unverified legacy data and never displayed as verified location/world presence. Unknown zone IDs remain unresolved. `data/world-assets.json` maps Zafaria to the project-owner supplied `zafaria` Discord asset key; the Linux live smoke test displayed the uploaded image. The owner reports uploaded Discord art keys for Aquila, Avalon, Azteca, Celestia, Dragonspyre, Grizzleheim, Khrysalis, Krokotopia, Marleybone, Mooshu, Wizard City, Wysteria, and Zafaria. This is an asset inventory only; it does not establish any zone/world relationship.

## Evidence limit

The original M1 fixture is reconstructed from literal patterns in the older Bacon1661 parser. M2 includes short sanitized excerpts from real [September 30](tests/fixtures/current-steam-2026-09-30-README.md) and [October 1](tests/fixtures/current-steam-2026-10-01-README.md) Linux Steam logs for client `W.1.610.21`. A controlled October 1 screenshot pair verifies one explicitly local combat-damage globe record; a later three-image sequence supports the exact `WizardClientMod MSG_UpdateHealth` source for out-of-combat recovery. Health observations carry per-record attribution, and only supported local observations update GameState. Other globe sources and general freshness remain unverified. M3 presence was live-tested with Linux Steam and Discord; Windows runtime verification is deferred. See [milestone status](spec/status.md).

[Wizard101 Central's Locations reference](https://wiki.wizard101central.com/wiki/Basic:Locations) is required for reviewing readable location and world relationships. The checked-in [176-page export](research/wizard101central-locations.json) is an incomplete research snapshot. Automated wiki HTML/API crawling is prohibited without site-owner authorization after Cloudflare blocks and a reported IP ban. No raw zone mapping is promoted to current-verified without observed client logs.

## License

WizRust101-RPC is licensed under the [MIT License](LICENSE).
