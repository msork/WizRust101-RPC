# October 1 Steam log window

These fixtures were extracted programmatically from the user-provided Linux Steam `WizardClient.log` for client `W.1.610.21`. The full log and the user's screenshot are not committed. Six-or-more-digit instance/actor IDs in the selected lines were replaced with `<redacted-id>`; timestamps, parser keywords, punctuation, and health numbers remain in original order.

| Fixture | Original 1-based lines | Evidence |
| --- | ---: | --- |
| `current-steam-2026-10-01-version.log` | 2 | Client version and revision |
| `current-steam-2026-10-01-zone.log` | 509 | Raw zone record at 19:35:27 |
| `current-steam-2026-10-01-health-window.log` | 534–550 | Sixteen consecutive health-globe records at 19:35:39, followed by the next non-health line |

The log's last health pair is `3868/3868`. The user corrected an earlier mistaken screenshot reading: it shows current health `3868` at approximately 19:35. The match strongly supports the logged current value as local at that moment. The screenshot does not independently show maximum health or a current-location label, and its exact capture time is unknown. These fixtures verify how the parser reads the log; they do not establish a general attribution or freshness rule.
