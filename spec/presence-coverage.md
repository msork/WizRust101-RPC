# Full Rich Presence coverage audit

Audited every canonical path in the pinned WizRust101-DB submodule at `56c9e4489f0b8f296f5dc016eb7d3579adbf1977`. The generated `out/zones.json` is a JSON object whose values have exactly two string fields, `world` and `zone`. `out/zones.diagnostics.json` is a parallel array of resolver diagnostics and is not used to gate or rewrite presence values.

RPC uses each `zone` value exactly as supplied for Details, including intentional internal/canonical-leaf troubleshooting fallbacks and whitespace. A `world` value other than the exact string `Unknown` is used verbatim for State. For `Unknown`, State is omitted. RPC never derives a world from a path segment. World artwork is selected by an explicit exact-DB-world-name registry; if no art is registered or the world is Unknown, the generic Discord asset key `wizard101` is used. The small image remains `wizrust101_rpc`.

## Counts

| Measure | Zones | Meaning |
|---|---:|---|
| Total DB zones | 3,346 | Canonical path entries in `out/zones.json`; all have one diagnostics record. |
| Known worlds | 2,713 | Entries whose DB `world` value is not `Unknown`. |
| Unknown worlds | 633 | Entries whose DB `world` value is exactly `Unknown`; State is omitted. |
| World-art matches | 2,510 | Entries with a known DB world and a matching exact-name registry key. |
| `wizard101` fallbacks | 836 | 633 Unknown-world entries plus 203 known-world entries without registered matching art. |
| Zones missing Details | 0 | All DB `zone` values are non-empty and passed through exactly. |

Counts of art matches and fallbacks are per zone, not distinct world labels. `world-assets.json` maps exact DB world strings to Discord asset keys; it does not infer zone membership.

## Regression coverage

`pinned_database_audit_enforces_exact_details_state_and_art_for_every_zone` iterates all 3,346 DB entries and checks the production GameState-to-Presence path against each exact JSON value. It checks world omission, matching-art and generic-art selection, the diagnostics count, zero missing Details, and small-art identity. Explicit examples span Wizard City, Krokotopia, Marleybone, MooShu, Dragonspyre, Celestia, Zafaria, Avalon, Azteca, Khrysalis, Aquila, Wysteria, Grizzleheim, housing, and gauntlets. `Test/Court_Test` confirms that an Unknown world does not hide the intentional `Court_Test` Details value, does not emit State, and uses `wizard101`.

## Remaining coverage work

- The DB has 633 zones whose world is Unknown; their exact DB Details remain shown, but State is intentionally omitted and generic art is used.
- Of the 2,713 zones with known worlds, 203 currently have no registered matching artwork; they retain exact State and use generic `wizard101` art.
- Linux live testing is unavailable in this environment. Linux packaging/runtime build validation is performed through GitHub Actions only; a graphical Linux Steam/Discord acceptance run remains outstanding.

No name or world review table is duplicated in RPC. Update the DB submodule pin and re-run the complete audit whenever the upstream generated output changes.
