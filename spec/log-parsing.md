# WizardClient.log parsing

## Source and scope

`WizardClient.log` is the initial source of game state. The [Bacon1661 reference parser](https://github.com/Bacon1661/Wizard101-RPC/blob/master/index.js) explicitly matches zone records containing `zone = <zone-id>,` and health records containing `Updating health globe (new health: N, new health max: M)`. It also recognizes `CHARACTER LIST`. These are verifiable legacy parser patterns, not captured records and not proof current clients always emit identical text. No publicly verifiable current-build raw log excerpt was found during M1 research.

## Parser contract

- Parse incrementally, line by line, from the current end on first attach. Do not read the full historical log on every poll.
- Accept line endings and partial trailing lines safely; defer an incomplete final line until completed.
- Emit typed events only for recognized records. Unknown records are ignored and may be counted for diagnostics without logging their full contents.
- Parse health-globe values as observed. Do not claim local-player attribution until a current captured log confirms which record belongs to the local character and how remote-player records are marked. Preserve the reference parser's exclusion rule only as a tested legacy compatibility behavior.
- Reject invalid numeric ranges and malformed zone identifiers without replacing last-known-good state.
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

M2 is limited to current-client verification and parser hardening. Record the capture's build/source context and enough neighboring lines to establish local health attribution before promoting a pattern to current support. No parser change may be based solely on a legacy repository or an unverified reconstruction.
