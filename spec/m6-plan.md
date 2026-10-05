# M6 plan: Linux Flatpak Steam/Proton tray app

## Goal

Deliver the first official target: a Linux Flatpak tray application that publishes presence for Wizard101 installed through Steam and running through Proton. Preserve the already verified Linux Steam log discovery and Discord presence behavior.

## Scope and acceptance criteria

- Research the currently used Linux Steam/Proton install and log layout on the available W.1.610.21 session. Verify any candidate path against the real `WizardClient.log`; do not assume a Proton prefix or Flatpak filesystem path.
- Research the current Flatpak packaging/runtime conventions and Discord local IPC access requirements before choosing manifest permissions or packaging tools.
- Remove standalone Wizard101 paths from active automatic discovery. Keep the optional user log path override and Steam discovery. Historical M1 standalone tests should be removed or relabeled as historical; active discovery tests must establish that only Steam candidates participate.
- Implement a usable tray application lifecycle around the existing watcher/presence pipeline, including status and recoverable errors without exposing log contents or personal data.
- Produce an installable Flatpak artifact with the project Discord Application ID embedded in the official release build. The ordinary user must not create/configure a Discord app or set an ID. Development/testing may retain `WIZRUST101_DISCORD_APP_ID` as an override.
- Acceptance evidence for this revision: installed Flatpak/tray with native Steam at `~/.local/share/Steam` and native Discord; verify discovery, location/world presence, artwork, elapsed timer, and tray behavior. Record other Steam/Discord combinations separately as unverified.
- Run standard formatting, tests, warning-denied Clippy, Windows-target check where applicable, and diff checks; document platform-specific checks and limitations.

## Plan before implementation

1. Update specs with findings from current Flatpak, Proton, Steam, and Discord IPC research.
2. Record the exact verified installation/log location and observed sandbox access; if access needs a Flatpak permission, justify and minimize it.
3. Select the tray and Flatpak packaging approach based on maintained current options and target constraints.
4. Implement the Linux-specific discovery/package/tray changes behind existing module boundaries; retain parser, state, mapping, and presence semantics.
5. Exercise offline tests and a live packaged Proton + Discord session, then update status with exact evidence and remaining risks.

## Out of scope

- Windows or macOS packaging and runtime claims.
- Standalone Wizard101 discovery/support.
- Additional stats, GUI settings editor, or unrelated asset changes.
- Guessing Proton/Flatpak paths or granting broad filesystem access without evidence.

## Status

## M6 research and decisions (2026-10-02)

### Local Steam/log evidence

- Flatpak CLI is present (`Flatpak 1.18.3`). `flatpak-builder` was absent during initial implementation but was available for the final M6 package rebuild; the successful build is recorded below and in status.
- Steam app manifest `appmanifest_799960.acf` is present under `~/.local/share/Steam/steamapps`; its `installdir` is `Wizard101`.
- A real `WizardClient.log` exists at `~/.local/share/Steam/steamapps/common/Wizard101/Bin/WizardClient.log` (observed size 204,277 bytes, modified 2026-10-02 10:31 local). The W.1.610.21 captures/fixtures independently establish current log syntax.
- No `WizardClient.log` was found below `~/.local/share/Steam/steamapps/compatdata/799960`. No Wizard101/Proton process was running for this filesystem inspection, so this verifies the observed Steam install location but does not independently prove that this particular session launched through Proton.
- The native Steam aliases `~/.steam/steam` and `~/.steam/root` resolve to `~/.local/share/Steam` on this machine.
- Flathub's current Steam manifest sets the Steam Flatpak XDG data root under `~/.var/app/com.valvesoftware.Steam`; its Steam root is represented below that app's private data home. The current host does not have that Steam Flatpak data directory, so that location remains a researched candidate, not locally verified game data.

### Permission and library strategy

