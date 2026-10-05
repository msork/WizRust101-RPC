use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    time::Instant,
};

use crate::{
    mapping::ZoneCatalog,
    parser::{GameEvent, LogParser},
    state::GameState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReplayReport {
    pub complete_lines: usize,
    pub zone_events: usize,
    pub selection_events: usize,
    pub health_events: usize,
}

/// Replays complete log records from the file prefix captured when the live
/// tailer was opened. The scan is forward-only and keeps memory bounded to one
/// line, while the tailer remains positioned at `byte_limit` for new data.
pub fn replay_existing_log(
    path: impl AsRef<Path>,
    byte_limit: u64,
    parser: &mut LogParser,
    state: &mut GameState,
    catalog: &ZoneCatalog,
    observed_at: Instant,
) -> io::Result<usize> {
    Ok(
        replay_existing_log_with_report(path, byte_limit, parser, state, catalog, observed_at)?
            .complete_lines,
    )
}

pub fn replay_existing_log_with_report(
    path: impl AsRef<Path>,
    byte_limit: u64,
    parser: &mut LogParser,
    state: &mut GameState,
    catalog: &ZoneCatalog,
    observed_at: Instant,
) -> io::Result<ReplayReport> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file.take(byte_limit));
    let mut line = Vec::new();
    let mut report = ReplayReport::default();

    loop {
        line.clear();
        let bytes = reader.read_until(b'\n', &mut line)?;
        if bytes == 0 || line.last() != Some(&b'\n') {
            break;
        }
        line.pop();
        if line.last() == Some(&b'\r') {
            line.pop();
        }
        let decoded = String::from_utf8_lossy(&line);
        for event in parser.parse_line(&decoded) {
            match event {
                GameEvent::ZoneChanged { .. } => report.zone_events += 1,
                GameEvent::CharacterSelection => report.selection_events += 1,
                GameEvent::HealthObserved(_) => report.health_events += 1,
            }
            state.apply(event, catalog, observed_at);
        }
        report.complete_lines += 1;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use std::{fs, time::Instant};

    use tempfile::tempdir;

    use super::*;

    fn catalog() -> ZoneCatalog {
        crate::mapping::runtime_catalog().expect("DB catalog")
    }

    #[test]
    fn restores_last_verified_zone_and_uses_restart_time() {
        let root = tempdir().expect("temporary directory");
        let path = root.path().join("WizardClient.log");
        fs::write(
            &path,
            concat!(
                "10/01/26 19:00:00 [STAT] CLIENT          CHARACTER LIST\n",
                "10/01/26 19:01:00 [STAT] CLIENT          ClientZone::Load() zone = WizardCity/WC_Ravenwood, (id=::<redacted>)\n",
                "10/01/26 19:02:00 [STAT] CLIENT          ClientZone::Load() zone = Zafaria/ZF_Z07_Stone_Town, (id=::<redacted>)\n"
            ),
        )
        .expect("write history");
        let startup = Instant::now();
        let mut parser = LogParser::default();
        let mut state = GameState::default();

        let count = replay_existing_log(
            &path,
            fs::metadata(&path).expect("metadata").len(),
            &mut parser,
            &mut state,
            &catalog(),
            startup,
        )
        .expect("replay history");

        assert_eq!(count, 3);
        let location = state.location.expect("restored location");
        assert_eq!(location.raw_zone_id, "Zafaria/ZF_Z07_Stone_Town");
        assert_eq!(
            location
                .mapping
                .expect("verified mapping")
                .world
                .unwrap()
                .as_str(),
            "Zafaria"
        );
        assert_eq!(location.entered_at, startup);
    }

    #[test]
    fn later_unknown_zone_does_not_fall_back_to_an_older_verified_zone() {
        let root = tempdir().expect("temporary directory");
        let path = root.path().join("WizardClient.log");
        fs::write(
            &path,
            concat!(
                "10/01/26 19:00:00 [STAT] CLIENT          zone = Zafaria/ZF_Z07_Stone_Town,\n",
                "10/01/26 19:01:00 [STAT] CLIENT          zone = Unverified/Unknown_Zone,\n"
            ),
        )
        .expect("write history");
        let mut parser = LogParser::default();
        let mut state = GameState::default();

        replay_existing_log(
            &path,
            fs::metadata(&path).expect("metadata").len(),
            &mut parser,
            &mut state,
            &catalog(),
            Instant::now(),
        )
        .expect("replay history");

        let location = state.location.expect("retain raw current zone");
        assert_eq!(location.raw_zone_id, "Unverified/Unknown_Zone");
        assert!(location.mapping.is_none());
    }

    #[test]
    fn selection_boundary_clears_preceding_location() {
        let root = tempdir().expect("temporary directory");
        let path = root.path().join("WizardClient.log");
        fs::write(
            &path,
            concat!(
                "10/01/26 19:00:00 [STAT] CLIENT          zone = Zafaria/ZF_Z07_Stone_Town,\n",
                "10/01/26 19:01:00 [STAT] CLIENT          CHARACTER LIST\n"
            ),
        )
        .expect("write history");
        let mut parser = LogParser::default();
        let mut state = GameState::default();

        replay_existing_log(
            &path,
            fs::metadata(&path).expect("metadata").len(),
            &mut parser,
            &mut state,
            &catalog(),
            Instant::now(),
        )
        .expect("replay history");

        assert!(state.location.is_none());
    }

    #[test]
    fn replays_only_the_captured_prefix_and_ignores_incomplete_record() {
        let root = tempdir().expect("temporary directory");
        let path = root.path().join("WizardClient.log");
        let prefix =
            b"10/01/26 19:02:00 [STAT] CLIENT          zone = Zafaria/ZF_Z07_Stone_Town,\n";
        fs::write(
            &path,
            [prefix.as_slice(), b"unfinished".as_slice()].concat(),
        )
        .expect("write history");
        let mut parser = LogParser::default();
        let mut state = GameState::default();

        let count = replay_existing_log(
            &path,
            prefix.len() as u64,
            &mut parser,
            &mut state,
            &catalog(),
            Instant::now(),
        )
        .expect("replay bounded prefix");

        assert_eq!(count, 1);
        assert_eq!(
            state.location.expect("restored complete zone").raw_zone_id,
            "Zafaria/ZF_Z07_Stone_Town"
        );
    }
}
