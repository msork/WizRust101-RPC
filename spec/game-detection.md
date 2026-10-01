# Game and log detection

## Requirement

Find Wizard101 log files automatically on Windows, including standalone and Steam installs, without requiring normal users to enter a path.

## Candidate search

Discovery should inspect known locations and installed-client metadata rather than scan every drive by default:

1. Known standalone path under `%PROGRAMDATA%\KingsIsle Entertainment\Wizard101\Bin\WizardClient.log`.
2. Steam library roots, including the default `steamapps\common\Wizard101\Bin\WizardClient.log` and additional roots declared by Steam library metadata.
3. Any other well-supported installation metadata/path evidence found during implementation research.
4. A user-configured path override as recovery.

Known paths from prior RPCs are starting candidates, not an exhaustive or permanently guaranteed install layout. Validate each candidate by the expected log/file structure. Do not ask for a path if a valid candidate exists.

## Active source selection

When multiple valid logs exist, prefer the log associated with an active Wizard101 process when that association can be made reliably; otherwise prefer the most recently updated valid log and continue monitoring candidates for changes. Do not merge events from multiple characters/installations. Record the selected path in diagnostics with personal path segments minimized where practical.

## Runtime behavior

- Start monitoring if the game is already open and its log is valid.
- Detect a later game launch and begin reading without requiring an app restart.
- Handle log creation, replacement, truncation, and temporary sharing/locking errors.
- On game exit, clear the Discord activity after a bounded grace period; if the game is merely between log updates, retain known state only for the freshness interval defined in the state spec.
- Re-run discovery after a missing/deleted log or source failure.

## Validation evidence needed during implementation

Capture representative paths and filesystem behavior for current standalone and Steam installs. Confirm whether `WizardClient.log` is shared by simultaneous clients and how Steam library metadata is represented. Do not encode a guessed path as the only Steam option.
