# M4: Verified Level and School stats

## Objective

Research current-client sources and add only Level and/or School values that can be reliably attributed to the selected local character. Add supported values to typed GameState and the existing Discord stat selector. Keep Health as the default. Character Name is excluded until the project owner makes a separate explicit privacy/product decision.

## Source and attribution gate

- Start with exact records in the sanitized/current client `W.1.610.21` Linux Steam `WizardClient.log` available for offline research, plus permitted reference implementations.
- For each candidate value, establish its record shape, selected-character association, and reset/lifecycle markers. Nearby log order, a plausible value, or a character name alone does not establish local attribution.
- If a candidate source cannot be distinguished from another character or an unknown owner, leave it unsupported and state what evidence is missing. Do not invent parser syntax or values.
- Do not publish or commit personal character names or full logs. Character Name presence stays out of M4.

## Plan

1. Inspect candidate Level and School records from available current-client evidence; record client version, evidence location, and sanitized literal syntax in research notes.
2. Update `log-parsing.md`, `game-state.md`, `configuration.md`, `discord-rpc.md`, and `testing.md` with the supported/unsupported source decisions before coding.
3. Add typed parser events and GameState fields only for sources that pass the local-character attribution gate. Unknown observations cannot populate or refresh fields. Clear character-scoped stats on character selection, log source reset/replacement, or any verified character switch.
4. Support `level` and/or `school` in `WIZRUST101_DISPLAY_STAT` only when backed by evidence. Preserve `health` as the default and `none` as the explicit no-stat choice. Missing or unverified selected values are omitted.
5. Add minimized, sanitized real-log fixtures and parser/state/presence tests for verified values, unknown/remote records, lifecycle resets, and selector behavior.
6. Run `cargo fmt`, `cargo test`, warning-denied Clippy, Windows-target `cargo check`, and `git diff --check`; update status with exact evidence and remaining gaps.

## Acceptance criteria

- Every exposed stat has a current-client fixture, local-character attribution evidence, typed GameState representation, and presence formatting test.
- Unknown or ambiguously owned records cannot populate displayed stats.
- Level and School are exposed individually only if their own sources pass the evidence gate. If neither does, report the missing evidence and leave both unsupported without guessing.
- Health remains the default. No GUI, character-name publishing, memory inspection, or remote data collection is included.
- Linux runtime behavior may be checked with the available Steam session. Windows remains compile-tested only until a Windows session is available.

## User decisions

No new decision blocks M4. Character Name publishing requires a separate explicit owner decision and is outside this milestone.
