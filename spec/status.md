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

**Status:** Historical access investigation complete; the later user-supplied export opens offline research only.

- Recorded the fully vibe-coded, Codex CLI and spec-driven development ownership throughout the purpose and workflow docs.
- Added [Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) as a required source for reviewing readable names, subordinate to observed current-client log evidence for raw zone relationships. Unknown zones remain unknown.
- On 2026-10-01, the exact page returned HTTP 403 through both the browser tool and direct HTTPS outside the sandbox. No substitute source or new mapping was used.
- A single read-only MediaWiki API query for `Basic:Locations` returned HTTP 444 with a Cloudflare `Access Blocked` HTML response; no MediaWiki data was returned. Links, categories, and related location/world pages were not queried after the block. See [research.md](research.md) for the exact endpoint and response details.
- At that point, M2 required authorized wiki access and a current-client log; the later user-supplied incomplete export and real Linux Steam logs now support offline work. Live Windows discovery validation is deferred because no Windows session is available. The user is not expected to save hundreds of linked pages manually.
- Documentation update checks: `cargo fmt --check` passed; `cargo test` passed (20 tests); `cargo clippy --all-targets --all-features -- -D warnings` passed; `git diff --check` passed. No Rust code or mappings changed.

## M2: Current-client verification and parser hardening

**Status:** Offline snapshot review and parser hardening in progress. Real Linux Steam logs from 2026-09-30 and 2026-10-01 identify client version `W.1.610.21` and verify specific record forms. An approximate-time screenshot does not resolve local health attribution. Live Windows discovery is deferred.

**Plan recorded before code changes:**

1. Preserve the user-supplied 176-page Wizard101 Central export unchanged; inspect its schema and coverage programmatically, and record it as an incomplete dated snapshot.
2. Compare selected captured readable names with existing Bacon candidates without promoting raw IDs to current-verified. Keep unknown zones unknown.
3. Harden the parser by ending zone IDs at the first comma and withholding pending health at end of input. Use sanitized fragments of the user-provided logs to verify observed zone, health, and remote-marker records; accept observed `current > maximum` values. Keep the October 1 screenshot's differing health reading as unresolved evidence rather than adding an unsupported coalescing or attribution rule.
4. Run `cargo fmt`, `cargo test`, warning-denied Clippy, and `git diff --check`; update this status with results and commit.
5. Investigate local-health attribution and freshness with a precisely timed health change if available. Defer live Windows discovery verification until a Windows session exists; do not let that prevent offline M2 progress.

No Discord RPC or UI implementation belongs to M2. Automated wiki HTML/API crawling remains prohibited without a site-owner-authorized method after HTTP 403/444 and the user-reported Error 1006/IP ban.

### M2 progress and evidence limits

- Preserved the user's JSON export unchanged. Programmatic inspection confirmed the root plus 175 captured `Location:` pages, dated `2026-10-01T23:19:35.457Z`; the snapshot has no page bodies or log IDs. The captured Wizard City ↔ Ravenwood links corroborate a legacy display-name candidate. The Zafaria page links to uncaptured Stone Town, so no Zafaria raw ID was promoted to the runtime catalog.
- Inspected a real 2026-09-30 Linux Steam `WizardClient.log` from client version `W.1.610.21` (revision `r806919.Wizard_1_610`). Added small sanitized fixture ranges for a zone, a consecutive health series, a remote-marker pair, character selection, and the version line. The full log remains outside Git.
- The parser now stops zone IDs at the first comma, discards pending health on source reset rather than emitting it at end of input, and accepts observed numeric health records where `current > maximum`. No new record syntax was invented.
- `cargo fmt --check`, `cargo test` (25 tests: 18 unit, 7 integration), `cargo clippy --all-targets --all-features -- -D warnings`, Windows-target `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. The Windows check only compiles; it is not a live smoke test.

**Unresolved:** The October 1 log records `3868/3868` at 19:35:39, while the user identifies the screenshot's current-health reading as `3866`. The screenshot has no exact capture timestamp, visible maximum-health value, or current-location label. No `3866` or further health/damage record appears after that log burst through about 19:36:17. This does not establish a local-health attribution or freshness rule. A precisely timed health change with the displayed current/max values and adjacent log lines would resolve the next question. No Windows Wizard101 session is available; live Windows discovery validation is deferred and does not block remaining offline M2 work.

Plan steps 1–4 are complete under the recorded evidence limits; step 5 remains open. No product decision is needed to interpret the captured records.

### October 1 screenshot and log review

- The user identifies the screenshot's current-health display as `3866` at approximately 19:35. The screenshot does not display maximum health or a current-location label; quest text mentions Stone Town. Its exact capture time is unavailable. It is not committed because it includes character names.
- The `W.1.610.21` log records `Zafaria/ZF_Z07_Stone_Town` at 19:35:27 and a burst of sixteen health records at 19:35:39, ending at `3868/3868`. From the end of that burst through 19:36:17, the inspected log contains no `3866` and no further health/damage record. The log cannot be equated to the screenshot's health reading; local attribution and freshness remain unknown.
- Added sanitized October 1 version, zone, and health-window fixtures plus an integration test for exactly what the parser reads from those lines. No burst coalescing, local-attribution rule, or location/world mapping was added from the ambiguous screenshot comparison.
- Windows live discovery validation is **deferred**, as requested, and does not block offline parser research or testing. Do not request a Windows session again unless the user says one is available.
- Checks after this evidence update: `cargo fmt --check` passed; `cargo test` passed (26 tests: 18 unit, 8 integration); `cargo clippy --all-targets --all-features -- -D warnings` passed; `cargo check --target x86_64-pc-windows-gnu --all-targets` passed; `git diff --check` passed. No parser behavior was changed in this follow-up.

**Next actionable M2 work:** obtain a precise timestamp for a health change visible in the game and compare the displayed current/max values with adjacent log lines from the same Linux Steam session. This can determine whether the log omits some health changes or whether the approximate screenshot time caused the mismatch. Preserve the Windows smoke check as a later release verification item.
