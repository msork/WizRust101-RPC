# M4: Verified Level and School stats

## Objective

Research current-client sources and add only Level and/or School values that can be reliably attributed to the selected local character. Add supported values to typed GameState and the existing Discord stat selector. Keep Health as the default. Character Name is excluded until the project owner makes a separate explicit privacy/product decision.

## Source and attribution gate

- Start with exact records in the sanitized/current client `W.1.610.21` Linux Steam `WizardClient.log` available for offline research, plus permitted reference implementations.
- For each candidate value, establish its record shape, selected-character association, and reset/lifecycle markers. Nearby log order, a plausible value, or a character name alone does not establish local attribution.
- If a candidate source cannot be distinguished from another character or an unknown owner, leave it unsupported and state what evidence is missing. Do not invent parser syntax or values.
- Do not publish or commit personal character names or full logs. Character Name presence stays out of M4.

## Earlier candidate review (2026-10-01 log session; reviewed 2026-10-02)

- The available Linux Steam `W.1.610.21` log has six case-insensitive `level` text hits and no `school` hits. Two hits are explicit values at 22:27:37 (`SYSTEM INFO: Level: 26.00`) and 22:27:45 (`<Value name="Level">26.00</Value>`). The latter is inside the `CPU` group, among CPU capability values; the first is in the same startup/system-information block. These are hardware/system information, not evidence of the Wizard101 character's level.
- The other four hits are incidental startup/resource/diagnostic text, not character-stat records. Their values and context do not establish game Level.
- No candidate record is tied to the selected Wizard101 character. No Level or School field will be added to GameState or offered in `WIZRUST101_DISPLAY_STAT` from this evidence.
- Existing selector behavior already offers verified Health (default) and `none`. This milestone will regression-test that `level` and `school` remain rejected until their own sources pass the evidence gate.
- Bacon1661's current parser matches zones and health only. ManaUp/WizRPC is archived and its README claims live stats generally, but the available documentation does not define a Level/School log source or local attribution rule. Neither reference establishes current-client truth.

## Controlled two-character capture result (2026-10-02)

- The user supplied four screenshots in one W.1.610.21 Linux Steam session. They show the first character at Level 74 (user-identified School Balance), then in-world; the second shows a different character at Level 1 (user-identified School Death), then in-world. Screenshot file modification times are 10:29:44.967, 10:30:17.536, 10:31:14.506, and 10:31:29.986 local; treat these as capture-time proxies.
- The corresponding 2,117-line log reports W.1.610.21 / `r806919.Wizard_1_610`. It records `CHARACTER LIST` at 10:29:18, Stone Town at 10:29:46, another `CHARACTER LIST` at 10:30:37, and `WizardCity/Interiors/WC_Headmistress_House` at 10:31:17. This sequence aligns with the screenshot pairs but has no stat values or selected-character identity attached to them.
- The log has 10 substring hits for `level`, but only two numeric `Level` fields: `26.00` in startup system information at 10:28:08 and `26.00` in a `CPU` capability value at 10:28:18. Both predate character selection. There is no `School` field. `Death` and `Balance` each appear once at 10:28:08 in startup messages tracking progression for both schools, not as a selected-character School value.
- The screenshots verify visible character values and provide a two-character comparison, but the matching log window emits neither Level 74/Level 1 nor selected-character School records. No Level/School parser source or selector choice is justified.
- Minimized log anchors and evidence limits are documented in [the fixture README](../tests/fixtures/current-steam-2026-10-02-README.md). The screenshots are not committed because they include character names.

## Controlled capture procedure

Use the Linux Steam client at `W.1.610.21`; record the exact version/revision from the log and use one consistent local timezone for notes and screenshot timestamps. Do not edit or restart the log between the capture windows.

1. Start at the character-selection screen. Capture a timestamped screenshot showing the selected character (redact the name) and, if the UI exposes it there, its Level and School. Note the screenshot's local time and timezone.
2. Select that character and enter the world. Capture a timestamped screenshot of the in-game character information panel with the visible Level and School, plus a second screenshot after world entry if the panel closes during transition. Preserve image metadata or filenames that provide capture times.
3. Collect the `WizardClient.log` window beginning about 30 seconds before the visible character-selection action and continuing through world entry and at least 30 seconds after the stat screenshot. Include the version/revision record, `CHARACTER LIST`, all intervening character selection/load records, every line containing a candidate stat field/value, and enough adjacent lines to retain actor/source markers and lifecycle context. Keep original timestamps, order, punctuation, field names, and stat numbers.
4. If a second character with a different Level and School is available, return to character selection and repeat steps 1–3. Use separate screenshots/times for each character. This comparison is especially useful for showing that candidate values follow the selected character rather than machine/system state. If no second character is available, state that explicitly.
5. Before sharing, redact character/account names and raw IDs. Use stable placeholders so equality and inequality remain visible (for example, `<selected-actor-A>` each time the same ID appears, and `<other-actor-B>` for a different ID). Do not redact source names, event keywords, timestamps, punctuation, Level/School values, or the fact that IDs match/differ. Send only the small relevant excerpts and screenshots, not the full log.

Attribution is sufficient only if visible values match the candidate log values and the log links the record to the selected local character through an explicit local/selected-character marker or a stable actor identity across selection and the stat record. A second-character run with different values should show the source following that selected character and resetting/replacing the prior character's values. A matching value that merely occurs after selection, a single screenshot without adjacent records, a name alone, or an unmarked global/statistics value is insufficient. If the log has no source-level ownership marker and a second-character comparison cannot establish ownership, Level/School remain unsupported and the app must omit them.

The requested screenshot comparison has now been supplied and correlates with two selection-to-zone sequences, but the log contains no candidate character-stat records. No additional screenshots are needed for this evidence set. The remaining evidence, if it exists, is a sanitized current-client log excerpt that actually contains the selected character's Level/School values and a local/selected-actor ownership marker or stable actor identity. If WizardClient.log does not emit such records, these stats remain unsupported; screenshots alone cannot make a log parser populate them.

## Plan

1. **Complete:** Inspect available W.1.610.21 log candidates and legacy references; record evidence and its attribution limits.
2. **Complete before code:** Update parser, state, configuration, presence, and test specs with the unsupported-candidate decision.
3. Add typed parser events and GameState fields only if new evidence passes local-character attribution. No such source is currently supported; do not add speculative fields.
4. Keep Health as the default and `none` as the only other selector value until Level/School pass the evidence gate. Unknown selected data must be omitted.
5. Add sanitized regression fixtures for the CPU/system `Level` candidate and the selection-to-zone anchors; test that the former is ignored, the latter emits only existing selection/zone events, and unsupported selector values remain rejected.
6. Run `cargo fmt`, `cargo test`, warning-denied Clippy, Windows-target `cargo check`, and `git diff --check`; update status with exact evidence and remaining gaps.

## Acceptance criteria

- Every exposed stat has a current-client fixture, local-character attribution evidence, typed GameState representation, and presence formatting test. For this evidence slice, Health remains the only game stat that passes this gate.
- Unknown or ambiguously owned records cannot populate displayed stats.
- Level and School are exposed individually only if their own sources pass the evidence gate. If neither does, report the missing evidence and leave both unsupported without guessing.
- Health remains the default. No GUI, character-name publishing, memory inspection, or remote data collection is included.
- Linux runtime behavior may be checked with the available Steam session. Windows remains compile-tested only until a Windows session is available.

## User decisions

No new decision blocks M4. Character Name publishing requires a separate explicit owner decision and is outside this milestone.
