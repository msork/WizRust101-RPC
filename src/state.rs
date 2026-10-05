use std::time::Instant;

use crate::{
    mapping::{ZoneCatalog, ZoneMapping},
    parser::{GameEvent, Health, HealthAttribution},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameActivity {
    Unknown,
    Running,
    CharacterSelection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentLocation {
    pub raw_zone_id: String,
    pub mapping: Option<ZoneMapping>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameState {
    pub activity: GameActivity,
    pub location: Option<CurrentLocation>,
    /// Start of the current uninterrupted Wizard101 session, shared by every
    /// zone visited during that session.
    pub session_started_at: Option<Instant>,
    /// Last explicitly attributed local health; it may become stale.
    pub health: Option<Health>,
    /// Local observation time, not the time the game changed health.
    pub health_observed_at: Option<Instant>,
    pub last_observed_at: Option<Instant>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            activity: GameActivity::Unknown,
            location: None,
            session_started_at: None,
            health: None,
            health_observed_at: None,
            last_observed_at: None,
        }
    }
}

impl GameState {
    pub fn apply(&mut self, event: GameEvent, catalog: &ZoneCatalog, now: Instant) {
        match event {
            GameEvent::ZoneChanged { raw_zone_id } => {
                self.last_observed_at = Some(now);
                let mapping = catalog.resolve(&raw_zone_id).cloned();
                self.session_started_at.get_or_insert(now);
                self.location = Some(CurrentLocation {
                    mapping,
                    raw_zone_id,
                });
                self.activity = GameActivity::Running;
            }
            GameEvent::CharacterSelection => {
                self.last_observed_at = Some(now);
                self.activity = GameActivity::CharacterSelection;
                self.location = None;
                self.session_started_at = None;
                self.health = None;
                self.health_observed_at = None;
            }
            GameEvent::HealthObserved(observation) => {
                if observation.attribution == HealthAttribution::Local {
                    self.last_observed_at = Some(now);
                    self.health = Some(observation.health);
                    self.health_observed_at = Some(now);
                    self.activity = GameActivity::Running;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::{
        mapping::{ZoneCatalog, ZoneMapping},
        parser::{Health, HealthObservation},
    };

    use super::*;

    fn empty_catalog() -> ZoneCatalog {
        ZoneCatalog::default()
    }

    #[test]
    fn tracks_unknown_zone_without_inventing_a_world() {
        let now = Instant::now();
        let mut state = GameState::default();
        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "UnknownWorld/UnknownZone".to_owned(),
            },
            &empty_catalog(),
            now,
        );

        let location = state.location.expect("raw location remains observable");
        assert_eq!(location.raw_zone_id, "UnknownWorld/UnknownZone");
        assert_eq!(location.mapping, None);
        assert_eq!(state.session_started_at, Some(now));
    }

    #[test]
    fn session_timer_is_continuous_across_zone_changes() {
        let catalog = empty_catalog();
        let entered = Instant::now();
        let mut state = GameState::default();
        let zone = || GameEvent::ZoneChanged {
            raw_zone_id: "WizardCity/WC_Ravenwood".to_owned(),
        };
        state.apply(zone(), &catalog, entered);
        state.apply(zone(), &catalog, entered + Duration::from_secs(30));
        assert_eq!(state.session_started_at, Some(entered));

        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/WC_Commons".to_owned(),
            },
            &catalog,
            entered + Duration::from_secs(45),
        );
        assert_eq!(state.session_started_at, Some(entered));
    }

    #[test]
    fn zone_alias_does_not_reset_session_timer() {
        let mut catalog = ZoneCatalog::default();
        let mapping = ZoneMapping {
            location: "Ravenwood".into(),
            world: Some("Wizard City".into()),
        };
        catalog.zones.insert("first".into(), mapping.clone());
        catalog.zones.insert("alias".into(), mapping);
        let entered = Instant::now();
        let mut state = GameState::default();
        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "first".into(),
            },
            &catalog,
            entered,
        );
        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "alias".into(),
            },
            &catalog,
            entered + Duration::from_secs(5),
        );
        let location = state.location.as_ref().expect("location");
        assert_eq!(location.raw_zone_id, "alias");
        assert_eq!(state.session_started_at, Some(entered));
    }

    #[test]
    fn health_update_preserves_location_and_selection_clears_game_data() {
        let catalog = empty_catalog();
        let entered = Instant::now();
        let mut state = GameState::default();
        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "WizardCity/WC_Ravenwood".to_owned(),
            },
            &catalog,
            entered,
        );
        state.apply(
            GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 75,
                    maximum: 100,
                },
                attribution: HealthAttribution::Local,
            }),
            &catalog,
            entered + Duration::from_secs(1),
        );
        assert_eq!(state.session_started_at, Some(entered));
        assert_eq!(
            state.health,
            Some(Health {
                current: 75,
                maximum: 100
            })
        );
        assert_eq!(
            state.health_observed_at,
            Some(entered + Duration::from_secs(1))
        );

        state.apply(
            GameEvent::CharacterSelection,
            &catalog,
            entered + Duration::from_secs(2),
        );
        assert_eq!(state.activity, GameActivity::CharacterSelection);
        assert_eq!(state.location, None);
        assert_eq!(state.session_started_at, None);
        assert_eq!(state.health, None);
        assert_eq!(state.health_observed_at, None);
    }

    #[test]
    fn unknown_health_leaves_local_state_unchanged() {
        let catalog = empty_catalog();
        let now = Instant::now();
        let mut state = GameState::default();
        state.apply(
            GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 1,
                    maximum: 9000,
                },
                attribution: HealthAttribution::Unknown,
            }),
            &catalog,
            now,
        );
        assert_eq!(state, GameState::default());
        state.apply(
            GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 75,
                    maximum: 100,
                },
                attribution: HealthAttribution::Local,
            }),
            &catalog,
            now,
        );
        let verified_state = state.clone();
        state.apply(
            GameEvent::HealthObserved(HealthObservation {
                health: Health {
                    current: 1,
                    maximum: 9000,
                },
                attribution: HealthAttribution::Unknown,
            }),
            &catalog,
            now + Duration::from_secs(1),
        );
        assert_eq!(state, verified_state);
    }
}
