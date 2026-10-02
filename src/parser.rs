#[derive(Clone, Debug, Eq, PartialEq)]
/// Raw values observed in a health-globe record.
/// The current client can report a current value above the temporary maximum.
pub struct Health {
    pub current: u32,
    pub maximum: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HealthAttribution {
    Local,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthObservation {
    pub health: Health,
    pub attribution: HealthAttribution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameEvent {
    ZoneChanged { raw_zone_id: String },
    CharacterSelection,
    HealthObserved(HealthObservation),
}

#[derive(Default)]
pub struct LogParser {
    pending_health: Option<HealthObservation>,
    local_hit_second: Option<String>,
}

impl LogParser {
    /// Discards any health update still waiting for its following log line.
    pub fn reset(&mut self) {
        self.pending_health = None;
        self.local_hit_second = None;
    }

    /// Parses one complete line using current-client record patterns.
    ///
    /// A preceding explicit local-hit marker can identify a cinematic health
    /// update. A verified client health-message source can identify an
    /// out-of-combat update. A following contradictory player marker makes
    /// either observation unknown.
    /// All other health-globe records remain unattributed.
    pub fn parse_line(&mut self, line: &str) -> Vec<GameEvent> {
        let mut events = Vec::new();
        if let Some(mut observation) = self.pending_health.take() {
            if line.contains("called for a player that is not this client's!") {
                observation.attribution = HealthAttribution::Unknown;
            }
            events.push(GameEvent::HealthObserved(observation));
        }

        if line.contains("CHARACTER LIST") {
            events.push(GameEvent::CharacterSelection);
        } else if let Some(raw_zone_id) = parse_zone_id(line) {
            events.push(GameEvent::ZoneChanged { raw_zone_id });
        }

        let previous_local_hit_second = self.local_hit_second.take();
        if line.contains("[DBGL] Cinematics      ProcessDamageEffect: Our client is getting hurt!")
        {
            self.local_hit_second = log_second(line).map(str::to_owned);
        }

        if let Some(health) = parse_health(line) {
            let local_hit = line
                .contains("[DBGL] Cinematics      ProcessDamageEffect: Updating health globe (")
                && previous_local_hit_second.as_deref() == log_second(line)
                && previous_local_hit_second.is_some();
            let local_client_update = line
                .contains("[DBGL] WizardClientMod MSG_UpdateHealth: Updating health globe (")
                && log_second(line).is_some();
            self.pending_health = Some(HealthObservation {
                health,
                attribution: if local_hit || local_client_update {
                    HealthAttribution::Local
                } else {
                    HealthAttribution::Unknown
                },
            });
        }
        events
    }
}

fn log_second(line: &str) -> Option<&str> {
    let second = line.get(..17)?;
    let bytes = second.as_bytes();
    (bytes[2] == b'/'
        && bytes[5] == b'/'
        && bytes[8] == b' '
        && bytes[11] == b':'
        && bytes[14] == b':'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 2 | 5 | 8 | 11 | 14) || byte.is_ascii_digit()))
    .then_some(second)
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
    fn unmarked_health_remains_unknown_after_a_non_remote_following_line() {
        let mut parser = LogParser::default();
        assert!(
            parser
                .parse_line("Updating health globe (new health: 125, new health max: 300)")
                .is_empty()
        );
        assert_eq!(
            parser.parse_line("continuing client update"),
            [GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 125,
                    maximum: 300
                },
                attribution: HealthAttribution::Unknown,
            })]
        );
    }

    #[test]
    fn contradictory_or_redacted_remote_phrase_keeps_health_unknown() {
        let mut parser = LogParser::default();
        parser.parse_line("Updating health globe (new health: 10, new health max: 100)");
        assert_eq!(
            parser.parse_line("Someone called for a player that is not this client's!"),
            [GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 10,
                    maximum: 100
                },
                attribution: HealthAttribution::Unknown,
            })]
        );
        assert!(parser.parse_line("continuing client update").is_empty());
    }

    #[test]
    fn explicit_cinematic_marker_attributes_only_the_adjacent_same_second_globe() {
        let marker = "10/01/26 19:51:19 [DBGL] Cinematics      ProcessDamageEffect: Our client is getting hurt!";
        let globe = "10/01/26 19:51:19 [DBGL] Cinematics      ProcessDamageEffect: Updating health globe (new health: 3455, new health max: 3868)";
        let mut parser = LogParser::default();
        assert!(parser.parse_line(marker).is_empty());
        assert!(parser.parse_line(globe).is_empty());
        assert_eq!(
            parser.parse_line("next cinematic record"),
            [GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 3455,
                    maximum: 3868
                },
                attribution: HealthAttribution::Local,
            })]
        );

        assert!(parser.parse_line(marker).is_empty());
        assert!(parser.parse_line("intervening record").is_empty());
        assert!(parser.parse_line(globe).is_empty());
        assert!(matches!(
            parser.parse_line("next record").as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));

        assert!(parser.parse_line(marker).is_empty());
        assert!(parser
            .parse_line("10/01/26 19:51:20 [DBGL] Cinematics      ProcessDamageEffect: Updating health globe (new health: 3455, new health max: 3868)")
            .is_empty());
        assert!(matches!(
            parser.parse_line("next record").as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));

        assert!(parser.parse_line(marker).is_empty());
        assert!(parser
            .parse_line("10/01/26 19:51:19 [DBGL] WizClientGameEf HandleStatisticUpdate: Updating health globe (new health: 3455, new health max: 3868)")
            .is_empty());
        assert!(matches!(
            parser.parse_line("next record").as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));
    }

    #[test]
    fn reset_discards_local_marker_and_remote_marker_downgrades_it() {
        let marker = "10/01/26 19:51:19 [DBGL] Cinematics      ProcessDamageEffect: Our client is getting hurt!";
        let globe = "10/01/26 19:51:19 [DBGL] Cinematics      ProcessDamageEffect: Updating health globe (new health: 3455, new health max: 3868)";
        let mut parser = LogParser::default();
        parser.parse_line(marker);
        parser.reset();
        parser.parse_line(globe);
        assert!(matches!(
            parser.parse_line("next record").as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));
        parser.parse_line(marker);
        parser.parse_line(globe);
        assert!(matches!(
            parser
                .parse_line("called for a player that is not this client's!")
                .as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));
    }

    #[test]
    fn verified_client_health_message_source_needs_valid_log_timestamp() {
        let mut parser = LogParser::default();
        assert!(parser
            .parse_line("10/01/26 20:13:27 [DBGL] WizardClientMod MSG_UpdateHealth: Updating health globe (new health: 2959, new health max: 3868)")
            .is_empty());
        assert_eq!(
            parser.parse_line("10/01/26 20:13:27 [DBGM] CORE_SEER       ------ CloseInteraction"),
            [GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 2959,
                    maximum: 3868,
                },
                attribution: HealthAttribution::Local,
            })]
        );
        assert!(parser
            .parse_line("WizardClientMod MSG_UpdateHealth: Updating health globe (new health: 3868, new health max: 3868)")
            .is_empty());
        assert!(matches!(
            parser.parse_line("next line").as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));

        assert!(parser
            .parse_line("10/01/26 20:13:38 [DBGL] WizardClientMod MSG_UpdateHealth: Updating health globe (new health: 3868, new health max: 3868)")
            .is_empty());
        assert!(matches!(
            parser
                .parse_line("10/01/26 20:13:38 [DBGM] CORE_SEER       HUDWindow::HandleUpdateHealth called for a player that is not this client's!  (<same-id> sent in, <same-id> is client's player)")
                .as_slice(),
            [GameEvent::HealthObserved(HealthObservation {
                attribution: HealthAttribution::Unknown,
                ..
            })]
        ));
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
