# Milestone status

WizRust101-RPC is fully vibe coded. Codex CLI drives research, spec maintenance, design, implementation, refactoring, and testing through the version-controlled spec-driven workflow. The human supplies product decisions and verification data when required.

## M0: Product and architecture specification

**Status:** Complete; initial spec set committed in Git.

**Scope:** Research both reference projects and Discord/Rust IPC options; define product, architecture, discovery, parser, state, mapping, RPC, configuration, testing, and future-feature requirements.

**Result:** Specs created and committed as `e0ad4bc`. No runtime code or tests were part of M0.

## M1: Verified data-ingestion slice

**Status:** Implementation complete; current-client verification pending.

**Scope:** Rust project structure; automatic standalone and Steam install discovery; efficient live tailing; location/zone and current/max health parsing; typed game state; unknown-safe mapping; Bacon catalog migration; provenance-labeled reference-derived fixtures; unit and integration tests. No Discord RPC, GUI, or settings UI.

**Research:** See [m1-research.md](m1-research.md). Verified legacy parser literals, the Bacon catalog schema, Steam app ID `799960`, and the documented `steamlocate` library API. A public captured current-build log was not found. Reference-derived fixtures must not be represented as actual/current captures.

**Plan (recorded before implementation):**

1. **Complete before implementation:** update relevant specs and status with M1 scope, evidence, acceptance criteria, and limits (spec commit `eba2bb4`).
2. **Complete:** create a Rust library and binaries with typed discovery, tailing, parsing, mapping, and state APIs.
3. **Complete:** implement standalone/Steam discovery and append-only tailing with replacement/truncation recovery.
4. **Complete:** implement legacy-compatible parsing and an imported versioned mapping catalog without guessing worlds.
5. **Complete:** add labeled fixtures and unit/filesystem integration tests.
6. **Complete:** run formatting, tests, warning-denied Clippy, and a Windows-target compile; record results and limitations below.

**Known risks:** This workspace has no Windows Wizard101 installation, so real Windows install discovery cannot be manually validated here. Current-build log formatting and local health attribution also remain unverified pending a sanitized captured log.

### Completion record

- Implemented the library crate, watcher executable, and Bacon mapping importer.
- `cargo fmt --check`: passed.
- `cargo test`: passed; 20 tests (16 unit, 4 integration), no failures.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo check --target x86_64-pc-windows-gnu --all-targets`: passed. This is a cross-target compile check, not a live Windows/game smoke test.
- `git diff --check`: passed after the final spec and test updates.
- Bacon's catalog importer was exercised against the researched upstream JSON and produced 2,672 entries tagged `unverified-legacy`. The checked-in runtime catalog remains empty until mappings are independently verified.
- The fixture is labeled `reference-derived`; its numeric health values are synthetic sentinels. No current-build WizardClient.log capture was available, so current format and local health attribution are still unverified.
- The watcher starts at the end of an existing log and does not reconstruct location/health from historical records. It resumes from append-only updates and retries discovery when the selected file disappears.
- File discovery and replacement handling are unit-tested with temporary filesystem fixtures; validation on actual Windows standalone and Steam installations remains outstanding.

M1 implementation is complete under these evidence limits. The implementation commit is recorded in Git history.

### Next milestone

**M2: current-client verification and parser hardening only.** Obtain a short sanitized `WizardClient.log` excerpt from a current Windows client, record build/source provenance, confirm local health attribution and zone record framing, then add only fixture-backed parser behavior and run a Windows installation smoke check. Discord IPC is a later milestone.

## Product decisions still open

- Which Discord application owns published presence/assets? Required before public Discord integration, not for M1.
- Should Health be the default selected stat? It is currently the safe initial default when available.
- Should the first public release have a GUI, or remain a small background/console app? UI is deferred.
- Should a verified character name ever be published by default? It remains future opt-in pending a decision.

No product decision blocks M1. A sanitized current-client log excerpt is evidence needed for M2, not a product preference.

## Pre-M2 permanent requirements update

**Status:** Documentation updated; M2 research and implementation stopped at the required-source access gate.

- Recorded the fully vibe-coded, Codex CLI and spec-driven development ownership throughout the purpose and workflow docs.
- Added [Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) as a required source for reviewing readable names, subordinate to observed current-client log evidence for raw zone relationships. Unknown zones remain unknown.
- On 2026-10-01, the exact page returned HTTP 403 through both the browser tool and direct HTTPS outside the sandbox. No substitute source or new mapping was used.
- M2 requires access to that page (saved HTML, copied text, exported pages, or a browser-enabled environment) and a short sanitized current-client log excerpt with build/source context. A live Windows smoke check remains outstanding.
- Documentation update checks: `cargo fmt --check` passed; `cargo test` passed (20 tests); `cargo clippy --all-targets --all-features -- -D warnings` passed; `git diff --check` passed. No Rust code or mappings changed.

**M2 plan after the access gate clears:** document the current-client sample's provenance; compare its location and health records to the M1 parser; confirm local health attribution; make only sample-backed parser/state corrections; test those records and run `cargo fmt`, `cargo test`, and warning-denied Clippy; perform a real Windows discovery/log smoke check; update status and commit. No Discord RPC or UI work belongs to M2.
