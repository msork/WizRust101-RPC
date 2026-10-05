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

Windows 11 x64 with native Steam and native Discord passed owner live acceptance for the installer and portable app; the current concise tray wording is a later text-only change without a separate owner retest. Linux Flatpak/AppImage outputs pass current CI packaging and headless checks, but the latest artifacts have not been live-tested; older M6 live acceptance covers only an installed Flatpak with native Steam and Discord. macOS Universal 2 packaging and native CI checks pass, but no real Mac/CrossOver/Discord acceptance has occurred and current artifacts are unsigned and unnotarized. macOS is not production-supported. See the explicit evidence and release-gate matrix in [status](status.md).

Wizard101 support is Steam-only. The planned first public version is `v26.10.05`, a Windows-first preview; Linux may be identified as CI-validated preview software, while macOS remains excluded from production support until live acceptance and Developer ID signing/notarization gates pass. Project releases use Calendar Versioning `vYY.MM.DD` with `.1`, `.2`, etc. for additional same-day releases. No release or tag exists yet. Standalone support remains deferred. Health remains internal; Level, School, and Character Name remain unsupported.
