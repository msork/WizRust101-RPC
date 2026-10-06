# Milestone status

## First-release preparation (2026-10-05)

**Version and scope:** The planned first public version is `v26.10.05`, a Windows-first preview. Project releases use Calendar Versioning `vYY.MM.DD`; additional releases on the same day use `.1`, `.2`, etc. Cargo package metadata uses normalized `26.10.5` because Cargo rejects leading-zero numeric components, while installer/app metadata and public release materials use exact `26.10.05`. No tag or release has been created. Linux may be offered with an explicit CI-validated/not-newly-live-tested label. macOS is excluded from production support until real acceptance and signing/notarization gates pass. This is not release approval.

**Evidence and support matrix:** Windows 11 x64 with native Steam Wizard101 and native Discord passed owner live acceptance for both installer and portable builds, including continuous timer, stationary restart restoration, Discord reconnect, tray lifecycle, icon, and known/Unknown-world behavior. The later concise tray-status text change has no separate owner live retest. Linux's current AppImage and Flatpak ZIPs are validated by Ubuntu 22.04 packaging CI (Flatpak 26.08, artifact metadata/resources/permissions, AppImage resources/executable, headless load and embedded ID); current artifacts have not received fresh live Linux desktop/Steam/Discord acceptance. An earlier M6 installed Flatpak/native Steam/native Discord live acceptance is historical and does not establish live acceptance of the current packages. macOS 15 CI validates native tests and warning-denied Clippy plus the Universal 2 app/pkg structure, architectures, resources, embedded ID, and payload. No real Mac/CrossOver/Steam/Discord/menu-bar acceptance has occurred. Current macOS packages are unsigned and unnotarized; Developer ID Application/Installer signing, Hardened Runtime, notarization/stapling, Gatekeeper behavior, and install/upgrade/removal testing remain mandatory gates. macOS is not production-supported.

**Release preparation automation:** `.github/workflows/release-prep.yml` is manual, defaults to release version `v26.10.05` and the dispatch ref's current HEAD, and allows overriding the source with another exact SHA that has successful platform package runs. Failed run `37389548449` on `2883e8a5c62d2303a707a3b2ebfac13afd0be11c` validated an empty optional `source_sha` before applying the dispatch-SHA fallback; it stopped before artifact discovery. The workflow now resolves the fallback before validation, checks out that exact commit, retrieves all six ZIP artifacts only from successful platform package runs for the same SHA, verifies each ZIP and its expected payload, computes `SHA256SUMS.txt`, and stages the six ZIPs plus checksum as separate raw workflow artifacts. It creates no Git tag or GitHub Release and does not wrap ZIPs in another archive. Running this workflow does not waive any support or signing gate.

**Release notes:** [CHANGELOG.md](../CHANGELOG.md) contains planned `v26.10.05` notes and explicitly states that no tag or release exists. Before a future release, review the Windows Inno Setup licensing assumption and platform support wording. Do not represent the current macOS artifact as signed/notarized or production-supported.

WizRust101-RPC is fully vibe coded. Codex CLI drives research, spec maintenance, design, implementation, refactoring, and testing through the version-controlled spec-driven workflow. The human supplies product decisions and verification data when required.

**Current roadmap (2026-10-05):** M4 Level/School research is closed; M5 config is complete; M6 Linux Flatpak and M7 Windows are closed for their documented live pairings. The latest Linux ZIPs are CI-validated but not newly live-tested. M8 implementation and Universal 2 packaging are present, but live macOS/CrossOver acceptance and signing/notarization remain open, so macOS is not production-supported. The first-release recommendation and evidence limits are recorded above. Standalone support remains deferred. See [future roadmap](future.md), [M6 plan](m6-plan.md), [M7 plan](m7-plan.md), and [M8 plan](m8-plan.md).

### Authoritative DB schema integration (2026-10-05)

Updated the WizRust101-DB submodule to `56c9e4489f0b8f296f5dc016eb7d3579adbf1977` after inspecting its generated output. The schema is now a canonical-path object whose values contain exact string fields `world` and `zone`; diagnostics remain a parallel array. RPC no longer carries a local zone-world association or confidence gate. Details always uses the exact DB `zone` value, including fallback values and whitespace. State uses the exact DB world value except exact `Unknown`, which is omitted. World images use exact-name entries from the explicit art registry and fall back to Discord `wizard101` when Unknown or unmatched; small art remains `wizrust101_rpc`.

The full 3,346-zone audit passes with 2,713 known-world zones, 633 Unknown-world zones, 2,510 world-art matches, 836 generic-art fallbacks, and 0 missing Details values. Tests also check diagnostics path/entry parity and representative zones across known worlds, housing, and gauntlets. Native Windows formatting, strict Clippy, and all 89 tests passed. The pushed integration commit's [Windows](https://github.com/msork/WizRust101-RPC/actions/runs/37352358389), [Linux](https://github.com/msork/WizRust101-RPC/actions/runs/37352358168), and [macOS](https://github.com/msork/WizRust101-RPC/actions/runs/37352358226) check and packaging workflows all passed. Linux live testing remains unavailable and is documented in [presence coverage](presence-coverage.md). This supersedes the earlier M6/M6-follow-up confidence-gating and world-association descriptions above; those entries record the former implementation state.

### M6 mapping integration follow-up (2026-10-04)

M6 live acceptance remains limited to the installed Linux Flatpak with native Steam and native Discord. Location-name resolution now reads pinned WizRust101-DB data instead of the RPC-owned location table. Runtime consumes the flat map with per-path diagnostics; raw internal-filename fallbacks never appear as Details. Stone Town's DB value comes directly from the DB, retains its `unverified_fallback` diagnostic, and is allowed by independently recorded exact-value owner evidence. World associations stay separate and independently evidenced; DB path prefixes are not world mappings. See [mapping research and implementation decision](zone-world-mapping.md) and [M6 plan](m6-plan.md).

The pinned DB revision classifies the Stone Town entry as `unverified_fallback`, despite its flat value being `Stone Town`. The exact value remains eligible through the separate project-owner confirmation already recorded for this path; its DB diagnostic remains unmodified and visible to the runtime model. If upstream changes the DB value, the exact-value evidence no longer matches and Details is omitted until reviewed. The previous M6 screenshot supports the same displayed name and native Steam/Discord pairing.

**Integration implementation and checks (2026-10-04):** Pinned WizRust101-DB at `d6318ea074be9a80fabebcd5b48908ff4543d7e1` as a submodule; replaced the RPC location catalog with the flat map plus original per-path diagnostics; separated world evidence into `data/zone-worlds.json`; removed the Bacon importer/runtime location table; discarded the obsolete `display_stat` field; preserved internal Health parsing and the owner-edited Flatpak icon. Windows native formatting/tests/Clippy passed: 89 tests (65 unit, 6 config integration, 14 ingestion, 4 presence pipeline), warning-denied Clippy, and `git diff --check`. CI check/package workflows now initialize submodules. No Flatpak build or live Linux retest was run in this Windows session.

