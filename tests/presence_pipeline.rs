use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use wizrust101_rpc::{
    discord::{DiscordTransport, PresencePublisher},
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig, WorldAssetCatalog},
    replay::{replay_existing_log, replay_existing_log_with_report},
    state::GameState,
};

/// Positive-control fixture using the authoritative pinned DB output.
fn verified_stone_test_catalog() -> ZoneCatalog {
    wizrust101_rpc::mapping::runtime_catalog().expect("DB catalog")
}

#[test]
fn captured_local_health_is_ingested_but_presence_uses_location_and_world() {
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = verified_stone_test_catalog();
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
        fallback_asset_key: assets.fallback,
    };
    let now = Instant::now();
    for line in include_str!("fixtures/current-steam-2026-10-01-zone.log").lines() {
        for event in parser.parse_line(line) {
            state.apply(event, &catalog, now);
        }
    }
    for line in include_str!("fixtures/current-steam-2026-10-01-recovery-rises.log").lines() {
        for event in parser.parse_line(line) {
            state.apply(event, &catalog, now);
        }
    }
    let before = Presence::from_game_state(&state, &config, now, SystemTime::now())
        .expect("verified location should produce presence");
    assert_eq!(before.state.as_deref(), Some("Zafaria"));
    assert_eq!(before.details.as_deref(), Some("Stone Town"));
    assert_eq!(before.large_image.as_deref(), Some("zafaria"));
    assert_eq!(before.large_text.as_deref(), Some("Zafaria"));
    assert_eq!(before.small_image.as_deref(), Some("wizrust101_rpc"));
    assert_eq!(before.small_text.as_deref(), Some("WizRust101-RPC"));

    for line in include_str!("fixtures/current-steam-2026-10-01-combat-health-repeat.log").lines() {
        for event in parser.parse_line(line) {
            state.apply(event, &catalog, now + Duration::from_secs(1));
        }
    }
    let after = Presence::from_game_state(
        &state,
        &config,
        now + Duration::from_secs(1),
        SystemTime::now(),
    )
    .expect("verified location remains present");
    assert_eq!(after.state, before.state);
    assert_eq!(state.health_observed_at, Some(now));
    assert_eq!(after.details.as_deref(), Some("Stone Town"));
    assert_eq!(after.large_image.as_deref(), Some("zafaria"));
}

#[test]
fn database_name_is_used_exactly_as_supplied() {
    let catalog = wizrust101_rpc::mapping::runtime_catalog().expect("pinned DB catalog");
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
        fallback_asset_key: assets.fallback,
    };
    let now = Instant::now();
    let mut state = GameState::default();
    state.apply(
        wizrust101_rpc::parser::GameEvent::ZoneChanged {
            raw_zone_id: "Zafaria/ZF_Z07_Stone_Town".into(),
        },
        &catalog,
        now,
    );

    let mapping = state.location.as_ref().unwrap().mapping.as_ref().unwrap();
    assert_eq!(mapping.location, "Stone Town");
    assert_eq!(mapping.world.as_deref(), Some("Zafaria"));
    let presence = Presence::from_game_state(&state, &config, now, SystemTime::now())
        .expect("independent owner evidence confirms the DB name");
    assert_eq!(presence.details.as_deref(), Some("Stone Town"));
    assert_eq!(presence.state.as_deref(), Some("Zafaria"));
    assert_eq!(presence.large_image.as_deref(), Some("zafaria"));
}

