# Future features

Level and School research is closed for the current log-only product scope; see the archived [M4 investigation](m4-plan.md). Character Name remains excluded. These values may be reconsidered only if a future Wizard101 client exposes reliable evidence explicitly attributable to the selected local character. Do not obtain them through memory scanning, process injection, packet interception, OCR, or guessed/indirect values.

## Distribution roadmap

1. **M6: Linux Flatpak tray app for Steam Wizard101. Accepted for one pairing (2026-10-02):** owner-tested installed Flatpak with native Steam at `~/.local/share/Steam` and native Discord. This verifies log discovery and the published location/world/art/timer for that pairing only. Steam Flatpak, Discord Flatpak, and a specific Proton launch/prefix remain unverified; do not generalize paths or compatibility.
2. **M7: Windows installer/setup tray app for native Steam Wizard101 (accepted 2026-10-04).** The owner verified the installed package, startup while stationary in Stone Town, Discord reconnect, expected payload/art/timer, and tray Quit on Windows 11. See [M7 plan](m7-plan.md) and [acceptance record](status.md#m7-final-windows-11-live-acceptance-2026-10-04).
3. **M8: macOS native menu-bar app for Steam Wizard101 through CrossOver (started 2026-10-04; Apple Silicon first, macOS 15+).** See [M8 plan and research](m8-plan.md). WizRust101-RPC runs natively on macOS; only Wizard101/Steam run in CrossOver. Implement macOS-specific bottle/Steam-library discovery, menu-bar integration, and app packaging while reusing the shared core. Close only after native CrossOver/Steam/Discord acceptance; do not start M9 before then.
4. **M9: Expand verified zone/world mapping coverage** from available current-client evidence and approved manual research. Unknown mappings remain unknown; Wiki access restrictions and source policy still apply.
5. **M10+: Standalone Wizard101 support, only after M6–M8 are complete and tested:** separate milestones for Chromebook Linux `.deb`, native Windows, and macOS. Research each target when its milestone begins; for macOS compare CrossOver/Wineskin with other researched approaches. Do not assume standalone or compatibility-layer paths.

Official builds embed the project Discord Application ID; only development/testing may use the environment override. It is never a normal user setting. The initial active game support remains Steam-only across M6–M8.

Not planned: memory scanning, process injection/modification, packet interception, OCR-based data extraction, credential collection, remote telemetry, or launching Wizard101 from the RPC tool. Unsupported stats are not to be approximated indirectly.