**M6 retest after this integration:** Upload `assets/icons/sizes/1024.png` to the Discord application's Rich Presence art assets under `wizrust101_rpc` if it is not already present. On Linux, install the newly built Flatpak. With native Steam running Wizard101 and native Discord signed in with activity sharing enabled, start already in Stone Town. Confirm Details `Stone Town`, State `Zafaria`, Zafaria large art, `wizrust101_rpc` small art with `WizRust101-RPC` hover, and a fresh timer. Confirm no Health is sent. Re-enter Stone Town and confirm the timer resets; restart while stationary and confirm startup replay restores the same activity. The DB diagnostic must remain `unverified_fallback` internally, while the exact owner evidence matches `Stone Town`. Also exercise a fixture path whose DB value equals its raw final filename and verify that value is never shown in Details. Acceptance scope remains native Steam plus native Discord only.

### Full Rich Presence coverage audit (2026-10-04)

Audited every entry and diagnostic at the pinned DB revision. Results: 3,346 total; 1,241 displayable Details; 26 raw/internal filename values suppressed; 1 exact-path verified world State; 1 registered large-art zone; 1 zone with correct Details + State + art; 1,240 Details lacking a world association/art; and 2,105 without readable Details. The detailed definitions, per-confidence counts, representative path coverage, and remaining gaps are in [presence-coverage.md](presence-coverage.md).

Added a full-dataset test that calls the production Presence builder for each path, checks every diagnostic and exact-path association, and pins the audited totals. Representative assertions span all requested worlds, housing and gauntlets, including hidden fallback cases. Native Windows validation passed: 90 tests (66 unit, 6 configuration, 14 ingestion, 4 presence pipeline), formatting, warning-denied Clippy, and diff checks. Linux live testing is unavailable; Linux/macOS/Windows Actions runs after the audit push remain the CI validation gate.

## M0: Product and architecture specification

**Status:** Complete; initial spec set committed in Git.

**Scope:** Research both reference projects and Discord/Rust IPC options; define product, architecture, discovery, parser, state, mapping, RPC, configuration, testing, and future-feature requirements.

**Result:** Specs created and committed as `e0ad4bc`. No runtime code or tests were part of M0.

## M1: Verified data-ingestion slice

**Status at M1 close:** Implementation complete; current-client verification was pending at that point and was subsequently pursued in M2.

**Historical M1 scope:** Rust project structure; automatic standalone and Steam install discovery; efficient live tailing; location/zone and current/max health parsing; typed game state; unknown-safe mapping; Bacon catalog migration; provenance-labeled reference-derived fixtures; unit and integration tests. No Discord RPC, GUI, or settings UI. Standalone discovery was implemented here but is now deferred from active product support; M6 removes it from the active path.

**Research:** See [m1-research.md](m1-research.md). Verified legacy parser literals, the Bacon catalog schema, Steam app ID `799960`, and the documented `steamlocate` library API. A public captured current-build log was not found. Reference-derived fixtures must not be represented as actual/current captures.

**Plan (recorded before implementation):**

1. **Complete before implementation:** update relevant specs and status with M1 scope, evidence, acceptance criteria, and limits (spec commit `eba2bb4`).
2. **Complete:** create a Rust library and binaries with typed discovery, tailing, parsing, mapping, and state APIs.
3. **Complete:** implement standalone/Steam discovery and append-only tailing with replacement/truncation recovery. The standalone portion is historical and deferred from active support.
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

### Next milestone at M1 close (historical; superseded)

**M2: current-client verification and parser hardening only.** Obtain a short sanitized `WizardClient.log` excerpt from a current Windows client, record build/source provenance, confirm local health attribution and zone record framing, then add only fixture-backed parser behavior and run a Windows installation smoke check. Discord IPC is a later milestone.

## Product decisions still open

- Public packaging/distribution details remain unspecified. The owner-provided application ID is used locally through an environment variable and is not compiled into the project.
- Should the first public release have a GUI, or remain a small background/console app? UI is deferred.

At M1 close, a sanitized current-client log excerpt was evidence needed for M2, not a product preference. That historical milestone has since been completed.

## Pre-M2 permanent requirements update

**Status:** Historical access investigation complete; the later user-supplied export opens offline research only.

- Recorded the fully vibe-coded, Codex CLI and spec-driven development ownership throughout the purpose and workflow docs.
- Added [Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) as a required source for reviewing readable names, subordinate to observed current-client log evidence for raw zone relationships. Unknown zones remain unknown.
- On 2026-10-01, the exact page returned HTTP 403 through both the browser tool and direct HTTPS outside the sandbox. No substitute source or new mapping was used.
- A single read-only MediaWiki API query for `Basic:Locations` returned HTTP 444 with a Cloudflare `Access Blocked` HTML response; no MediaWiki data was returned. Links, categories, and related location/world pages were not queried after the block. See [research.md](research.md) for the exact endpoint and response details.
- At that point, M2 required authorized wiki access and a current-client log; the later user-supplied incomplete export and real Linux Steam logs now support offline work. Live Windows discovery validation is deferred because no Windows session is available. The user is not expected to save hundreds of linked pages manually.
- Documentation update checks: `cargo fmt --check` passed; `cargo test` passed (20 tests); `cargo clippy --all-targets --all-features -- -D warnings` passed; `git diff --check` passed. No Rust code or mappings changed.

## M2: Current-client verification and parser hardening

**Status:** Offline M2 is complete for verified client `W.1.610.21` syntax, the explicit local-damage path, and the screenshot-correlated `MSG_UpdateHealth` recovery source. Other health sources and general freshness remain unknown. Live Windows discovery is deferred.

**Plan recorded before code changes:**

1. Preserve the user-supplied 176-page Wizard101 Central export unchanged; inspect its schema and coverage programmatically, and record it as an incomplete dated snapshot.
2. Compare selected captured readable names with existing Bacon candidates without promoting raw IDs to current-verified. Keep unknown zones unknown.
3. Harden the parser by ending zone IDs at the first comma and withholding pending health at end of input. Use sanitized fragments of the user-provided logs to verify observed zone, health, and remote-marker records; accept observed `current > maximum` values. Treat the October 1 screenshot match as evidence for one local current value without adding an unsupported general coalescing or attribution rule.
4. Run `cargo fmt`, `cargo test`, warning-denied Clippy, and `git diff --check`; update this status with results and commit.
5. Investigate local-health attribution and freshness with a precisely timed health change if available. Defer live Windows discovery verification until a Windows session exists; do not let that prevent offline M2 progress.

No Discord RPC or UI implementation belongs to M2. Automated wiki HTML/API crawling remains prohibited without a site-owner-authorized method after HTTP 403/444 and the user-reported Error 1006/IP ban.

### M2 progress and evidence limits

