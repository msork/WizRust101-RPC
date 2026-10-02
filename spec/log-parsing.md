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
- Attribute each syntactically valid health-globe record separately. For observed client `W.1.610.21`, `Local` requires either the immediately preceding `Cinematics ProcessDamageEffect: Our client is getting hurt!` record from the same log second and matching cinematic globe source, or the exact verified `WizardClientMod MSG_UpdateHealth` globe source with a valid log timestamp. A following `HUDWindow::HandleUpdateHealth called for a player that is not this client's!` phrase downgrades either to `Unknown`; one current-client line prints identical sent/client IDs despite that phrase. All other globe records are `Unknown`. Lack of a remote marker, a matching number, or an owner ID does not promote a record. No generic latency bound or rule for other client versions is established.

## Event candidates

- `ZoneChanged { raw_zone_id }` from verified zone records and known menu/character-list records.
- `HealthObserved(HealthObservation { health, attribution })` from recognized health-globe records after the following line is inspected. Only `Local` may update the local character's health in GameState; `Unknown` retains its attribution without changing local health.
- `GameEnded` from verified quit/logout records.
- Future `LevelChanged`, `SchoolKnown`, and `CharacterNameKnown` events only when a real log source has been found and fixture-backed.

### M4 Level/School research result (2026-10-02)

The available user-provided Linux Steam log for client `W.1.610.21` contains six case-insensitive occurrences of the text `level` and no occurrence of `school`. Only two occurrences are explicit numeric values: `SYSTEM INFO: Level: 26.00` at 22:27:37 and `<Value name="Level">26.00</Value>` at 22:27:45. The latter is nested in the `CPU` capability group; both occur in startup system-information output. The same value in those records therefore does not identify Wizard101 character level. Four other hits occur in incidental startup/resource/diagnostic text and do not expose an attributed game stat.

No school candidate was observed. Neither field is parsed or stored. The selector continues to accept only `health` (default) and `none`; candidate `level` and `school` remain invalid until a current-client source and selected-character attribution are established. See the fixture note in `tests/fixtures/current-steam-2026-10-01-README.md` and the M4 evidence request in [m4-plan.md](m4-plan.md).

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

Historical plan before the 20:13 recovery screenshots: the `19:50:53` and `19:51:23` pair correlated with `Cinematics ProcessDamageEffect`, so it did not verify a non-cinematic source. At that point the parser carried `Local` / `OtherPlayer` / `Unknown`; later evidence below corrects the remote-marker classification and verifies the exact `MSG_UpdateHealth` source. The current parser contract is stated above.

| Local log time | Relevant records in the current session | Per-record conclusion |
| --- | --- | --- |
| 19:50:33–19:50:48 | `HandleStatisticUpdate` max-health series ending `3868/3868`, `HandleEnterCombat` `3868/3868`, and `MSG_CombatHealth` `3868/3868` | Values agree with the 19:50:53 before image; these records have no positive per-record local marker, so attribution stays `Unknown`. |
| 19:50:47–19:50:48 | `ClientHealthMeter` lines for `1530/1530` and `3868/3868` | Meter lines are not globe observations; mixed combatant values cannot serve as a local-only source. |
| 19:50:56 | `ModifyHealth` with `m_playerHealth:3868`, `a_deltaHealth:-413` | Internal calculation correlates with the hit; its owner ID alone is not a parser identity rule. |
| 19:51:19 | Explicit `Our client is getting hurt!` immediately followed in the same log second by `Cinematics ProcessDamageEffect` globe `3455/3868`, then `OldHP:3868, Delta:-413, NewHP:3455` and a matching meter | `Local` for this globe record; current value is corroborated by the 19:51:23 after image. |
| 19:51:29–19:51:30 | Mixed `1530/1530` and `3455/3868` meters; `MSG_CombatHealth` globe repeats `3455/3868` | Meters remain ignored; the repeated globe is `Unknown` because value equality does not transfer the earlier attribution. |
| 19:52:08 | Another `MSG_CombatHealth` repeat of `3455/3868` | `Unknown`; after the supplied screenshots. |
| 19:52:44–19:52:56 | Two further explicit local-hit markers followed by `Cinematics ProcessDamageEffect` globes `3151/3868` and `2916/3868` | Same explicitly local record pattern, after the supplied screenshots; no independent visible values supplied for these later hits. |
| 19:53:14–19:53:15 | `MSG_UpdateHealth` and `HandleStatisticUpdate` report `2916/3868` | At the time, neither source had a controlled non-cinematic screenshot window; the later 20:13 recovery series verifies the exact `MSG_UpdateHealth` source, while `HandleStatisticUpdate` remains `Unknown`. |

