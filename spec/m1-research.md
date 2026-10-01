# M1 research and plan

Research checked 2026-10-01. Upstream implementation behavior establishes legacy compatibility patterns only; it does not prove the current Wizard101 client emits the same records.

## Findings

- Bacon1661's parser matches zone records containing \`zone = <zone-id>,\`, health records containing \`Updating health globe (new health: N, new health max: M)\`, and the literal \`CHARACTER LIST\`. It also attempts to reject records associated with another client. See the [reference parser](https://github.com/Bacon1661/Wizard101-RPC/blob/master/index.js).
- The fetched Bacon \`zones.json\` is 186,831 bytes. Its top-level string entries map raw zone IDs to display locations; a \`zoneNames\` object maps world IDs to display names, and \`CHARACTER LIST\` maps to \`Character Selection\`. Example: \`WizardCity/WC_Ravenwood\` maps to \`Ravenwood\`. See the [upstream catalog](https://github.com/Bacon1661/Wizard101-RPC/blob/master/zones.json). This catalog is historical and imports as unverified legacy data.
- The [Steam Wizard101 page](https://store.steampowered.com/app/799960/Wizard101/) verifies app ID \`799960\` and notes that the NA Steam client began using an all-file install process on November 13, 2024. Discovery should obtain the install directory from Steam's app manifest rather than assume directory spelling.
- [\`steamlocate\` 2.1.1](https://docs.rs/steamlocate/latest/steamlocate/) documents locating Steam roots, enumerating libraries, and finding installed apps by ID. It is the M1 candidate dependency.
- A public captured current-build raw log sample was not found. Exact current formatting and local-character health attribution therefore remain unverified.

## Fixture decision

M1 can include minimal reconstructed log fixtures matching the exact literals in the reference parser. Each fixture is labeled \`reference-derived\`, not as a real or current captured log. Tests against them verify only compatibility with documented legacy patterns. Current-build support requires a sanitized real sample and a Windows smoke check.

## M1 plan

1. Update product/architecture/parser/discovery/mapping/testing/status specs with research, scope, acceptance gates, and limitations.
2. Create a Rust library and small binaries for watching and mapping import.
3. Implement standalone and Steam discovery plus incremental tailing with buffered partial lines and file replacement/truncation handling.
4. Implement typed parser events, unknown-safe mapping, versioned catalog loading, and a legacy Bacon catalog importer.
5. Add provenance-labeled fixtures and comprehensive unit/filesystem integration tests.
6. Run \`cargo fmt\`, \`cargo test\`, and \`cargo clippy --all-targets --all-features -- -D warnings\`; record exact outcomes and remaining limitations.

## Acceptance gates

- No full-log reread during normal tail operation; tests prove only the appended byte range is consumed.
- Discovery tests cover standalone roots, Steam app-manifest install directories, multiple libraries, absent files, and malformed metadata.
- Parser/state/mapping tests cover recognized and malformed records, health values, menu selection, unknown zones/worlds, duplicate location transitions, and state preservation on invalid lines.
- The Bacon importer preserves every legacy raw ID and location string, maps world metadata only with explicit evidence, and marks all imported rows unverified legacy.
- No Discord, GUI, settings UI, or gameplay controls are introduced.
