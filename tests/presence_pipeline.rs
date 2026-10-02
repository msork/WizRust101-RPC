use std::time::{Duration, Instant, SystemTime};

use wizrust101_rpc::{
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig, WorldAssetCatalog},
    replay::replay_existing_log,
    state::GameState,
};

#[test]
fn captured_local_health_is_ingested_but_presence_uses_location_and_world() {
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = ZoneCatalog::from_reader(include_bytes!("../data/zones.json").as_slice())
        .expect("verified runtime zone catalog");
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
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
    let catalog = ZoneCatalog::from_reader(include_bytes!("../data/zones.json").as_slice())
        .expect("verified runtime zone catalog");
    let assets =
        WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
            .expect("world asset catalog");
    let config = PresenceConfig {
        world_asset_keys: assets.worlds,
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
