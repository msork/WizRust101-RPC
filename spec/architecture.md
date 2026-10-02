# Architecture

## Development ownership

WizRust101-RPC is fully vibe coded. Codex CLI owns research, architecture, implementation, refactoring, testing, and version-controlled spec maintenance under the milestone workflow in [README.md](README.md). The human supplies product decisions and verification data when required.

## Design

Use a small Rust application with one-way data flow:

`installation discovery -> active log source -> incremental parser -> normalized game events -> game state -> presence model -> Discord IPC adapter`

The configuration loader supplies discovery overrides and display preferences. The mapping catalog is a separate versioned data asset and is injected into normalization/presence construction. Logging reports operational errors without copying complete game log lines or character values into routine logs.

## Module boundaries

- `discovery`: enumerate candidate installations/logs and select the active source.
- `log_tailer`: follow append-only file growth; detect replacement, truncation, and rotation.
- `parser`: convert individual log records into typed events; no filesystem or Discord dependency.
- `mapping`: resolve game zone identifiers to verified location/world metadata.
- `state`: own current game/session/location/stat values and transition timestamps.
- `presence`: format normalized state into Discord activity fields and asset keys.
- `discord`: connect, publish, clear, retry, and report local IPC status.
- `config`: load defaults, validate user overrides, and persist supported settings.
- `app`: coordinate lifecycle and shutdown.

M1 created ingestion modules. M3 adds a pure `presence` builder, a `discord` transport/publisher boundary, and minimal environment-backed development settings. A persistent settings loader and UI remain later work.

Prefer synchronous components until a concrete concurrency requirement exists. Keep OS-specific discovery and named-pipe behavior behind interfaces so pure logic remains portable to unit tests, while the product target remains Windows.

## M1 implementation choices

- Use a library crate plus binaries so ingestion logic is testable without starting the watcher executable.
- Use `steamlocate` to enumerate Steam installations/libraries and locate Wizard101 app `799960`; also test the documented default Steam path and standalone `%PROGRAMDATA%` path. Discovery is read-only and validates the `Bin\\WizardClient.log` candidate.
- Implement the tailer with standard file I/O and byte offsets; it must read only appended bytes during normal operation, buffer incomplete UTF-8/line data, and reopen on truncation or replacement.
- Store zone mappings in versioned JSON. Include an importer for Bacon1661's legacy JSON object, which maps raw zone identifiers to location strings and has `zoneNames` plus `CHARACTER LIST` special entries. Imported rows retain legacy provenance; world identity is assigned only when the raw key prefix matches a known legacy world key.
- Keep M1 fixture and parser scope labeled as legacy-reference compatibility until a sanitized current game log capture confirms the exact current records.

## Data ownership and error handling

- Parser events carry source evidence (event kind and parsed values), not arbitrary raw text.
- Health observations carry `Local` or `Unknown` attribution. GameState accepts only `Local` health; matching numbers or absent remote markers do not promote an observation. A contradictory next-line marker downgrades a record to `Unknown`.
- State owns freshness and transition policy; the parser does not decide presence presentation.
- Mapping data carries provenance and a stable zone identifier. Unknown IDs are explicit unknowns.
- Errors are typed at boundaries. Transient file/Discord errors retry with bounded backoff; invalid configuration is reported clearly and has safe defaults where possible.
- No network call is needed for normal operation. No self-updater or remote mapping fetch is part of initial architecture.

## Discord technology choice

Use `discord-rich-presence` 1.1.0 behind the `discord` adapter. Keep activity construction pure and the transport replaceable. The publisher owns desired-state deduplication and bounded reconnect attempts; watcher errors cannot unwind through IPC.

## Non-goals for first release

GUI, game launching, memory/process-memory inspection, game modification, account authentication, remote telemetry, non-Windows support, and automatic data downloads.
