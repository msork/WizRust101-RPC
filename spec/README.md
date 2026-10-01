# WizRust101-RPC specifications

These documents are the product and engineering source of truth. If code and a spec disagree, update the spec and resolve the difference in the same milestone; do not silently treat behavior as specified.

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

Milestone M1 builds the ingestion slice described in [status.md](status.md): automatic discovery, incremental log tailing, zone/health parsing, typed game state, legacy mapping import, and fixture-backed tests. Discord RPC, GUI, and user-facing settings remain out of scope.
