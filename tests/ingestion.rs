use std::{fs, io::Write, time::Instant};

use tempfile::tempdir;
use wizrust101_rpc::{
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::{GameEvent, Health, HealthAttribution, HealthObservation, LogParser},
    state::{GameActivity, GameState},
};

fn health_event(current: u32, maximum: u32, attribution: HealthAttribution) -> GameEvent {
    GameEvent::HealthObserved(HealthObservation {
        health: Health { current, maximum },
        attribution,
    })
}

#[test]
fn reference_derived_fixture_flows_through_parser_and_typed_state() {
    let fixture = include_str!("fixtures/reference-derived.log");
    let mut parser = LogParser::default();
    let catalog = ZoneCatalog::default();
    let mut state = GameState::default();
    let now = Instant::now();
    let mut observed = Vec::new();

    for line in fixture.lines().filter(|line| !line.starts_with('#')) {
        for event in parser.parse_line(line) {
            observed.push(event.clone());
            state.apply(event, &catalog, now);
        }
    }

    assert_eq!(
        observed,
        [
            GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/WC_Ravenwood".to_owned()
            },
            health_event(125, 300, HealthAttribution::Unknown),
            health_event(10, 100, HealthAttribution::Unknown),
            GameEvent::CharacterSelection
        ]
    );
    assert_eq!(state.activity, GameActivity::CharacterSelection);
    assert_eq!(state.location, None);
    assert_eq!(state.health, None);
}

#[test]
fn captured_steam_zone_and_selection_records_parse() {
    let mut parser = LogParser::default();
    let zone = include_str!("fixtures/current-steam-zone.log")
        .lines()
        .next()
        .expect("zone line");
    assert_eq!(
        parser.parse_line(zone),
        [GameEvent::ZoneChanged {
            raw_zone_id: "Zafaria/ZF_Z07_Stone_Town".to_owned()
        }]
    );

    let selection = include_str!("fixtures/current-steam-selection.log")
        .lines()
        .next()
        .expect("selection line");
    assert_eq!(
        parser.parse_line(selection),
        [GameEvent::CharacterSelection]
    );
}

#[test]
fn system_information_level_candidate_is_not_a_game_event() {
    let mut parser = LogParser::default();
    for line in include_str!("fixtures/current-steam-2026-10-01-stats-candidate.log").lines() {
        assert!(
            parser.parse_line(line).is_empty(),
            "system-information line parsed: {line}"
        );
    }
}

#[test]
fn controlled_two_character_window_parses_only_selection_and_zone_events() {
    let mut parser = LogParser::default();
    let events = include_str!("fixtures/current-steam-2026-10-02-selection-stats.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();

    assert_eq!(
        events,
        [
            GameEvent::CharacterSelection,
            GameEvent::ZoneChanged {
                raw_zone_id: "Zafaria/ZF_Z07_Stone_Town".to_owned()
            },
            GameEvent::CharacterSelection,
            GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/Interiors/WC_Headmistress_House".to_owned()
            }
        ]
    );
}