- Flatpak denies host file access by default and recommends specific read-only paths and portals over broad home/host access ([Flatpak sandbox permissions](https://docs.flatpak.org/en/latest/sandbox-permissions.html)). Manifest access is limited to the native Steam roots/aliases and candidate Steam Flatpak roots, all read-only.
- An additional Steam library may live outside those roots (for example, a mounted volume), and static Flatpak permissions cannot safely cover arbitrary unknown roots without broad filesystem access. **Add Steam library…** opens the XDG FileChooser portal for a library folder. The portal makes the selected folder available and keeps its document access across sessions ([FileChooser portal](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html)). The app stores only portal-returned folder paths in its own versioned app-data registry. It validates for Wizard101's Steam app manifest and never changes the normal user config.
- Discord IPC for Flatpak Discord is under `$XDG_RUNTIME_DIR/app/com.discordapp.Discord/discord-ipc-0`; the maintained `discord-rich-presence` implementation searches that path, and Flathub's Discord wrapper exposes that socket there ([crate Unix IPC implementation](https://docs.rs/discord-rich-presence/latest/src/discord_rich_presence/ipc_unix.rs.html), [Flathub Discord wrapper](https://github.com/flathub/com.discordapp.Discord/blob/master/discord.sh)). The manifest grants read-only access to this app-specific runtime directory. Native Discord IPC is at `$XDG_RUNTIME_DIR/discord-ipc-0`; reconnect behavior through a Flatpak's per-file mount must be live-tested and is not claimed from research alone.
- Flatpak needs only the session-bus talk permission for `org.kde.StatusNotifierWatcher` for the Linux tray. The `ksni` sandbox mode disables its well-known bus name as required by its docs. No network, host filesystem, all-home, or all-device permission is specified.

### Dependency choice

- Selected `ksni` 0.3.6 (Unlicense, Rust 1.80 MSRV) for the Linux StatusNotifierItem and menu. It supports a blocking API and Flatpak's no-well-known-name mode. The project MSRV 1.85 covers it.
- Considered `tray-icon` 0.24.2, maintained by Tauri and cross-platform for Windows/macOS/Linux. Its default Linux path adds GTK/AppIndicator/libxdo dependencies and its KSNI backend is available in newer releases; for this Flatpak milestone, using `ksni` directly keeps native build dependencies small and exposes the needed Flatpak behavior. `src/tray` isolates the UI contract for later platform adapters.
- `ashpd` 0.12.3 (Rust 1.75) provides the maintained XDG FileChooser portal wrapper for selecting additional library roots; `tokio` powers portal requests and `ksni`'s asynchronous service.

### Plan for implementation and verification

1. Remove standalone candidates from active discovery; add native Steam, Steam Flatpak, known Steam aliases, manifests, and explicit portal-granted external library roots. Skip inaccessible libraries without preventing discovery in accessible roots.
2. Add a persistent, versioned external-library access registry that preserves malformed data and does not change M5 `config.json`.
3. Add a Linux StatusNotifierItem with live status, **Add Steam library…**, and **Quit** actions. Run the watcher automatically in the background and keep Discord retry behavior in the watcher.
4. Embed the Application ID only when `WIZRUST101_RELEASE_DISCORD_APP_ID` is supplied at build time; keep the runtime env override for development/testing. Never check the literal ID into source.
5. Add a Flatpak manifest, desktop metadata/icon generated from source, and a build helper that requires the release ID at packaging time. Verify the artifact build; record live testing only for the exact game/Discord combination actually exercised.

Research sources are references, not claims of runtime compatibility. The development environment has no Steam Flatpak installation or running Proton session; exact Proton and Steam Flatpak behavior therefore remain unverified.

**Status:** Accepted on 2026-10-02 for the owner-tested installed Flatpak with native Steam and native Discord. No claim is made for Steam Flatpak or Discord Flatpak combinations. Development-host rebuild availability is recorded below.

### Implemented M6 slice and verified limits

- Runtime discovery now uses Steam app ID `799960` only. It finds Steam roots via `steamlocate` and platform defaults, enumerates library metadata and `installdir`, reads `Bin/WizardClient.log`, and searches explicitly portal-authorized additional roots. Inaccessible libraries are skipped. Standalone paths no longer participate.
- Added a `ksni` Linux tray frontend with an automatic background watcher, changing status, `Add Steam library…` directory portal action, and Quit. `ashpd` opens the XDG FileChooser; a validated v1 registry is stored in the app's private data directory. Malformed/unsupported registry files are preserved.
- Added the `io.github.msork.WizRust101RPC` Flatpak manifest, desktop metadata and source-drawn icon. It targets the current Freedesktop 26.08 runtime/SDK. Initially, its source module used the Rust SDK extension and the official flatpak-cargo-generator source list to build offline from the exact Cargo.lock. The release package helper now builds the locked Rust binary on the host because Flathub has not published the matching 26.08 Rust extension; it inserts the binary into the same Flatpak runtime and permission model. The application ID is supplied only at build time, and the repository does not contain the ID.
- Manifest filesystem permissions are read-only and constrained to native Steam root/aliases and researched Steam Flatpak data roots, plus the Flatpak Discord socket directory and native Discord's conventional `discord-ipc-0` path. User-chosen other Steam libraries receive per-folder portal access. No blanket home/host or network permission is granted. It requests session bus access for the StatusNotifier watcher and desktop portal, Wayland/fallback X11 for desktop integration.
- Maintained tray options reviewed: `ksni` 0.3.6 has the direct StatusNotifierItem protocol support and Flatpak mode needed here; `tray-icon` 0.24.2 offers a reusable cross-platform abstraction but its Linux backend pulls more native UI dependencies. The shared `tray` module keeps platform frontends isolated. Flatpak Cargo packaging follows the official [flatpak-cargo-generator](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo) documented manifest format.
- Automatic IPC reconnect remains handled by the existing bounded retry publisher. Whether an exact mounted socket permission sees a socket recreated after Discord restarts is a Flatpak runtime property, so live reconnect is an acceptance check, not a verified claim.

### Checks and acceptance record

Offline unit/integration coverage includes Steam-only discovery, extra libraries, inaccessible roots, registry validation/preservation, tray menu/icon, and the existing parser/presence/Discord behavior. Final M6 code and documentation checks are recorded in status. Previous packaging checks covered Flatpak YAML/locked source consistency for all 142 registry crates, XML and strict offline AppStream validation, and build script syntax.

Owner-reported live acceptance verified the installed Flatpak/tray, automatic discovery of native Steam at `~/.local/share/Steam`, native Discord publication, Stone Town Details, Zafaria State and artwork, and elapsed location timer. The screenshot and live run verify this exact native Steam + native Discord pairing. Steam Flatpak, Discord Flatpak, additional-library portal selection, Discord restart recovery, and other pairings remain unverified unless included in that live run. This is not evidence of a particular Proton launch or prefix. Windows remains compile-tested only.

The user replaced `packaging/flatpak/icon.svg` with the design based on `assets/icons/original.png`; preserve that artwork. The change is already present in commit `859ea78`.

### M6 final presence requirements

- Details is the verified location; State is the verified world.
- Large image and hover text use the mapped world's registered Discord asset key.
- Small image uses uploaded project-logo asset key `wizrust101_rpc`; hover text is `WizRust101-RPC`. Upload source: `assets/icons/sizes/1024.png`.
- Elapsed time is based on verified location entry and resets on location change.
- Health remains available internally in GameState but is never sent to Discord. The M5 `display_stat` selector/environment override is removed. Old `display_stat` JSON fields are ignored and preserved as unknown fields.
