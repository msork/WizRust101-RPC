# Changelog

All notable changes are recorded here. Project releases follow Calendar Versioning (`vYY.MM.DD`); additional same-day releases append `.1`, `.2`, and so on.

## v26.10.05 - planned first public release

The planned initial scope is a Windows-first preview. Do not create this tag or release until its release checklist has been reviewed.

- Windows 11 x64 installer and standalone portable app for native Steam Wizard101 and native Discord; owner live acceptance passed before the concise tray-status wording update.
- Linux Flatpak and AppImage packages are built and validated by Ubuntu 22.04 CI. Their latest ZIPs have not received live desktop/Steam/Discord acceptance.
- macOS 15 Universal 2 app and installer are unsigned and unnotarized CI artifacts. macOS is not production-supported; real CrossOver/Steam/Discord acceptance and Developer ID signing/notarization remain mandatory gates.
- Six separate ZIPs: `WizRust101-RPC-Windows-Setup.zip`, `WizRust101-RPC-Windows-Portable.zip`, `WizRust101-RPC-macOS-App.zip`, `WizRust101-RPC-macOS-Pkg.zip`, `WizRust101-RPC-Linux-Flatpak.zip`, and `WizRust101-RPC-Linux-AppImage.zip`. The manual release-preparation workflow validates each payload and stages these files with `SHA256SUMS.txt`.
- Current Wizard101 zone/world values come from the pinned WizRust101-DB dataset. Discord presence uses continuous session timing across zone changes, reconnect republishing, and concise tray status.

No tag or GitHub Release has been created for this version. Cargo metadata uses `26.10.5` because Cargo's semver parser rejects leading-zero numeric components; public release identifiers and installer metadata retain the exact CalVer value `26.10.05`.
