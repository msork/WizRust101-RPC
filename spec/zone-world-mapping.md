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

## Data maintenance

Reference repositories contain zone mapping catalogs but are old and are not authoritative for current coverage. Treat them as candidate inputs only. Future contributions should include raw IDs, display names, world assignment evidence, and fixture coverage.

[Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) is the required authoritative research source for readable location and world relationships. Use the [176-page dated export](../research/wizard101central-locations.json) as an **incomplete** snapshot; record the captured page URL and export date for each reviewed name. Its links support research, but cannot alone prove containment or a raw game log ID. Current-client log evidence decides raw ID relationships and takes precedence in a conflict. Missing locations and worlds remain unknown unless verified elsewhere.

Automated HTML and MediaWiki API crawling are prohibited unless the site owner provides an authorized method. Earlier requests returned Cloudflare HTTP 403/444; the user reports a subsequent Error 1006/IP ban. Do not bypass the block. No manual export of hundreds of pages is expected.

Bacon1661 and WizRPC mappings are candidates with historical provenance. A Wizard101 Central name/link match can corroborate readable text, but does not upgrade a candidate raw ID to current-game verified. Keep the evidence types separate in reviews and catalog status.

The current runtime catalog contains one project-owner verified row: `Zafaria/ZF_Z07_Stone_Town` → `Stone Town` → `Zafaria`. The raw ID is recorded in sanitized current-client `W.1.610.21` fixtures; the project owner confirmed the readable location/world relationship on 2026-10-01. Wizard101 Central was inaccessible and was not consulted for this mapping. The 176-page export's `Location:Zafaria` link to the uncaptured `Stone Town` page and Bacon's legacy labels are background candidates only; they are not cited as verification. Other raw Zafaria IDs in older logs remain unmapped until separately verified. Future manual Wizard101 Central research should record the exact page and date as `wizard101-central-manual` evidence, alongside current-client evidence for raw IDs.

## M1 import format

The importer accepts Bacon1661's legacy `zones.json`: top-level raw zone ID to display-location string entries, `zoneNames` mapping world ID to display name, and the `CHARACTER LIST` special entry. It produces the project's versioned catalog with provenance (`Bacon1661/Wizard101-RPC`, source file, import date, and `unverified-legacy` review status). The import command must be repeatable and must not silently resolve IDs absent from the input.

When an imported raw zone ID begins with a key present in `zoneNames` followed by `/`, that prefix may be retained as legacy world evidence. Other IDs, including short asset-group IDs, receive no inferred world. Imported display strings remain available for migration but are not marked current-verified.
