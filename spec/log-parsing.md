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
- `HealthChanged { current, maximum }` from recognized health-globe records that lack the next-line remote marker. This event currently carries observed values, not a general local-player guarantee.
- `GameEnded` from verified quit/logout records.
- Future `LevelChanged`, `SchoolKnown`, and `CharacterNameKnown` events only when a real log source has been found and fixture-backed.

## Evidence and constraints

Prior source code observes `zone = ...` and `CHARACTER LIST`, `Updating health globe (...)`, and `GameClient::HandleQuit()` / away-from-keyboard logout strings. Treat every pattern as version-sensitive. Add sanitized fixture lines with source/version notes before relying on them. Do not infer a field just because the reference app advertises it.

M1 fixture policy: tests may include minimal reference-pattern fixtures reconstructed directly from the cited parser's literal patterns. Their headers must say `reference-derived` and must not claim to be captured Wizard101 logs. Current-build compatibility and health ownership remain unverified until a sanitized real sample is available.

M2 is limited to current-client verification and parser hardening. Do not recognize new game record formats based solely on a legacy repository or an unverified reconstruction. Stop a zone ID at its first comma; never publish a pending health value at end of input before the following attribution line arrives. Preserve syntactically valid observed health values even when current exceeds maximum. Mark exact record syntax as observed in the 2026-09-30 Steam log, while general local health attribution and live Windows behavior remain unverified.

A second log from client `W.1.610.21` was written on 2026-10-01 near a user-supplied screenshot from about 19:35 local time. It records `Zafaria/ZF_Z07_Stone_Town` at 19:35:27, then sixteen consecutive health-globe lines at 19:35:39, ending at `3868/3868`. The user corrected a prior misreading: the screenshot shows **current health `3868`**. This matching value and approximate timing strongly support the final logged current value as the local player's at that moment. The screenshot does not independently display maximum health or a current-location label. Its quest text mentions Stone Town, which is not a direct location label.

The inspected log has no further health/damage record after the 19:35:39 series through about 19:36:17. The match supports a recent observation in this session, but the screenshot has no exact capture timestamp and cannot establish a general update-latency bound or that every unmarked health record is local. The log reports maximum `3868`; that maximum is not independently visible in the screenshot. Do not introduce burst coalescing or a positive local-attribution rule from this one match.

### Controlled October 1 health change (M2 evidence and implementation plan)

Two user-supplied Linux Steam screenshots have filename/file-modification times of `19:50:53.730` and `19:51:23.530` local time. The first visibly shows the local character at `3868/3868`; the second shows current health `3455` and floating damage `413`, but does not independently show maximum health. Treat file times as capture-time proxies, not instrumented game-event times. The same `W.1.610.21` log records an internal `ModifyHealth` calculation `3868 + (-413)` at `19:50:56`, then at `19:51:19` says `ProcessDamageEffect: Our client is getting hurt!`, logs a health-globe update `3455/3868`, a matching `OldHP`/`Delta`/`NewHP` record, and a `ClientHealthMeter::UpdateHealth( 3455 / 3868 )` record. A later `MSG_CombatHealth` globe line repeats `3455/3868` at `19:51:30`.

The controlled observation verifies that **this** local-character decrease appears in the health-globe format and that the `19:51:19` record is local: the explicit client marker, arithmetic, and screenshots agree. The internal calculation precedes the globe record by about 23 seconds during a combat cinematic. The visual change happened after the first screenshot and by the second; it cannot be timed more precisely from two still images. The globe record precedes the second image's file time by about 4.5 seconds. Neither gap is a general update-latency guarantee. Health-meter lines for other combatants (`1530/1530`) occur nearby; an unlabeled health meter or absence of the legacy remote marker is not a universal local-player classifier. The existing next-line remote-marker exclusion remains supported by the September log, while positive attribution outside this explicit damage path remains unresolved. Do not derive local identity from an owner ID or a matching number alone.

Plan before code: preserve short, timestamped, sanitized contiguous log ranges for the internal damage calculation, explicit local-damage/globe sequence, and nearby mixed health meters; add regression tests for the already supported globe parser and its non-globe behavior. Do not add a new generic attribution, coalescing, or timing rule from this one encounter. Run the required formatting, test, Clippy, Windows cross-target, and diff checks, then record M2 status and commit.

The user-provided log has 23,559 lines: 17 `zone =` records across five raw IDs, 380 health-globe records, 17 instances of the remote-player exclusion phrase, and two `CHARACTER LIST` records. Every observed remote-player phrase immediately follows a health-globe line. Of the 380 health records, 251 have `current > maximum`; a consecutive sequence moves from `3841/2474` to `3841/3841`. Tests use short sanitized excerpts with original line ranges documented; the full log is not version-controlled. This is a Linux Steam path, so it cannot close the Windows smoke-test gate.
