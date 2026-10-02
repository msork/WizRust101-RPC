# Product requirements

## Goal

WizRust101-RPC is a Rust application that reads supported Wizard101 Steam client logs and publishes accurate game activity to the user's running Discord desktop client. The active scope is Steam installations only.

The project is fully vibe coded. Codex CLI performs research, spec maintenance, design, implementation, refactoring, and testing using spec-driven development. The human supplies product decisions and verification data when required, without manually programming the application.

## Required user experience

- Find supported Wizard101 Steam installations and their `Bin\WizardClient.log` files automatically, including non-default Steam libraries. Standalone Wizard101 discovery is deferred and is not an active supported source.
- Detect when Wizard101 is active and track the current location/world and the local character's health when the log provides it.
- Show `Wizard101` as the Discord activity title, the mapped world's PNG as the large image, the current location, a configurable stat, and elapsed time since entering the current location.
- Reset the location timer when the resolved location changes. Do not reset it for health/stat updates or duplicate log events.
- Require no manual game-data entry during ordinary use. Manual path override may exist as recovery/configuration, not as the normal path.
- Display only values supported by observed game data or verified mapping sources. Unknown or stale values must remain unknown/omitted; never guess.
- Be modular, idiomatic Rust, with boundaries that allow parsing, mapping, state, and Discord transport to be tested independently.

## Stat choices

The data model supports Health (default) and `none` (omit the stat). Health is the only stat supported by the currently verified log-only approach. Level and School are unsupported; Character Name is excluded. Reconsider these only if a future Wizard101 client exposes reliable evidence attributable to the selected local character. Never obtain these values by memory scanning, process injection, packet interception, OCR, or guessed/indirect inference. An unavailable value is omitted, never fabricated.

## Quality requirements

- Initial official release targets, in order: Linux Flatpak tray app for Steam Wizard101 through Proton; Windows installer/setup tray app for native Steam Wizard101; macOS packaged menu-bar app for Steam Wizard101 through CrossOver.
- Official releases use the project-owned Discord Application ID embedded at build/release time. Users never create an application or configure an ID. A runtime environment override may remain for development and testing; the ID is not a normal user setting.
- Wizard101 support is Steam-only throughout the initial three release targets. Each platform's actual Steam/log layout and compatibility behavior must be researched and tested when its milestone begins; do not assume Proton or CrossOver paths.
- Only after all three initial targets are complete and tested may standalone support be researched and implemented, in separate platform milestones: Chromebook Linux `.deb`, native Windows, then macOS using CrossOver/Wineskin or the best approach established by research. Research each platform at its milestone; do not assume paths or compatibility.
- Absence of the game or Discord must not crash the app. Discovery and IPC failures must be diagnosable and recoverable.
- Log reading is read-only. The app must not modify Wizard101 files or launch the game.
- Avoid uploading log contents or character details to any service. The only outbound activity connection is to the local Discord client IPC.
- Unknown data, menu/loading states, malformed records, and log rotation/truncation are expected operating conditions.

## Acceptance criteria for the first functional release

1. With a supported Steam Wizard101 installation and Discord desktop client running, the app discovers the active log without asking for a path.
2. A captured, documented log fixture produces the expected world, location, and health state.
3. Presence contains the title, location, world image key when known, selected stat when known, and a timestamp that resets on a location change.
4. Unknown locations do not display a guessed world or image.
5. Disconnects, missing logs, and malformed/new log lines are handled without a crash; the app recovers when the source returns.
6. Offline tests cover discovery candidates, parser fixtures, state transitions, mapping, configuration validation, and presence construction.
