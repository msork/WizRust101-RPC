# Game and log detection

## Requirement

Find Wizard101 Steam log files automatically on the currently supported target without requiring normal users to enter a path. Standalone Wizard101 is deferred and is not active product support.

## Candidate search

Discovery should inspect known locations and installed-client metadata rather than scan every drive by default:

1. Steam library roots discovered from Steam installation metadata, including non-default libraries; the Wizard101 Steam app is `799960`. Check its manifest install directory for `Bin\WizardClient.log` and also check the historical default `steamapps\common\Wizard101\Bin\WizardClient.log` candidate.
2. Any other well-supported Steam installation metadata/path evidence found during the current platform milestone's research.
3. A user-configured path override as recovery. It is used only when it names an existing regular `WizardClient.log`; otherwise log a warning and continue normal Steam discovery.

Platform path requirements are milestone-specific. Linux Flatpak must research and test the actual Steam-through-Proton log location and sandbox access before choosing candidates. Windows native Steam and macOS Steam through CrossOver each require fresh path and IPC research at their milestones. Never infer compatibility-layer paths from Windows/Linux conventions. Existing verified Linux Steam discovery must continue to pass its regression tests. The implementation still contains historical standalone candidates from M1; M6 must remove them from active automatic discovery and update tests while preserving the Linux Steam path.

Known paths from prior RPCs are starting candidates, not an exhaustive or permanently guaranteed install layout. Validate each candidate by the expected log/file structure. Do not ask for a path if a valid candidate exists.

## Active source selection

When multiple valid logs exist, prefer the log associated with an active Wizard101 process when that association can be made reliably; otherwise prefer the most recently updated valid log and continue monitoring candidates for changes. Do not merge events from multiple characters/installations. Record the selected path in diagnostics with personal path segments minimized where practical.

## Runtime behavior

- Start monitoring if the game is already open and its log is valid.
- Detect a later game launch and begin reading without requiring an app restart.
- Handle log creation, replacement, truncation, and temporary sharing/locking errors.
- On game exit, clear the Discord activity after a bounded grace period; if the game is merely between log updates, retain known state only for the freshness interval defined in the state spec.
- Re-run discovery after a missing/deleted log or source failure.

## M1 scope and evidence

Steam's store page identifies the current Steam app and notes its all-file install process for North American Steam players since November 13, 2024. Steam library enumeration should use the app manifest's `installdir`, not hard-code a directory spelling. See [Steam Wizard101](https://store.steampowered.com/app/799960/Wizard101/).

M1 tests discovery using injected environment/library roots and temporary filesystem layouts. The current workspace is not a Windows Wizard101 machine, so actual installation layouts and multi-client log sharing remain manual verification items; do not claim those as tested.
