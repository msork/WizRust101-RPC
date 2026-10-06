# Zone and world mappings

## Authoritative source

[WizRust101-DB](https://github.com/msork/WizRust101-DB) is the authoritative source for both location and world values. RPC pins it as a Git submodule at `vendor/WizRust101-DB` and compiles directly from `out/zones.json`; do not copy the database into RPC-owned tables. Use DB values without deriving display names from paths. For a normal world, `zone` is Details and `world` is State. A DB world of `House` keeps the zone as Details, omits State, and selects the `house` image. A DB world of `Unknown` omits State and uses generic `wizard101` art. A DB zone of `Unknown` omits Details. If both values are Unknown, presence still contains the session timer and the standard large and small icons. Never display the literal `House` or `Unknown` in presence text.

The pinned generated schema is:

```json
{
  "Aquila/AQ_Z01_MountOlympus": {
    "world": "Aquila",
    "zone": "Mount Olympus"
  }
}
```

`out/zones.diagnostics.json` is a parallel diagnostic array describing candidate provenance and resolver confidence. It is inspected and audited alongside the generated output, but diagnostics do not modify the selected `zone`/`world` strings or suppress presence. Keep both upstream files unchanged. Housing paths receive no special handling; their DB `world` value alone determines presence behavior.

## Discord artwork

`data/world-assets.json` explicitly maps exact DB world display strings to registered Discord large-image keys. For normal worlds, show a matching asset and use the DB world as hover text; if no image is registered, use generic `wizard101`. `House` uses the registered `house` key with no world hover text. `Unknown` uses generic `wizard101`. The small image is `wizrust101_rpc` with hover text `WizRust101-RPC`.

Artwork registration is independent from mapping generation. Never manufacture a zone/world relationship from an image filename, a canonical path, historical candidate tables, or a human-readable substring. A source DB update, world-art update, or schema change must be checked by the complete dataset audit in [presence-coverage.md](presence-coverage.md).

## Data maintenance

Pin a reviewed WizRust101-DB commit and use its generated `zones.json` schema directly at runtime. Review `zones.diagnostics.json` for changes in source evidence and confidence, and report full-dataset coverage counts. When DB output changes shape or gains/removes entries, update the parser/tests and consciously re-baseline counts. Runtime does not access the network.

The dated Wizard101 Central export and historical Bacon mappings remain research artifacts only; they are not parallel runtime location/world sources. Automated crawling of Wizard101 Central remains prohibited under the recorded access restriction. Use the DB project as the preferred mapping source unless the project owner explicitly directs otherwise.
