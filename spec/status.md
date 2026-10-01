# Milestone status

## M0: Product and architecture specification

**Status:** Complete; initial spec set committed in Git.

**Scope:** Research both reference projects and Discord/Rust IPC options; define product, architecture, discovery, parser, state, mapping, RPC, configuration, testing, and future-feature requirements.

**Result:** Specs created and committed as \`e0ad4bc\`. No runtime code or tests were part of M0.

## M1: Verified data-ingestion slice

**Status:** In progress.

**Scope:** Rust project structure; automatic standalone and Steam install discovery; efficient live tailing; location/zone and current/max health parsing; typed game state; unknown-safe mapping; Bacon catalog migration; realistic provenance-labeled fixtures; comprehensive unit and integration tests. No Discord RPC, GUI, or settings UI.

**Research:** See [m1-research.md](m1-research.md). Verified legacy parser literals, the Bacon catalog schema, Steam app ID \`799960\`, and the documented \`steamlocate\` library API. A public captured current-build log was not found. Reference-derived fixtures must not be represented as actual/current captures.

**Plan (recorded before implementation):**

1. Update relevant specs and status with M1 scope, evidence, acceptance criteria, and limits.
2. Create a Rust library and binaries with typed discovery, tailing, parsing, mapping, and state APIs.
3. Implement standalone/Steam discovery and append-only tailing with replacement/truncation recovery.
4. Implement legacy-compatible parsing and an imported versioned mapping catalog without guessing worlds.
5. Add labeled fixtures and unit/filesystem integration tests.
6. Run formatting, tests, and warning-denied Clippy; record results and limitations.

**Known risks:** This workspace has no Windows Wizard101 installation, so real Windows install discovery cannot be manually validated here. Current-build log formatting and local health attribution also remain unverified pending a sanitized captured log.

## M1 completion record

Implementation, test results, commit SHA, and any remaining limitations will be recorded here before M1 is considered complete.

## Product decisions still open

- Which Discord application owns published presence/assets? Required before public Discord integration, not for M1.
- Should Health be the default selected stat? It is currently the safe initial default when available.
- Should the first public release have a GUI, or remain a small background/console app? UI is deferred.
- Should a verified character name ever be published by default? It remains future opt-in pending a decision.
