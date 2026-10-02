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
- Attribute each syntactically valid health-globe record separately. `Local` requires the immediately preceding `Cinematics ProcessDamageEffect: Our client is getting hurt!` record from the same log second and the matching `Cinematics ProcessDamageEffect` globe source, with no following remote-player marker. The explicitly observed next-line `HUDWindow::HandleUpdateHealth called for a player that is not this client's!` marks the preceding globe record as `OtherPlayer`. All remaining globe records are `Unknown`; lack of a remote marker is not positive local evidence. A later duplicate value does not inherit attribution.

## Event candidates

- `ZoneChanged { raw_zone_id }` from verified zone records and known menu/character-list records.
- `HealthObserved(HealthObservation { health, attribution })` from recognized health-globe records after the following line is inspected. Only `Local` may update the local character's health in GameState; `Unknown` and `OtherPlayer` retain their attribution without changing local health.
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

### Conservative attribution hardening plan

The user confirmed that the alleged non-cinematic before/after evidence is the existing `19:50:53` and `19:51:23` screenshot pair. Its correlated health-globe record is explicitly `Cinematics ProcessDamageEffect`, so it does **not** verify a non-cinematic source or a prompt-update bound for one. Implement typed `Local` / `OtherPlayer` / `Unknown` attribution for each globe observation. Use the adjacent explicit local marker and same log-second/source context only for the verified local damage path; use the adjacent September remote marker as an exclusion; leave `HandleStatisticUpdate`, `MSG_CombatHealth`, `MSG_UpdateHealth`, bare meters, and any other unmarked globe record `Unknown`. Do not carry local attribution to repeated values. Make GameState ignore unknown/other-player health, including its local observation timestamp. Update existing fixtures/tests to assert the revised state boundary, then run all M2 checks and record remaining evidence limits.

| Local log time | Relevant records in the current session | Per-record conclusion |
| --- | --- | --- |
| 19:50:33–19:50:48 | `HandleStatisticUpdate` max-health series ending `3868/3868`, `HandleEnterCombat` `3868/3868`, and `MSG_CombatHealth` `3868/3868` | Values agree with the 19:50:53 before image; these records have no positive per-record local marker, so attribution stays `Unknown`. |
| 19:50:47–19:50:48 | `ClientHealthMeter` lines for `1530/1530` and `3868/3868` | Meter lines are not globe observations; mixed combatant values cannot serve as a local-only source. |
| 19:50:56 | `ModifyHealth` with `m_playerHealth:3868`, `a_deltaHealth:-413` | Internal calculation correlates with the hit; its owner ID alone is not a parser identity rule. |
| 19:51:19 | Explicit `Our client is getting hurt!` immediately followed in the same log second by `Cinematics ProcessDamageEffect` globe `3455/3868`, then `OldHP:3868, Delta:-413, NewHP:3455` and a matching meter | `Local` for this globe record; current value is corroborated by the 19:51:23 after image. |
| 19:51:29–19:51:30 | Mixed `1530/1530` and `3455/3868` meters; `MSG_CombatHealth` globe repeats `3455/3868` | Meters remain ignored; the repeated globe is `Unknown` because value equality does not transfer the earlier attribution. |
| 19:52:08 | Another `MSG_CombatHealth` repeat of `3455/3868` | `Unknown`; after the supplied screenshots. |
| 19:52:44–19:52:56 | Two further explicit local-hit markers followed by `Cinematics ProcessDamageEffect` globes `3151/3868` and `2916/3868` | Same explicitly local record pattern, after the supplied screenshots; no independent visible values supplied for these later hits. |
| 19:53:14–19:53:15 | `MSG_UpdateHealth` and `HandleStatisticUpdate` report `2916/3868` | `Unknown` as individual records; no controlled non-cinematic screenshot window. |

The current session contains no next-line remote-player marker. The September fixture demonstrates that marker's exclusion syntax, but no rule can classify all other records as local. The health-globe line appears four to five seconds before the after screenshot's file time; this only confirms it was logged by that image, not a general promptness bound relative to the actual visual transition.

Plan before code: preserve short, timestamped, sanitized contiguous log ranges for the internal damage calculation, explicit local-damage/globe sequence, and nearby mixed health meters; add regression tests for the already supported globe parser and its non-globe behavior. Do not add a new generic attribution, coalescing, or timing rule from this one encounter. Run the required formatting, test, Clippy, Windows cross-target, and diff checks, then record M2 status and commit.

The user-provided log has 23,559 lines: 17 `zone =` records across five raw IDs, 380 health-globe records, 17 instances of the remote-player exclusion phrase, and two `CHARACTER LIST` records. Every observed remote-player phrase immediately follows a health-globe line. Of the 380 health records, 251 have `current > maximum`; a consecutive sequence moves from `3841/2474` to `3841/3841`. Tests use short sanitized excerpts with original line ranges documented; the full log is not version-controlled. This is a Linux Steam path, so it cannot close the Windows smoke-test gate.
