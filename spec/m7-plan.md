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
- [Official Inno Setup 6.7.3 download and licensing notes](https://jrsoftware.org/isdl.php), [Inno Setup script help](https://jrsoftware.org/ishelp/), [commercial license FAQ](https://jrsoftware.org/isorder.php)

## Plan before implementation

1. Record M6's final visual acceptance and update product, architecture, discovery, testing, roadmap, README, and status specs for M7's exact supported scope, choices, and unverified Windows acceptance.
2. Add isolated Windows tray/event-loop and native folder-selection code; reuse the watcher and Steam library registry.
3. Add Windows tray/discovery regression tests and Windows-target compile checks without claiming they replace live Windows tests.
4. Add icon resources, Inno Setup script, and Windows build/package helper with build-only app-ID injection; no committed app ID or auto-start.
5. Run fmt, tests, warning-denied Clippy, Windows-target check, static installer/script checks, and diff checks; update M7 status and preserve M6 behavior.

## Status

**Implementation complete pending validation and Windows acceptance.** The Windows tray module, Steam discovery, library picker, Inno Setup source, and packaging workflow are implemented. Native Windows runtime, installed setup, named-pipe IPC, tray interaction, and log discovery cannot be live-verified in this environment. Leave exact owner acceptance steps for the M7 closeout.
