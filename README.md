![image](assets/icons/sizes/128.png)
# WizRust101-RPC

WizRust101-RPC is a fully vibe-coded project. Codex CLI drives research, specs, design, implementation, testing, and refactoring through the version-controlled spec-driven workflow. The author supplies product decisions and verification data when required; ordinary development does not require the author to program.

Rust Wizard101 Steam log and Discord presence application. It tails logs incrementally, parses zone and health records, maintains typed game state, and builds a conservative Discord presence. Short sanitized fixtures include records captured from client version `W.1.610.21`. M6's installed Flatpak/tray path passed final visual acceptance with native Steam and native Discord. M7's Windows installer and portable builds passed owner live retest for timer continuity, Discord reconnect, stationary restoration, tray relaunch/quit lifecycle, embedded icon, and known/Unknown-world presence before the current concise-status text change; see [the M7 record](spec/status.md#m7-final-windows-11-live-acceptance-2026-10-04).

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

Replace `<id>` with the project Discord Application ID for development/testing. The app selects the most recently modified valid Steam log, snapshots its current end offset, replays the existing complete history into state, then tails only later complete lines. A later character-selection event clears any earlier location.

Active automatic discovery is Steam-only. The app selects the most recently modified valid log and replays its existing records before reading appended lines. M6 live acceptance covers the installed Flatpak with native Steam rooted at `~/.local/share/Steam` and native Discord. M7 live acceptance covers the installed Windows package with native Steam and native Discord; other Steam/Discord packaging combinations remain unverified.

For development/testing, `WIZRUST101_DISCORD_APP_ID` may select a Discord application registered with the title `Wizard101`. If unset in an ordinary development build, the watcher continues without IPC. Official release builds embed the project-owned Application ID, so users do not create a Discord application or set an ID. The ID is not part of normal user configuration. Discord Details shows the DB location, State shows a known DB world, the large image uses matching world artwork or the generic Wizard101 asset, and the small image is the project logo. Its timer measures the continuous Wizard101 session and does not reset on zone changes; restarting RPC begins a fresh timer. Health remains internal and is not shown. The IPC publisher checks command acknowledgements and retries/re-publishes after Discord disconnects.

## Linux packages

The Flatpak and AppImage both start the same Linux tray/RPC application. Flatpak runs within its existing sandbox permissions; AppImage runs directly from its executable file and needs no install, terminal, or environment setup. Both menus show concise watcher/Discord status, **Add Steam library…** (a folder portal), and **Quit WizRust101-RPC**. They discover the known native Steam and Steam Flatpak directories and use the folder portal for another library. The downloadable `WizRust101-RPC-Linux.zip` contains both the AppImage and Flatpak at its root. M6 live acceptance covered the installed Flatpak with native Steam/Discord; AppImage desktop/tray/RPC behavior still requires live Linux acceptance. On Linux desktops that block execution of downloaded files, mark the downloaded AppImage executable in the file manager's Properties dialog; no terminal command is required.

GitHub Actions publishes `WizRust101-RPC-Linux.zip` containing `WizRust101-RPC-linux.flatpak` and `WizRust101-RPC-linux.AppImage` at its root. The workflow uploads this ZIP as-is, without an extra artifact wrapper. The official build embeds the Discord Application ID from the `WIZRUST101_RELEASE_DISCORD_APP_ID` Actions secret. The AppImage uses upstream `appimagetool` 1.9.1 and the versioned Type 2 runtime 20251108 to wrap the app executable, desktop entry, and icon; CI checks both downloaded assets against their GitHub release digests, extracts and inspects the final image, then launches its embedded executable in headless load-check mode. Flatpak CI installs the copy from the final ZIP, checks exported resources and permissions, and runs the same load check. See the Linux packaging notes in [M8 and packaging status](spec/status.md).

## Windows installer (M7)

Windows official packaging uses Inno Setup 6.7.3 and creates a per-user 64-bit setup executable. It installs the application and icon under `%LOCALAPPDATA%\Programs\WizRust101-RPC`, adds Start Menu launch and uninstall shortcuts, and registers the standard uninstaller. It does not add auto-start. The app starts as a tray process, watches Steam automatically, and offers concise status, **Add Steam library…**, and **Quit**. Additional libraries are validated for Wizard101 Steam app `799960` before they are stored in the per-user library registry. The executable embeds and verifies the Common Controls v6 manifest needed by `TaskDialogIndirect`. The owner live-tested both installer and portable builds, including stationary restart restoration, continuous timer, Discord reconnect, presence mappings, portable icon, and tray lifecycle. The project assumes non-commercial use of Inno Setup; review its current licensing if release circumstances change.

Maintainers with Windows, Rust stable/MSVC, and Inno Setup 6.7.3 can run `scripts/build-windows.ps1`. Set `WIZRUST101_RELEASE_DISCORD_APP_ID` in the build environment for an official package; the GitHub workflow reads it from the `WIZRUST101_RELEASE_DISCORD_APP_ID` repository secret. Before Inno Setup runs, the script checks the built EXE's embedded manifest and icon and launches it in an import-resolution probe. The downloadable `WizRust101-RPC-Windows.zip` contains `WizRust101-RPC-Windows-Setup.exe` and the standalone `WizRust101-RPC-Windows-app.exe` directly; CI validates and load-checks the portable EXE after extracting that final ZIP. The installed app and portable EXE need no ID variable or user Discord setup. Output is written to `target/windows-installer/`.

## macOS app (M8)

M8 targets a native menu-bar app on macOS 15+ with Apple Silicon first. Wizard101/Steam run in CrossOver; WizRust101-RPC does not. CrossOver bottles and Steam libraries are discovered from documented macOS roots/preferences and the bottle's drive mappings. Use **Add CrossOver bottle…** for an additional bottle. The build script produces an unsigned `.app` and `.pkg` for local testing; installing the app requires no terminal or environment setup. Official packages embed the project Discord Application ID at build time and require Developer ID signing, hardened runtime, and notarization for direct distribution.

Maintainers can build on macOS with `bash scripts/build-macos.sh`; set `WIZRUST101_RELEASE_DISCORD_APP_ID` for official builds to embed Discord RPC support. Apple Silicon is the default; set `WIZRUST101_MACOS_ARCH=x86_64` or `universal2` to build those variants. GitHub Actions runs native macOS 15 tests and warning-denied Clippy, then validates and publishes `WizRust101-RPC-macOS.zip`, containing the Universal 2 `WizRust101-RPC.app` bundle and `WizRust101-RPC-macOS.pkg`. The artifact is the ZIP itself, not another wrapper ZIP. PR validation receives no secret. Once Intel support is completely obsolete, the release artifact can return to arm64-only. These development artifacts are not signed or notarized; Developer ID signing/notarization and live macOS/CrossOver/Discord acceptance remain open. See [M8 research and acceptance](spec/m8-plan.md).

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

The original M1 fixture is reconstructed from literal patterns in the older Bacon1661 parser. M2 includes short sanitized excerpts from real [September 30](tests/fixtures/current-steam-2026-09-30-README.md) and [October 1](tests/fixtures/current-steam-2026-10-01-README.md) Linux Steam logs for client `W.1.610.21`. A controlled October 1 screenshot pair verifies one explicitly local combat-damage globe record; a later three-image sequence supports the exact `WizardClientMod MSG_UpdateHealth` source for out-of-combat recovery. Health observations carry per-record attribution, and only supported local observations update GameState. Other globe sources and general freshness remain unverified. M3 presence was live-tested with Linux Steam and Discord; Windows runtime verification is deferred. See [milestone status](spec/status.md).

Zone Details and world State come directly from the pinned [WizRust101-DB](https://github.com/msork/WizRust101-DB) generated dataset. See the [mapping contract](spec/zone-world-mapping.md) and [full-dataset coverage audit](spec/presence-coverage.md). The checked-in Wizard101 Central export is a historical research artifact, not a runtime mapping source.

## License

WizRust101-RPC is licensed under the [MIT License](LICENSE).