The 19:50 session contains no next-line remote-player marker. The September fixture demonstrates the phrase's syntax but redacts both IDs without preserving equality; the later 20:11 capture shows why the phrase alone is not a reliable `OtherPlayer` classification. The 19:51:19 globe line appears four to five seconds before the after screenshot's file time; this only confirms it was logged by that image, not a general promptness bound relative to the actual visual transition.

### Controlled October 1 out-of-combat recovery: spec update before code

The user supplied three new screenshots of the same local character in the open world, with file times `20:13:14.107`, `20:13:33.640`, and `20:13:41.403` local. The visible current-health values are `1992`, `2959`, and `3868`, respectively; the third is described by the user as full health. The screenshots do not independently show numeric maximum health. The `W.1.610.21` log records exact `WizardClientMod MSG_UpdateHealth` globe pairs `1992/3868` at `20:13:06`, `2959/3868` at `20:13:27`, and `3868/3868` at `20:13:38`. The latter two records occur between the corresponding before/after screenshots, six to seven and about three seconds before the next screenshot's file time. Each log pair matches the next visible current value. This verifies two out-of-combat increases for this source and strongly supports this exact source as local for the observed client build; it does not prove a general update-latency bound or apply to other globe sources. The first record occurs during combat resolution, though the first screenshot is in the open world. The two increases occur after combat and have no `Cinematics ProcessDamageEffect` marker.

An earlier `20:11:26` `HandleStatisticUpdate` globe line is immediately followed by `HUDWindow::HandleUpdateHealth called for a player that is not this client's!`, but the full current log prints the **same numeric ID** for “sent in” and “client's player”. This contradicts a blanket `OtherPlayer` interpretation. Preserve the raw record's unknown attribution; the marker still prevents promotion to local. The September fixture redacted both IDs, so it cannot resolve whether that older line's IDs differed. Do not parse/redact-identical placeholders as if they proved inequality.

Revised conservative policy for `W.1.610.21`: label a globe record `Local` only for the existing immediately adjacent explicit cinematic local-hit sequence or the exact `WizardClientMod MSG_UpdateHealth` source verified by the controlled recovery series, with a valid log timestamp. Any following “not this client's” marker downgrades it to `Unknown`, because current evidence shows the phrase can conflict with the IDs. Other globe sources, bare health meters, and ID-only correlations remain `Unknown`. A repeated matching value does not inherit local attribution. The parser retains one-line lookahead; GameState accepts only `Local`. Keep the source rule version-sensitive in documentation and do not claim behavior for other client builds.

Plan: add short sanitized contiguous `20:13` ranges and the contradictory marker range, update typed attribution and tests for the two verified recovery increases and marker conflict, then run formatting, tests, warning-denied Clippy, Windows-target check, and Git diff checks. Record exact evidence limits and M2 closure status before committing.

Plan before code: preserve short, timestamped, sanitized contiguous log ranges for the internal damage calculation, explicit local-damage/globe sequence, and nearby mixed health meters; add regression tests for the already supported globe parser and its non-globe behavior. Do not add a new generic attribution, coalescing, or timing rule from this one encounter. Run the required formatting, test, Clippy, Windows cross-target, and diff checks, then record M2 status and commit.

The user-provided log has 23,559 lines: 17 `zone =` records across five raw IDs, 380 health-globe records, 17 instances of the remote-player exclusion phrase, and two `CHARACTER LIST` records. Every observed remote-player phrase immediately follows a health-globe line. Of the 380 health records, 251 have `current > maximum`; a consecutive sequence moves from `3841/2474` to `3841/3841`. Tests use short sanitized excerpts with original line ranges documented; the full log is not version-controlled. This is a Linux Steam path, so it cannot close the Windows smoke-test gate.
