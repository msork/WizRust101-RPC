# Product requirements

## Goal

WizRust101-RPC is a Windows Rust application that automatically reads Wizard101's local game logs and publishes accurate game activity to the user's running Discord desktop client.

## Required user experience

- Find supported Wizard101 installations and their `Bin\WizardClient.log` files automatically, including Steam installations and non-default Steam libraries.
- Detect when Wizard101 is active and track the current location/world and the local character's health when the log provides it.
- Show `Wizard101` as the Discord activity title, the mapped world's PNG as the large image, the current location, a configurable stat, and elapsed time since entering the current location.
- Reset the location timer when the resolved location changes. Do not reset it for health/stat updates or duplicate log events.
- Require no manual game-data entry during ordinary use. Manual path override may exist as recovery/configuration, not as the normal path.
- Display only values supported by observed game data or verified mapping sources. Unknown or stale values must remain unknown/omitted; never guess.
- Be modular, idiomatic Rust, with boundaries that allow parsing, mapping, state, and Discord transport to be tested independently.

## Stat choices

The initial data model must support the selectable stat enum and renderer. Health is the only product-required stat for the first functional milestone, subject to validation against current logs. Level, school, character name, and other candidate values remain unavailable until their source and parser are verified. An unavailable selected stat renders as unavailable or is omitted according to the presence spec; it is never fabricated.

## Quality requirements

- Windows is the initial supported OS.
- Absence of the game or Discord must not crash the app. Discovery and IPC failures must be diagnosable and recoverable.
- Log reading is read-only. The app must not modify Wizard101 files or launch the game.
- Avoid uploading log contents or character details to any service. The only outbound activity connection is to the local Discord client IPC.
- Unknown data, menu/loading states, malformed records, and log rotation/truncation are expected operating conditions.

## Acceptance criteria for the first functional release

1. With a supported Wizard101 installation and Discord desktop client running, the app discovers the active log without asking for a path.
2. A captured, documented log fixture produces the expected world, location, and health state.
3. Presence contains the title, location, world image key when known, selected stat when known, and a timestamp that resets on a location change.
4. Unknown locations do not display a guessed world or image.
5. Disconnects, missing logs, and malformed/new log lines are handled without a crash; the app recovers when the source returns.
6. Offline tests cover discovery candidates, parser fixtures, state transitions, mapping, configuration validation, and presence construction.
