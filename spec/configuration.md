# Configuration

## Principles

- Normal startup requires no prompts or manual game data.
- Start with safe built-in defaults. Missing or invalid optional configuration must not prevent discovery.
- Use a documented, versioned configuration format and preserve unknown fields where feasible when rewriting.
- Keep the optional game-log path override and selected stat. Do not expose unsupported stat choices as working features.
- Store configuration in the per-user Windows application data directory, not beside the executable or in the Wizard101 install.
- Do not store Discord tokens, account credentials, or private game data.

## Initial settings

- `game_log_path`: optional explicit override; automatic discovery remains default.
- `display_stat`: initially `health`; future values include `level`, `school`, and `character_name` only when supported.
- `log_level`: optional operational verbosity with privacy-preserving defaults.

## Future settings

Polling/debounce intervals and activity presentation toggles may be added when there is a product need. Keep operational timing values bounded and defaulted; do not require users to tune them.

## Validation and migration

Validate types, enum values, and path existence/shape. On invalid values, report the field and continue with a safe default where possible. Add a config version and explicit migration policy before introducing incompatible format changes.
