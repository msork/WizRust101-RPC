# Testing strategy

## Required test layers

- Unit tests for parser records and malformed/partial input, mapping lookup/unknown behavior, state transitions and timer reset semantics, configuration validation, and presence payload formatting.
- Fixture tests with sanitized real `WizardClient.log` snippets labeled with game build/source date. If no real capture is available, minimal reference-derived fixtures may test only exact documented legacy patterns and must be labeled as reconstructed; they cannot establish current-build support or local-player health ownership.
- Filesystem integration tests using temporary directory trees for standalone/Steam paths, multiple Steam library roots, running-log changes, truncation, and replacement.
- Discord adapter tests using a fake transport for connect/disconnect/retry, set/clear behavior, payload deduplication, and error handling.
- Platform smoke tests verify live Discord transport and game discovery on supported environments. Linux Steam and Discord presence were verified in M3. Windows named-pipe and game-discovery runtime verification remains a deferred Windows release check; the Windows-target check is compile-only.

## Milestone gates

Before each milestone, state which checks apply and what evidence closes them. At minimum, run formatting, linting, unit and fixture tests, and a Windows build for code milestones; add integration/manual checks as the feature requires. Do not consider an untested zone, parser pattern, stat, or asset mapping supported.

## Test data privacy

Fixtures must be minimized and sanitized. Do not commit full user logs, account identifiers, or personal character names without consent. Prefer a short record excerpt sufficient to prove the parser behavior.

## M1 gates

- `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` must pass.
- Unit coverage must include recognized and malformed zone/health records, unknown zones, partial lines, duplicate location events, and state preservation on invalid data.
- Integration coverage must prove normal-path and Steam-library discovery using temporary trees, and tailer behavior for append, partial-line completion, truncation, and replacement without rereading prior bytes.
- Current game-build parsing remains unverified until a sanitized user capture is tested. Windows-only registry/Steam and real-game discovery still require manual verification on Windows.

## M2 gates

- Record a sanitized current-client `WizardClient.log` excerpt, including capture date/build context and enough adjacent lines to verify zone framing and local health attribution. If no such excerpt is available, do not claim current-client support.
- Add parser tests for every confirmed format or attribution change and retain unknown-safe mapping behavior.
- Use short October 1 log fixtures to verify that the parser recognizes its actual zone and health records and reports the logged final `3868/3868` pair. The corrected screenshot reading of current health `3868` supports that single local-current observation; it does not independently verify the maximum or a general local-attribution rule.
- Use the controlled October 1 before/after screenshot observation and short contiguous log ranges to verify the explicit local-hit marker followed by `3455/3868`, and ensure internal damage calculations and mixed combat health meters are not parsed as globe events. Record the 23-second internal-calculation-to-globe gap separately from the bounded visual transition. Do not turn this sample into a generic freshness or attribution assertion.
- Regression tests must verify per-record `Local` and `Unknown` attribution, including adjacency and same-log-second requirements for the explicit local marker, remote-marker downgrade, and parser reset. A same-value `MSG_CombatHealth` repeat remains unknown. Unknown observations must leave every local GameState field unchanged. The 19:50 screenshot pair does not verify a non-cinematic update because its matching globe record is from `Cinematics ProcessDamageEffect`.
- The 20:13 screenshot sequence verifies the exact `WizardClientMod MSG_UpdateHealth` source through visible `1992 → 2959 → 3868` current-health readings. Regression fixtures must preserve the three corresponding log pairs and the equality of IDs in the contradictory 20:11 remote-marker line. Do not infer maximum health from the screenshots or classify `HandleStatisticUpdate`/`MSG_CombatHealth` as local.

## M3 presence and IPC acceptance

- Pure presence tests cover verified/unverified/unknown locations, duplicate and changed-zone timestamps, locally attributed health only, health expiry, optional stat selection, missing/registered world assets, character selection, and clock edge cases.
- Fake-transport tests cover initial connect, deduplicated publication, periodic heartbeat, clear, connect failure, publish failure, bounded retry, and republish after reconnection. No running Discord is required for these tests.
- Integration test feeds a sanitized current-client fixture through parser and state into the presence builder; `Unknown` health must not change the published State.
- Run `cargo fmt`, `cargo test`, warning-denied Clippy, Windows-target `cargo check`, and Git diff checks. Cross-target compilation is not a live Windows/Discord verification.
- M3 live smoke evidence (2026-10-01): screenshot confirms Wizard101 activity, Stone Town, Zafaria artwork, `Last logged health: 3868/3868`, and elapsed time `0:24`; the watcher reported automatic discovery of the Linux Steam log. Do not claim Windows live verification from this evidence.
- Review readable names against the supplied [incomplete Wizard101 Central export](../research/wizard101central-locations.json); record the captured page and export date. Never use a page title or link to invent a raw zone ID relationship.
- Run `cargo fmt --check`, `cargo test`, warning-denied Clippy, Windows-target `cargo check`, and `git diff --check` after changes. Live Windows discovery verification is deferred because no Windows session is available; continue offline M2 parser work and keep the Windows limitation explicit before any public Windows release.
