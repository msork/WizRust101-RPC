# Product requirements

## Goal

WizRust101-RPC is a Rust application that reads supported Wizard101 Steam client logs and publishes accurate game activity to the user's running Discord desktop client. The active scope is Steam installations only.

The project is fully vibe coded. Codex CLI performs research, spec maintenance, design, implementation, refactoring, and testing using spec-driven development. The human supplies product decisions and verification data when required, without manually programming the application.

## Required user experience

- Find supported Wizard101 Steam installations and their `Bin\WizardClient.log` files automatically, including non-default Steam libraries. Standalone Wizard101 discovery is deferred and is not an active supported source.
- Detect when Wizard101 is active and track the current location/world. Continue parsing locally attributable health internally, but do not publish it because the current log behavior can make the displayed value inconsistent.
- Show `Wizard101` as the Discord activity title, the mapped world's PNG as the large image, the current location in Details, the known world name in State, the uploaded project logo as the small image, and elapsed time for the continuous Wizard101 session.
- Keep one elapsed timestamp for a continuous Wizard101 session; zone changes and health updates do not reset it. Character selection clears the session, while RPC restart begins a fresh timer for a replay-restored session.
- Require no manual game-data entry during ordinary use. Manual path override may exist as recovery/configuration, not as the normal path.
- Display only values supported by observed game data or verified mapping sources. Unknown or stale values must remain unknown/omitted; never guess.
- Be modular, idiomatic Rust, with boundaries that allow parsing, mapping, state, and Discord transport to be tested independently.

## Stat scope

Discord presence does not have a stat selector. Health remains in the internal parser/GameState evidence path but is never published. Level, School, and Character Name are unsupported/excluded. Reconsider unsupported stats only if a future Wizard101 client exposes reliable evidence attributable to the selected local character. Never obtain these values by memory scanning, process injection, packet interception, OCR, or guessed/indirect inference.

## Quality requirements

- Initial intended platform targets are Linux Flatpak/AppImage for Steam Wizard101 through Proton, Windows installer/portable tray app for native Steam Wizard101, and macOS menu-bar app for Steam Wizard101 through CrossOver. Platform availability and support claims must follow the evidence matrix in `status.md`; an implemented or CI-built target is not automatically production-supported.
- Official releases use the project-owned Discord Application ID embedded at build/release time. Users never create an application or configure an ID. A runtime environment override may remain for development and testing; the ID is not a normal user setting.
- Linux Flatpak and AppImage and Windows installer/portable builds open as tray apps, begin Steam log discovery automatically, show useful watcher/Discord status, and offer Quit and additional Steam-library selection. No Cargo command or environment setup is part of ordinary use. Windows Start Menu launch and uninstall are provided by the installer; auto-start is not. The portable Windows ZIP contains the standalone executable.
- Tray status stays concise and user-facing: waiting for the game, watching the game, reconnecting Discord, or a short retry/selection result. Replay counts, log state, parser terms, raw zone IDs, and diagnostic details belong in application logs.
- Publish six actual ZIPs, each containing exactly one installation method and no Actions-generated wrapper or nested ZIP: `WizRust101-RPC-Windows-Setup.zip` contains the setup EXE; `WizRust101-RPC-Windows-Portable.zip` contains the portable EXE; `WizRust101-RPC-macOS-App.zip` contains the `.app` bundle; `WizRust101-RPC-macOS-Pkg.zip` contains the `.pkg`; `WizRust101-RPC-Linux-Flatpak.zip` contains the `.flatpak`; `WizRust101-RPC-Linux-AppImage.zip` contains the `.AppImage`. Release preparation must find successful platform workflow artifacts from the exact requested/dispatch SHA, validate every ZIP and expected payload, then stage all six unchanged with `SHA256SUMS.txt`.
- Linux Flatpak packaging uses Freedesktop Platform/SDK 26.08. It grants read-only access to the known native/Steam Flatpak Steam roots, uses a folder portal for other libraries, exposes the Discord IPC socket paths needed by supported Discord Desktop packaging, and does not grant home-wide or host-wide access. The locked Rust release binary is built on the host and inserted into the Flatpak, so packaging does not depend on the not-yet-published 26.08 Rust SDK extension.
- Wizard101 support is Steam-only throughout the initial three release targets. Each platform's actual Steam/log layout and compatibility behavior must be researched and tested when its milestone begins; do not assume Proton or CrossOver paths.
- Only after all three initial targets are complete and tested may standalone support be researched and implemented, in separate platform milestones: Chromebook Linux `.deb`, native Windows, then macOS using CrossOver/Wineskin or the best approach established by research. Research each platform at its milestone; do not assume paths or compatibility.
- Absence of the game or Discord must not crash the app. Discovery and IPC failures must be diagnosable and recoverable.
- Log reading is read-only. The app must not modify Wizard101 files or launch the game.
- Avoid uploading log contents or character details to any service. The only outbound activity connection is to the local Discord client IPC.
- Unknown data, menu/loading states, malformed records, and log rotation/truncation are expected operating conditions.

## Acceptance criteria for the first functional release

1. With a supported Steam Wizard101 installation and Discord desktop client running, the app discovers the active log without asking for a path.
2. A captured, documented log fixture produces the expected world and location state. Health may be parsed internally but is never required in published presence.
3. Presence contains the title, DB-provided location/world semantics, matching world image when registered or generic art otherwise, project-logo small image, and a continuous session timestamp that does not reset on zone changes.
4. Unknown locations do not display a guessed world or image.
5. Disconnects, missing logs, and malformed/new log lines are handled without a crash; the app recovers when the source returns.
6. Offline tests cover discovery candidates, parser fixtures, state transitions, mapping, configuration validation, and presence construction.
