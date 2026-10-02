# Configuration

## Principles

- Normal startup requires no prompts or manual game data.
- Start with safe built-in defaults. Missing or invalid optional configuration must not prevent discovery.
- Use the versioned JSON format below. Preserve unknown fields through typed decode/encode round-trips; this release does not automatically rewrite a user's file.
- Keep the optional game-log path override and selected stat. Do not expose unsupported stat choices as working features.
- Store configuration in the platform-native per-user configuration directory, not beside the executable or in the Wizard101 install. Use the maintained Rust `directories` crate `ProjectDirs::config_dir()` API: [directories 6.0.0 docs](https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html), which follows [XDG Base Directory](https://specifications.freedesktop.org/basedir/0.8/) on Linux and the Windows Known Folder API ([Microsoft reference](https://learn.microsoft.com/windows/win32/shell/knownfolderid)).
- Do not store Discord tokens, account credentials, or private game data.

## Initial settings

- `game_log_path`: optional explicit override; automatic discovery remains default.
- `display_stat`: `health` by default or `none`. Health is the only supported stat for the current verified log-only approach. Level, School, and Character Name are unsupported/excluded and must be rejected; reconsider only if a future client exposes reliable selected-character-attributed evidence.
- `log_level`: `warn` by default; accepted values are `error`, `warn`, `info`, and `debug`.

## Version 1 schema and location

The file is named `config.json` inside the `ProjectDirs::from("com", "msork", "WizRust101-RPC").config_dir()` directory:

- Linux: `$XDG_CONFIG_HOME/wizrust101-rpc/config.json`, or `~/.config/wizrust101-rpc/config.json` when `XDG_CONFIG_HOME` is unset/empty.
- Windows: `%APPDATA%\msork\WizRust101-RPC\config\config.json` (Roaming AppData).

Example:

```json
{
  "schema_version": 1,
  "game_log_path": null,
  "display_stat": "health",
  "log_level": "warn"
}
```

`game_log_path` may be an absolute path or a path relative to the process working directory, and must name an existing regular file named `WizardClient.log`. Invalid/unavailable overrides produce a warning and fall back to automatic discovery. If the file is absent, all built-in defaults apply; the app does not create or rewrite a config file automatically. Users may create/edit the example file themselves.

Configuration precedence for user settings is built-in defaults, then valid per-field values from `config.json`, then valid environment overrides. `WIZRUST101_DISPLAY_STAT` accepts `health` or `none`; `WIZRUST101_LOG_LEVEL` accepts `error`, `warn`, `info`, or `debug`. Invalid environment values warn and fall back to the file value or default. The Discord Application ID is not a user setting and must not be added to this schema. Official release builds embed the project-owned ID as a release build value, so users do not create a Discord application or configure an ID. Development and testing may use `WIZRUST101_DISCORD_APP_ID` as a runtime override; the ID is not persisted. The verified Linux development launch form remains `WIZRUST101_DISCORD_APP_ID=<id> cargo run --bin wizrust101-rpc`.

M4's controlled two-character research found no attributable Level or School record; both are unsupported and rejected. Character Name remains excluded. These fields are not to be obtained through memory scanning, injection, packet interception, OCR, or guessed/indirect values. The versioned `data/world-assets.json` registry maps verified world IDs to approved uploaded asset keys; Zafaria currently maps to `zafaria`.

## Future settings

Polling/debounce intervals and activity presentation toggles may be added when there is a product need. Keep operational timing values bounded and defaulted; do not require users to tune them.

## Validation and migration

Validate JSON shape, exact integer schema version, field types, enum values, and the configured path's filename/existence/type. A malformed root or unsupported `schema_version` loads built-in defaults with a diagnostic and leaves the original file byte-for-byte untouched. Invalid individual fields use their safe defaults while valid sibling fields are retained. Version 1 is the only supported format; there is no automatic migration or rewrite. Future incompatible schemas require an explicit migration milestone; until then, unsupported files are preserved and users may back them up and replace them with a documented v1 file. Unknown fields are retained in the typed document for forward-compatible decode/encode round-trips.
