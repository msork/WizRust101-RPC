# Rich Presence zone coverage audit

Audited the complete pinned WizRust101-DB submodule at `d6318ea074be9a80fabebcd5b48908ff4543d7e1`. Counts use the exact flat map, per-path diagnostics, RPC-owned exact-path world evidence in `data/zone-worlds.json`, and registered world asset keys in `data/world-assets.json`. No world is inferred from a path prefix.

## Counts

| Measure | Zones | Meaning |
|---|---:|---|
| Total DB zones | 3,346 | Every canonical path in `out/zones.json`; the diagnostics file has a corresponding record for each. |
| Direct DB `verified` names | 1,240 | Diagnostics mark the selected location as `verified`. |
| Displayable Details | 1,241 | The 1,240 directly verified names plus Stone Town, whose exact DB value has separate project-owner evidence. |
| Suppressed raw/internal filename values | 26 | Values equal the final path filename; all are suppressed even if another world association is later added. |
| Other suppressed `unverified_fallback` names | 2,079 | Of 2,080 fallback rows, the one Stone Town value has exact independent evidence; the rest remain hidden. |
| Verified world State | 1 | Only `Zafaria/ZF_Z07_Stone_Town` has an exact-path world association. |
| Registered large artwork | 1 | The same path resolves to the registered `zafaria` asset through its Zafaria association. |
| Correct Details + State + artwork | 1 | Stone Town / Zafaria / Zafaria artwork. |
| Details missing world mapping and dependent artwork | 1,240 | These have readable Details but no independently verified world association. |
| Details with world mapping but missing artwork | 0 | No current world association is missing its registered art. |
| Missing readable Details | 2,105 | 2,079 unreviewed fallback names plus 26 raw/internal filename values. |

DB diagnostic totals are 1,240 `verified`, 2,080 `unverified_fallback`, and 26 `unknown`. “Registered artwork” means a key exists in the runtime RPC world-asset registry, not merely that the owner reports an asset uploaded to Discord.

## Regression coverage

`pinned_database_full_presence_coverage_is_audited_and_stable` iterates every pinned path and diagnostic. It asserts the totals above, checks the user-facing `Presence` Details/State/art for every zone, proves raw filenames never become Details, and verifies that only exact-path world evidence supplies State. Fixed examples cover Wizard City, Krokotopia, Marleybone, MooShu, DragonSpire, Celestia, Zafaria, Avalon, Azteca, Khrysalis, Aquila, Wysteria, Grizzleheim, housing, and gauntlets. Wizard City's Ravenwood candidate is also checked as an unverified fallback and stays hidden.

## Remaining coverage work

- Add reviewed, exact-path world associations for the 1,240 currently displayable Details zones that lack State; add their world IDs to the artwork registry only after separately verifying the mapping and registering the Discord asset.
- Review the 2,079 non-raw fallback candidates against independent evidence or wait for WizRust101-DB diagnostics to verify them. Preserve the raw values and provenance in either case.
- The 26 raw/internal filename values require a readable name source; never display those filenames as location Details.
- Linux packaging CI can validate builds and tests, but no Linux desktop/game/Discord live test was available for this audit.

The hard-coded audit totals intentionally fail when the submodule data changes, requiring a deliberate re-audit before updating the Gitlink.
