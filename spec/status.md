# Milestone status

## M0: Product and architecture specification

**Status:** Complete as a document set; Git version control is currently unavailable in the supplied workspace.

**Scope:** Research reference implementations and Discord/Rust IPC options; define product, architecture, discovery, parser, state, mapping, RPC, configuration, testing, and future-feature requirements. No app implementation.

**Plan:** Inspect workspace and reference sources; record evidence and unknowns; author the spec set; review coverage against the product requirements.

**Result:** Specs created under `spec/`. No runtime code or tests were added.

**Verification:** Reviewed the files against the requested spec topics and product requirements. Not applicable to execute application tests in this documentation-only milestone.

**Environment limitation:** The supplied workspace contains no project files. `.git` is an empty, read-only directory and `git rev-parse --show-toplevel` reports that this is not a Git repository. The files are ready for version control, but cannot be committed or made version-controlled here unless repository metadata is restored/initialized with write access.

## Next milestone candidates

1. M1: Windows game/log discovery research and sanitized current-version fixture collection; update specs before implementation.
2. M2: parser, mapping schema, and game-state model design with fixture-backed acceptance tests.
3. M3: first executable vertical slice: discovery -> parser -> state -> Discord IPC.

## Open decisions for the product owner

- Which Discord application should own the published Rich Presence and world image assets? A public release needs a stable application ID and assets; this can be deferred until the IPC/asset milestone.
- Should Health be the default stat when known, or should the state line be blank until the user chooses a stat?
- Should the eventual app be console/tray-only, or does the first public release need a GUI? The architecture currently assumes a small background app and defers UI.
- Should a verified character name ever be published to Discord by default, given it is personal profile information? It remains an opt-in future stat until decided.
