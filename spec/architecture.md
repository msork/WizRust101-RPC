# Architecture

## Design

Use a small Rust application with one-way data flow:

`installation discovery -> active log source -> incremental parser -> normalized game events -> game state -> presence model -> Discord IPC adapter`

The configuration loader supplies discovery overrides and display preferences. The mapping catalog is a separate versioned data asset and is injected into normalization/presence construction. Logging reports operational errors without copying complete game log lines or character values into routine logs.

## Module boundaries

- `discovery`: enumerate candidate installations/logs and select the active source.
- `log_reader`: follow append-only file growth; detect replacement, truncation, and rotation.
- `parser`: convert individual log records into typed events; no filesystem or Discord dependency.
- `mapping`: resolve game zone identifiers to verified location/world metadata.
- `state`: own current game/session/location/stat values and transition timestamps.
- `presence`: format normalized state into Discord activity fields and asset keys.
- `discord`: connect, publish, clear, retry, and report local IPC status.
- `config`: load defaults, validate user overrides, and persist supported settings.
- `app`: coordinate lifecycle and shutdown.

Prefer synchronous components until a concrete concurrency requirement exists. Keep OS-specific discovery and named-pipe behavior behind interfaces so pure logic remains portable to unit tests, while the product target remains Windows.

## Data ownership and error handling

- Parser events carry source evidence (event kind and parsed values), not arbitrary raw text.
- State owns freshness and transition policy; the parser does not decide presence presentation.
- Mapping data carries provenance and a stable zone identifier. Unknown IDs are explicit unknowns.
- Errors are typed at boundaries. Transient file/Discord errors retry with bounded backoff; invalid configuration is reported clearly and has safe defaults where possible.
- No network call is needed for normal operation. No self-updater or remote mapping fetch is part of initial architecture.

## Provisional technology choice

Use a maintained Rust Discord IPC crate behind the `discord` adapter rather than implementing the wire protocol directly. `discord-rich-presence` and `presenceforge` are research candidates; compare API fit, named-pipe support, reconnect behavior, release activity, and license during the Discord integration milestone. The initial choice is intentionally not frozen before dependency review.

## Non-goals for first release

GUI, game launching, memory/process-memory inspection, game modification, account authentication, remote telemetry, non-Windows support, and automatic data downloads.
