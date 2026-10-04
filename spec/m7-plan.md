# M7 plan: native Windows Steam tray app and installer

## Goal

Deliver the second official target: a Windows-native tray app for Wizard101 installed through Steam, distributed in a normal setup installer. Steam is the only active game source. Preserve M6's verified Discord payload and do not add auto-start.

The project is fully vibe coded. Codex CLI performs research, planning, implementation, testing, and spec maintenance through spec-driven development; the owner supplies product decisions and verification data when needed.

## Scope and acceptance criteria

- Discover Steam through the maintained `steamlocate` Windows registry lookup, standard `%ProgramFiles(x86)%\Steam` fallback, and all libraries listed in `steamapps/libraryfolders.vdf`.
- Resolve the Wizard101 Steam app `799960` manifest's `installdir`; watch only `Bin\WizardClient.log`. Do not add standalone discovery.
- Reuse the incremental tailer, typed parser/state/mapping, M6 presence builder, Discord publisher/retry behavior, and user config. Official installer builds embed the project Discord Application ID at build time; installed users need no environment variable or Discord Developer Portal setup.
- Add a Windows-specific tray adapter isolated from `src/tray/linux.rs`, with useful watcher/Discord status, **Add Steam library…**, and **Quit**. Selecting a folder validates that it is a Steam library containing the Wizard101 app manifest, then persists it in the existing versioned per-user library registry. No elevation is needed for app execution; no auto-start is added.
- Preserve M6 presence exactly: Details is verified location, State is verified world, registered world large image, `wizrust101_rpc` project-logo small image with `WizRust101-RPC` hover text, and elapsed verified-location timer. Health remains internal and is never sent.
- Produce a 64-bit Windows setup installer that installs the executable and icon, adds Start Menu launch/uninstall shortcuts, and registers a normal Windows uninstaller. Do not add a desktop shortcut, auto-start, or game-launch action.
- Embed the Microsoft.Windows.Common-Controls v6 dependency manifest in the actual release executable. Before installer creation/publication, extract and validate RT_MANIFEST resource #1 and launch the EXE in a side-effect-free loader probe.
- Complete the actual MSVC-hosted Rust tests, Windows EXE manifest/load probe, packaging checks, and live Windows/Steam/Discord acceptance. Cross-target compilation alone does not close these gates.

## Research and choices (2026-10-02)

- `steamlocate` 2.1.1 is already used in the app. Its Windows source reads the Steam install path from `HKLM\SOFTWARE\Wow6432Node\Valve\Steam` and then `HKLM\SOFTWARE\Valve\Steam`; its Steam library parser reads `steamapps/libraryfolders.vdf`. The project adds a `%ProgramFiles(x86)%\Steam` fallback and tests library manifest traversal. These are discovery candidates, not live Windows evidence.
- The same dependency resolves each Steam library's `appmanifest_799960.acf` and `installdir`. The app constructs the relative log path only from that manifest value. Missing/inaccessible libraries are skipped; no recursive drive scan or standalone path is allowed.
- `discord-rich-presence` 1.1.0's Windows transport tries named pipes `\\?\pipe\discord-ipc-0` through `discord-ipc-9`. Keep the shared retry publisher; this is source-level evidence, not live Windows IPC validation.
- Selected maintained `tray-icon` 0.26.0 (MIT/Apache-2.0; current upstream release on 2026-09-30) with `winit` 0.30.13 as the native event loop, and `rfd` 0.17.2 with native Windows folder dialog and no Linux backend features. The tray crate requires a running event loop on Windows. Windows dependencies are target-specific; Linux keeps its M6 `ksni` adapter. `tray-icon` 0.26.0 raises the workspace MSRV to Rust 1.90.
- Selected Inno Setup 6.7.3 for the initial installer, using its signed official release and script format. It supports installation file declarations, Start Menu shortcuts, and a registered uninstaller. The project uses it on the assumption that release/build use is non-commercial; Inno Setup's current terms request a paid license for commercial use. Revisit licensing if the project is distributed commercially. The installer is per-user under Local AppData, requires no elevation, and does not create an auto-start entry. The workflow downloads the immutable official 6.7.3 release directly rather than relying on a package mirror.
- Use the existing project icon artwork to create a Windows `.ico`, embed the PNG for the tray, and install the `.ico` beside the executable for Start Menu/uninstall display.

Research sources:

