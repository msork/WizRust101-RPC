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
- `display_stat`: `health` by default or `none`. Health is the only supported stat for the current verified log-only approach. Level, School, and Character Name are unsupported/excluded and must be rejected; reconsider only if a future client exposes reliable selected-character-attributed evidence.
- `log_level`: optional operational verbosity with privacy-preserving defaults.

M3 uses `WIZRUST101_DISCORD_APP_ID` as a development-only Discord application ID override; absent or invalid ID disables IPC while ingestion continues. The verified Linux launch command is `WIZRUST101_DISCORD_APP_ID=<id> cargo run --bin wizrust101-rpc`; replace the placeholder locally. Public packaging needs one project-owned registered application named `Wizard101` and uploaded PNG assets so ordinary users need no ID. `WIZRUST101_DISPLAY_STAT` accepts `health` (default) or `none`; invalid values fall back to `health` with a diagnostic. M4's controlled two-character research found no attributable Level or School record; both are unsupported and rejected. Character Name remains excluded. These fields are not to be obtained through memory scanning, injection, packet interception, OCR, or guessed/indirect values. The versioned `data/world-assets.json` registry maps verified world IDs to approved uploaded asset keys; Zafaria currently maps to `zafaria`. M5 will implement the already specified persistent, versioned user configuration; see [m5-plan.md](m5-plan.md).

## Future settings

Polling/debounce intervals and activity presentation toggles may be added when there is a product need. Keep operational timing values bounded and defaulted; do not require users to tune them.

## Validation and migration

Validate types, enum values, and path existence/shape. On invalid values, report the field and continue with a safe default where possible. Add a config version and explicit migration policy before introducing incompatible format changes.
