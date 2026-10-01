# WizardClient.log parsing

## Source and scope

`WizardClient.log` is the initial source of game state. The existing projects demonstrate log-derived zone and health signals, including zone text shaped like `zone = <world>/<zone>` and health text shaped like `Updating health globe (new health: N, new health max: M)`. These are leads from older implementations, not a promise that current clients always emit identical text.

## Parser contract

- Parse incrementally, line by line, from the current end on first attach unless a validated startup replay policy is chosen. Do not read the full historical log on every poll.
- Accept line endings and partial trailing lines safely; defer an incomplete final line until completed.
- Emit typed events only for recognized records. Unknown records are ignored and may be counted for diagnostics without logging their full contents.
- Parse only local-character health. Existing regex behavior attempts to exclude health messages identified as belonging to another client/player; this exclusion must be verified against current fixture lines before implementation.
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
