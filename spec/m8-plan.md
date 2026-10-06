# M8 plan: native macOS menu-bar app for Steam Wizard101 through CrossOver

## Goal and scope

Build WizRust101-RPC as a native macOS menu-bar app for macOS 15 and later, Apple Silicon first. Wizard101 is supported only through Steam installed in a CrossOver bottle. WizRust101-RPC itself is a native macOS process; it must never be hosted in CrossOver. Reuse the parser, typed GameState, startup replay, presence builder, Discord IPC adapter, and bounded reconnect publisher. Do not start M9 or add unverified mappings.

The menu must expose a useful watcher status, selection of an additional CrossOver bottle when discovery needs it, and Quit. The app must start without terminal commands or user environment variables. Official builds embed the project Discord Application ID at build time.

## Research and decisions (2026-10-04)

### CrossOver and Steam discovery

- CodeWeavers documents private bottles under `~/Library/Application Support/CrossOver/Bottles` by default. CrossOver 24+ lets users change the bottle folder in Settings; the `BottleDir` preference and `CX_BOTTLE_PATH` configuration can relocate or add bottle roots. The app will read supported macOS preferences for bottle directories and also include default private and system managed/published bottle locations. It will not depend on CrossOver's own environment variables being inherited by the RPC process.
- CrossOver bottles contain an isolated Windows `C:` drive and `dosdevices` mappings for Windows drive letters. Discovery searches conventional Steam install locations inside each bottle, reads that Steam installation's `steamapps/libraryfolders.vdf`, maps drive-qualified library paths through the selected bottle's `dosdevices`, and resolves app `799960` through `appmanifest_799960.acf` and its `installdir`. The final log is that install directory plus `Bin/WizardClient.log`. No Windows host, Linux Proton, or native macOS Steam path is substituted for this metadata flow.
- Users may add a bottle through the macOS folder chooser. Selection is validated against a readable Steam install and Wizard101 manifest before its root is persisted under the app's Application Support directory. Discovery is read-only; the app does not launch or modify CrossOver, Steam, or Wizard101.
- These paths are research-backed candidates, not yet live-verified with a macOS CrossOver Steam installation. CrossOver versions, custom bottle locations, mapped external libraries, log permissions, and actual Wizard101 log format/path must be verified during macOS acceptance. Unknown layouts must report a useful tray status rather than guess.

### Discord IPC

Discord documents local IPC sockets on macOS using `XDG_RUNTIME_DIR`, `TMPDIR`, `TMP`, `TEMP`, then `/tmp`, and socket names `discord-ipc-0` through `discord-ipc-9`. `discord-rich-presence` 1.1.0's Unix transport searches these environment/temp locations and retries through the shared publisher. M8 will reuse that transport and make no network connection. Live Discord readiness, permissions, reconnect after client restart, and Apple Silicon behavior still require native acceptance.

### Native app, architecture, package, and permissions

- `tray-icon` 0.26.0 supports macOS status items, but its upstream requires its event loop and status-item creation on the main thread. Use the existing `winit` event-loop approach, with a macOS-specific adapter and an accessory/menu-bar-only app bundle (`LSUIElement`); do not expose a Dock app window.
- Build and test `aarch64-apple-darwin` first. Rust supports separate Apple Silicon and Intel macOS targets; a Universal 2 executable can be assembled from both slices with Apple's `lipo` workflow. All native dependencies must provide both architectures and both need runtime validation. The macOS CI compiles both slices on Apple Silicon and publishes a Universal 2 app bundle; Apple Silicon remains the initial supported Mac platform. Keep Universal 2 packaging while Intel macOS support is in scope, then switch CI/release packaging to arm64-only once Intel is completely unsupported.
- Package a normal `WizRust101-RPC.app` bundle with `Contents/MacOS`, `Contents/Resources`, `Info.plist`, and an `.icns` app icon. A build script stages the native executable and icon; developers can run unsigned local builds. For public download, sign code with Developer ID Application, enable Hardened Runtime, sign a `.pkg` installer with Developer ID Installer if distributing a package, and notarize/staple the distributed outer artifact. The expected direct-install target is `/Applications/WizRust101-RPC.app`; app/package identifiers and upgrade behavior will be tested before release.
- GitHub Actions uses a macOS 15 Apple Silicon (`macos-15`) runner. The official Discord ID is read from the existing `WIZRUST101_RELEASE_DISCORD_APP_ID` Actions secret only on non-PR package jobs; pull requests still run native tests and warning-denied Clippy without secrets. The package job produces one Universal 2 artifact set containing arm64 and x86_64 slices, then inspects the actual app ZIP and expanded installer. A headless `--ci-load-check` verifies the Mach-O can load and that it contains the exact build secret before the tray UI is started. CI output is unsigned and is not notarized. Continue Universal 2 while Intel macOS support remains in scope; switch to arm64-only once Intel is completely unsupported.
- Do not enable App Sandbox or request unrelated TCC permissions for the first release. Scan documented bottle roots read-only. The native folder chooser is the user-authorized route for a custom bottle root. Do not request Accessibility, Automation, Full Disk Access, or game/process inspection. Whether macOS 15 protects any selected CrossOver/external-volume path in practice is a live acceptance question.

