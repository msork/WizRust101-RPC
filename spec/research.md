# Research notes and decisions

Research checked 2026-10-01. Links are primary project/documentation sources where available; upstream project behavior is evidence about prior implementations, not proof of current game behavior.

## Reference projects

- [Bacon1661/Wizard101-RPC](https://github.com/Bacon1661/Wizard101-RPC): Windows console app; reads health and zone data from `WizardClient.log`; documents standalone `%PROGRAMDATA%` and default Steam paths. Its source looks for `zone = ...` / `CHARACTER LIST`, parses `Updating health globe (...)`, excludes some other-player health records, and uses `GameClient::HandleQuit()` / away-from-keyboard logout as quit signals. It has an extensive `zones.json` catalog. Its current implementation falls back to `WizardCity` for an unrecognized world and starts elapsed time from a location transition; our accuracy policy rejects the fallback. The implementation can launch the game and uses an old Node Discord IPC library; neither behavior is carried forward.
- [ManaUp/WizRPC](https://github.com/ManaUp/WizRPC): archived October 25, 2021. Node app requiring the user to configure the install path; describes live location/stats and updates presence every 15–25 seconds. It contains `zones.csv`/`zones.txt` mapping assets and planned zone API, GUI, and Linux support. Treat these catalogs as candidate historical data, not authoritative current coverage.

## Discord

- [Discord RPC documentation](https://discord.com/developers/docs/topics/rpc): Rich Presence can be set over IPC to a running local Discord client. The IPC transport supports Windows named pipes (`\\?\pipe\discord-ipc-{n}`) and command `SET_ACTIVITY`. Current docs recommend Discord Social SDK for new integrations embedded in games; this project is a companion application, so a local IPC client crate is the matching implementation shape to evaluate.
- [Discord Rich Presence overview](https://discord.com/developers/docs/rich-presence/overview): activities support text, timestamps, and uploaded art assets. Discord displays the registered application name as the title; asset keys must refer to application assets.
- [Discord RPC source repository](https://github.com/discord/discord-rpc): documents connection, reconnection, and protocol handling tradeoffs for direct IPC implementations.

## Rust IPC candidates

- [`discord-rich-presence` on docs.rs](https://docs.rs/discord-rich-presence/latest/discord_rich_presence/): documented synchronous Rust API over Discord IPC with Windows and Unix implementations; supports connecting and `set_activity`.
- [`presenceforge` on docs.rs](https://docs.rs/presenceforge/latest/presenceforge/): documented Windows named-pipe and Unix socket transport, sync/async APIs, activity builder, and clear activity. Newer candidate with a larger runtime/API surface.
- Decision: no crate is selected in this spec-only milestone. Select one during the IPC implementation milestone after reviewing source/release history, license, compatibility, and integration testability. Do not implement raw Discord IPC unless candidate review establishes a concrete need.

## Confidence and unverified facts

- Historical logs demonstrated zone and health signals, but current Wizard101 log versions and exact local-player filtering are unverified.
- Current location catalogs and world assets are unverified for completeness, accuracy, permission, and Discord application ownership.
- Non-health stats have no verified source in the research performed for this milestone.
- Steam's additional-library discovery and installation metadata need implementation-time validation on Windows.

## Decisions captured

| Decision | Status | Reason |
| --- | --- | --- |
| Windows first | Accepted | Product requirement and reference implementations target Windows. |
| Read logs; do not inspect game memory | Accepted | Meets automatic tracking requirement with a low-impact local source and is testable. |
| Unknown world stays unknown | Accepted | Product requirement forbids invented game data. |
| IPC crate behind adapter | Accepted, crate TBD | Discord documents local IPC; isolates protocol choices. |
| Automatic discovery is default; explicit path override is recovery | Accepted | Satisfies no-manual-entry requirement while allowing diagnosis. |
