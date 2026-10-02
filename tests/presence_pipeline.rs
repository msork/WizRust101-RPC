use std::time::{Duration, Instant, SystemTime};

use wizrust101_rpc::{
    mapping::ZoneCatalog,
    parser::LogParser,
    presence::{Presence, PresenceConfig},
    state::GameState,
};

#[test]
fn captured_local_recovery_publishes_health_but_unknown_repeat_does_not_replace_it() {
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = ZoneCatalog::from_reader(include_bytes!("../data/zones.json").as_slice())
        .expect("verified runtime zone catalog");
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
    let before =
        Presence::from_game_state(&state, &PresenceConfig::default(), now, SystemTime::now())
            .expect("observed local health");
    assert_eq!(
        before.state.as_deref(),
        Some("Last logged health: 3868/3868")
    );
    assert_eq!(before.details.as_deref(), Some("Stone Town"));
    assert_eq!(
        before.large_image, None,
        "no uploaded asset key is configured"
    );

    for line in include_str!("fixtures/current-steam-2026-10-01-combat-health-repeat.log").lines() {
        for event in parser.parse_line(line) {
            state.apply(event, &catalog, now + Duration::from_secs(1));
        }
    }
    let after = Presence::from_game_state(
        &state,
        &PresenceConfig::default(),
        now + Duration::from_secs(1),
        SystemTime::now(),
    )
    .expect("last local health retained");
    assert_eq!(after.state, before.state);
    assert_eq!(state.health_observed_at, Some(now));
    assert_eq!(after.details.as_deref(), Some("Stone Town"));
    assert_eq!(after.large_image, None);
}
