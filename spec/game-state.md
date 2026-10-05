# Game state

## State model

Maintain a normalized snapshot with:

- game activity: `Unknown`, `Running`, or `Ended`;
- current zone identifier and mapped location/world, each optional/unknown;
- health current and maximum, optional and tagged with last observed time;
- no character Level, School, or Name fields in the current log-only scope;
- location-entry timestamp based on a monotonic clock during the process lifetime;
- observation timestamps for staleness and diagnostics.

M1 implements the essentials only: activity, raw zone ID, optional mapped location/world, optional current/max health, and observation state. Level and School remain unsupported: the controlled two-character W.1.610.21 comparison showed that the log did not expose the selected characters' Level 74/School Balance or Level 1/School Death. Startup `Level: 26.00` records are system/CPU information, and startup school progression messages do not identify a selected character. Character Name remains excluded. Reconsider these fields only if a future client exposes reliable local-character-attributed data. Memory scanning, process injection, packet interception, OCR, and guessed/indirect values are prohibited sources.

The M2 client capture shows syntactically valid health records with `current > maximum`; store raw observed values without treating that relationship as a parsing failure. The October 1 controlled hit shows one local character's displayed health moving from `3868/3868` to `3455`, matching an explicitly local `3455/3868` globe record. This verifies that path, while the screenshot does not show the after maximum. It does not prove all unmarked health records belong to the local player. A future presence renderer must require a verified attribution policy before publishing health generally.

Health means the numeric value parsed from a recognized health-globe record. The parser carries per-record attribution; GameState stores only `Local` health. This does not establish a general local-character guarantee for unmarked globe records.

M2 hardening changes the event boundary to carry per-record `Local` or `Unknown` attribution. Only a supported `Local` health observation changes GameState's local health and its dedicated monotonic `health_observed_at` timestamp. An unknown observation must leave the last verified local health unchanged; future freshness policy may clear an aging value, but it must not replace it with an unknown observation. `health_observed_at` records parser receipt, not the game's internal change or on-screen transition. The evidence establishes no safe expiration interval, so the stored value is *last verified*, not guaranteed current. `Local` currently covers the explicitly marked cinematic hit and the screenshot-verified `WizardClientMod MSG_UpdateHealth` source for observed client `W.1.610.21`.

## Transition rules

- The first zone observation in a running Wizard101 session establishes the session timer.
- Zone changes, duplicate reports, and aliases update the current location without resetting the session timer. Character selection clears the current session; the next zone starts a new session timer.
- Health updates never reset the timer. Unknown-world zones retain their DB-provided Details and still use the current session timer.
- On RPC startup replay, the latest active zone establishes a fresh timer at process startup because historical log times do not establish a reliable session start.
- A menu or character-select event clears in-world location and health. It must not report a stale previous location as current.
- Unknown/unmapped zone identifiers are retained as unknown location evidence. They do not become a guessed default world.
- Missing or stale health becomes unavailable after a configured internal freshness interval; do not replace it with zero.
- A verified end event or confirmed process exit marks the game ended and clears presence. Transient pauses in log writes alone do not immediately end the session.

## Time representation

Keep one monotonic instant for the active Wizard101 session and convert to a Unix timestamp when constructing Discord activity. Zone changes update location/world but do not reset this session timestamp. Character selection ends the session; a replay-restored session starts a fresh timer when WizRust101-RPC starts because the log does not establish a trusted earlier entry time.
