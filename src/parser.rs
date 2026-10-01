#[derive(Clone, Debug, Eq, PartialEq)]
/// Raw values observed in a health-globe record; local ownership is not established.
/// The current client can report a current value above the temporary maximum.
pub struct Health {
    pub current: u32,
    pub maximum: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameEvent {
    ZoneChanged { raw_zone_id: String },
    CharacterSelection,
    HealthChanged(Health),
}

#[derive(Default)]
pub struct LogParser {
    pending_health: Option<Health>,
}

impl LogParser {
    /// Discards any health update still waiting for its following log line.
    pub fn reset(&mut self) {
        self.pending_health = None;
    }

    /// Parses one complete line using the legacy-reference record patterns.
    ///
    /// The legacy parser associates a health-globe record with the following
    /// line: a following "called for a player that is not this client's!"
    /// message means the health record was not for this client. Current-client
    /// health attribution is unverified until a captured log establishes it.
    pub fn parse_line(&mut self, line: &str) -> Vec<GameEvent> {
        let mut events = Vec::new();
        if let Some(health) = self.pending_health.take() {
            if !line.contains("called for a player that is not this client's!") {
                events.push(GameEvent::HealthChanged(health));
            }
        }

        if line.contains("CHARACTER LIST") {
            events.push(GameEvent::CharacterSelection);
        } else if let Some(raw_zone_id) = parse_zone_id(line) {
            events.push(GameEvent::ZoneChanged { raw_zone_id });
        }

        if let Some(health) = parse_health(line) {
            self.pending_health = Some(health);
        }
        events
    }
}

fn parse_zone_id(line: &str) -> Option<String> {
    let marker = "zone = ";
    let start = line.find(marker)? + marker.len();
    let value = &line[start..];
    let (raw_zone_id, _) = value.split_once(',')?;
    let raw_zone_id = raw_zone_id.trim();
    (!raw_zone_id.is_empty()).then(|| raw_zone_id.to_owned())
}

fn parse_health(line: &str) -> Option<Health> {
    let marker = "Updating health globe (new health: ";
    let start = line.find(marker)? + marker.len();
    let value = &line[start..];
    let (current, remainder) = value.split_once(", new health max: ")?;
    let (maximum, _) = remainder.split_once(')')?;
    let current = current.parse::<u32>().ok()?;
    let maximum = maximum.parse::<u32>().ok()?;
    if maximum == 0 {
        return None;
    }
    Some(Health { current, maximum })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_zone_change_and_character_selection() {
        let mut parser = LogParser::default();
        assert_eq!(
            parser.parse_line("[log] zone = WizardCity/WC_Ravenwood,"),
            [GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/WC_Ravenwood".to_owned()
            }]
        );
        assert_eq!(
            parser.parse_line("CHARACTER LIST"),
            [GameEvent::CharacterSelection]
        );
    }

    #[test]
    fn zone_id_ends_at_first_comma_even_with_later_fields() {
        let mut parser = LogParser::default();
        assert_eq!(
            parser.parse_line("zone = WizardCity/WC_Ravenwood, extra = value, status = ready"),
            [GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/WC_Ravenwood".to_owned()
            }]
        );
    }

    #[test]
    fn emits_legacy_health_after_a_non_remote_following_line() {
        let mut parser = LogParser::default();
        assert!(
            parser
                .parse_line("Updating health globe (new health: 125, new health max: 300)")
                .is_empty()
        );
        assert_eq!(
            parser.parse_line("continuing client update"),
            [GameEvent::HealthChanged(Health {
                current: 125,
                maximum: 300
            })]
        );
    }

    #[test]
    fn ignores_legacy_remote_player_health_update() {
        let mut parser = LogParser::default();
        parser.parse_line("Updating health globe (new health: 10, new health max: 100)");
        assert!(
            parser
                .parse_line("Someone called for a player that is not this client's!")
                .is_empty()
        );
        assert!(parser.parse_line("continuing client update").is_empty());
    }

    #[test]
    fn reset_discards_unattributed_health_at_end_of_source() {
        let mut parser = LogParser::default();
        assert!(
            parser
                .parse_line("Updating health globe (new health: 125, new health max: 300)")
                .is_empty()
        );
        parser.reset();
        assert!(parser.parse_line("new source line").is_empty());
    }

    #[test]
    fn malformed_or_invalid_health_does_not_emit_an_event() {
        let mut parser = LogParser::default();
        assert!(
            parser
                .parse_line("Updating health globe (new health: 20, new health max: 0)")
                .is_empty()
        );
        assert!(
            parser
                .parse_line("Updating health globe (new health: invalid, new health max: 100)")
                .is_empty()
        );
        assert!(parser.parse_line("zone = ,").is_empty());
        parser.reset();
    }
}
