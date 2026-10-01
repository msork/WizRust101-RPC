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

## M1 import format

The importer accepts Bacon1661's legacy `zones.json`: top-level raw zone ID to display-location string entries, `zoneNames` mapping world ID to display name, and the `CHARACTER LIST` special entry. It produces the project's versioned catalog with provenance (`Bacon1661/Wizard101-RPC`, source file, import date, and `unverified-legacy` review status). The import command must be repeatable and must not silently resolve IDs absent from the input.

When an imported raw zone ID begins with a key present in `zoneNames` followed by `/`, that prefix may be retained as legacy world evidence. Other IDs, including short asset-group IDs, receive no inferred world. Imported display strings remain available for migration but are not marked current-verified.
