# October 2 controlled character comparison

The user supplied four screenshots from a Linux Steam session running client `W.1.610.21`. They identify two different selected characters: the first screenshot pair shows Level 74 / Balance and the second shows Level 1 / Death. The screenshots and their visible character names are not committed. File modification times are capture-time proxies, not instrumented game timestamps:

| Image order | File time (local) | UI evidence |
| --- | --- | --- |
| 1 | 10:29:44.967 | First character selected; UI shows Level 74. The user identifies its School as Balance. |
| 2 | 10:30:17.536 | First character in-world in Stone Town. |
| 3 | 10:31:14.506 | Second character selected; UI shows Level 1. The user identifies its School as Death. |
| 4 | 10:31:29.986 | Second character in-world. |

`current-steam-2026-10-02-selection-stats.log` contains minimized, non-contiguous lines from the same 2,117-line log. The version record is line 2. Startup records at lines 63–64 list progression for both Death and Balance; lines 87 and 124 contain the only explicit numeric `Level` fields, both `26.00`, with the latter inside the CPU group. Both occur before the selection cycles. The two relevant `CHARACTER LIST` / zone pairs come from lines 742/892 and 1957/2010. Actor IDs are replaced with `<redacted-id>`; no character names or machine details are included.

The selection and zone timestamps align with the screenshot order, but the log contains no Level 74 or Level 1 character-stat record, no `School` field, and no local/selected-character ownership marker for Level or School. The school words are startup class-progression records listing both schools, not evidence of which school belongs to the selected character. This capture verifies the on-screen values and a two-character selection sequence; it does **not** verify a parseable, locally attributable Level or School log source.
