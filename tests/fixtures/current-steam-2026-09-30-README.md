# Current Steam log excerpts

These five small fixtures were extracted programmatically from a user-provided `WizardClient.log` under a Linux Steam installation. The log is dated 2026-09-30 and identifies the client as `W.1.610.21` (revision `r806919.Wizard_1_610`). The full log is not committed. Only 6-or-more-digit instance/player IDs were replaced with `<redacted-id>`; parser keywords, punctuation, health values, timestamps, and order within each range are retained.

| Fixture | Original 1-based line range | Evidence |
| --- | ---: | --- |
| `current-steam-version.log` | 2 | Client version and revision |
| `current-steam-zone.log` | 599 | Zone record and comma-delimited raw ID |
| `current-steam-health-series.log` | 621–636 | Consecutive health records, including `current > maximum` and later equal values |
| `current-steam-remote-health.log` | 1029–1030 | Health record immediately followed by the “not this client's” phrase; ID equality was not preserved during redaction |
| `current-steam-selection.log` | 23537 | `CHARACTER LIST` record |

The health series and zone record were separated by other log lines in the source. Fixtures do not establish which accepted health value belonged to the local player. The marker fixture establishes the phrase and line order, but cannot prove a distinct other player because both numeric IDs became the same redaction placeholder. This Linux Steam capture also does not verify Windows discovery or runtime behavior.
