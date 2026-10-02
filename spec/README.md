# WizRust101-RPC specifications

These documents are the product and engineering source of truth. If code and a spec disagree, update the spec and resolve the difference in the same milestone; do not silently treat behavior as specified.

WizRust101-RPC is fully vibe coded. Codex CLI owns research, requirements maintenance, planning, implementation, testing, and refactoring through this spec-driven workflow. The human provides product decisions and verification data when required; the workflow does not require the human to write code.

## Milestone workflow

For every milestone:

1. Update requirements, acceptance criteria, and open decisions in the relevant specs.
2. Write a short plan with scope, dependencies, and risks before coding.
3. Implement only the approved milestone scope.
4. Run the checks required by [testing.md](testing.md), including focused tests and the relevant full build checks.
5. Update the affected specs and [status.md](status.md) with what changed, evidence, test results, and remaining work.

Specs and status must be committed alongside the implementation they govern. New behavior without a spec change is incomplete.

## Spec index

- [Product requirements](product.md)
- [Architecture](architecture.md)
- [Game detection](game-detection.md)
- [WizardClient.log parsing](log-parsing.md)
- [Game state](game-state.md)
- [Zone and world mapping](zone-world-mapping.md)
- [Discord Rich Presence](discord-rpc.md)
- [Configuration](configuration.md)
- [Testing](testing.md)
- [Future features](future.md)
- [Research notes and decisions](research.md)
- [M1 research and plan](m1-research.md)
- [Milestone status](status.md)
- [M4 closed investigation: Level and School](m4-plan.md)
- [M5 plan: persistent user configuration](m5-plan.md)
- [M6 plan: Linux Flatpak Steam/Proton target](m6-plan.md)
- [M7 plan: Windows native Steam tray and installer](m7-plan.md)

## Current scope

M1's ingestion slice, offline M2 parser hardening, M3 Discord IPC, M5 configuration, and M6 Linux Flatpak tray target are implemented under the evidence limits in [status.md](status.md). M6 live acceptance covers an installed Flatpak with native Steam and native Discord: Stone Town, Zafaria artwork, elapsed location time, tray/runtime behavior, and automatic log discovery. Health remains internal and is not displayed. Other health sources and Windows live behavior remain unverified. One Stone Town → Zafaria row is verified through project-owner confirmation and current-client log evidence. Level/School research is closed for the current log-only approach and Character Name remains excluded.

Active Wizard101 support is Steam-only. Official targets, in order, are Linux Flatpak tray app, Windows installer/setup tray app for native Steam, then macOS packaged menu-bar app for Steam through CrossOver. M6 is accepted only for the tested native Steam + native Discord pairing; other combinations remain unverified. M7 implementation and offline checks are complete; packaged live Windows acceptance is pending. Standalone support is deferred until the initial targets are complete and tested.
