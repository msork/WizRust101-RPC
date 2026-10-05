# Changelog

All notable changes are recorded here. Versioning follows Semantic Versioning.

## 0.1.0 — planned initial release

The recommended initial scope is a Windows-first preview. Do not create this release until its release checklist has been reviewed.

- Windows 11 x64 installer and standalone portable app for native Steam Wizard101 and native Discord; owner live acceptance passed before the concise tray-status wording update.
- Linux Flatpak and AppImage packages are built and validated by Ubuntu 22.04 CI. The latest combined ZIP has not received live desktop/Steam/Discord acceptance.
- macOS 15 Universal 2 app and installer are unsigned and unnotarized CI artifacts. macOS is not production-supported; real CrossOver/Steam/Discord acceptance and Developer ID signing/notarization remain mandatory gates.
- One combined ZIP per platform, containing the final platform packages directly, plus SHA-256 checksums from the manual release-preparation workflow.
- Current Wizard101 zone/world values come from the pinned WizRust101-DB dataset. Discord presence uses continuous session timing across zone changes, reconnect republishing, and concise tray status.

No tag or GitHub Release has been created for this version.
