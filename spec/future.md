# Future features

Level and School research is closed for the current log-only product scope; see the archived [M4 investigation](m4-plan.md). Character Name remains excluded. These values may be reconsidered only if a future Wizard101 client exposes reliable evidence explicitly attributable to the selected local character. Do not obtain them through memory scanning, process injection, packet interception, OCR, or guessed/indirect values.

## Distribution roadmap

1. **M6: Linux Flatpak tray app for Steam Wizard101 through Proton.** Research, Steam-only runtime/tray implementation, manifest, and locked offline source list are complete. Close after packaged live tests of Steam/Proton log discovery, Flatpak file permissions, Discord IPC/reconnect, tray status/quit, and optional additional-library portal. No compatibility path may be assumed.
2. **M7: Windows installer/setup tray app for native Steam Wizard101.** Research and test native Steam discovery, IPC, installation, and upgrade/uninstall behavior on Windows at milestone start.
3. **M8: macOS packaged menu-bar app for Steam Wizard101 through CrossOver.** Research and test actual CrossOver Steam/log discovery and Discord integration at milestone start.
4. **M9: Expand verified zone/world mapping coverage** from available current-client evidence and approved manual research. Unknown mappings remain unknown; Wiki access restrictions and source policy still apply.
5. **M10+: Standalone Wizard101 support, only after M6–M8 are complete and tested:** separate milestones for Chromebook Linux `.deb`, native Windows, and macOS. Research each target when its milestone begins; for macOS compare CrossOver/Wineskin with other researched approaches. Do not assume standalone or compatibility-layer paths.

Official builds embed the project Discord Application ID; only development/testing may use the environment override. It is never a normal user setting. The initial active game support remains Steam-only across M6–M8.

Not planned: memory scanning, process injection/modification, packet interception, OCR-based data extraction, credential collection, remote telemetry, or launching Wizard101 from the RPC tool. Unsupported stats are not to be approximated indirectly.
