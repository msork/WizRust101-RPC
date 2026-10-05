# Zone and world mappings

## Authoritative source

[WizRust101-DB](https://github.com/msork/WizRust101-DB) is the authoritative source for both location and world values. RPC pins it as a Git submodule at `vendor/WizRust101-DB` and compiles directly from `out/zones.json`; do not copy the database into RPC-owned tables. For every canonical path, use the generated `zone` string exactly as Discord Details and the generated `world` string exactly as Discord State unless it equals `Unknown`, in which case omit State. Do not suppress fallback names and do not infer world from canonical paths or their first segment.

The pinned generated schema is:

```json
{
  "Aquila/AQ_Z01_MountOlympus": {
    "world": "Aquila",
    "zone": "Mount Olympus"
  }
}
```

`out/zones.diagnostics.json` is a parallel diagnostic array describing candidate provenance and resolver confidence. It is inspected and audited alongside the generated output, but diagnostics do not modify the selected `zone`/`world` strings or suppress presence. Keep both upstream files unchanged.

## Discord artwork

`data/world-assets.json` explicitly maps exact DB world display strings to registered Discord large-image keys. When a DB world has a matching registry entry, show that asset and the DB world label as hover text. If the DB world is `Unknown` or the exact world label has no registered image, use the generic Discord asset `wizard101`; Unknown still omits State. The small image is `wizrust101_rpc` with hover text `WizRust101-RPC`.

Artwork registration is independent from mapping generation. Never manufacture a zone/world relationship from an image filename, a canonical path, historical candidate tables, or a human-readable substring. A source DB update, world-art update, or schema change must be checked by the complete dataset audit in [presence-coverage.md](presence-coverage.md).

## Data maintenance

Pin a reviewed WizRust101-DB commit and use its generated `zones.json` schema directly at runtime. Review `zones.diagnostics.json` for changes in source evidence and confidence, and report full-dataset coverage counts. When DB output changes shape or gains/removes entries, update the parser/tests and consciously re-baseline counts. Runtime does not access the network.

The dated Wizard101 Central export and historical Bacon mappings remain research artifacts only; they are not parallel runtime location/world sources. Automated crawling of Wizard101 Central remains prohibited under the recorded access restriction. Use the DB project as the preferred mapping source unless the project owner explicitly directs otherwise.
