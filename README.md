![image](assets/icons/sizes/128.png)
# WizRust101-RPC

WizRust101-RPC is a Rust tray/menu-bar app that reads Wizard101 Steam logs and publishes the current location to the running Discord desktop client. It is fully vibe coded: Codex CLI drives research, specifications, implementation, and tests. Wizard101 support is Steam-only; the app never launches or modifies the game.

The recommended first public version is **v0.1.0**, as a Windows-first preview. The Windows installer and portable app passed owner live acceptance on Windows 11 with native Steam Wizard101 and native Discord. Linux packages pass native CI build and package validation, but the current Linux artifacts have not received a new live desktop/Discord test. macOS packages are unsigned and unnotarized, and macOS has not passed live CrossOver/Steam/Discord acceptance; macOS is not production-supported yet. These gates remain open. No public release has been made.

## Download and install

The packaging workflows publish one combined ZIP per platform. Until the first GitHub Release is approved, download these ZIPs from the successful run in [GitHub Actions](https://github.com/msork/WizRust101-RPC/actions). Each workflow artifact is already the platform ZIP; do not expect or extract another wrapper archive.

| Platform ZIP | Contents | Tested configuration and status |
|---|---|---|
| `WizRust101-RPC-Windows.zip` | `WizRust101-RPC-Windows-Setup.exe` and `WizRust101-RPC-Windows-app.exe` | Windows 11 x64, native Steam and native Discord; owner live-tested installer and portable lifecycle. |
| `WizRust101-RPC-Linux.zip` | `WizRust101-RPC-linux.AppImage` and `WizRust101-RPC-linux.flatpak` | Ubuntu 22.04 packaging CI and Flatpak 26.08 runtime/SDK; current artifacts are CI-validated but not newly live-tested. |
| `WizRust101-RPC-macOS.zip` | Universal 2 `WizRust101-RPC.app` and `WizRust101-RPC-macOS.pkg` | macOS 15 hosted CI; unsigned, unnotarized, and not live-tested on a Mac. Not production-supported yet. |

On Windows, extract the ZIP. Run the setup executable for a per-user Start Menu installation and standard uninstaller, or run the portable executable directly from the extracted folder without installing. Both use automatic Steam discovery and need no Discord Application ID setup.

On Linux, extract the ZIP. The AppImage can be run directly; if the desktop marks it non-executable, enable **Allow executing file as program** in file Properties. To install the Flatpak bundle, open it with the desktop's Software installer or use the Flatpak installer provided by your distribution. Linux package behavior still needs fresh live Steam/Discord acceptance; the prior live acceptance applies to an earlier installed Flatpak build only.

The macOS ZIP is currently a development/CI artifact only. It contains an unsigned app and package. Do not treat it as a supported production installation: Developer ID signing, hardened runtime, notarization/stapling, Gatekeeper validation, and real macOS 15+ CrossOver/Steam/native Discord acceptance remain release gates.

All official package builds embed the project's Discord Application ID from the Actions secret. Users do not create a Discord application, set an environment variable, or configure a token. The app shows the DB-provided location in Discord Details, known DB world in State, matching world art when registered (otherwise generic Wizard101 art), and the project logo as small art. The timer continues across zone changes and starts fresh when RPC itself restarts. Health is parsed internally but not shown.

The owner live-retested Windows installer and portable artifacts for continuous timer behavior, Discord restart/reconnect, stationary restart restoration, tray quit/relaunch, portable icon, and known/Unknown-world presence before the recent concise tray-status text change. That text-only change has not received a separate owner live retest. See [the exact milestone record](spec/status.md#m7-final-windows-11-live-acceptance-2026-10-04).

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

Replace `<id>` with the project Discord Application ID for development/testing. The app selects the most recently modified valid Steam log, snapshots its current end offset, replays the existing complete history into state, then tails only later complete lines. A later character-selection event clears any earlier location.

Active automatic discovery is Steam-only. The app selects the most recently modified valid log and replays its existing records before reading appended lines. M6 live acceptance covers the installed Flatpak with native Steam rooted at `~/.local/share/Steam` and native Discord. M7 live acceptance covers the installed Windows package with native Steam and native Discord; other Steam/Discord packaging combinations remain unverified.

For development/testing, `WIZRUST101_DISCORD_APP_ID` may select a Discord application registered with the title `Wizard101`. If unset in an ordinary development build, the watcher continues without IPC. Official release builds embed the project-owned Application ID, so users do not create a Discord application or set an ID. The ID is not part of normal user configuration. Discord Details shows the DB location, State shows a known DB world, the large image uses matching world artwork or the generic Wizard101 asset, and the small image is the project logo. Its timer measures the continuous Wizard101 session and does not reset on zone changes; restarting RPC begins a fresh timer. Health remains internal and is not shown. The IPC publisher checks command acknowledgements and retries/re-publishes after Discord disconnects.

## Linux package details

The Flatpak and AppImage start the same Linux tray/RPC application. Both include concise watcher/Discord status, **Add Steam library…**, and **Quit WizRust101-RPC**. Current release-candidate builds are checked on Ubuntu 22.04 CI, including embedded ID, Flatpak metadata/resources/permissions, AppImage executable/resources, and headless load. Live testing of the latest ZIP is unavailable; previous M6 live acceptance covered an installed Flatpak with native Steam and Discord, not this current AppImage+Flatpak pair.

GitHub Actions builds `WizRust101-RPC-Linux.zip` with `WizRust101-RPC-linux.flatpak` and `WizRust101-RPC-linux.AppImage` at its root. The AppImage uses upstream `appimagetool` 1.9.1 and Type 2 runtime 20251108; CI validates the final ZIP and headless-loads both packages. See [packaging and release status](spec/status.md).

## Windows package details

Windows packaging uses Inno Setup 6.7.3 and creates a per-user 64-bit setup executable. It installs under `%LOCALAPPDATA%\Programs\WizRust101-RPC`, adds Start Menu launch and uninstall shortcuts, and does not add auto-start. The portable executable runs from the extracted folder. Owner live acceptance and its precise scope are recorded in [M7 status](spec/status.md#m7-final-windows-11-live-acceptance-2026-10-04). The project assumes non-commercial use of Inno Setup; review its current licensing if release circumstances change.

The combined ZIP contains `WizRust101-RPC-Windows-Setup.exe` and standalone `WizRust101-RPC-Windows-app.exe` directly. CI checks the embedded manifest/icon and load-checks the portable executable after extracting the final ZIP.

## macOS package status

The native macOS menu-bar implementation targets macOS 15+ and Steam/Wizard101 in CrossOver. Current CI creates an unsigned Universal 2 app and `.pkg`, validates the bundle and payload, and checks the embedded ID. These artifacts are not production-supported or live-accepted.

The ZIP contains `WizRust101-RPC.app` and `WizRust101-RPC-macOS.pkg` directly. Developer ID Application/Installer signing, hardened runtime, notarization/stapling, Gatekeeper behavior, installation tests, and live macOS/CrossOver/Discord acceptance are mandatory release gates. See [M8 research and acceptance](spec/m8-plan.md).

The Discord small-image asset key is `wizrust101_rpc`. Upload `assets/icons/sizes/1024.png` under that exact key; hover text is `WizRust101-RPC`.

Maintainers can build and install a development Flatpak after installing Rust stable, `flatpak-builder`, and the Freedesktop 26.08 SDK/runtime, then running `WIZRUST101_RELEASE_DISCORD_APP_ID=<project-id> scripts/build-flatpak.sh`. The script builds the locked Rust release binary on the host and packages it into the existing Flatpak runtime and permissions. Linux release packages use `scripts/build-linux-packages.sh`; CI verifies the pinned AppImage tool and runtime against the digest metadata on their versioned GitHub releases. The ID is never written into the repository. These maintainer commands are not part of normal user setup.

## User configuration

Configuration is optional. Without a file, the app uses automatic log discovery and warning-level diagnostics. Create `config.json` in the platform config directory to customize it:

- Linux: `$XDG_CONFIG_HOME/wizrust101-rpc/config.json`, or `~/.config/wizrust101-rpc/config.json` by default.
- Windows: `%APPDATA%\msork\WizRust101-RPC\config\config.json`.

```json
{
  "schema_version": 1,
  "game_log_path": null,
  "log_level": "warn"
}
```

Set `game_log_path` to an existing `WizardClient.log` to override discovery; relative paths are resolved from the process working directory. `log_level` accepts `error`, `warn`, `info`, or `debug`; valid `WIZRUST101_LOG_LEVEL` overrides the file setting. The `WIZRUST101_DISCORD_APP_ID` environment variable is a development/testing override only and is never stored in this file. Official releases embed the project ID. Settings take effect on the next app start. Missing files use defaults. Malformed or unsupported-version files are left unchanged and ignored safely; schema version 1 has no automatic migration or rewrite. Unknown fields survive config round-trips. The removed `display_stat` field is ignored and discarded.

Location names come from the pinned [WizRust101-DB](https://github.com/msork/WizRust101-DB) submodule at `vendor/WizRust101-DB/out/zones.json`, paired with `out/zones.diagnostics.json`. DB rows marked `verified` are eligible directly; a fallback name needs exact independent evidence for that DB value. Raw internal-filename fallbacks remain unresolved. The submodule revision is committed with the RPC source so builds work offline and use the same data. Clone with `git clone --recurse-submodules` or initialize with `git submodule update --init --recursive`. World relationships are maintained separately only when independently verified; RPC never infers a world from a path prefix.

See the [full pinned-DB Rich Presence coverage audit](spec/presence-coverage.md) for audited counts and remaining mapping/art gaps.

Unknown zone IDs remain unresolved. `data/world-assets.json` maps the separately verified Zafaria world ID to the project-owner supplied `zafaria` Discord asset key; the Linux live smoke test displayed the uploaded image. The owner reports uploaded Discord art keys for Aquila, Avalon, Azteca, Celestia, Dragonspyre, Grizzleheim, Khrysalis, Krokotopia, Marleybone, Mooshu, Wizard City, Wysteria, and Zafaria. This is an asset inventory only; it does not establish any zone/world relationship.

## Evidence limit

The original M1 fixture is reconstructed from literal patterns in the older Bacon1661 parser. M2 includes short sanitized excerpts from real [September 30](tests/fixtures/current-steam-2026-09-30-README.md) and [October 1](tests/fixtures/current-steam-2026-10-01-README.md) Linux Steam logs for client `W.1.610.21`. A controlled October 1 screenshot pair verifies one explicitly local combat-damage globe record; a later three-image sequence supports the exact `WizardClientMod MSG_UpdateHealth` source for out-of-combat recovery. Health observations carry per-record attribution, and only supported local observations update GameState. Other globe sources and general freshness remain unverified. M3's Linux live test is historical; Windows 11 live acceptance is recorded in M7, while current Linux artifact live acceptance and macOS acceptance remain open. See [milestone status](spec/status.md).

Zone Details and world State come directly from the pinned [WizRust101-DB](https://github.com/msork/WizRust101-DB) generated dataset. See the [mapping contract](spec/zone-world-mapping.md) and [full-dataset coverage audit](spec/presence-coverage.md). The checked-in Wizard101 Central export is a historical research artifact, not a runtime mapping source.

## License

WizRust101-RPC is licensed under the [MIT License](LICENSE).