- Preserved the user's JSON export unchanged. Programmatic inspection confirmed the root plus 175 captured `Location:` pages, dated `2026-10-01T23:19:35.457Z`; the snapshot has no page bodies or log IDs. The captured Wizard City ↔ Ravenwood links corroborate a legacy display-name candidate. The Zafaria page links to uncaptured Stone Town, so no Zafaria raw ID was promoted to the runtime catalog.
- Inspected a real 2026-09-30 Linux Steam `WizardClient.log` from client version `W.1.610.21` (revision `r806919.Wizard_1_610`). Added small sanitized fixture ranges for a zone, a consecutive health series, a remote-marker pair, character selection, and the version line. The full log remains outside Git.
- The parser now stops zone IDs at the first comma, discards pending health on source reset rather than emitting it at end of input, and accepts observed numeric health records where `current > maximum`. No new record syntax was invented.
- `cargo fmt --check`, `cargo test` (25 tests: 18 unit, 7 integration), `cargo clippy --all-targets --all-features -- -D warnings`, Windows-target `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. The Windows check only compiles; it is not a live smoke test.

**Unresolved:** The screenshot and log match on current health `3868` near 19:35, strongly supporting one local observation. The screenshot has no exact capture timestamp, visible maximum-health value, or current-location label. This single match cannot establish a general local-health attribution or freshness rule. A precisely timed health change with displayed current/max values and adjacent log lines would test that rule. No Windows Wizard101 session is available; live Windows discovery validation is deferred and does not block remaining offline M2 work.

Plan steps 1–4 are complete under the recorded evidence limits; step 5 remains open. No product decision is needed to interpret the captured records.

### October 1 screenshot and log review

- **Verified:** The `W.1.610.21` log records `Zafaria/ZF_Z07_Stone_Town` at 19:35:27 and sixteen health records at 19:35:39, ending at `3868/3868`; the parser test returns that pair. The user corrected the screenshot reading to **current health `3868`** at approximately 19:35. The inspected log has no further health/damage record through 19:36:17.
- **Strongly supported:** The matching current value and nearby timing indicate that the final logged current health was the local character's and recently observed in this session. The screenshot is not committed because it includes character names.
- **Still ambiguous:** The screenshot does not independently show maximum health, an exact capture timestamp, or a textual current-location label. Its Stone Town quest text does not establish the current location by itself. One matched sample does not prove all unmarked health records are local or that future health changes reach the log promptly.
- **Deferred:** Windows live discovery validation is unavailable and does not block offline parser research or testing. Do not request a Windows session again unless the user says one is available.
- Sanitized October 1 version, zone, and health-window fixtures test exactly what the parser reads. No burst coalescing, general local-attribution rule, or location/world mapping was added from this one matched sample.
- Checks rerun after correcting the screenshot reading: `cargo fmt --check` passed; `cargo test` passed (26 tests: 18 unit, 8 integration); `cargo clippy --all-targets --all-features -- -D warnings` passed; `cargo check --target x86_64-pc-windows-gnu --all-targets` passed; `git diff --check` passed. No parser behavior was changed in this follow-up.

**Next actionable M2 work:** compare a precisely timed visible health change with displayed current/max values and adjacent log lines in the same Linux Steam session. This tests the general attribution and update-freshness rules. Preserve the Windows smoke check as a later release verification item. The controlled observation below completes this comparison for one combat-damage path, while other health sources remain unresolved.

### Controlled health-change review plan

The user supplied before/after Linux Steam screenshots from `19:50:53` and `19:51:23` and the matching `W.1.610.21` log. Review the exact adjacent records and any other-combatant health records; add only short sanitized real-log fixtures and tests for observed parser syntax and behavior. Record the measured calculation-to-globe gap separately from the bounded visual transition; retain uncertainty about any general attribution or freshness rule. Windows live discovery remains deferred and does not block this offline work.

### Controlled health-change result

- **Verified for this encounter:** The before screenshot shows the local character at `3868/3868`; the after screenshot shows current `3455` and floating `413` damage. The log calculates `3868 - 413` at `19:50:56`, then records `Our client is getting hurt!`, a `3455/3868` globe update, matching damage arithmetic, and a matching health meter at `19:51:19`. The after screenshot does not independently show maximum health. Its visible `3455` corroborates the logged current value; the explicit client marker establishes local attribution for that globe record. The parser returns the observed pair from this sanitized log range.
- **Timing:** Screenshot filename and file-modification times are `19:50:53.730` and `19:51:23.530` local; they are capture-time proxies. The internal calculation is about 23 seconds before the globe record, consistent with a combat cinematic. The visual transition is bounded only by the still images, about 29.8 seconds apart. The globe record is about 4.5 seconds before the after image's file time. These are observations from one encounter, not a general log or display latency bound.
- **Attribution limits:** The current session has 28 globe records across five source components. Three `Cinematics ProcessDamageEffect` globe records have the explicit preceding `Our client is getting hurt!` marker; the other 25 do not. Nearby `1530/1530` combat meters coexist with the local `3455/3868` meter. Bare meter values, an owner ID, and absence of the next-line remote marker do not prove local ownership. The September log independently verifies the next-line remote-marker exclusion syntax, but this controlled session has no such marker. The parser still emits unmarked globe values as *observed* health, so publishing them as local health needs a stricter attribution design and more evidence.
- **Changes and checks:** Added three contiguous, sanitized fixture ranges for the damage calculation, explicit local hit, and mixed meters; two integration tests cover the existing parser behavior. Programmatic comparison confirmed each sanitized fixture matches its documented source range after ID redaction. `cargo fmt`, `cargo fmt --check`, `cargo test` (28 tests: 18 unit, 10 integration), `cargo clippy --all-targets --all-features -- -D warnings`, `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. The Windows check is a compile check, not a live installation check.

**M2 closure decision:** The current-client record syntax and one local combat-damage path are verified, but M2's broader local-health attribution goal is not complete. Keep M2 open for a conservative per-record attribution policy and fixture-backed verification of at least one non-cinematic health source. The Windows live discovery check stays deferred as a separate release validation item and does not block this offline work. The next actionable slice is to classify the currently unmarked `HandleStatisticUpdate` and `MSG_CombatHealth` records using additional controlled Linux Steam evidence or leave them explicitly unattributed; do not publish them as local by assumption. No new product decision is required.

### M2 attribution hardening plan

The user identified the existing `19:50:53` full-health and `19:51:23` lower-health screenshots as the proposed non-cinematic evidence. Their matching `19:51:19` log record is labeled `Cinematics ProcessDamageEffect`, so it cannot verify a non-cinematic health path. The available log ends at `19:53:23`; no newer screenshots or log window were present. Proceed with conservative typed attribution based on the verified explicit local-hit and adjacent remote-player markers, keep every other record unknown, and prevent unknown/other-player observations from changing local GameState. Rework fixture tests, run all required checks, and document the unresolved non-cinematic source and Windows deferral before committing.

### M2 attribution hardening result

