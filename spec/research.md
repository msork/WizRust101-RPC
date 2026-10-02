# Research notes and decisions

## M3 Discord IPC dependency review (2026-10-01)

- [Discord's official RPC IPC documentation](https://docs.discord.com/developers/topics/rpc) specifies local IPC, Windows named pipes, `SET_ACTIVITY` fields, and an example `timestamps.start` using `time(nullptr)` (Unix seconds). [Rich Presence documentation](https://docs.discord.com/developers/platform/rich-presence) describes the application asset model. These are primary protocol sources.
- [`discord-rich-presence` 1.1.0 documentation](https://docs.rs/discord-rich-presence/latest/discord_rich_presence/) and its [GitHub source/history](https://github.com/vionya/discord-rich-presence) show a maintained MIT crate with a synchronous client, Windows IPC support, and needed activity fields. Its timestamp struct's source comment says milliseconds; the official Discord RPC example uses seconds, which M3 follows. The adapter is isolated to permit correction if a live Discord smoke check disagrees.
- [`presenceforge` 0.3.0 documentation](https://docs.rs/presenceforge/latest/presenceforge/) shows Windows named-pipe support plus sync and async APIs. It is viable but adds a broader API than needed for the current synchronous watcher. Choose `discord-rich-presence` and implement retry policy in our own testable publisher.
- Do not query Wizard101 Central HTML or API during M3. Its captured export is incomplete and site access is blocked; manually supplied page evidence is welcome. Bacon mappings remain candidates, never automatic truth.

## Stone Town mapping evidence (2026-10-01)

- The sanitized client `W.1.610.21` zone record in `tests/fixtures/current-steam-2026-10-01-zone.log` unambiguously names raw ID `Zafaria/ZF_Z07_Stone_Town`.
- The project owner manually confirms that Stone Town is a location in Zafaria, dated 2026-10-01. The resulting runtime mapping is `Zafaria/ZF_Z07_Stone_Town` → `Stone Town` → `Zafaria`, with both evidence types recorded on the row.
- This is **not** Wizard101 Central verification. The site remained inaccessible to the project owner and was not queried or consulted. The captured Zafaria wiki page and Bacon catalog are not included as verification evidence. Other raw IDs remain unknown.

## M3 live Linux Steam and Discord verification (2026-10-01)

- The project owner reported a successful live smoke test on Linux Steam and supplied a Discord profile screenshot captured around 22:09 local. The screenshot shows activity title `Wizard101`, location `Stone Town`, Zafaria world art, `Last logged health: 3868/3868`, and an elapsed location timer reading `0:24`; the owner confirms the timer was running.
- The watcher output showed automatic discovery of `/home/maxim/.local/share/Steam/steamapps/common/Wizard101/Bin/WizardClient.log`. Together, the live run and screenshot verify the Linux discovery-to-Discord path for this session.
- The screenshot is not committed because it includes personal Discord profile information. No Application ID is recorded in specs, code, launch examples, or other project data. Windows named-pipe/discovery behavior remains unverified at runtime; Windows validation is compile-only.
- The command form verified for Linux is `WIZRUST101_DISCORD_APP_ID=<id> cargo run --bin wizrust101-rpc`, with the placeholder replaced locally. Fake transport tests cover reconnect and error cases; live Discord restart recovery was not demonstrated by this screenshot.

Research checked 2026-10-01. Links are primary project/documentation sources where available; upstream project behavior is evidence about prior implementations, not proof of current game behavior.

## Reference projects

- [Bacon1661/Wizard101-RPC](https://github.com/Bacon1661/Wizard101-RPC): Windows console app; reads health and zone data from `WizardClient.log`; documents standalone `%PROGRAMDATA%` and default Steam paths. Its source looks for `zone = ...` / `CHARACTER LIST`, parses `Updating health globe (...)`, excludes some other-player health records, and uses `GameClient::HandleQuit()` / away-from-keyboard logout as quit signals. It has an extensive `zones.json` catalog. Its current implementation falls back to `WizardCity` for an unrecognized world and starts elapsed time from a location transition; our accuracy policy rejects the fallback. The implementation can launch the game and uses an old Node Discord IPC library; neither behavior is carried forward.
- [ManaUp/WizRPC](https://github.com/ManaUp/WizRPC): archived October 25, 2021. Node app requiring the user to configure the install path; describes live location/stats and updates presence every 15–25 seconds. It contains `zones.csv`/`zones.txt` mapping assets and planned zone API, GUI, and Linux support. Treat these catalogs as candidate historical data, not authoritative current coverage.

## Required location reference

- [Wizard101 Central: Basic:Locations](https://wiki.wizard101central.com/wiki/Basic:Locations) is the required authoritative research source for readable location and world relationships. Observed current-client logs establish raw zone IDs and take precedence if they conflict with a candidate mapping. Never fill unknown worlds from a guessed prefix or fallback.
- Access check on 2026-10-01: the browser tool received HTTP 403 for the exact page, and direct HTTPS access outside the sandbox also received HTTP 403. Search index snippets from other pages did not provide the requested source content. Review paused until the later offline export arrived; the user is not expected to save hundreds of linked pages manually.
- MediaWiki API check on 2026-10-01: one read-only request with the descriptive `WizRust101-RPC/0.1` User-Agent to `https://wiki.wizard101central.com/wiki/api.php?action=query&format=json&titles=Basic%3ALocations&prop=info` returned HTTP/2 **444**, `Content-Type: text/html`, and a 2,395-byte Cloudflare page titled `Access Blocked` saying the request was blocked by its security system. It returned no MediaWiki JSON or page metadata. No links, categories, `Location:` pages, or world pages were queried after that security block, so API hierarchy traversal could not be determined. Do not route around the block or use another source as a substitute.

### Exported research snapshot

- The user supplied [the dated JSON export](../research/wizard101central-locations.json), with `source` pointing to `Basic:Locations`, `exportedAt` `2026-10-01T23:19:35.457Z`, and `pageCount` 176. Programmatic inspection confirms 176 entries: the root page plus 175 `Location:` pages. SHA-256: `d7d427a15341753c15154f9d5a796c01125b12d4edac42f8cd8a450a5fad81d9`.
- Each entry contains `title`, `path`, `url`, and `links`; page bodies, infobox fields, and raw WizardClient.log identifiers are absent. The export is an **incomplete snapshot**, not full wiki coverage. Across its 1,824 recorded links, 774 point to captured pages. A missing page or name proves nothing about whether the location exists.
- The captured `Location:Wizard City` page links to captured `Location:Ravenwood`, and the latter links back to Wizard City. The legacy Bacon candidate maps `WizardCity/WC_Ravenwood` to `Ravenwood`; the snapshot corroborates those readable names and a link relationship, **not** the raw ID's current-client meaning. `Upper Zigazag` from the small checked-in Bacon fixture has no captured page, so it remains a legacy candidate.
- Bacon1661 and WizRPC catalogs supply **candidate raw-ID mappings**. Keep their legacy provenance distinct from Wizard101 Central name evidence and from current-game verification. Do not promote a candidate to current-verified without a real current-client log observation.
- The user reports that subsequent attempts ended in Cloudflare Error 1006 / an IP ban. **Automated HTML and API crawling are prohibited** unless the site owner provides an authorized access method. Work only from the supplied export and other already available evidence; do not retry, route around, or bypass the block.

### User-provided game log evidence

- A `WizardClient.log` at a user-provided Linux Steam install path was readable locally. The user dated it 2026-09-30, and its file modification time was 2026-09-30 17:20 EDT. Its second line reports client version `W.1.610.21` and revision `r806919.Wizard_1_610`. It contains real zone, health, remote-player marker, and character-selection records matching the legacy literal patterns; see [log-parsing.md](log-parsing.md) for counts and evidence limits. An in-game health reading was not supplied, and this capture does not verify Windows behavior.
- The captured raw Zafaria IDs map to `Stone Town` in the Bacon legacy catalog. The exported `Location:Zafaria` page has a `Stone Town` link, but the target page was not captured. This supports only a candidate readable name; it does not verify the raw-ID relationship or full wiki coverage. No runtime mapping was promoted.
- On 2026-10-01, the same Linux Steam log path contained a new `W.1.610.21` session. Its zone record at 19:35:27 was `Zafaria/ZF_Z07_Stone_Town`; sixteen consecutive health records at 19:35:39 ended at `3868/3868`. The user corrected an earlier mistaken reading of the screenshot: it shows **current health `3868`** at about 19:35. The matching value and nearby timestamps strongly support this final logged current value as local and recently observed. The screenshot does not independently show maximum health or a current-location label; quest text mentions Stone Town. The inspected log has no further health/damage record through about 19:36:17. Exact screenshot capture time and the general local-attribution/freshness rules remain unknown. The screenshot is not committed because it includes character names.
- The user later supplied a controlled before/after health change from the same client and Linux Steam session. Screenshot filename/file times are `19:50:53.730` and `19:51:23.530` local. The first visibly shows the local character at `3868/3868`; the second shows current `3455` and floating `413` damage, but no independent maximum. The log's internal `ModifyHealth` calculation at `19:50:56` uses `3868` and `-413`; at `19:51:19` an explicit `Our client is getting hurt!` marker immediately precedes `Updating health globe ... 3455/3868`, followed by matching damage arithmetic and a `3455/3868` meter. A `MSG_CombatHealth` globe line repeats the pair at `19:51:30`. This verifies the local decrease in the log for this encounter. The 23-second calculation-to-globe gap is combat-cinematic timing, not a general log delay; the exact on-screen transition is unknown between still images. Nearby `1530/1530` meters demonstrate that bare meter lines are not local-only. See [log-parsing.md](log-parsing.md) and the [fixture provenance](../tests/fixtures/current-steam-2026-10-01-README.md).
- The user subsequently confirmed that this same screenshot pair was intended as the proposed non-cinematic evidence. The matching `19:51:19` globe line is explicitly `Cinematics ProcessDamageEffect`; at that stage no distinct non-cinematic pair was available. At `19:51:30` and `19:52:08`, `MSG_CombatHealth` repeats `3455/3868`, but those lines have no explicit local marker. Matching a known local value establishes contextual consistency for the encounter, not a reusable per-record local attribution rule. A `19:53:14` `MSG_UpdateHealth` and `19:53:15` `HandleStatisticUpdate` report a later `2916/3868` value after further combat damage; the later 20:13 evidence verifies the exact `MSG_UpdateHealth` source, while `HandleStatisticUpdate` remains unknown.
- Three subsequently supplied open-world screenshots from `20:13:14.107`, `20:13:33.640`, and `20:13:41.403` local show current health `1992`, `2959`, and `3868`. The client `W.1.610.21` log has matching `WizardClientMod MSG_UpdateHealth` pairs `1992/3868` at `20:13:06`, `2959/3868` at `20:13:27`, and `3868/3868` at `20:13:38`. The two increases are outside the `Cinematics ProcessDamageEffect` path; their records precede the next screenshots by about 6.6 and 3.4 seconds. This strongly supports the exact source as local in this build, but the screenshots do not display numeric maximum health or measure the instant of change. A `20:11:26` “not this client's” marker prints identical sent/client-player IDs in the full log. The marker therefore only excludes positive local attribution; it does not prove a distinct other player. The older September marker fixture lost ID equality during redaction. See [log-parsing.md](log-parsing.md) and the [sanitized fixture provenance](../tests/fixtures/current-steam-2026-10-01-README.md).

## Discord

- [Discord RPC documentation](https://discord.com/developers/docs/topics/rpc): Rich Presence can be set over IPC to a running local Discord client. The IPC transport supports Windows named pipes (`\\?\pipe\discord-ipc-{n}`) and command `SET_ACTIVITY`. Current docs recommend Discord Social SDK for new integrations embedded in games; this project is a companion application, so a local IPC client crate is the matching implementation shape to evaluate.
- [Discord Rich Presence overview](https://discord.com/developers/docs/rich-presence/overview): activities support text, timestamps, and uploaded art assets. Discord displays the registered application name as the title; asset keys must refer to application assets.
- [Discord RPC source repository](https://github.com/discord/discord-rpc): documents connection, reconnection, and protocol handling tradeoffs for direct IPC implementations.

## Rust IPC candidates

- [`discord-rich-presence` on docs.rs](https://docs.rs/discord-rich-presence/latest/discord_rich_presence/): documented synchronous Rust API over Discord IPC with Windows and Unix implementations; supports connecting and `set_activity`.
- [`presenceforge` on docs.rs](https://docs.rs/presenceforge/latest/presenceforge/): documented Windows named-pipe and Unix socket transport, sync/async APIs, activity builder, and clear activity. Newer candidate with a larger runtime/API surface.
- Decision: no crate is selected in this spec-only milestone. Select one during the IPC implementation milestone after reviewing source/release history, license, compatibility, and integration testability. Do not implement raw Discord IPC unless candidate review establishes a concrete need.

## Confidence and unverified facts

- Client `W.1.610.21` demonstrates zone and health record syntax, an explicitly local damage/globe sequence, and a screenshot-correlated `WizardClientMod MSG_UpdateHealth` recovery sequence. Other versions, local attribution for other globe sources, and a general health-freshness bound remain unverified.
- Current location catalogs and world assets are unverified for completeness, accuracy, permission, and Discord application ownership.
- Non-health stats have no verified source in the research performed for this milestone.
- Steam's additional-library discovery and installation metadata need implementation-time validation on Windows.

## Decisions captured

| Decision | Status | Reason |
| --- | --- | --- |
| Windows first | Accepted | Product requirement and reference implementations target Windows. |
| Read logs; do not inspect game memory | Accepted | Meets automatic tracking requirement with a low-impact local source and is testable. |
| Unknown world stays unknown | Accepted | Product requirement forbids invented game data. |
| IPC crate behind adapter | Accepted, crate TBD | Discord documents local IPC; isolates protocol choices. |
| Automatic discovery is default; explicit path override is recovery | Accepted | Satisfies no-manual-entry requirement while allowing diagnosis. |