Primary research sources:

- [CodeWeavers: Change Where CrossOver Stores Bottles](https://support.codeweavers.com/en_US/change-the-bottle-directory-in-crossover)
- [CodeWeavers: CrossOver Mac User Guide](https://support.codeweavers.com/crossover-mac-user-guide?kb_language=en_US)
- [CodeWeavers: Installing Steam in CrossOver](https://support.codeweavers.com/installing-steam-in-crossover?kb_language=en_US)
- [Discord RPC IPC paths](https://discord.com/developers/docs/topics/rpc)
- [`tray-icon` macOS main-thread/event-loop notes](https://github.com/tauri-apps/tray-icon)
- [CrossOver system requirements](https://www.codeweavers.com/crossover/)
- [Apple: Building a universal macOS binary](https://developer.apple.com/documentation/apple-silicon/building-a-universal-macos-binary)
- [Apple: `LSUIElement` agent-app key](https://developer.apple.com/documentation/bundleresources/information-property-list/lsuielement)
- [Apple: notarizing software before distribution](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
- [Apple: packaging macOS software for distribution](https://developer.apple.com/documentation/xcode/packaging-mac-software-for-distribution)

WizRust101-DB remains the preferred location display-name candidate source. Consult exact canonical paths and diagnostics confidence; it does not prove a world relationship. Unknown names/worlds stay unresolved.

## Acceptance gates

- Native Apple Silicon build on macOS 15+, native menu-bar-only launch, tray/menu status, add-bottle chooser, persistence, and Quit.
- Discover Wizard101 from an existing CrossOver Steam bottle and resolve the selected Steam library/app manifest/log using actual host and `dosdevices` mappings. Validate custom bottle roots and any supported external library case; do not inspect outside selected/documented roots.
- Start while already in a verified mapped zone and restore presence from existing log history without a new zone event; verify selection clears state, zone/timer updates, and Discord reconnect.
- Verify native Discord IPC and the exact Details/State/art/timer payload; no Health or guessed mappings.
- Build and exercise the unsigned `.app` locally. Before public release, verify Developer ID signatures, Hardened Runtime, notarization/stapling and package install/upgrade/removal. No public release is part of implementation acceptance.
- Run portable unit/integration tests, formatting, warning-denied Clippy, native Apple Silicon checks and Universal 2 target build/package checks on macOS CI, then owner live acceptance on macOS 15+ Apple Silicon. Linux M6 and Windows M7 regressions must continue passing.
- Before release, add Developer ID Application and Installer signing, hardened runtime validation, notarization and stapling; current CI artifacts are unsigned development builds. Test install/upgrade/removal and Gatekeeper behavior on supported macOS.

## Status

M8 research and implementation are in place: host-side CrossOver bottle/Steam discovery, drive-letter path translation, validated/persisted bottle selection, an accessory-mode macOS menu-bar adapter, an `.app`/`.pkg` build script, and GitHub Actions for native macOS tests, warning-denied Clippy, a Universal 2 build, artifact validation, and unsigned artifact uploads. Hosted CI results are being recorded in the milestone status. A real CrossOver bottle and Discord client on macOS 15+ are still required to close live acceptance; Developer ID signing/notarization remain release gates. See [status](status.md#m8-native-macos-menu-bar-app-with-crossover-hosted-steam-wizard101).
