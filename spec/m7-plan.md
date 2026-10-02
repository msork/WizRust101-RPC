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
- Complete Linux-hosted tests and Windows-target compilation. Windows installer build and live Windows/Steam/Discord acceptance remain explicit gates because no Windows session is available.

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

**Owner-reported packaged acceptance mostly passed; restart state restoration is now the M7 fix milestone.** The owner confirms the app starts, tray works, native Steam and Discord integrate, Stone Town presence is correct, Discord reconnect works, and Quit works. A new startup defect remains: after quitting in Stone Town, relaunching without changing zones does not restore Stone Town/Zafaria until a later zone transition.

### Startup restoration requirements and plan (2026-10-02)

- At each newly discovered log session, open the incremental tailer at its current end offset, then stream the already-existing bytes from offset zero through that captured boundary into the normal parser and typed state. This reconstructs the latest state while the tailer remains positioned to read only subsequently appended bytes. Do not hold the entire file in memory.
- Replay state transitions in original order, including character-selection boundaries and unknown zone events. Publish only the final state produced by the existing verified mapping catalog; a later unknown zone must not leave an older mapped zone visible.
- A restart resets the elapsed location timer to application startup time. The log records do not provide a sufficiently verified wall-clock entry time for portable timer restoration; do not infer it from log text.
- If the file is truncated/replaced after the startup snapshot, retain existing generation behavior: clear parser/game state and consume the new file from its beginning. If the log is unavailable, preserve discovery retry behavior.
- Reproduction fixture: existing history ends with the captured W.1.610.21 `Zafaria/ZF_Z07_Stone_Town` entry; a fresh state is replayed and immediately produces the same verified Stone Town/Zafaria Discord payload before any append. Also verify selection clears previous location, later unknown zone does not fall back to Stone Town, and appended lines are still consumed once.

Plan: 1) update these requirements and record the owner acceptance result; 2) research RavenDex's published zone reference against the current catalog without bulk-import; 3) implement streaming startup replay behind a testable module, preserving the incremental tailer's initial end offset; 4) add replay, restart-payload, unknown-zone, and timer-reset tests; 5) run standard checks and update status. No new mapping is promoted unless independent evidence meets the existing provenance policy.

### RavenDex comparison (2026-10-02)

RavenDex is a public Go Wizard101 Rich Presence project; its repository is [MeisterSchwarz/RavenDex](https://github.com/MeisterSchwarz/RavenDex). The inspected `i18n/de/zones/Zafaria.json` maps `Zafaria/ZF_Z07_Stone_Town` to `Steinstadt` (German for Stone Town) and groups several interior IDs under that readable zone. This matches the one existing Stone Town raw ID/name pair as a reference candidate. That row was already verified by the current-client raw ID and project-owner manual confirmation; RavenDex is corroboration only and does not change its provenance or verification status. No other RavenDex rows are promoted or bulk-imported. The repository's English zone directory is empty at the inspected revision, so its German catalog was used only to compare the unambiguous raw ID and translated name.

The owner must repeat Windows restart acceptance after this correction before M7 closes. Required live sequence: enter Stone Town, confirm presence, Quit from tray, relaunch without moving zones, and confirm Stone Town/Zafaria/art/timer return; then change zones and confirm Details/State/art update and timer reset. Also recheck app startup, tray status, Discord reconnect, and Quit. No Windows runtime success is claimed until reported.

Previous first-start failure and its manifest correction remain recorded below. The Windows package's embedded Common Controls v6 manifest validation and loader probe remain mandatory.

The fix adds `embed-manifest` to the Windows build script, which links a manifest declaring `Microsoft.Windows.Common-Controls` version `6.0.0.0` into the executable. Windows packaging now extracts RT_MANIFEST resource #1 using Windows SDK `mt.exe`, checks the dependency identity/version, then starts the actual EXE with `--ci-load-check`; this exits before starting watcher/tray, after the Windows loader resolves static imports. A failed manifest check, process load, nonzero exit, or timeout stops packaging before installer generation. Cross-target GNU build-script output is confirmed to include an x64 COFF `.rsrc` object. The owner screenshot is recorded as evidence but not committed.

The owner has since reported a successful corrected-package retest for ordinary startup, tray, native Steam/Discord, Stone Town presence, reconnect, and Quit. The startup-replay code and offline checks are complete, but its Windows runtime behavior is not yet verified. Rebuild and install the new artifact; M7 remains open until stationary relaunch restores presence.
