# Game state

## State model

Maintain a normalized snapshot with:

- game activity: `Unknown`, `Running`, or `Ended`;
- current zone identifier and mapped location/world, each optional/unknown;
- health current and maximum, optional and tagged with last observed time;
- future verified character fields (level, school, name) as optional values with freshness/source metadata;
- location-entry timestamp based on a monotonic clock during the process lifetime;
- observation timestamps for staleness and diagnostics.

M1 implements the essentials only: activity, raw zone ID, optional mapped location/world, optional current/max health, and observation state. Character stats, time freshness expiry, and session timestamps are reserved for later milestones.

The M2 client capture shows syntactically valid health records with `current > maximum`; store raw observed values without treating that relationship as a parsing failure. The October 1 controlled hit shows one local character's displayed health moving from `3868/3868` to `3455`, matching an explicitly local `3455/3868` globe record. This verifies that path, while the screenshot does not show the after maximum. It does not prove all unmarked health records belong to the local player. A future presence renderer must require a verified attribution policy before publishing health generally.

Health means the numeric value parsed from a recognized health-globe record. The parser carries per-record attribution; GameState stores only `Local` health. This does not establish a general local-character guarantee for unmarked globe records.

M2 hardening changes the event boundary to carry per-record `Local` or `Unknown` attribution. Only a supported `Local` health observation changes GameState's local health and its dedicated monotonic `health_observed_at` timestamp. An unknown observation must leave the last verified local health unchanged; future freshness policy may clear an aging value, but it must not replace it with an unknown observation. `health_observed_at` records parser receipt, not the game's internal change or on-screen transition. The evidence establishes no safe expiration interval, so the stored value is *last verified*, not guaranteed current. `Local` currently covers the explicitly marked cinematic hit and the screenshot-verified `WizardClientMod MSG_UpdateHealth` source for observed client `W.1.610.21`.

## Transition rules

- First verified location observation establishes the location timer.
- A change in normalized zone identity resets the location timer, even when two zones share a display label. Duplicate reports for the same zone do not reset it.
- Health/stat updates never reset the location timer.
- A menu or character-select event clears in-world location and health. It must not report a stale previous location as current.
- Unknown/unmapped zone identifiers are retained as unknown location evidence. They do not become a guessed default world.
- Missing or stale health becomes unavailable after a configured internal freshness interval; do not replace it with zero.
- A verified end event or confirmed process exit marks the game ended and clears presence. Transient pauses in log writes alone do not immediately end the session.

## Time representation

Keep a monotonic instant for transition logic; convert to a Unix timestamp only when constructing Discord activity. A location change resets the timestamp. If the app restarts mid-location, elapsed time may begin at first observation because no durable trusted location-entry time exists in the log; do not invent past duration.
