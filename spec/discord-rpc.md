# Discord Rich Presence

## Activity layout

- Application/activity title: `Wizard101` (Discord displays the registered application's name).
- Large image: current world's PNG asset key, if the world is verified and the asset is configured.
- Image hover text: verified world name.
- Details/location line: verified current location.
- State/stat line: the configured stat, initially Health when available. Support the future stat selector without displaying unsupported data.
- Elapsed time: Discord start timestamp set when the current normalized location is entered. It stays stable as health changes and resets on location changes.

## IPC behavior

- Talk to the user's running Discord desktop client through local IPC on Windows; no Discord web API, bot, OAuth, or remote network service is needed for basic presence.
- Connect/reconnect when Discord starts or restarts. Publish only changed activity payloads, except for retries after reconnect.
- Clear activity when Wizard101 ends, transitions to character selection, or remains unavailable beyond the agreed grace period.
- Handle missing Discord as a recoverable state and continue game monitoring.
- Use the app's registered application ID and uploaded asset keys. Keep the ID centralized/configurable for development where appropriate; never require users to create an app for ordinary use.
- Respect Discord payload field limits and rate limits; debounce bursts of log updates and avoid republishing unchanged state.

## Rust transport decision

Use a Rust crate that supports Discord's current RPC over IPC and Windows named pipes. Keep the crate API behind an adapter so it can be replaced. Before selecting it, check crate maintenance, license, Windows transport, reconnection semantics, async/runtime costs, and activity field support against current docs and source.

## Limits

Rich Presence data is visible to Discord users according to Discord activity/privacy settings. The app should send only the configured stat and verified game context. Discord image assets are registered to an application; local PNGs are not automatically sent through IPC.
