# October 1 Steam log window

These fixtures were extracted programmatically from the user-provided Linux Steam `WizardClient.log` for client `W.1.610.21`. The full log and the user's screenshot are not committed. Six-or-more-digit instance/actor IDs in the selected lines were replaced with `<redacted-id>`; timestamps, parser keywords, punctuation, and health numbers remain in original order.

| Fixture | Original 1-based lines | Evidence |
| --- | ---: | --- |
| `current-steam-2026-10-01-version.log` | 2 | Client version and revision |
| `current-steam-2026-10-01-zone.log` | 509 | Raw zone record at 19:35:27 |
| `current-steam-2026-10-01-health-window.log` | 534–550 | Sixteen consecutive health-globe records at 19:35:39, followed by the next non-health line |
| `current-steam-2026-10-01-damage-calculation.log` | 881–885 | Internal 413-point damage calculation at 19:50:56; IDs redacted |
| `current-steam-2026-10-01-local-hit.log` | 1161–1167 | Explicit local-hit marker, 3455/3868 globe record, damage arithmetic, and meter at 19:51:19; IDs redacted |
| `current-steam-2026-10-01-mixed-meters.log` | 1308–1310 | Nearby 1530/1530 and 3455/3868 combat health meters; no globe records |
| `current-steam-2026-10-01-combat-health-repeat.log` | 1378–1380 | Same `3455/3868` pair repeated by `MSG_CombatHealth` without an explicit local marker; duel ID redacted |
| `current-steam-2026-10-01-contradictory-marker.log` | 2134–2135 in the later 20:10 session | A “not this client's” marker after a statistic update, but both IDs are equal in the original; represented by `<same-id>` |
| `current-steam-2026-10-01-recovery-baseline.log` | 4029–4031 in the later session | `MSG_UpdateHealth` `1992/3868` before the first 20:13 screenshot; duel ID redacted |
| `current-steam-2026-10-01-recovery-rises.log` | 4081–4090 in the later session | Contiguous lines around `MSG_UpdateHealth` `2959/3868` and `3868/3868`; no player IDs |
| `current-steam-2026-10-01-stats-candidate.log` | 87 and selected lines 118–124 in the 22:27 session | Two `Level: 26.00` startup/system-information values; the XML value is under the `CPU` group. Lines selected programmatically; no personal identifiers or hardware details retained. These are not Wizard101 character stats. No `School` match was found in this captured log. |

The log's last health pair is `3868/3868`. The user corrected an earlier mistaken screenshot reading: it shows current health `3868` at approximately 19:35. The match strongly supports the logged current value as local at that moment. The screenshot does not independently show maximum health or a current-location label, and its exact capture time is unknown. These fixtures verify how the parser reads the log; they do not establish a general attribution or freshness rule.

The controlled screenshots from about 19:50:53 and 19:51:23 show local health `3868/3868` before and current health `3455` plus floating damage `413` after. Their file times are capture-time proxies. The `19:51:19` log explicitly says the client is hurt immediately before the `3455/3868` globe record. The internal calculation is 23 seconds earlier, while the visual transition is only bounded by the two still images. The mixed-meter fixture shows why a bare health-meter number is insufficient for local attribution. The after screenshot does not independently show maximum health. These snippets are each contiguous within their documented ranges; the gaps between ranges are intentional. The full log and screenshots remain outside Git.

The user confirmed these are the proposed non-cinematic before/after images. The matching `19:51:19` globe record is explicitly from `Cinematics ProcessDamageEffect`, so these images do not verify a non-cinematic source. The `MSG_CombatHealth` repeat has the same pair, but value equality is not a per-record local marker. It remains `Unknown` under the conservative M2 attribution policy.

The later supplied 20:13 screenshot sequence is distinct and verifies out-of-combat increases from visible current health `1992` to `2959` to `3868`. Its `MSG_UpdateHealth` records precede the three screenshot file times by about 8.1, 6.6, and 3.4 seconds respectively. The images do not independently show numeric maximum health; the log reports `3868` throughout, and the user describes the last image as full health. The screenshots and full log are not committed. The contradictory-marker fixture preserves the **equality** of the two original numeric IDs without committing either ID. The September remote-marker fixture had both IDs redacted without preserving whether they were equal, so it cannot prove `OtherPlayer` for that record.