- [`steamlocate::locate_all`](https://docs.rs/steamlocate/2.1.1/steamlocate/fn.locate_all.html) and [`SteamDir`](https://docs.rs/steamlocate/2.1.1/steamlocate/struct.SteamDir.html)
- [`steamlocate` Windows registry source](https://docs.rs/crate/steamlocate/2.1.1/source/src/locate/windows.rs) and [library VDF parser source](https://docs.rs/crate/steamlocate/2.1.1/source/src/library.rs)
- [`discord-rich-presence` Windows IPC source](https://docs.rs/crate/discord-rich-presence/1.1.0/source/src/ipc_windows.rs)
- [`tray-icon` v0.26.0 release](https://github.com/tauri-apps/tray-icon/releases/tag/tray-icon-v0.26.0), [API docs](https://docs.rs/tray-icon/0.26.0/tray_icon/), and [`winit` 0.30.13 docs](https://docs.rs/winit/0.30.13/winit/)
- [`rfd` 0.17.2 docs](https://docs.rs/rfd/0.17.2/rfd/)
- [`rfd` source explaining its TaskDialogIndirect/Common Controls v6 requirement](https://docs.rs/crate/rfd/0.17.2/source/src/lib.rs), [`muda` source documenting its v6 About-dialog import](https://docs.rs/crate/muda/0.21.0/source/README.md), and [Microsoft's Common Controls activation manifest example](https://learn.microsoft.com/en-us/windows-hardware/drivers/taef/activation-context)
- [`embed-manifest` 1.5.1 docs](https://docs.rs/embed-manifest/1.5.1/embed_manifest/), which documents linking a generated manifest into both MSVC and GNU Windows executables
- [Official Inno Setup 6.7.3 download and licensing notes](https://jrsoftware.org/isdl.php), [Inno Setup script help](https://jrsoftware.org/ishelp/), [commercial license FAQ](https://jrsoftware.org/isorder.php)

## Plan before implementation

1. Record M6's final visual acceptance and update product, architecture, discovery, testing, roadmap, README, and status specs for M7's exact supported scope, choices, and unverified Windows acceptance.
2. Add isolated Windows tray/event-loop and native folder-selection code; reuse the watcher and Steam library registry.
3. Add Windows tray/discovery regression tests and Windows-target compile checks without claiming they replace live Windows tests.
4. Add icon resources, Inno Setup script, and Windows build/package helper with build-only app-ID injection; no committed app ID or auto-start.
5. Run fmt, tests, warning-denied Clippy, Windows-target check, static installer/script checks, and diff checks; update M7 status and preserve M6 behavior.

## Status

**M7 passed Windows 11 live acceptance on 2026-10-04.** The installed Inno Setup package was tested with native Steam Wizard101 and native Discord. The owner verified stationary restart restoration in Stone Town, Discord reconnect, full payload artwork/timer, and tray Quit. See the final acceptance record in [status](status.md#m7-final-windows-11-live-acceptance-2026-10-04).

### Startup replay flow and observed cause (2026-10-04)

- The watcher discovers a Steam log, opens the incremental tailer at its current end, replays the bounded existing prefix through the ordinary parser and `GameState`, then on its first poll builds `Presence` and calls `PresencePublisher::tick`. Publisher retry retains the desired snapshot and republishes it after Discord reconnects.
- The Windows Steam log available during this investigation contains a Stone Town zone event followed later by `CHARACTER LIST`. The parser treats that later record as character selection and `GameState` clears the location. This produces exactly the reported tray status; `PresencePublisher` receives `None`, so its connection retry path is not reached with a desired activity. Do not restore an older zone across a later selection event.
- Added tray diagnostics that distinguish a replay ending at character selection, an unmapped zone, and no recognized zone record. Startup replay diagnostics also count complete lines and zone, selection, and health events without logging raw lines or personal identifiers.
- Added a pipeline regression that starts from a log containing a selection boundary followed by verified Stone Town, produces no appended line, fails its first Discord connect, then publishes the retained Stone Town/Zafaria payload on retry. The existing selection-boundary regression ensures a later selection still clears prior location.
- The Windows screenshot/log evidence shows a mismatch between the reported in-world state and the log's final selection event. M7 acceptance must confirm the live game is in-world and that its selected Steam log has a final recognized zone event after any selection record. If the new tray diagnostic still reports selection while the client is visibly in-world, capture a short sanitized sequence of the last `CHARACTER LIST` and `zone =` records plus timestamps; do not weaken selection clearing without that evidence.

### Startup restoration requirements

- At each newly discovered log session, capture the tailer's end offset, then stream the existing prefix through normal parser/state transitions. Preserve original event order, character-selection boundaries, and unknown zones. Do not read the whole file into memory.
- Reset elapsed time to application startup when a verified location is restored; the log does not provide a trusted cross-restart location-entry time.
- If a log is truncated or replaced after the snapshot, reset state and consume the new generation from its beginning. Windows replacement identity uses the file handle's volume/file ID when available.
- The diagnostic tray status should separate absent/unknown state from Discord reconnecting so support can locate failures along the startup flow.

### Preferred mapping source

Use [WizRust101-DB](https://github.com/msork/WizRust101-DB) for readable-name candidates, following the confidence and exact-path rules in [zone-world-mapping.md](zone-world-mapping.md). Its current output maps `Zafaria/ZF_Z07_Stone_Town` to `Stone Town`; no additional rows or world relationships were imported.

### Windows 11 live acceptance (passed 2026-10-04)

The owner reports that the native MSVC release and Inno Setup package install and launch normally with native Steam and Discord. While already in Stone Town, restarting WizRust101-RPC restores activity without a new zone event. Restarting Discord also restores activity while stationary. Discord displays Stone Town in Details, Zafaria in State with Zafaria artwork, the project logo small image, and a fresh timer. Tray status shows `Watching Wizard101 (Steam)` and tray Quit works. Owner-provided screenshots are intentionally not committed because they contain personal Discord information. The native Windows automated gates and manifest/load probe are recorded in [status](status.md#m7-native-windows-diagnosis-and-verification-2026-10-04).

The initial `TaskDialogIndirect` manifest failure and its correction are recorded below. The package verification must continue extracting RT_MANIFEST resource #1 and launching the built EXE with `--ci-load-check` before Inno Setup runs.

The user supplied a screenshot link, then provided a local copy after the image host could not be opened. The screenshot itself is not committed.

### Common Controls v6 manifest correction (2026-10-02)

The first installed Windows package failed before tray creation with `TaskDialogIndirect` missing. The `rfd` and `muda` Common Controls v6 features import this API; Windows requires the `Microsoft.Windows.Common-Controls` 6.0 activation manifest. Added `build.rs` with `embed-manifest` and a packaging gate that extracts resource #1 with `mt.exe`, validates version `6.0.0.0`, then starts the actual executable in `--ci-load-check` mode. This is a loader check, not a tray, game, Discord, installer, or uninstall acceptance test.