#[test]
fn captured_steam_health_series_preserves_observed_over_max_values_as_unknown() {
    let mut parser = LogParser::default();
    let events = include_str!("fixtures/current-steam-health-series.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();

    assert_eq!(events.len(), 15);
    assert_eq!(
        events.first(),
        Some(&health_event(3841, 2474, HealthAttribution::Unknown))
    );
    assert!(events.contains(&health_event(3841, 3841, HealthAttribution::Unknown)));
    parser.reset();
    assert!(parser.parse_line("next source line").is_empty());
}

#[test]
fn redacted_remote_marker_does_not_claim_other_player_attribution() {
    let mut parser = LogParser::default();
    let events = include_str!("fixtures/current-steam-remote-health.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();
    assert_eq!(
        events,
        [health_event(3841, 2474, HealthAttribution::Unknown)]
    );
}

#[test]
fn current_session_equal_ids_keep_ambiguous_marker_unknown() {
    let mut parser = LogParser::default();
    let events = include_str!("fixtures/current-steam-2026-10-01-contradictory-marker.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();
    assert_eq!(
        events,
        [health_event(2705, 2501, HealthAttribution::Unknown)]
    );
}

#[test]
fn october_steam_log_window_parses_recorded_zone_and_health() {
    let mut parser = LogParser::default();
    let zone = include_str!("fixtures/current-steam-2026-10-01-zone.log")
        .lines()
        .next()
        .expect("captured zone line");
    assert_eq!(
        parser.parse_line(zone),
        [GameEvent::ZoneChanged {
            raw_zone_id: "Zafaria/ZF_Z07_Stone_Town".to_owned()
        }]
    );

    let observed = include_str!("fixtures/current-steam-2026-10-01-health-window.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();
    assert_eq!(observed.len(), 16);
    assert_eq!(
        observed.first(),
        Some(&health_event(3868, 2501, HealthAttribution::Unknown))
    );
    assert_eq!(
        observed.last(),
        Some(&health_event(3868, 3868, HealthAttribution::Unknown))
    );
}

#[test]
fn controlled_local_hit_globe_record_parses_the_observed_health_pair() {
    let mut parser = LogParser::default();
    let mut lines = include_str!("fixtures/current-steam-2026-10-01-local-hit.log").lines();
    assert!(
        parser
            .parse_line(lines.next().expect("explicit local hit marker"))
            .is_empty()
    );
    assert!(
        parser
            .parse_line(lines.next().expect("health globe record"))
            .is_empty()
    );
    assert_eq!(
        parser.parse_line(lines.next().expect("following sound record")),
        [health_event(3455, 3868, HealthAttribution::Local)]
    );
    for line in lines {
        assert!(parser.parse_line(line).is_empty());
    }
}

#[test]
fn same_value_combat_health_repeat_does_not_inherit_local_attribution() {
    let mut parser = LogParser::default();
    let mut state = GameState::default();
    let catalog = ZoneCatalog::default();
    let first = Instant::now();
    for line in include_str!("fixtures/current-steam-2026-10-01-local-hit.log").lines() {
        for event in parser.parse_line(line) {
            state.apply(event, &catalog, first);
        }
    }
    let verified_state = state.clone();
    assert_eq!(
        state.health,
        Some(Health {
            current: 3455,
            maximum: 3868
        })
    );

    let events = include_str!("fixtures/current-steam-2026-10-01-combat-health-repeat.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();
    assert_eq!(
        events,
        [health_event(3455, 3868, HealthAttribution::Unknown)]
    );
    for event in events {
        state.apply(event, &catalog, first + std::time::Duration::from_secs(11));
    }
    assert_eq!(state, verified_state);
}

#[test]
fn controlled_recovery_messages_update_verified_local_health() {
    let mut parser = LogParser::default();
    let catalog = ZoneCatalog::default();
    let mut state = GameState::default();
    let mut events = Vec::new();
    let now = Instant::now();
    for line in include_str!("fixtures/current-steam-2026-10-01-recovery-baseline.log")
        .lines()
        .chain(include_str!("fixtures/current-steam-2026-10-01-recovery-rises.log").lines())
    {
        for event in parser.parse_line(line) {
            state.apply(event.clone(), &catalog, now);
            events.push(event);
        }
    }
    assert_eq!(
        events,
        [
            health_event(1992, 3868, HealthAttribution::Local),
            health_event(2959, 3868, HealthAttribution::Local),
            health_event(3868, 3868, HealthAttribution::Local),
        ]
    );
    assert_eq!(
        state.health,
        Some(Health {
            current: 3868,
            maximum: 3868
        })
    );
    assert_eq!(state.health_observed_at, Some(now));
}

#[test]
fn controlled_damage_calculation_and_mixed_meters_are_not_globe_updates() {
    let mut parser = LogParser::default();
    for line in include_str!("fixtures/current-steam-2026-10-01-damage-calculation.log")
        .lines()
        .chain(include_str!("fixtures/current-steam-2026-10-01-mixed-meters.log").lines())
    {
        assert!(parser.parse_line(line).is_empty());
    }
}

#[test]
fn tailer_reads_fixture_bytes_only_once_from_the_current_offset() {
    let directory = tempdir().expect("temp dir");
    let path = directory.path().join("WizardClient.log");
    fs::write(&path, "").expect("create file");
    let mut tailer = LogTailer::open(&path, StartPosition::Beginning).expect("open tailer");
    let line = b"zone = WizardCity/WC_Ravenwood,\n";
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open append")
        .write_all(line)
        .expect("write line");

    assert_eq!(
        tailer.poll().expect("read first append"),
        ["zone = WizardCity/WC_Ravenwood,"]
    );
    assert_eq!(tailer.bytes_read_last_poll(), line.len());
    assert!(tailer.poll().expect("poll unchanged").is_empty());
    assert_eq!(tailer.bytes_read_last_poll(), 0);
}

#[test]
fn unknown_raw_zone_survives_without_catalog_fallback() {
    let mut state = GameState::default();
    state.apply(
        GameEvent::ZoneChanged {
            raw_zone_id: "UnknownWorld/UnknownZone".to_owned(),
        },
        &ZoneCatalog::default(),
        Instant::now(),
    );
    let location = state.location.expect("retain raw zone");
    assert_eq!(location.raw_zone_id, "UnknownWorld/UnknownZone");
    assert!(location.mapping.is_none());
}
