use std::{fs, io::Write, process::Command, time::Instant};

use tempfile::tempdir;
use wizrust101_rpc::{
    log_tailer::{LogTailer, StartPosition},
    mapping::ZoneCatalog,
    parser::{GameEvent, Health, LogParser},
    state::{GameActivity, GameState},
};

#[test]
fn reference_derived_fixture_flows_through_parser_and_typed_state() {
    let fixture = include_str!("fixtures/reference-derived.log");
    let mut parser = LogParser::default();
    let catalog = ZoneCatalog::import_bacon_json(
        include_bytes!("fixtures/bacon-zones-excerpt.json").as_slice(),
        "fixture excerpt",
        1,
    )
    .expect("load catalog");
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
            GameEvent::HealthChanged(Health {
                current: 125,
                maximum: 300
            }),
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
fn captured_steam_health_series_preserves_observed_over_max_values() {
    let mut parser = LogParser::default();
    let events = include_str!("fixtures/current-steam-health-series.log")
        .lines()
        .flat_map(|line| parser.parse_line(line))
        .collect::<Vec<_>>();

    assert_eq!(events.len(), 15);
    assert_eq!(
        events.first(),
        Some(&GameEvent::HealthChanged(Health {
            current: 3841,
            maximum: 2474
        }))
    );
    assert!(events.contains(&GameEvent::HealthChanged(Health {
        current: 3841,
        maximum: 3841
    })));
    parser.reset();
    assert!(parser.parse_line("next source line").is_empty());
}

#[test]
fn captured_steam_remote_marker_excludes_its_preceding_health_record() {
    let mut parser = LogParser::default();
    for line in include_str!("fixtures/current-steam-remote-health.log").lines() {
        assert!(parser.parse_line(line).is_empty());
    }
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
        Some(&GameEvent::HealthChanged(Health {
            current: 3868,
            maximum: 2501
        }))
    );
    assert_eq!(
        observed.last(),
        Some(&GameEvent::HealthChanged(Health {
            current: 3868,
            maximum: 3868
        }))
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
        [GameEvent::HealthChanged(Health {
            current: 3455,
            maximum: 3868
        })]
    );
    for line in lines {
        assert!(parser.parse_line(line).is_empty());
    }
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

#[test]
fn bacon_importer_binary_writes_a_loadable_catalog() {
    let directory = tempdir().expect("temp dir");
    let output_path = directory.path().join("zones.json");
    let status = Command::new(env!("CARGO_BIN_EXE_import-bacon-zones"))
        .arg("tests/fixtures/bacon-zones-excerpt.json")
        .arg(&output_path)
        .status()
        .expect("run importer");

    assert!(status.success());
    let catalog =
        ZoneCatalog::from_reader(fs::File::open(output_path).expect("open imported catalog"))
            .expect("load imported catalog");
    assert_eq!(
        catalog
            .resolve("WizardCity/WC_Ravenwood")
            .expect("imported Ravenwood")
            .location,
        "Ravenwood"
    );
}
