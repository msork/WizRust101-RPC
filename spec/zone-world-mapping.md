# Zone and world mapping

## Required mapping

Resolve each verified raw zone identifier to:

- stable raw zone ID;
- human-readable location;
- world identifier/name;
- Discord asset key for the world's PNG, when an approved asset exists;
- provenance: source, retrieval/version date, and review status.

The mapping catalog is version-controlled data. Normalize only known, documented syntax variants; preserve unknown raw IDs for diagnostics and future mapping work.

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

[Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) is a required research reference for reviewing readable location and world names. Review relevant pages where practical and record page URL, retrieval date, and the specific names used as evidence. Community names can help validate display text, but observed current-client log IDs and context take precedence if they conflict. A community page alone does not prove that a raw log ID belongs to a named world; keep unknown zones and worlds unknown.

Prefer read-only MediaWiki API queries for this wiki when the API is accessible, using a descriptive User-Agent and a conservative request rate. Use page metadata, links, and categories to traverse relevant location and world pages without scraping normal HTML. If access is denied, stop before substituting another source or promoting new mappings; do not bypass access controls or require a manual export of hundreds of pages. On 2026-10-01, the normal page returned HTTP 403 and the API query for `Basic:Locations` returned HTTP 444 with a Cloudflare `Access Blocked` page. API hierarchy access therefore remains unverified, and no mapping was changed from this source.

## M1 import format

The importer accepts Bacon1661's legacy `zones.json`: top-level raw zone ID to display-location string entries, `zoneNames` mapping world ID to display name, and the `CHARACTER LIST` special entry. It produces the project's versioned catalog with provenance (`Bacon1661/Wizard101-RPC`, source file, import date, and `unverified-legacy` review status). The import command must be repeatable and must not silently resolve IDs absent from the input.

When an imported raw zone ID begins with a key present in `zoneNames` followed by `/`, that prefix may be retained as legacy world evidence. Other IDs, including short asset-group IDs, receive no inferred world. Imported display strings remain available for migration but are not marked current-verified.