#[test]
fn startup_zone_is_published_after_discord_connects_without_a_new_zone_line() {
    let root = tempfile::tempdir().expect("temporary log directory");
    let path = root.path().join("WizardClient.log");
    let history = concat!(
        "10/01/26 19:00:00 [DBGM] CORE_SEER       CHARACTER LIST\n",
        "10/01/26 19:01:00 [STAT] CLIENT          ClientZone::Load() zone = Zafaria/ZF_Z07_Stone_Town, (id=::<redacted>)\n"
    );
    std::fs::write(&path, history).expect("write existing game history");

    // Keep production startup ordering: position the live tailer first, then
    // replay only the bounded prefix before the first presence publication.
    let mut tailer = LogTailer::open(&path, StartPosition::End).expect("open live tailer");
    let boundary = tailer.offset();
    let startup = Instant::now();
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = verified_stone_test_catalog();
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
        fallback_asset_key: assets.fallback,
    };
    let replay = replay_existing_log_with_report(
        &path,
        boundary,
        &mut parser,
        &mut state,
        &catalog,
        startup,
    )
    .expect("replay startup history");
    assert_eq!(replay.zone_events, 1);
    assert_eq!(replay.selection_events, 1);
    assert!(tailer.poll().expect("no new zone line arrived").is_empty());

    let desired = Presence::from_game_state(&state, &config, startup, SystemTime::now())
        .expect("verified zone remains the final game state");
    let observed = Arc::new(Mutex::new((Vec::new(), Vec::new())));
    let mut publisher = PresencePublisher::new(ConnectOnceUnavailable {
        fail_first_connect: true,
        observed: Arc::clone(&observed),
    });

    assert!(publisher.tick(Some(&desired), startup).is_some());
    assert!(!publisher.is_connected());
    assert!(
        publisher
            .tick(Some(&desired), startup + Duration::from_millis(999))
            .is_none()
    );
    assert!(!publisher.is_connected());
    assert!(
        publisher
            .tick(Some(&desired), startup + Duration::from_secs(1))
            .is_none()
    );
    assert!(publisher.is_connected());

    let (calls, published) = observed.lock().expect("fake transport state").clone();
    assert_eq!(calls, ["connect", "disconnect", "connect", "publish"]);
    assert_eq!(published, [desired]);
}

struct ConnectOnceUnavailable {
    fail_first_connect: bool,
    observed: Arc<Mutex<(Vec<&'static str>, Vec<Presence>)>>,
}

impl DiscordTransport for ConnectOnceUnavailable {
    fn connect(&mut self) -> Result<(), String> {
        let mut observed = self.observed.lock().expect("fake transport state");
        observed.0.push("connect");
        if self.fail_first_connect {
            self.fail_first_connect = false;
            Err("Discord is still starting".into())
        } else {
            Ok(())
        }
    }

    fn publish(&mut self, presence: &Presence) -> Result<(), String> {
        let mut observed = self.observed.lock().expect("fake transport state");
        observed.0.push("publish");
        observed.1.push(presence.clone());
        Ok(())
    }

    fn clear(&mut self) -> Result<(), String> {
        self.observed
            .lock()
            .expect("fake transport state")
            .0
            .push("clear");
        Ok(())
    }

    fn disconnect(&mut self) {
        self.observed
            .lock()
            .expect("fake transport state")
            .0
            .push("disconnect");
    }
}

#[test]
fn restart_restores_existing_verified_zone_before_any_new_zone_event() {
    let root = tempfile::tempdir().expect("temporary log directory");
    let path = root.path().join("WizardClient.log");
    let mut existing = include_str!("fixtures/current-steam-2026-10-01-zone.log").to_owned();
    existing.push('\n');
    std::fs::write(&path, existing).expect("write existing WizardClient.log history");

    // Match app startup ordering: capture the end offset first, then replay
    // only the history that existed at that point.
    let mut tailer = LogTailer::open(&path, StartPosition::End).expect("open live tailer");
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = verified_stone_test_catalog();
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
        fallback_asset_key: assets.fallback,
    };
    let startup = Instant::now();
    let boundary = tailer.offset();

    replay_existing_log(&path, boundary, &mut parser, &mut state, &catalog, startup)
        .expect("replay startup history");

    let restored = Presence::from_game_state(&state, &config, startup, SystemTime::now())
        .expect("startup should immediately restore verified presence");
    assert_eq!(restored.details.as_deref(), Some("Stone Town"));
    assert_eq!(restored.state.as_deref(), Some("Zafaria"));
    assert_eq!(restored.large_image.as_deref(), Some("zafaria"));
    assert_eq!(
        state
            .location
            .as_ref()
            .expect("restored location")
            .entered_at,
        startup,
        "restart intentionally begins a fresh location timer"
    );

    let append = b"10/01/26 19:36:00 [STAT] CLIENT          zone = Unverified/Unknown_Zone,\n";
    use std::io::Write;
    std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open appended log")
        .write_all(append)
        .expect("append subsequent event");
    let lines = tailer.poll().expect("tail only bytes after boundary");
    assert_eq!(lines.len(), 1);
    assert!(tailer.bytes_read_last_poll() <= append.len());
    for line in lines {
        for event in parser.parse_line(&line) {
            state.apply(event, &catalog, startup + Duration::from_secs(5));
        }
    }
    let after_unknown = Presence::from_game_state(
        &state,
        &config,
        startup + Duration::from_secs(5),
        SystemTime::now(),
    );
    assert!(
        after_unknown.is_none(),
        "unknown zone must clear readable presence"
    );
}
