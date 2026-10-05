# Zone and world mapping

## Required mapping

Resolve each verified raw zone identifier to:

- stable raw zone ID;
- human-readable location;
- world identifier/name;
- Discord asset key for the world's PNG, when an approved asset exists;
- provenance: source, retrieval/version date, and review status.

The mapping catalog is version-controlled data. Normalize only known, documented syntax variants; preserve unknown raw IDs for diagnostics and future mapping work. Each row records provenance and may include typed evidence references (`current-client-log`, `project-owner-manual`, `wizard101-central-manual`, or `bacon-candidate`) plus an ISO verification date. A verified row must state which evidence verified the raw ID and which evidence supports its readable location/world. Do not call project-owner confirmation Wizard101 Central verification.

## Accuracy rules

- Never assign Wizard City or any other world as a fallback for an unknown identifier.
- For an unmapped zone, show a safe generic location label such as `Unknown location` (or omit location) and omit the world image.
- Do not infer that the first path segment is a world unless validated against the mapping catalog and fixtures.
- A mapping change must include evidence and tests for affected IDs.
- Track aliases where source logs have changed identifiers, rather than deleting old IDs without migration evidence.

## World images

Presence uses Discord application asset keys, not a local PNG file path. The image catalog therefore needs an approved PNG per supported world uploaded/configured under the app's Discord application. Until an asset is available, omit the large image while retaining verified text. Asset ownership, image sourcing, licensing, naming, and the application ID must be settled before a public build advertises complete world artwork.

The project owner reports that these Discord application art keys are already uploaded: `aquila`, `avalon`, `azteca`, `celestia`, `dragonspyre`, `grizzleheim`, `khrysalis`, `krokotopia`, `marleybone`, `mooshu`, `wizard_city`, `wysteria`, and `zafaria`. This inventory is separate from the DB location data and proves only that art is available under those keys. It does not create, verify, or imply any raw-zone-to-world or world-ID mapping. Add/use a key only after the relevant world mapping is independently verified.

## Data maintenance

Use [WizRust101-DB](https://github.com/msork/WizRust101-DB) as the canonical source for readable location names. Consume it as a pinned Git submodule at `vendor/WizRust101-DB`, reading `out/zones.json` and `out/zones.diagnostics.json` directly; do not copy its location table into RPC-owned data. The JSON map is a flat canonical-path-to-name object. Diagnostics are an array keyed by `path`, with `confidence`, selected candidate, and selection provenance. A DB `verified` entry can be shown directly. An unverified entry can be shown only when RPC has independent, exact-value evidence for that path; this evidence records the expected DB string so an upstream value change invalidates it. A value equal to the canonical path's final internal filename is always unresolved, as are `Unknown`, absent diagnostics, and unreviewed fallbacks. Preserve upstream files and diagnostics unchanged and pin the exact DB revision used by each RPC commit. DB data does not establish a world relationship. Verify each world separately; never infer one from the path prefix. Record independent evidence in the RPC-owned zone-world association file.

At DB commit [`d6318ea074be9a80fabebcd5b48908ff4543d7e1`](https://github.com/msork/WizRust101-DB/commit/d6318ea074be9a80fabebcd5b48908ff4543d7e1), `out/zones.json` contains 3,346 flat entries and maps `Zafaria/ZF_Z07_Stone_Town` to `Stone Town`. Diagnostics classify it as `unverified_fallback` (WizardZone-only candidate). RPC preserves that diagnostic verbatim and sources the displayed string only from the DB. The existing project-owner confirmation is recorded as independent exact-value evidence (`Stone Town`); it allows this one DB value to remain available without maintaining a second location map. The separate world association is also independently evidenced. Other unverified entries, especially raw internal filenames, remain hidden.

[Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) is the required authoritative research source for readable location and world relationships. Use the [176-page dated export](../research/wizard101central-locations.json) as an **incomplete** snapshot; record the captured page URL and export date for each reviewed name. Its links support research, but cannot alone prove containment or a raw game log ID. Current-client log evidence decides raw ID relationships and takes precedence in a conflict. Missing locations and worlds remain unknown unless verified elsewhere.

Automated HTML and MediaWiki API crawling are prohibited unless the site owner provides an authorized method. Earlier requests returned Cloudflare HTTP 403/444; the user reports a subsequent Error 1006/IP ban. Do not bypass the block. No manual export of hundreds of pages is expected.

Bacon1661 and WizRPC mappings are candidates with historical provenance. A Wizard101 Central name/link match can corroborate readable text, but does not upgrade a candidate raw ID to current-game verified. Keep the evidence types separate in reviews and catalog status.

The DB flat map contains `Zafaria/ZF_Z07_Stone_Town` → `Stone Town`, but its current diagnostics classify it as an unverified fallback. The RPC-owned association therefore records exact-value project-owner evidence for `Stone Town`, alongside the independently verified world `Zafaria`, based on the sanitized current-client `W.1.610.21` fixture and project-owner confirmation on 2026-10-01. The display string itself still comes only from the DB. Wizard101 Central was inaccessible and was not consulted for this mapping. The 176-page export's `Location:Zafaria` link to the uncaptured `Stone Town` page and Bacon's legacy labels are background candidates only; they are not cited as verification. Other zones remain unresolved unless the pinned DB diagnostics mark the exact canonical path verified or there is separate exact-value evidence.

## Historical import behavior

M1 included a Bacon catalog importer and a one-row RPC-owned runtime catalog. These are retired. Current builds read the pinned WizRust101-DB flat map and diagnostics; RPC retains only separate, independently sourced zone-world evidence and approved world artwork keys. No world is inferred from a canonical path prefix.
