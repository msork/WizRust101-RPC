# Testing strategy

## Required test layers

- Unit tests for parser records and malformed/partial input, mapping lookup/unknown behavior, state transitions and timer reset semantics, configuration validation, and presence payload formatting.
- Fixture tests with sanitized real `WizardClient.log` snippets labeled with game build/source date. Tests must demonstrate local-health selection and the currently supported zone patterns.
- Filesystem integration tests using temporary directory trees for standalone/Steam paths, multiple Steam library roots, running-log changes, truncation, and replacement.
- Discord adapter tests using a fake transport for connect/disconnect/retry, set/clear behavior, payload deduplication, and error handling.
- Windows manual verification for real named-pipe connection, real game log discovery, and Discord-rendered world image/timestamp. These require actual Windows, game, Discord client, registered app, and assets.

## Milestone gates

Before each milestone, state which checks apply and what evidence closes them. At minimum, run formatting, linting, unit and fixture tests, and a Windows build for code milestones; add integration/manual checks as the feature requires. Do not consider an untested zone, parser pattern, stat, or asset mapping supported.

## Test data privacy

Fixtures must be minimized and sanitized. Do not commit full user logs, account identifiers, or personal character names without consent. Prefer a short record excerpt sufficient to prove the parser behavior.