- The parser now emits typed health observations with `Local`, `OtherPlayer`, or `Unknown` attribution. `Local` requires the immediately preceding exact `Cinematics ProcessDamageEffect: Our client is getting hurt!` marker, the same log second, and a `Cinematics ProcessDamageEffect` globe line. A following remote-player marker takes precedence and marks the observation `OtherPlayer`. Every other globe record is `Unknown`; later equal values do not inherit attribution.
- GameState accepts only `Local` health observations and records their monotonic receipt time separately. Unknown and other-player observations leave health, activity, location, and local observation time unchanged. The last verified value may become stale; no expiration interval is inferred. A parser reset also clears any pending local marker or health observation.
- The full current-session log contains 28 globe lines across five source components. Three cinematic globe lines have adjacent explicit local-hit markers. The remaining 25, including the `19:51:30` same-value `MSG_CombatHealth` repeat, stay unknown. The September remote-marker fixture establishes the exclusion syntax. Bare `ClientHealthMeter` lines for multiple combatants are ignored. See the [per-record timeline](log-parsing.md#conservative-attribution-hardening-plan).
- The before/after screenshots prove a local `3868/3868` to `3455` visible transition and match the explicit `19:51:19` cinematic globe record. The after image does not independently show maximum health. The internal damage calculation at `19:50:56` precedes the globe line by about 23 seconds, while the globe line precedes the after screenshot's file time by about 4.5 seconds. Two still images cannot time the visual transition or establish a general update-latency bound.
- Added a sanitized contiguous `MSG_CombatHealth` repeat fixture, updated previous fixture tests for per-record attribution, and added parser/state regression cases for adjacency, timestamp mismatch, reset, remote precedence, and unknown-state preservation. No Discord RPC, UI, new zone mapping, or inferred health source was added.
- `cargo fmt`, `cargo fmt --check`, `cargo test` (32 tests: 21 unit, 11 integration), `cargo clippy --all-targets --all-features -- -D warnings`, `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. Programmatic comparison confirmed the new sanitized fixture matches source log lines 1378–1380 after ID redaction. The Windows check only compiles the target; it is not a live installation check.

**Offline M2 closure:** Still open for verified non-cinematic local health attribution and a general freshness assessment. The user's clarification identified the same cinematic screenshot pair, so the required non-cinematic observation is not yet available. The conservative safety boundary is implemented and tested; unknown values no longer contaminate local GameState. The next evidence needed is a timed Linux Steam before/after health change outside a combat cinematic, with adjacent log records and visible current/max values where possible. Windows live discovery remains deferred as a separate release check, not an offline M2 blocker. No product decision is needed.

### M2 controlled out-of-combat recovery update

**Evidence and result:** The user subsequently supplied a distinct three-screenshot sequence from the open world at `20:13:14.107`, `20:13:33.640`, and `20:13:41.403` local. Visible current health rises `1992 → 2959 → 3868`; the user calls the last value full health, while numeric maximum is not independently shown on those images. Client `W.1.610.21` logs exact `WizardClientMod MSG_UpdateHealth` globe pairs `1992/3868` at `20:13:06`, `2959/3868` at `20:13:27`, and `3868/3868` at `20:13:38`. The latter two are out-of-combat increases and precede the next screenshots by about 6.6 and 3.4 seconds. Each line is already present before the screenshot showing its value. The images do not identify the exact moment of either increase, so no general latency bound follows.

**Attribution correction:** A separate `20:11:26` line says `HUDWindow::HandleUpdateHealth called for a player that is not this client's!`, yet the original line prints identical numeric sent/client-player IDs. The September fixture redacted both IDs and cannot establish whether they differed. Revise the parser to `Local` or `Unknown`: the adjacent explicit cinematic-hit path and the exact screenshot-verified `WizardClientMod MSG_UpdateHealth` source are `Local` for the observed build, unless the immediately following contradictory marker downgrades them to `Unknown`. Other sources, bare health meters, repeated values, and ID-only correlations stay `Unknown`. Do not claim that the phrase proves a different player. Only `Local` observations update GameState.

**Implementation and evidence:** Added contiguous sanitized fixture ranges for the three recovery records and the contradictory marker, preserving the equality of the original IDs without committing them. Updated parser and integration tests for the recovered local values and unknown marker. Programmatic comparison confirmed fixture text and ID equality against the supplied log. The full log and screenshots remain outside Git.

**Checks:** `cargo fmt`, `cargo fmt --check`, `cargo test` (35 tests: 22 unit, 13 integration), `cargo clippy --all-targets --all-features -- -D warnings`, `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. The Windows check is compilation only; it does not verify live discovery.

**Revised offline M2 closure:** The offline current-client verification and parser-hardening scope is complete for the observed `W.1.610.21` zone syntax, explicitly local cinematic-hit path, and controlled `MSG_UpdateHealth` recovery source. Unknown sources remain unknown. General update latency, other client builds, and a safe health-expiry interval remain unverified and are recorded limits, not guessed rules. Windows live discovery remains a deferred release validation check and does not block offline M2 closure. The next milestone is M3: specify and implement the presence model and Discord IPC adapter using only verified mapping assets and locally attributed health; retain unknown world/stat behavior.

## M3: Presence model and Discord IPC adapter

**Status:** Complete for the Linux Steam runtime path. Windows live verification remains deferred; its target was compile-tested only.

**Plan:**

1. Specify trusted-field construction, a presentation-only health age limit, verified mapping/asset gating, and clear/retry semantics; review current IPC crates and official protocol docs.
2. Implement pure presence construction and a replaceable Discord transport with fake-transport tests.
3. Integrate the publisher with watcher polling, using a development application ID override and safe defaults when absent. No GUI or persistent settings UI.
4. Run formatting, tests, warning-denied Clippy, Windows-target check, and diff checks; update status with exact results and limits, then commit.

**Planning risks and resolution:** At planning time there were no verified runtime rows, app ID, or uploaded world keys. The project owner then supplied manual Stone Town mapping evidence, the Zafaria asset key, and an application ID via the local environment. The live Linux test below closes the main M3 integration goal. Windows remains an explicitly deferred platform smoke test.

### M3 result

- Added a pure presence model. Details, timer, and world art require verified mappings; world art additionally requires an approved uploaded asset key. Unknown/legacy-only zones remain undisplayed. The stat selector supports `health` (default) and `none`; Health is labeled “Last logged health” and omitted 60 seconds after the last locally attributed observation. Unknown observations never populate or refresh it.
- Updated GameState to retain the entry time across verified raw-ID aliases of the same displayed location/world. A different displayed location resets it. Discord receives Unix-second timestamps calculated from monotonic elapsed time and current wall time.
- Added `discord-rich-presence` 1.1.0 behind a transport trait and a publisher with deduplication, 60-second heartbeat, and bounded reconnect backoff. IPC errors are reported without stopping log monitoring; discovery errors now retry, source replacement resets state before republishing, and routine state/health printing was removed.
- Added development environment settings for the Discord application ID and stat, plus a versioned world-asset catalog. At this point no project-owned application ID or world PNG keys were configured. Persistent settings UI and GUI remain deferred; subsequent owner-supplied mapping and asset configuration are recorded below.
- Added fake-transport, wire-field serialization, pure-model, state-alias, catalog-schema, and captured-log-to-presence regression tests. After the Stone Town mapping update, all 49 tests passed (35 unit, 14 integration). `cargo fmt`, `cargo fmt --check`, warning-denied Clippy, Windows-target `cargo check --target x86_64-pc-windows-gnu --all-targets`, and `git diff --check` passed. The Windows check is compilation, not a live smoke test.

**M3 closure:** The project owner supplied a live Discord profile screenshot and confirmed the smoke test succeeded on Linux Steam. See the [M3 live verification record](#m3-live-linux-steam-and-discord-verification). No Windows live verification is claimed. Wizard101 Central was not consulted for the project-owner verified Stone Town mapping. Continue to record future mappings with source-typed evidence and leave unsupported IDs unknown.

### M3 project-owner mapping update

- Added `Zafaria/ZF_Z07_Stone_Town` → `Stone Town` → `Zafaria` to `data/zones.json`. The raw ID is explicitly present in the sanitized October 1 client `W.1.610.21` fixture. The project owner confirmed the readable location/world relationship on 2026-10-01.
- Added typed per-row evidence provenance and a test asserting the log and owner-manual evidence, date, verified status, and absence of Wizard101 Central evidence. No wiki request was made; the wiki is not cited as supporting this row. Other raw Zafaria IDs remain unknown.
- Added an end-to-end sanitized log → parser → GameState → presence test proving Stone Town Details and the configured `zafaria` large image appear alongside verified local health.
- The project owner supplied an Application ID through `WIZRUST101_DISCORD_APP_ID` and confirmed the uploaded Zafaria asset key `zafaria`. The key is committed in `data/world-assets.json`; no Application ID is embedded in runtime code or launch documentation.

### M3 live Linux Steam and Discord verification

- On 2026-10-01, the watcher automatically discovered `/home/maxim/.local/share/Steam/steamapps/common/Wizard101/Bin/WizardClient.log` and connected to the available Discord Desktop IPC client on Linux.
- The project-owner supplied Discord profile screenshot (22:09:05 local; not committed because it contains personal profile information) shows the Wizard101 activity, `Stone Town`, Zafaria artwork, `Last logged health: 3868/3868`, and elapsed location time `0:24`. The owner confirms the elapsed timer was running.
- This verifies the integrated Linux Steam discovery → current-client data → verified mapping → presence model → Discord IPC/rendering path for the tested session. It does not verify Windows runtime discovery or named-pipe behavior. Reconnect/error recovery is covered by fake-transport tests; a live Discord restart test was not part of this screenshot evidence.
- The verified Linux launch form is `WIZRUST101_DISCORD_APP_ID=<id> cargo run --bin wizrust101-rpc`; replace `<id>` locally. The actual ID is not included in project source or documentation.

**M3 conclusion:** Closed for the scoped presence model and Linux IPC integration. Windows runtime validation remains a deferred release check, not an M3 blocker.

## M4: Level/School investigation (closed for current scope)

**Status:** Closed/cancelled as of 2026-10-02. The current WizardClient.log-only approach does not expose safely attributable selected-character Level or School values. No further capture is requested. At M4 close Health/none were the presence selector; M6 superseded this, removing the selector and all health display. Character Name remains unsupported and excluded.

The owner supplied two-character screenshots showing Level 74 / Balance and Level 1 / Death, with corresponding selection-to-zone sequences in the W.1.610.21 log. The log contains only pre-selection system/CPU `Level: 26.00` values and startup school progression messages, not the selected characters' values. The screenshots establish the game UI values but do not create a log parser source or local attribution. The [sanitized fixture notes](../tests/fixtures/current-steam-2026-10-02-README.md) and [research record](m4-plan.md) preserve the negative evidence and context; character names are omitted.

Do not obtain these values through memory scanning, process injection, packet interception, OCR, or guessed/indirect inference. Reconsider Level or School only if a future Wizard101 client exposes new reliable evidence attributable to the selected local character. Character Name is excluded and may not be exposed under this scope.

**Closure checks:** `cargo fmt --all --check` passed; `cargo test --all-targets` passed (51 tests); `cargo clippy --all-targets --all-features -- -D warnings` passed; `cargo check --target x86_64-pc-windows-gnu --all-targets` passed; `git diff --check` passed. This closure changes specifications only; no runtime behavior changed.

**Owner-reported art inventory:** Uploaded Discord keys are `aquila`, `avalon`, `azteca`, `celestia`, `dragonspyre`, `grizzleheim`, `khrysalis`, `krokotopia`, `marleybone`, `mooshu`, `wizard_city`, `wysteria`, and `zafaria`. This establishes artwork availability only. `data/zones.json` remains unchanged and `data/world-assets.json` still contains only the verified Zafaria mapping's asset entry.

## M5: Versioned per-user configuration

**Status:** Complete (2026-10-02).

Implemented `src/config.rs` with v1 JSON parsing, safe defaults, independent field validation, unknown-field round-trips, environment precedence, log verbosity filtering, and native config path resolution through `directories` 6.0.0. The watcher prefers a valid configured `WizardClient.log` and falls back to automatic discovery when it is missing or invalid. At M5 completion the development runtime accepted `WIZRUST101_DISCORD_APP_ID`; the revised release requirement is to embed the project ID in official builds while retaining the variable only as a development/testing override. No config is auto-created or rewritten; malformed and unsupported-version files remain unchanged.

**Acceptance criteria:**

1. Missing config uses defaults without prompting; auto-discovery remains normal.
2. Version 1 JSON loads from the platform-native user config path; file settings remain persistent across launches and take effect on the next start.
3. Only `health` / `none` and `error` / `warn` / `info` / `debug` are accepted for their respective settings; unsupported stats are rejected.
4. Malformed/unsupported-version files remain byte-for-byte untouched and safe defaults keep the watcher running; invalid log paths fall back to auto-discovery.
5. Config loading, precedence, path selection, validation, and recovery are covered without a live game or Discord.

**Checks:** `cargo fmt --all --check` passed; `cargo test --all-targets` passed (67 tests: 45 unit, 6 configuration integration, 15 ingestion integration, 1 presence integration); warning-denied Clippy passed; `cargo check --target x86_64-pc-windows-gnu --all-targets` passed; `git diff --check` passed. The Windows check is compile-only; per-user path behavior was executed on Linux and the Windows path is source-verified and compile-tested.

### Revised next milestone

**M6: Linux Flatpak tray app for Steam Wizard101. Status: accepted for the tested setup.** The owner reports a successful installed Flatpak/tray run using native Steam at `~/.local/share/Steam` and native Discord. Automatic log discovery, Discord presence, Stone Town Details, Zafaria State/artwork, and the running location timer were verified. The activity now includes the small project logo; Health is no longer displayed. This evidence does not verify Steam Flatpak, Discord Flatpak, or other combinations. The user's icon design in `packaging/flatpak/icon.svg` is preserved from commit `859ea78`. Windows runtime remains deferred; the Windows-target check is compile-only. Standalone is excluded from active discovery. Unknown mappings remain unknown. M7 is not started by this change.

The ordered initial distribution targets and later standalone gate are defined in [future.md](future.md). Keep user config free of the Discord Application ID; official builds embed it. Do not store Discord tokens, account credentials, or private game data.

### Distribution plan update (2026-10-02)

Updated product, architecture, game detection, configuration, Discord setup, testing, README, and roadmap specs for Steam-only active support and the ordered Linux Flatpak/Proton, Windows native Steam installer, and macOS Steam/CrossOver targets. Official releases embed the project Application ID; the environment override remains for development/testing and is not normal user configuration. Standalone support is gated until all three initial targets are complete and tested, then split into Chromebook `.deb`, native Windows, and researched macOS milestones. M6 removed standalone discovery from the active runtime. See [M6 plan](m6-plan.md).

Checks for this documentation update: `cargo fmt --all --check` passed; `cargo test --all-targets` passed (67 tests); `cargo clippy --all-targets --all-features -- -D warnings` passed; `cargo check --target x86_64-pc-windows-gnu --all-targets` passed; `git diff --check` passed. These checks do not establish Proton/Flatpak, Windows runtime, or CrossOver compatibility; those are per-milestone verification requirements.

### M6 Linux Flatpak implementation update (2026-10-02)

**Implemented:** Steam-only Linux discovery with researched native and Steam Flatpak roots; per-library manifest lookup; portal-granted external roots in a versioned app-data registry; automatic background watcher; StatusNotifier tray status/Add Steam library/Quit; Flatpak manifest, desktop entry, metadata, and icon; build script requiring a build-time embedded release ID while retaining the runtime development override. Unknown mappings and unsupported stats remain unchanged. No standalone paths participate in discovery.

**Host evidence:** The real native log is `~/.local/share/Steam/steamapps/common/Wizard101/Bin/WizardClient.log`. Steam metadata says app `799960`, install directory `Wizard101`. The `.steam/steam` and `.steam/root` aliases point to this root. No log was found under `compatdata/799960`, and no game/Proton process was active during inspection. Flathub's Steam manifest identifies private Steam data roots under `~/.var/app/com.valvesoftware.Steam`; no Steam Flatpak was installed locally. Thus native Steam log discovery is locally evidenced, while Proton runtime and Steam Flatpak paths remain candidates for live testing.

**Sandbox boundary:** The manifest permits read-only access to the known native Steam root/aliases and candidate Steam Flatpak roots, app-scoped Flatpak Discord socket directory, and native Discord `discord-ipc-0`. External Steam libraries need individual folder portal authorization. There is no home-wide, host-wide, device, or network access. Exact permissions and their sources are detailed in [M6 plan](m6-plan.md). Discord socket recreation under Flatpak grants remains unverified.

**Final M6 validation (2026-10-02):** `cargo fmt --all` passed; `cargo test --all-targets` passed (78 tests: 56 unit, 6 configuration integration, 15 ingestion integration, 1 presence integration); warning-denied Clippy passed; Windows-target `cargo check` passed; `git diff --check` passed. The first Flatpak build attempt found no `/dev/fuse`; the build helper now uses Flatpak Builder's supported `--disable-rofiles-fuse` staging mode. A sandboxed retry could not allocate a Flatpak build instance; the approved unsandboxed rebuild succeeded using the installed Freedesktop 26.08 SDK and Rust extension, and installed the Flatpak. The project ID was supplied only as a build environment value, not committed. AppStream compose completed successfully.

**Owner-reported live acceptance:** Installed Flatpak/tray with native Steam at `~/.local/share/Steam` and native Discord. Verified automatic log discovery, Discord activity, Stone Town Details, Zafaria State/artwork, and running elapsed location timer. Health is absent. M6 is accepted for this exact pairing. Steam Flatpak, Discord Flatpak, another library via portal, Discord restart recovery, and specific Proton behavior remain unverified. Windows is compile-tested only. No M7 work has begun.

**M6 follow-up retest after this commit:** launch `flatpak run io.github.msork.WizRust101RPC` with native Steam and native Discord running; enter or re-enter Stone Town. Confirm Discord shows `Stone Town` as Details, `Zafaria` as State, Zafaria large artwork, the project small image, and a timer that resets on a location change. Confirm no health text appears. The repository icon is `packaging/flatpak/icon.svg`; Discord small art is a separate uploaded asset.

The live test establishes only native Steam + native Discord with the installed Flatpak. Steam Flatpak, Discord Flatpak, specific Proton paths, Windows runtime, and macOS runtime remain unverified. M7 is the next milestone; it is not part of this work.

### M6 final visual acceptance (2026-10-02 13:23 local)

The owner reports the final installed-Flatpak retest passed with native Steam at `~/.local/share/Steam` and native Discord. The supplied screenshot visibly shows Wizard101, Stone Town as Details, Zafaria as State, Zafaria large art, the WizRust101-RPC small image, and an advancing location timer; no Health text appears. The screenshot contains personal Discord information and is not committed. This closes M6 for that exact pairing only; no Proton-specific, Steam Flatpak, Discord Flatpak, Windows, or macOS runtime claim is added.

## M7: Windows native Steam tray app and installer

**Status:** Accepted on Windows 11 on 2026-10-04. The owner verified the native MSVC release and Inno Setup package with Steam and Discord, including stationary restart restoration, Discord reconnect, expected presence artwork/timer, and tray Quit. Plan and evidence are recorded in [m7-plan.md](m7-plan.md).

**Scope:** Steam-only native Windows discovery through registry/default Steam roots and all metadata-listed libraries; validated tray action for an additional library; automatic log watcher; status, Add Steam library, Quit; same M6 Discord presence; embedded release app ID; per-user setup installer with Start Menu launch/uninstall. No standalone support or auto-start.

**Acceptance scope:** The owner-confirmed run covers installation and launch, native Steam discovery, native Discord RPC, stationary restart restoration, Discord reconnect, expected Stone Town/Zafaria payload and art, fresh timer, tray status, and Quit. The provided screenshots are excluded from version control because they show personal Discord information. Additional Steam library picker, installer upgrade/uninstall, selection-clear behavior, and other Steam/Discord packaging combinations were not included in this final report and are not claimed as live-tested here.

**Implemented:** Windows-only tray/event-loop module with embedded project icon, automatic shared watcher, live status menu item, validated native Add Steam library folder picker, persistent per-user Steam library registry, and Quit. Discovery uses `steamlocate` registry/library metadata plus the `%ProgramFiles(x86)%\Steam` fallback and resolves the app manifest's install directory. The installer source is a per-user Inno Setup 6 script with executable/icon installation, Start Menu launch/uninstall shortcuts, and standard uninstall registration; no auto-start entry. A Windows packaging workflow and build script embed the release Application ID from a build-only environment input. M6 presence construction and transport are reused unchanged.

**Checks for this correction:** `cargo fmt --all --check` passed; `cargo test --all-targets` passed (79 tests: 57 unit, 6 configuration integration, 15 ingestion integration, 1 presence integration); warning-denied Clippy passed on Linux and for the Windows GNU target; Windows GNU `cargo check --all-targets` passed; the generated x64 COFF `.rsrc` object contains the Common Controls v6 manifest; Flatpak rebuilt and installed successfully at commit `9b3eb39ff3f4b3a6153cef867e77dd9ce0003701db52f103e93e4b5b8d7c3d2e`; `git diff --check` passed. The PowerShell verifier and release EXE/MSVC resource inspection could not be executed here because this environment has no PowerShell, Windows SDK `mt.exe`, Windows MSVC toolchain, or Inno compiler. The CI verifier is configured to fail packaging before installer upload if either manifest/resource or runtime-load checks fail.

**Historical checklist:** This predated the final package contract and later owner acceptance. Current downloads are `WizRust101-RPC-Windows-Setup.zip` and `WizRust101-RPC-Windows-Portable.zip`; the final live evidence is recorded in the 2026-10-04 and 2026-10-05 M7 entries below. Do not use the older generated setup filename as a current download name.

### M7 packaged startup failure and manifest correction (2026-10-02)

The owner reports that the first installed Windows package failed before application/tray startup with `TaskDialogIndirect` missing. The application enabled `rfd` 0.17.2's `common-controls-v6` feature for its native startup error dialog. `rfd` source imports `TaskDialogIndirect` from ComCtl32 v6; `muda` 0.21.0 is also built with the tray's `muda-common-controls-v6` feature and imports that API for its About menu item. Windows must activate the `Microsoft.Windows.Common-Controls` 6.0 side-by-side assembly through the application manifest; without it, the default activation context does not provide the imported entry point. This matches the owner's reported error and the source-level dependency graph. No tray feature was removed.

Added `build.rs` using `embed-manifest` 1.5.1 to link a generated Common Controls v6 manifest into Windows EXEs. The Windows package build now runs `scripts/verify-windows-exe.ps1` against the actual `target/x86_64-pc-windows-msvc/release/wizrust101-rpc.exe`: Windows SDK `mt.exe` must extract resource #1, the XML must declare Common Controls version `6.0.0.0`, and a 15-second `--ci-load-check` process launch must exit successfully before Inno Setup runs. The probe returns before tray/watcher startup; the loader must still resolve the EXE's static imports. `cargo check --target x86_64-pc-windows-gnu --all-targets` confirms the build script emits a COFF `.rsrc` object, but this environment cannot produce or inspect the MSVC release EXE.

**M7 remains open:** The Common Controls manifest fix passed the owner's subsequent packaged startup. The owner then reported tray/discovery, native Steam/Discord, mapped Stone Town presence, reconnect, and Quit working. Startup restoration after quitting and relaunching in the same zone remains the specific live acceptance gate. Repeat that flow with the new package, then finish remaining installer/uninstall checks if not already covered. The initial failed screenshot is owner-supplied evidence and is not in the repository.

### M7 Windows retest and startup restoration (2026-10-02)

The owner reports that the corrected packaged app starts; tray, native Steam + native Discord, Stone Town presence, Discord reconnect, and Quit all work. The retest exposed one remaining bug: after quitting in Stone Town and relaunching without a zone change, presence does not return until the next zone event. M7 remains open. The planned correction replays the existing log snapshot into typed state before live tailing, preserving selection and unknown-zone events. The location timer will restart at app launch because this log evidence does not establish a reliable cross-restart entry timestamp.

The earlier candidate-reference comparison has been superseded by the preferred-source review in [zone-world-mapping.md](zone-world-mapping.md). The existing Stone Town row remains grounded in current-client raw-ID evidence and project-owner confirmation; no bulk import was made.

### M7 startup restoration implementation (2026-10-02)

Implemented streaming replay of the captured pre-tail log prefix after opening the incremental tailer at its current end offset. The ordinary parser and `GameState` transitions restore the most recent session location before the first presence publication; selection and unknown-zone records retain their normal clearing behavior. A restored location timer begins at app startup. Bytes appended during replay remain available from the tailer's captured offset and are consumed incrementally.

Verification: `cargo fmt --all --check`, `cargo test --all-targets` (84 tests: 61 unit, 6 configuration integration, 15 ingestion integration, 2 presence integration), host warning-denied Clippy, Windows GNU-target check and warning-denied Clippy, and `git diff --check` passed. These do not replace a fresh Windows installer/live test. M7 remains open until relaunching in a known zone restores presence without a zone change on Windows.

### M7 native Windows diagnosis and verification (2026-10-04)

The owner reproduced startup without presence and supplied the tray screenshot. Its status, `Watching Steam (waiting for verified game data)`, means `Presence::from_game_state` returned no payload; the watcher was not waiting for Discord. The native Steam `WizardClient.log` available during the investigation had a Stone Town zone event followed by a later `CHARACTER LIST` event. The parser and state machine correctly treat that later selection boundary as current and clear the location. This explains the status and preserves the requirement not to show a stale location while at selection.

Added structured replay counts (complete lines, zone events, selection events, health events) and a tray status that distinguishes a final selection event, an unmapped zone, and no recognized zone. These counters avoid raw log contents and account or character data. Added a pipeline test with existing selection-plus-zone history, no appended zone record, first Discord connection failure, and successful later publication; it verifies the complete Stone Town/Zafaria payload is sent once the transport reconnects. The existing later-selection test still ensures selection clears presence.

Native Windows tests also found that the Steam VDF fixture did not escape Windows path separators and Windows file identity based only on creation time could miss a rapid replacement. The fixture now writes valid escaped paths. The tailer uses the Windows handle's volume serial number/file index when available, retaining creation time as a fallback. Both native Windows regression tests now pass.

**Native validation:** `cargo fmt --all -- --check`; `cargo test --all-targets` (84 tests: 60 unit, 6 configuration integration, 15 ingestion integration, 3 presence integration); `cargo clippy --all-targets --all-features -- -D warnings`; and `git diff --check` all pass on `stable-x86_64-pc-windows-msvc`. The release MSVC executable builds successfully. `scripts/verify-windows-exe.ps1` extracted and validated its Common Controls v6 manifest and launched the same EXE with `--ci-load-check` successfully.

**M7 native validation before owner retest:** `cargo fmt --all -- --check`; `cargo test --locked --all-targets` (84 tests); warning-denied Clippy; MSVC release build; Common Controls v6 manifest extraction and loader probe all passed on Windows. These automated results are complemented by the live acceptance below.

### M7 final Windows 11 live acceptance (2026-10-04)

The owner confirms the native Windows MSVC release and Inno Setup package install and launch normally. With native Steam Wizard101 and native Discord, the tray reports `Watching Wizard101 (Steam)`, the running game is discovered, and a restart while already in Stone Town restores activity without a zone transition. Discord reconnect restores activity while the player remains stationary. Presence contains Stone Town Details, Zafaria State, Zafaria artwork, the WizRust101-RPC small image, and a fresh timer. Tray Quit works. The user-provided screenshots are not committed because they contain personal Discord information. M7 is accepted and closed. This evidence does not extend to additional Steam-library selection, installer upgrade/uninstall, or alternate Steam/Discord package pairings.

### M7 Windows release-blocker regressions (2026-10-05)

After the earlier acceptance, live tests found that zone transitions reset the elapsed timer, Discord restart could leave stale IPC without republishing until another log event, portable EXEs lacked the project icon resource, and immediate relaunch after tray Quit could start without a tray icon. Fixes now use one session timestamp retained across zones, validate each Discord command acknowledgement and heartbeat every five seconds so failed IPC reconnects and republishes retained state, embed and validate the project ICO in the executable, remove the reused fixed tray GUID, and drop the tray icon before an interruptible watcher join. Regression tests cover timer continuity, heartbeat reconnect with retained presence, and prompt watcher shutdown. Commit `96873988ca836160e791cb1f40678fe1c96ccb9d` passed native Windows tests, warning-denied Clippy, and the Windows packaging job. Linux and macOS checks/package workflows also passed for that commit.

The owner then live-retested both the Windows installer and portable build successfully for continuous timing across zone changes, Discord restart recovery without zone activity, stationary presence restoration, tray Quit and immediate relaunch, the portable executable icon, and known- and Unknown-world presence behavior. Screenshots containing personal Discord information are not committed. This reaccepts the Windows runtime and lifecycle behavior for both package types.

The owner requested shorter tray wording after that retest. Tray statuses now use `Status: Waiting for Wizard101`, `Status: Watching Wizard101`, and concise retry/reconnect/library-selection wording. Replay counts, log state, parser terminology, raw zone IDs, and other diagnostics remain in application logs rather than the normal tray UI. This text-only UX update is automated-test and platform-CI validated; it has not received a separate live owner retest.

## M8: native macOS menu-bar app with CrossOver-hosted Steam Wizard101

**Status:** Research and initial implementation started 2026-10-04. Target macOS 15+ with Apple Silicon as the first supported architecture. WizRust101-RPC itself must run natively on macOS; CrossOver hosts only Steam/Wizard101.

**Research decisions:** CodeWeavers documents the default private bottle directory, `BottleDir`/`CX_BOTTLE_PATH` customization, published bottle locations, bottle drive mappings, and Steam installation within a bottle. Discovery will inspect CrossOver bottle roots, map Steam library paths from Windows drive syntax through `dosdevices`, then validate Steam app `799960` from its manifest before opening `Bin/WizardClient.log`. Native Discord RPC uses Unix sockets and the existing retrying IPC transport. The detailed research and source list are in [M8 plan](m8-plan.md). WizRust101-DB remains the preferred name candidate source; unknown zones and world relationships remain unresolved.

**Packaging direction:** A native accessory/menu-bar `.app` bundle, optionally delivered in a signed/notarized `.pkg` for `/Applications`. Local unsigned development is supported; public distribution requires Developer ID signing, Hardened Runtime, and notarization. No sandbox or unrelated TCC permissions are planned. Apple Silicon `aarch64-apple-darwin` is first. A Universal 2 arm64+x86_64 build is feasible with `lipo` if native dependencies and current CrossOver support can be verified on Intel macOS 15+.

**Implemented so far:** CrossOver bottle discovery using documented roots and `BottleDir`/managed-directory preferences; Steam library path translation via bottle drive mappings; app `799960` manifest/log resolution; validated/persisted additional bottle selection; native accessory-mode menu-bar adapter; `.app`/`.pkg` builder with optional Developer ID signing/notarization; and GitHub Actions for native macOS tests, warning-denied Clippy, one Universal 2 build, final artifact inspection, embedded release-ID verification, and artifact upload. PRs run checks without secrets; package jobs use `WIZRUST101_RELEASE_DISCORD_APP_ID`. The uploaded `.app.zip` and `.pkg` files are unsigned development artifacts. Windows-native validation passes with the shared code. Workflow run [37248356437](https://github.com/msork/WizRust101-RPC/actions/runs/37248356437) passed on macOS 15 Apple Silicon. It uploaded [WizRust101-RPC-macOS-universal2](https://github.com/msork/WizRust101-RPC/actions/runs/37248356437/artifacts/11319534499), containing `WizRust101-RPC-universal2.app.zip` and `WizRust101-RPC-universal2.pkg`. Checks covered both Mach-O slices, bundle directories and executable permissions, `Info.plist` (`LSUIElement`, minimum macOS 15 and bundle metadata), `.icns` decoding, headless executable load and embedded Actions-secret match, unsigned package state and `/Applications` metadata, expanded package payload contents, and ZIP/payload executable equivalence. PR checks receive no secret. These artifacts are unsigned and not notarized.

**Open acceptance and release gates:** A real macOS 15+ Apple Silicon machine with CrossOver, Steam, and native Discord is required to validate discovery paths and permissions, menu-bar behavior, native RPC, stationary startup replay, zone/selection transitions, reconnect, and package installation. Developer ID Application/Installer signing, Hardened Runtime, notarization/stapling, Gatekeeper behavior, and install/upgrade/removal validation remain release gates. Hosted macOS CI and Windows portable tests do not close live acceptance. Do not begin M9 until M8 acceptance is complete.

## Cross-platform release artifact packaging (2026-10-04)

**Scope:** Standardize CI output filenames and add Linux release package builds without publishing a GitHub Release or changing platform watcher behavior. M8 remains open; no later milestone begins.

**Artifact contract:** Publish six actual ZIPs with no Actions-generated wrapper and exactly one installation method per ZIP: `WizRust101-RPC-Windows-Setup.zip` → `WizRust101-RPC-Windows-Setup.exe`; `WizRust101-RPC-Windows-Portable.zip` → `WizRust101-RPC-Windows-app.exe`; `WizRust101-RPC-macOS-App.zip` → Universal 2 `WizRust101-RPC.app`; `WizRust101-RPC-macOS-Pkg.zip` → `WizRust101-RPC-macOS.pkg`; `WizRust101-RPC-Linux-Flatpak.zip` → `WizRust101-RPC-linux.flatpak`; and `WizRust101-RPC-Linux-AppImage.zip` → `WizRust101-RPC-linux.AppImage`. CI validates extracted final archive contents and the Windows portable executable's icon/load behavior. `actions/upload-artifact@v7` uploads each finished ZIP with `archive: false`. Workflows upload artifacts only and do not create releases.

**Linux implementation:** The existing Flatpak 26.08 runtime, permissions, and app behavior remain unchanged. Flathub currently publishes the Freedesktop 26.08 SDK/runtime but returns 404 for the matching Rust stable SDK extension. To keep 26.08 without depending on an unavailable extension, `scripts/build-flatpak.sh` now builds the lockfile-pinned release executable with host Rust and packages that executable into the unchanged Flatpak runtime. The default helper still installs the Flatpak; CI can instead export a single-file bundle. `scripts/build-linux-packages.sh` builds that same release binary with the Actions-secret Discord ID and packages it as a Type 2 AppImage. The AppImage includes its executable, desktop file, icon, and AppRun launcher; it directly uses the existing Linux tray, log discovery, Steam-library chooser, and Discord RPC. `appimagetool` 1.9.1 is the selected upstream low-level AppDir packager because this app needs no framework deployment plugin; the standalone Type 2 runtime is explicitly pinned to version 20251108. CI resolves both asset digests from their version-tagged GitHub release metadata and checks downloaded bytes before execution. A separate headless check runs the actual AppImage runtime and installed Flatpak executable, confirms the embedded secret matches, checks executable permissions and icon/desktop resources, and verifies Flatpak's installed permissions/exported resources. Actual tray rendering and live Discord RPC still require a graphical Linux acceptance run. The manifest disables AppStream catalog composition because this is a standalone bundle rather than a Flathub catalog submission; it still packages the desktop entry, icon, and metainfo. Sources: [appimagetool usage](https://github.com/AppImage/appimagetool), [Type 2 runtime releases](https://github.com/AppImage/type2-runtime/releases), [AppImage run/extract guide](https://docs.appimage.org/user-guide/run-appimages.html), [Flatpak single-file bundles](https://docs.flatpak.org/en/latest/first-build.html#5-share-the-application), and [Flatpak manifest guidance for AppStream composition](https://github.com/flatpak/flatpak-docs/blob/master/docs/extension.rst).

**Workflow validation:** Linux, Windows, and macOS workflows run platform-native formatting, tests, and warning-denied Clippy, then perform non-PR package jobs using `WIZRUST101_RELEASE_DISCORD_APP_ID`. Pull requests never receive the secret. Linux CI uses Ubuntu 22.04 as the AppImage build baseline, installs the Flatpak 26.08 SDK/runtime, builds the release binary on the host, produces separate Flatpak and AppImage ZIPs, and validates each final payload. macOS produces and validates separate app-bundle and installer ZIPs while preserving app executable permissions. Windows validates the per-user Inno installer and standalone portable EXE inside their independent ZIPs. Workflows upload each finished ZIP without an additional archive wrapper. Package signatures/notarization are not added; macOS signing/notarization and live M8 acceptance remain release gates. Linux AppImage/Flatpak GUI/RPC acceptance remains open. Windows installer and portable behavior passed owner live acceptance before the concise tray-text change recorded above.

**Flatpak script mode:** The pre-existing worktree-only mode transition to non-executable is not part of the change. The README invokes `scripts/build-flatpak.sh` directly, so the tracked executable bit is required and remains `100755` in Git.
