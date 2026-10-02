# M4 investigation: Level, School, and Character Name (closed)

## Status and decision

**Closed/cancelled on 2026-10-02 for the current WizardClient.log-only product scope.** The controlled two-character comparison did not find safely attributable Level or School records. No further captures are requested. M6 subsequently removed Health and the `none` selector from Discord presence; health is internal only. Character Name remains unsupported and excluded.

Reconsider Level or School only if a future Wizard101 client exposes new reliable evidence attributable to the selected local character. Do not use memory scanning, process injection, packet interception, OCR, or guessed/indirect values. Character Name remains excluded from this project scope.

## Preserved research

- The 2026-10-01 and 2026-10-02 Linux Steam logs identify client `W.1.610.21` / revision `r806919.Wizard_1_610`.
- The earlier log has six case-insensitive `level` hits, no `school` hits, and only two numeric `Level: 26.00` candidates: startup system information and a CPU capability group. Neither is character data. The other hits are incidental startup/resource/diagnostic text.
- The controlled two-character screenshots show Character A at Level 74 (owner-identified School Balance) and Character B at Level 1 (owner-identified School Death). The corresponding log has `CHARACTER LIST` / zone transitions aligned to screenshot file-time proxies, but no Level 74/Level 1 records and no selected-character School field. `Balance` and `Death` appear only in startup school-progression messages tracking both schools, before selection. Character names are intentionally omitted from this record.
- The visible screenshots verify UI values, not a parser source or local log attribution. No Level or School state, parser, or selector option was implemented. Character Name is excluded.
- Bacon1661's parser contains zone and health extraction but no Level/School parser. ManaUp/WizRPC does not document a current-client Level/School record shape or attribution rule. Neither source overrides observed current-client evidence.
- Sanitized log anchors and analysis are retained in [research.md](research.md), [log-parsing.md](log-parsing.md), and the [fixture notes](../tests/fixtures/current-steam-2026-10-02-README.md). Screenshots and names are not committed.

## Historical plan completion

The original investigation planned to inspect current-client candidates, compare a controlled two-character selection-to-world sequence, and implement only independently attributable fields. The supplied comparison completed that evidence gate negatively. Existing regression tests ensure the CPU/system `Level: 26.00` candidate emits no game event. At M4 close Health remained the default and unsupported stats were rejected; M6 later removed the Health/None selector entirely. No additional implementation or capture is pending.

## M4 acceptance result

- **Met:** Current-client evidence and reference implementations were reviewed; the screenshot/log sequence was compared; negative evidence was sanitized and preserved.
- **Met (at M4 close):** No unsupported Level/School values were added to GameState or Discord presence. The then-current Health/none selector was subsequently removed in M6.
- **Met:** Character Name remains excluded; prohibited extraction/inference methods are explicitly out of scope.
- **Closed:** M4 has no remaining evidence gate. New evidence would require a new scoped milestone only if the Wizard101 client itself exposes reliable, locally attributable data.

Windows runtime discovery remains a separately documented deferred validation item; Windows is compile-tested only.
