# Full Rich Presence coverage audit

Audited every canonical path in the pinned WizRust101-DB submodule at `abb4ee95c9422ec7e8ce3889d8765b0caa26846b`. The generated `out/zones.json` is a JSON object whose values have exactly two string fields, `world` and `zone`. `out/zones.diagnostics.json` is a parallel array of resolver diagnostics and is not used to gate or rewrite presence values.

For normal values, RPC uses `zone` exactly for Details and `world` exactly for State. It omits Details when zone is exactly `Unknown`. It omits State when world is `Unknown` or `House`; House uses the `house` large image without showing the word “House”. Unknown worlds use generic `wizard101` art. If both values are Unknown, the timer and standard large/small images remain present. The small image remains `wizrust101_rpc`. No path-based world or housing inference is performed.

## Counts

| Measure | Zones | Meaning |
|---|---:|---|
| Total DB zones | 3,346 | Canonical path entries in `out/zones.json`; all have one diagnostics record. |
| Known DB worlds | 3,346 | Includes 495 entries with `world` = `House`. |
| `House` world entries | 495 | State omitted; Details retained; `house` art selected. |
| Unknown worlds | 0 | This DB revision has no `world` = `Unknown` entries; behavior is covered synthetically. |
| World-art matches | 3,005 | 2,510 ordinary exact-world art matches plus 495 House entries. |
| `wizard101` fallbacks | 341 | Known non-House world entries without registered matching art. |
| Zones missing Details | 0 | This DB revision has no zone value `Unknown`; behavior is covered synthetically. |

Counts of art matches and fallbacks are per zone, not distinct world labels. `world-assets.json` maps exact DB world strings to Discord asset keys; it does not infer zone membership.

## Regression coverage

`pinned_database_audit_enforces_exact_details_state_and_art_for_every_zone` iterates all 3,346 DB entries and checks the production GameState-to-Presence path against each exact JSON value. It checks normal worlds, House, Unknown omission rules, artwork selection, diagnostics coverage, and small-art identity. A synthetic regression covers Unknown zone and both values Unknown because this DB revision does not currently contain those values. Tests also use a House value on a non-Housing path and a normal world on a Housing path to ensure behavior comes from DB data only.

## Remaining coverage work

- 341 zones have recognized non-House world values without matching registered art; they retain exact State and use generic `wizard101` art.
- Linux live testing is unavailable in this environment. Linux packaging/runtime build validation is performed through GitHub Actions only; a graphical Linux Steam/Discord acceptance run remains outstanding.

No name or world review table is duplicated in RPC. Update the DB submodule pin and re-run the complete audit whenever the upstream generated output changes.
