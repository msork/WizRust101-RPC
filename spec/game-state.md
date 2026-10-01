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

Health means the numeric value parsed from a recognized health-globe record. Without a current fixture proving local ownership, represent it as observed game health without stronger local-character attribution guarantees.

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
