# M6 plan: Linux Flatpak Steam/Proton tray app

## Goal

Deliver the first official target: a Linux Flatpak tray application that publishes presence for Wizard101 installed through Steam and running through Proton. Preserve the already verified Linux Steam log discovery and Discord presence behavior.

## Scope and acceptance criteria

- Research the currently used Linux Steam/Proton install and log layout on the available W.1.610.21 session. Verify any candidate path against the real `WizardClient.log`; do not assume a Proton prefix or Flatpak filesystem path.
- Research the current Flatpak packaging/runtime conventions and Discord local IPC access requirements before choosing manifest permissions or packaging tools.
- Remove standalone Wizard101 paths from active automatic discovery. Keep the optional user log path override and Steam discovery. Historical M1 standalone tests should be removed or relabeled as historical; active discovery tests must establish that only Steam candidates participate.
- Implement a usable tray application lifecycle around the existing watcher/presence pipeline, including status and recoverable errors without exposing log contents or personal data.
- Produce an installable Flatpak artifact with the project Discord Application ID embedded in the official release build. The ordinary user must not create/configure a Discord app or set an ID. Development/testing may retain `WIZRUST101_DISCORD_APP_ID` as an override.
- Test a real install and run with Steam Wizard101 through Proton and Discord Desktop, verifying discovery, location, verified world art, supported health behavior, elapsed timer, close/exit behavior, and IPC recovery where practical.
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

Planned only. No tray, Flatpak, or packaging implementation has begun. Research of actual Proton/Flatpak paths is intentionally deferred until this milestone begins.
