use std::time::{Duration, Instant, SystemTime};

use wizrust101_rpc::{
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig, WorldAssetCatalog},
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
