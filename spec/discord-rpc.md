# Discord Rich Presence

## Activity layout

- Application/activity title: `Wizard101` (Discord displays the registered application's name).
- Large image: current world's PNG asset key, if the world is verified and the asset is configured.
- Image hover text: verified world name.
- Details/location line: verified current location.
- State/stat line: the configured stat, initially Health when verified as the local character's value. Support the future stat selector without displaying unsupported or unattributed data.
- Elapsed time: Discord start timestamp (Unix seconds) derived from the monotonic entry time of a verified displayed-location change. It stays stable for duplicate zone records, verified raw-ID aliases of the same location/world, and health changes. Omit it when the location mapping is unverified or clocks cannot be reconciled.

## M3 presence construction contract

- Construct an owned, comparable presence value from `GameState` without Discord or filesystem dependencies. Publish only while activity is `Running`; otherwise clear any prior presence.
- A location and world may be displayed only from a `Verified` mapping. An unknown or legacy-only raw zone yields no Details, world image, world hover text, or elapsed location timer. Never display a raw zone identifier as a readable location.
- Default State to `Health` from GameState's locally attributed health and observation time. Because no game freshness bound is verified, label it as a *last logged* value and use a conservative 60-second presentation lifetime measured from parser receipt. Expiry only removes the field; it does not change GameState or claim a game-side update bound. Missing timestamp, future timestamp, or expired data omits State. An `Unknown` observation cannot populate or refresh it.
- Permit `health` or `none` as currently working stat choices. Level, school, and character name remain future choices until their sources are verified.
- A verified world needs a separately configured, uploaded Discord application PNG asset key before `large_image` and `large_text` are sent. The versioned `data/world-assets.json` registry starts empty. Do not derive an asset key from the world ID or a local filename. Omit both asset fields if the registered key is unavailable.
- Avoid empty activity payloads: if no trusted Details or State exists, clear the existing presence. Discord title comes from the registered application named `Wizard101`; the adapter also sets the activity name to `Wizard101` when supported.

## IPC behavior

- Talk to the user's running Discord desktop client through local IPC on Windows; no Discord web API, bot, OAuth, or remote network service is needed for basic presence.
- Connect/reconnect when Discord starts or restarts. Publish only changed activity payloads, except for retries after reconnect.
- Clear activity when Wizard101 ends, transitions to character selection, or the selected log becomes unavailable. Treat discovery failures as recoverable and clear stale activity.
- Handle missing Discord as a recoverable state and continue game monitoring.
- Use the app's registered application ID and uploaded asset keys. Keep the ID centralized/configurable for development where appropriate; never require users to create an app for ordinary use.
- Truncate user-facing fields at 128 UTF-8 bytes, and avoid republishing unchanged payloads within a 60-second heartbeat interval. The heartbeat probes for a broken connection so Discord restarts can be detected even when the game state is unchanged. Polling provides a natural debounce for log bursts.
- Keep desired presence separate from connection state. On connect or publication failure, discard the connection, retain the latest desired value, and retry after bounded backoff. Reconnect republishes the latest activity; a pending clear does not require connecting solely to clear. IPC errors must not stop the log watcher.

## Rust transport decision

M3 selects `discord-rich-presence` 1.1.0 (MIT) behind a synchronous adapter. It supports Windows named pipes and Linux sockets, `connect`/`set_activity`/`clear_activity`/`close`, and the needed Details, State, timestamps, and assets without an async runtime. `presenceforge` 0.3.0 is an alternative with sync/async APIs and built-in retry features; its wider surface is unnecessary for this synchronous watcher. Our publisher owns reconnect policy so it is independently testable. See [research.md](research.md) for dated primary sources and the timestamp-unit discrepancy between the crate comment and Discord's official IPC example; send Unix **seconds** per Discord's example.

## Limits

Rich Presence data is visible to Discord users according to Discord activity/privacy settings. The app should send only the configured stat and verified game context. Discord image assets are registered to an application; local PNGs are not automatically sent through IPC.

## Live smoke-test setup

1. In the [Discord Developer Portal](https://discord.com/developers/applications), create an application for this project and use `Wizard101` as its displayed name.
2. Copy its **Application ID** from **General Information**. This numeric ID is the development value for `WIZRUST101_DISCORD_APP_ID`; it is not a bot token or secret.
3. Under **Rich Presence → Art Assets**, upload a square Zafaria world PNG. Discord recommends artwork at least 1024×1024 pixels. Name the uploaded asset with a stable key (suggested: `zafaria`) and save it. Provide the exact key so it can be added to `data/world-assets.json` under world ID `Zafaria`.
4. Keep Discord Desktop running and signed in, enable activity sharing, and run the watcher with the application ID environment variable. The watcher starts at the current log end; cause a new zone entry or verified local health update after startup to produce presence. First verify `Stone Town` Details and the Health State; verify the image after the matching uploaded key is configured.

The IPC integration needs no OAuth flow or bot token. Do not put tokens or credentials in the repository.
