# M5 plan: versioned per-user configuration

WizRust101-RPC is fully vibe coded: Codex CLI carries out research, spec maintenance, planning, implementation, testing, and refactoring through spec-driven development. The human supplies product decisions and verification data when required.

## Objective

Implement the persistent, versioned user configuration already required by [configuration.md](configuration.md). This makes existing runtime choices usable without environment variables while preserving automatic game discovery and the verified Health/None stat boundary.

## Scope

- Versioned JSON configuration stored under the per-user application configuration directory (Windows AppData for the initial supported OS; Linux user config directory to preserve the verified developer/smoke-test workflow).
- Supported fields: optional `game_log_path` recovery override, `display_stat` (`health` default or `none`), and privacy-preserving `log_level`.
- Precedence: a valid environment override takes precedence over the file; otherwise use a valid file value, then the built-in default. Preserve the existing `WIZRUST101_DISPLAY_STAT` behavior and Linux launch flow. `WIZRUST101_DISCORD_APP_ID` remains a local environment override and is never written to config.
- Automatic discovery remains the normal path. A missing, malformed, unsupported-version, or invalid optional config must not crash the watcher or prevent automatic discovery; emit a concise diagnostic and apply safe defaults where possible.
- Preserve unknown JSON fields when feasible during config rewrites. Define explicit version/migration behavior before implementation.
- No GUI/settings UI, new stats, credentials, or private game data.

## Plan

1. Re-read and update the product, architecture, configuration, discovery, privacy, and testing specs before implementation; resolve the config schema and platform path APIs in the milestone research.
2. Add a typed config model with defaults, version validation, path/stat/log-level validation, and deterministic source precedence.
3. Integrate config loading into the watcher without coupling it to parser, presence construction, or Discord transport.
4. Add isolated tests for default behavior, round-trip and unknown-field preservation, invalid and future-version files, environment precedence, path override fallback, and stat rejection (`level`, `school`, `character_name`).
5. Run format, full tests, warning-denied Clippy, Windows-target check, and diff checks; update status and this plan with results.

## Acceptance criteria

1. First run works without prompts or manually entered game data and discovers Wizard101 automatically.
2. Config can be loaded/saved in the platform-appropriate per-user location, not beside the executable or game install.
3. Only Health and None are accepted for Discord State; Health remains default.
4. Missing/bad config and invalid optional paths fail safely and do not stop watcher startup or discovery retries.
5. Application IDs, credentials, account data, character names, and raw log content are never written to config.
6. Tests cover the schema, migration/version rules, precedence, and recovery paths without requiring a live game, Discord, or GUI.

## Risks and decisions

- The exact Rust platform-directory API and config serialization dependency should be checked against maintained current crates during M5 research before dependency selection.
- Platform directory lookup failure must degrade safely without writing beside the executable or game installation.
- Config save behavior must avoid overwriting unknown fields where feasible and use an atomic replacement strategy.

No product decision currently blocks M5.
