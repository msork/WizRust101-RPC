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

## Current scope

M1's ingestion slice and offline M2 current-client parser hardening are implemented under the evidence limits in [status.md](status.md). M3 adds the pure presence model and Discord IPC adapter. The incomplete Wizard101 Central export enables offline name research; real Linux Steam logs verify specific parser record forms, one explicitly local combat-damage path, and a screenshot-correlated out-of-combat recovery source. Other health sources and Windows live behavior remain unverified. One Stone Town → Zafaria row is verified through project-owner confirmation and current-client log evidence. The Zafaria asset key is configured; its live display remains to be tested.
