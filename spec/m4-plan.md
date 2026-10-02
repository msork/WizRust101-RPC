# M4: Verified Level and School stats

## Objective

Research current-client sources and add only Level and/or School values that can be reliably attributed to the selected local character. Add supported values to typed GameState and the existing Discord stat selector. Keep Health as the default. Character Name is excluded until the project owner makes a separate explicit privacy/product decision.

## Source and attribution gate

- Start with exact records in the sanitized/current client `W.1.610.21` Linux Steam `WizardClient.log` available for offline research, plus permitted reference implementations.
- For each candidate value, establish its record shape, selected-character association, and reset/lifecycle markers. Nearby log order, a plausible value, or a character name alone does not establish local attribution.
- If a candidate source cannot be distinguished from another character or an unknown owner, leave it unsupported and state what evidence is missing. Do not invent parser syntax or values.
- Do not publish or commit personal character names or full logs. Character Name presence stays out of M4.

## Research result (2026-10-02)

- The available Linux Steam `W.1.610.21` log has six case-insensitive `level` text hits and no `school` hits. Two hits are explicit values at 22:27:37 (`SYSTEM INFO: Level: 26.00`) and 22:27:45 (`<Value name="Level">26.00</Value>`). The latter is inside the `CPU` group, among CPU capability values; the first is in the same startup/system-information block. These are hardware/system information, not evidence of the Wizard101 character's level.
- The other four hits are incidental startup/resource/diagnostic text, not character-stat records. Their values and context do not establish game Level.
- No candidate record is tied to the selected Wizard101 character. No Level or School field will be added to GameState or offered in `WIZRUST101_DISPLAY_STAT` from this evidence.
- Existing selector behavior already offers verified Health (default) and `none`. This milestone will regression-test that `level` and `school` remain rejected until their own sources pass the evidence gate.
- Bacon1661's current parser matches zones and health only. ManaUp/WizRPC is archived and its README claims live stats generally, but the available documentation does not define a Level/School log source or local attribution rule. Neither reference establishes current-client truth.

To revisit Level/School, provide a short sanitized W.1.610.21 log excerpt and timestamped screenshots from a controlled session: capture the visible character level and school after character selection, include the adjacent `CHARACTER LIST`/selection records and subsequent stat-like records through world entry, and repeat after selecting a different character if available. Preserve record keywords, punctuation, timestamps, record order, and equality/inequality of any IDs while redacting their values and all character/account names. This is needed to tie each candidate record to the selected character and distinguish it from hardware/system output. A screenshot of the values without adjacent log records is insufficient.

## Plan

1. **Complete:** Inspect available W.1.610.21 log candidates and legacy references; record evidence and its attribution limits.
2. **Complete before code:** Update parser, state, configuration, presence, and test specs with the unsupported-candidate decision.
3. Add typed parser events and GameState fields only if new evidence passes local-character attribution. No such source is currently supported; do not add speculative fields.
4. Keep Health as the default and `none` as the only other selector value until Level/School pass the evidence gate. Unknown selected data must be omitted.
5. Add a sanitized regression fixture for the system-information `Level` candidate and tests that it is ignored and unsupported selector values remain rejected.
6. Run `cargo fmt`, `cargo test`, warning-denied Clippy, Windows-target `cargo check`, and `git diff --check`; update status with exact evidence and remaining gaps.

## Acceptance criteria

- Every exposed stat has a current-client fixture, local-character attribution evidence, typed GameState representation, and presence formatting test. For this evidence slice, Health remains the only game stat that passes this gate.
- Unknown or ambiguously owned records cannot populate displayed stats.
- Level and School are exposed individually only if their own sources pass the evidence gate. If neither does, report the missing evidence and leave both unsupported without guessing.
- Health remains the default. No GUI, character-name publishing, memory inspection, or remote data collection is included.
- Linux runtime behavior may be checked with the available Steam session. Windows remains compile-tested only until a Windows session is available.

## User decisions

No new decision blocks M4. Character Name publishing requires a separate explicit owner decision and is outside this milestone.
