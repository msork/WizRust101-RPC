use std::{
    collections::BTreeMap,
    io::Read,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;

use crate::state::{GameActivity, GameState};

const FIELD_MAX_BYTES: usize = 128;
pub const WORLD_ASSET_SCHEMA_VERSION: u32 = 1;
pub const APPLICATION_LOGO_ASSET_KEY: &str = "wizrust101_rpc";
pub const APPLICATION_LOGO_HOVER_TEXT: &str = "WizRust101-RPC";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct WorldAssetCatalog {
    pub schema_version: u32,
    #[serde(default)]
    pub worlds: BTreeMap<String, String>,
}

impl Default for WorldAssetCatalog {
    fn default() -> Self {
        Self {
            schema_version: WORLD_ASSET_SCHEMA_VERSION,
            worlds: BTreeMap::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorldAssetError {
    #[error("invalid world asset catalog: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported world asset catalog schema version {0}")]
    UnsupportedSchema(u32),
}

impl WorldAssetCatalog {
    pub fn from_reader(reader: impl Read) -> Result<Self, WorldAssetError> {
        let catalog: Self = serde_json::from_reader(reader)?;
        if catalog.schema_version != WORLD_ASSET_SCHEMA_VERSION {
            return Err(WorldAssetError::UnsupportedSchema(catalog.schema_version));
        }
        Ok(catalog)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PresenceConfig {
    /// Approved, uploaded Discord asset keys indexed by verified world ID.
    pub world_asset_keys: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Presence {
    pub details: Option<String>,
    pub state: Option<String>,
    pub start_unix_seconds: Option<i64>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

impl Presence {
    pub fn from_game_state(
        game: &GameState,
        config: &PresenceConfig,
        now: Instant,
        wall_now: SystemTime,
    ) -> Option<Self> {
        if game.activity != GameActivity::Running {
            return None;
        }

        let mut presence = Self {
            details: None,
            state: None,
            start_unix_seconds: None,
            large_image: None,
            large_text: None,
            small_image: Some(APPLICATION_LOGO_ASSET_KEY.into()),
            small_text: Some(APPLICATION_LOGO_HOVER_TEXT.into()),
        };

        if let Some(location) = game.location.as_ref()
            && let Some(mapping) = location.mapping.as_ref()
        {
            if mapping.location_name_verified {
                presence.details = field(&mapping.location);
                presence.start_unix_seconds = now
                    .checked_duration_since(location.entered_at)
                    .and_then(|elapsed| {
                        wall_now
                            .duration_since(UNIX_EPOCH)
                            .ok()?
                            .checked_sub(elapsed)
                    })
                    .and_then(|entry| i64::try_from(entry.as_secs()).ok());
            }

            // World identity is a separate evidence source. It may be shown
            // even when the DB's location-name confidence is only fallback.
            if let Some(world) = &mapping.world {
                presence.state = field(&world.name);
                if let Some(key) = config
                    .world_asset_keys
                    .get(&world.id)
                    .and_then(|key| asset_key(key))
                    && let Some(text) = field(&world.name)
                {
                    presence.large_image = Some(key);
                    presence.large_text = Some(text);
                }
            }
        }

        (presence.details.is_some() || presence.state.is_some()).then_some(presence)
    }
}

fn field(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let end = value
        .char_indices()
        .map(|(index, character)| index + character.len_utf8())
        .take_while(|end| *end <= FIELD_MAX_BYTES)
        .last()
        .unwrap_or(0);
    (end > 0).then(|| value[..end].to_owned())
}

fn asset_key(value: &str) -> Option<String> {
    (!value.trim().is_empty() && value.trim() == value && value.len() <= FIELD_MAX_BYTES)
        .then(|| value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mapping::{MappingProvenance, ReviewStatus, WorldMapping, ZoneCatalog, ZoneMapping},
        parser::{GameEvent, Health, HealthAttribution, HealthObservation},
    };
    use std::time::Duration;

    fn catalog(review_status: ReviewStatus) -> ZoneCatalog {
        let mut catalog = ZoneCatalog::default();
        catalog.zones.insert(
            "world/zone".into(),
            ZoneMapping {
                location: "Ravenwood".into(),
                location_name_verified: review_status == ReviewStatus::Verified,
                world: Some(WorldMapping {
                    id: "WizardCity".into(),
                    name: "Wizard City".into(),
                }),
                provenance: MappingProvenance {
                    review_status,
                    diagnostic: None,
                },
            },
        );
        catalog
    }

    fn health(current: u32, attribution: HealthAttribution) -> GameEvent {
        GameEvent::HealthObserved(HealthObservation {
            health: Health {
                current,
                maximum: 100,
            },
            attribution,
        })
    }

    #[test]
    fn verified_location_has_stable_timer_and_asset_only_when_registered() {
        let origin = Instant::now();
        let wall = UNIX_EPOCH + Duration::from_secs(1_800_000_100);
        let mut game = GameState::default();
        let catalog = catalog(ReviewStatus::Verified);
        let zone = || GameEvent::ZoneChanged {
            raw_zone_id: "world/zone".into(),
        };
        game.apply(zone(), &catalog, origin);
        let mut config = PresenceConfig::default();
        let without_asset =
            Presence::from_game_state(&game, &config, origin + Duration::from_secs(10), wall)
                .unwrap();
        assert_eq!(without_asset.details.as_deref(), Some("Ravenwood"));
        assert_eq!(without_asset.state.as_deref(), Some("Wizard City"));
        assert_eq!(without_asset.start_unix_seconds, Some(1_800_000_090));
        assert_eq!(without_asset.large_image, None);
        assert_eq!(
            without_asset.small_image.as_deref(),
            Some(APPLICATION_LOGO_ASSET_KEY)
        );
        assert_eq!(
            without_asset.small_text.as_deref(),
            Some(APPLICATION_LOGO_HOVER_TEXT)
        );
        game.apply(zone(), &catalog, origin + Duration::from_secs(5));
        config
            .world_asset_keys
            .insert("WizardCity".into(), "wizard_city_png".into());
        let with_asset =
            Presence::from_game_state(&game, &config, origin + Duration::from_secs(10), wall)
                .unwrap();
        assert_eq!(
            with_asset.start_unix_seconds,
            without_asset.start_unix_seconds
        );
        assert_eq!(with_asset.large_image.as_deref(), Some("wizard_city_png"));
        assert_eq!(with_asset.large_text.as_deref(), Some("Wizard City"));
        let mut other = catalog.resolve("world/zone").expect("mapping").clone();
        other.location = "The Commons".into();
        let mut catalog = catalog;
        catalog.zones.insert("world/other".into(), other);
        game.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "world/other".into(),
            },
            &catalog,
            origin + Duration::from_secs(11),
        );
        let changed =
            Presence::from_game_state(&game, &config, origin + Duration::from_secs(12), wall)
                .unwrap();
        assert_eq!(changed.details.as_deref(), Some("The Commons"));
        assert_eq!(changed.start_unix_seconds, Some(1_800_000_099));
        game.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "other/zone".into(),
            },
            &catalog,
            origin + Duration::from_secs(11),
        );
        let unknown =
            Presence::from_game_state(&game, &config, origin + Duration::from_secs(12), wall);
        assert_eq!(unknown, None);
    }

    #[test]
    fn fallback_location_name_is_hidden_but_independent_world_stays_visible() {
        let origin = Instant::now();
        let wall = UNIX_EPOCH + Duration::from_secs(1_800_000_100);
        let mut game = GameState::default();
        game.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "world/zone".into(),
            },
            &catalog(ReviewStatus::UnverifiedFallback),
            origin,
        );
        let presence = Presence::from_game_state(&game, &PresenceConfig::default(), origin, wall)
            .expect("separate verified world can remain visible");
        assert_eq!(presence.details, None);
        assert_eq!(presence.state.as_deref(), Some("Wizard City"));
    }

    #[test]
    fn health_is_never_displayed_and_selection_clears_game_presence() {
        let origin = Instant::now();
        let wall = UNIX_EPOCH + Duration::from_secs(1_800_000_100);
        let mut game = GameState::default();
        let catalog = ZoneCatalog::default();
        game.apply(health(90, HealthAttribution::Unknown), &catalog, origin);
        assert_eq!(
            Presence::from_game_state(&game, &PresenceConfig::default(), origin, wall),
            None
        );
        game.apply(health(80, HealthAttribution::Local), &catalog, origin);
        game.apply(
            health(1, HealthAttribution::Unknown),
            &catalog,
            origin + Duration::from_secs(1),
        );
        let recent = Presence::from_game_state(
            &game,
            &PresenceConfig::default(),
            origin + Duration::from_secs(60),
            wall,
        );
        assert_eq!(recent, None);
        assert_eq!(
            Presence::from_game_state(
                &game,
                &PresenceConfig::default(),
                origin + Duration::from_secs(61),
                wall
            ),
            None
        );
        game.apply(
            GameEvent::CharacterSelection,
            &catalog,
            origin + Duration::from_secs(62),
        );
        assert_eq!(
            Presence::from_game_state(
                &game,
                &PresenceConfig::default(),
                origin + Duration::from_secs(62),
                wall
            ),
            None
        );
    }

    #[test]
    fn clock_and_field_edges_omit_invalid_values() {
        let origin = Instant::now();
        let mut game = GameState::default();
        game.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: "world/zone".into(),
            },
            &catalog(ReviewStatus::Verified),
            origin,
        );
        let early =
            Presence::from_game_state(&game, &PresenceConfig::default(), origin, UNIX_EPOCH)
                .unwrap();
        assert_eq!(early.start_unix_seconds, Some(0));
        let future = Presence::from_game_state(
            &game,
            &PresenceConfig::default(),
            origin - Duration::from_secs(1),
            UNIX_EPOCH,
        )
        .unwrap();
        assert_eq!(future.start_unix_seconds, None);
        assert_eq!(field(""), None);
        assert!(field(&"é".repeat(100)).unwrap().len() <= FIELD_MAX_BYTES);
        assert_eq!(asset_key(&"x".repeat(129)), None);
    }

    #[test]
    fn world_asset_catalog_loads_registered_key_and_rejects_future_schema() {
        let catalog =
            WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
                .unwrap();
        assert_eq!(
            catalog.worlds.get("Zafaria").map(String::as_str),
            Some("zafaria")
        );
        assert!(matches!(
            WorldAssetCatalog::from_reader(r#"{"schema_version":2,"worlds":{}}"#.as_bytes()),
            Err(WorldAssetError::UnsupportedSchema(2))
        ));
    }
}
