# WizardClient.log parsing

## Source and scope

`WizardClient.log` is the initial source of game state. The [Bacon1661 reference parser](https://github.com/Bacon1661/Wizard101-RPC/blob/master/index.js) explicitly matches zone records containing `zone = <zone-id>,` and health records containing `Updating health globe (new health: N, new health max: M)`. It also recognizes `CHARACTER LIST`. M1 used these as legacy patterns. A user-provided Steam log captured on 2026-09-30 identifies client version `W.1.610.21` (revision `r806919.Wizard_1_610`) and supplies real samples of those record forms. Windows behavior remains unknown.

## Parser contract

- Parse incrementally, line by line, from the current end on first attach. Do not read the full historical log on every poll.
- Accept line endings and partial trailing lines safely; defer an incomplete final line until completed.
- Emit typed events only for recognized records. Unknown records are ignored and may be counted for diagnostics without logging their full contents.
- Parse health-globe values as observed. A 2026-09-30 capture shows `current > maximum` during some reported updates, so this relationship is not a parse error. Reject malformed integers and zero maximum. Do not claim local-player attribution until the record is tied to an observed in-game health value. The capture supports the legacy next-line remote-player exclusion pattern, but absence of that marker alone does not establish local ownership.
- Reject malformed zone identifiers without replacing last-known-good state.
- Support file rotation/truncation without losing the ability to resume from the new file.
- Keep raw text handling private to the parser. Redact or omit character names and full log payloads from ordinary logs.

## Event candidates

- `ZoneChanged { raw_zone_id }` from verified zone records and known menu/character-list records.
- `HealthChanged { current, maximum }` from verified local health-globe records.
- `GameEnded` from verified quit/logout records.
- Future `LevelChanged`, `SchoolKnown`, and `CharacterNameKnown` events only when a real log source has been found and fixture-backed.

## Evidence and constraints

Prior source code observes `zone = ...` and `CHARACTER LIST`, `Updating health globe (...)`, and `GameClient::HandleQuit()` / away-from-keyboard logout strings. Treat every pattern as version-sensitive. Add sanitized fixture lines with source/version notes before relying on them. Do not infer a field just because the reference app advertises it.

M1 fixture policy: tests may include minimal reference-pattern fixtures reconstructed directly from the cited parser's literal patterns. Their headers must say `reference-derived` and must not claim to be captured Wizard101 logs. Current-build compatibility and health ownership remain unverified until a sanitized real sample is available.

M2 is limited to current-client verification and parser hardening. Do not recognize new game record formats based solely on a legacy repository or an unverified reconstruction. Stop a zone ID at its first comma; never publish a pending health value at end of input before the following attribution line arrives. Preserve syntactically valid observed health values even when current exceeds maximum. Mark exact record syntax as observed in the 2026-09-30 Steam log, while local health attribution and live Windows behavior remain unverified.

The user-provided log has 23,559 lines: 17 `zone =` records across five raw IDs, 380 health-globe records, 17 instances of the remote-player exclusion phrase, and two `CHARACTER LIST` records. Every observed remote-player phrase immediately follows a health-globe line. Of the 380 health records, 251 have `current > maximum`; a consecutive sequence moves from `3841/2474` to `3841/3841`. Tests use short sanitized excerpts with original line ranges documented; the full log is not version-controlled. This is a Linux Steam path, so it cannot close the Windows smoke-test gate.
