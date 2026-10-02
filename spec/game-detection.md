# Game and log detection

## Requirement

Find Wizard101 Steam log files automatically on the currently supported target without requiring normal users to enter a path. Standalone Wizard101 is deferred and is not active product support.

## Candidate search

Discovery should inspect known locations and installed-client metadata rather than scan every drive by default:

1. Steam library roots discovered from Steam installation metadata, including non-default libraries; the Wizard101 Steam app is `799960`. Check its manifest install directory for `Bin\WizardClient.log` and also check the historical default `steamapps\common\Wizard101\Bin\WizardClient.log` candidate.
2. Any other well-supported Steam installation metadata/path evidence found during the current platform milestone's research.
3. A user-configured path override as recovery. It is used only when it names an existing regular `WizardClient.log`; otherwise log a warning and continue normal Steam discovery.

Platform path requirements are milestone-specific. Linux discovery searches native Steam roots (`~/.local/share/Steam`, aliases `~/.steam/steam` and `~/.steam/root`) and candidate Steam Flatpak roots (`~/.var/app/com.valvesoftware.Steam/.local/share/Steam` and `~/.var/app/com.valvesoftware.Steam/data/Steam`). Steam metadata identifies app `799960` and supplies each library's `installdir`; the log candidate is under that install's `Bin/WizardClient.log`. The exact native root was observed on this host and the log exists there. The Steam Flatpak data roots come from Flathub's Steam manifest research and are not locally verified. The available host had no running Wizard101/Proton process or Proton prefix log, so the observed log cannot itself establish a Proton-specific path. The Flatpak manifest grants read-only access to these known Steam roots; any additional library must be selected through the folder portal, with its returned directory persisted in a separate app-data registry. No home/host-wide permission is allowed. On this host inaccessible paths are skipped while accessible libraries remain searchable. Windows native Steam and macOS Steam through CrossOver each require fresh path and IPC research at their milestones. Never infer compatibility-layer paths from Windows/Linux conventions. Standalone discovery is excluded from the active product path.

### Windows native Steam (M7 research)

`steamlocate` 2.1.1 obtains the Windows Steam root from the read-only registry values `HKLM\SOFTWARE\Wow6432Node\Valve\Steam\InstallPath` or `HKLM\SOFTWARE\Valve\Steam\InstallPath`. The app also checks `%ProgramFiles(x86)%\Steam` as a fallback. For each root, `steamlocate` reads Steam's `steamapps/libraryfolders.vdf`, enumerates listed library folders, and resolves `appmanifest_799960.acf` and its `installdir`; the log is `<library>\steamapps\common\<installdir>\Bin\WizardClient.log`. An **Add Steam library…** tray action opens a native folder picker for a library omitted by metadata and validates the Wizard101 manifest before saving the folder in the per-user `steam-libraries.json` registry. No recursive volume scan or standalone fallback is permitted. These are implementation-source findings and fixture cases, not live Windows observations.

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
