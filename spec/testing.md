# Testing strategy

## Required test layers

- Unit tests for parser records and malformed/partial input, mapping lookup/unknown behavior, state transitions and timer reset semantics, configuration validation, and presence payload formatting.
- Fixture tests with sanitized real `WizardClient.log` snippets labeled with game build/source date. If no real capture is available, minimal reference-derived fixtures may test only exact documented legacy patterns and must be labeled as reconstructed; they cannot establish current-build support or local-player health ownership.
- Filesystem integration tests using temporary directory trees for standalone/Steam paths, multiple Steam library roots, running-log changes, truncation, and replacement.
- Discord adapter tests using a fake transport for connect/disconnect/retry, set/clear behavior, payload deduplication, and error handling.
- Windows manual verification for real named-pipe connection, real game log discovery, and Discord-rendered world image/timestamp. These require actual Windows, game, Discord client, registered app, and assets.

## Milestone gates

Before each milestone, state which checks apply and what evidence closes them. At minimum, run formatting, linting, unit and fixture tests, and a Windows build for code milestones; add integration/manual checks as the feature requires. Do not consider an untested zone, parser pattern, stat, or asset mapping supported.

## Test data privacy

Fixtures must be minimized and sanitized. Do not commit full user logs, account identifiers, or personal character names without consent. Prefer a short record excerpt sufficient to prove the parser behavior.

## M1 gates

- `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` must pass.
- Unit coverage must include recognized and malformed zone/health records, unknown zones, partial lines, duplicate location events, and state preservation on invalid data.
- Integration coverage must prove normal-path and Steam-library discovery using temporary trees, and tailer behavior for append, partial-line completion, truncation, and replacement without rereading prior bytes.
- Current game-build parsing remains unverified until a sanitized user capture is tested. Windows-only registry/Steam and real-game discovery still require manual verification on Windows.
