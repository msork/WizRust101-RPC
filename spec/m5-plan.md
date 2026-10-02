# M5 plan: versioned per-user configuration

Historical M5 scope/results are recorded below. M6 removed the Discord stat selector and its environment override; Health is internal only. The current config schema contains the log path override and log verbosity, while preserving old `display_stat` values as inert unknown fields.

WizRust101-RPC is fully vibe coded: Codex CLI carries out research, spec maintenance, planning, implementation, testing, and refactoring through spec-driven development. The human supplies product decisions and verification data when required.

## Historical objective at M5 start

Implement the persistent, versioned user configuration already required by [configuration.md](configuration.md). This makes the existing path and display preferences available from a per-user file while preserving automatic Steam discovery and the then-current Health/None stat boundary. The Discord Application ID is not part of user config: official releases embed the project ID, and development/testing may use the separate runtime environment override. M6 subsequently removed the display preference; see the current config contract in [configuration.md](configuration.md).

## Scope

- Versioned JSON configuration stored at `ProjectDirs::from("com", "msork", "WizRust101-RPC").config_dir()/config.json`, using `directories` 6.0.0. This resolves to XDG config on Linux and Roaming AppData on Windows; see [configuration.md](configuration.md) for exact forms and sources.
- Supported fields: optional `game_log_path` recovery override, `display_stat` (`health` default or `none`), and privacy-preserving `log_level`.
- User-setting precedence: built-in defaults, then valid per-field file values, then valid `WIZRUST101_DISPLAY_STAT` / `WIZRUST101_LOG_LEVEL` environment overrides. `WIZRUST101_DISCORD_APP_ID` is a development/testing runtime override only, never a user setting, and is never written to config. The official release build supplies the embedded project ID.
- Automatic discovery remains the normal path. A missing, malformed, unsupported-version, or invalid optional config must not crash the watcher or prevent automatic discovery; emit a concise diagnostic and apply safe defaults where possible.
- Preserve unknown JSON fields in typed decode/encode round-trips. The app does not auto-create or rewrite config; malformed and unsupported-version files remain untouched. Version 1 is the only supported schema and there is no automatic migration.
- No GUI/settings UI, new stats, credentials, or private game data.

## Plan

1. **Complete before implementation:** updated product, architecture, configuration, discovery, privacy, and testing specs; researched the maintained `directories` crate and documented schema, paths, precedence, and migration policy.
2. **Complete:** added a typed config model with defaults, version validation, path/stat/log-level validation, unknown-field preservation, and deterministic source precedence.
3. **Complete:** integrated config loading into the watcher without coupling it to parser, presence construction, or Discord transport.
4. **Complete:** added unit/integration tests for defaults, typed loading, round-trip/unknown-field preservation, malformed and unsupported-version files, environment precedence, invalid path fallback, log filtering, and stat rejection.
5. **Complete:** format, full tests, warning-denied Clippy, Windows-target check, and diff checks pass; see milestone status for results.

## Acceptance criteria

1. First run works without prompts or manually entered game data and discovers supported Steam Wizard101 automatically.
2. Config loads from the platform-appropriate per-user location, not beside the executable or game install; the app never silently creates, truncates, or replaces a user's config.
3. Only Health and None are accepted for Discord State; Health remains default.
4. Missing config uses defaults; malformed/unsupported config remains unchanged, warns, and does not stop watcher startup or discovery retries. Invalid optional paths warn and fall back to automatic discovery.
5. Application IDs, credentials, account data, character names, and raw log content are never written to config.
6. Tests cover the schema, migration/version rules, precedence, and recovery paths without requiring a live game, Discord, or GUI.

## Risks and decisions

- Research selected `directories` 6.0.0 for standard per-user config paths and existing `serde_json` for the schema; primary references are linked in [configuration.md](configuration.md).
- Platform directory lookup failure must degrade safely without writing beside the executable or game installation.
- There is no settings UI or automatic config writer in this milestone; explicit config editing belongs to the user. Do not add a writer that could overwrite malformed/future-version files.

No product decision blocked M5.
