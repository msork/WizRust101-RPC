use std::{collections::BTreeMap, io::Read};

use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneMapping {
    /// Exact `zone` value emitted by the pinned WizRust101-DB.
    pub location: String,
    /// The DB's exact world label; `Unknown` is represented as no State.
    pub world: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct DbZone {
    world: String,
    zone: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MappingError {
    #[error("could not read mapping source: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid WizRust101-DB zones.json: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ZoneCatalog {
    pub zones: BTreeMap<String, ZoneMapping>,
}

impl ZoneCatalog {
    pub fn from_reader(reader: impl Read) -> Result<Self, MappingError> {
        let zones: BTreeMap<String, DbZone> = serde_json::from_reader(reader)?;
        let zones = zones
            .into_iter()
            .map(|(path, db_zone)| {
                let world = (db_zone.world != "Unknown").then_some(db_zone.world);
                (
                    path,
                    ZoneMapping {
                        location: db_zone.zone,
                        world,
                    },
                )
            })
            .collect();
        Ok(Self { zones })
    }

    pub fn resolve(&self, canonical_zone_path: &str) -> Option<&ZoneMapping> {
        self.zones.get(canonical_zone_path)
    }
}

/// Builds from the exact upstream DB output checked in through the Git
/// submodule. Runtime never reaches the network and does not duplicate DB data.
pub fn runtime_catalog() -> Result<ZoneCatalog, MappingError> {
    ZoneCatalog::from_reader(include_bytes!("../vendor/WizRust101-DB/out/zones.json").as_slice())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use crate::{
        parser::GameEvent,
        presence::{Presence, PresenceConfig, WorldAssetCatalog},
        state::GameState,
    };

    #[test]
    fn consumes_db_zone_and_world_exactly_and_omits_unknown_state() {
        let catalog = ZoneCatalog::from_reader(
            r#"{
                "A/known": {"world":"Wizard City","zone":"Exact DB Name"},
                "A/unknown": {"world":"Unknown","zone":"Internal_Zone_Fallback"}
            }"#
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            catalog.resolve("A/known").unwrap().location,
            "Exact DB Name"
        );
        assert_eq!(
            catalog.resolve("A/known").unwrap().world.as_deref(),
            Some("Wizard City")
        );
        assert_eq!(
            catalog.resolve("A/unknown").unwrap().location,
            "Internal_Zone_Fallback"
        );
        assert_eq!(catalog.resolve("A/unknown").unwrap().world, None);
    }

    #[test]
    fn pinned_database_audit_enforces_exact_details_state_and_art_for_every_zone() {
        let catalog = runtime_catalog().expect("pinned DB zones.json loads");
        let assets =
            WorldAssetCatalog::from_reader(include_bytes!("../data/world-assets.json").as_slice())
                .expect("world art catalog loads");
        let presence_config = PresenceConfig {
            world_asset_keys: assets.worlds.clone(),
            fallback_asset_key: assets.fallback.clone(),
        };
        let db: serde_json::Value =
            serde_json::from_slice(include_bytes!("../vendor/WizRust101-DB/out/zones.json"))
                .unwrap();
        let diagnostics: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../vendor/WizRust101-DB/out/zones.diagnostics.json"
        ))
        .unwrap();
        let diagnostics = diagnostics.as_array().unwrap();
        assert_eq!(diagnostics.len(), catalog.zones.len());
        let diagnostic_paths: std::collections::BTreeSet<_> = diagnostics
            .iter()
            .map(|record| record["path"].as_str().expect("diagnostic path"))
            .collect();
        assert_eq!(diagnostic_paths.len(), catalog.zones.len());
        assert!(
            catalog
                .zones
                .keys()
                .all(|path| diagnostic_paths.contains(path.as_str()))
        );

        let mut known_worlds = 0;
        let mut unknown_worlds = 0;
        let mut house_worlds = 0;
        let mut unknown_zones = 0;
        let mut world_art_matches = 0;
        let mut generic_art_fallbacks = 0;
        let mut missing_details = 0;

        for (path, db_entry) in db.as_object().unwrap() {
            let db_world = db_entry["world"].as_str().unwrap();
            let db_zone = db_entry["zone"].as_str().unwrap();
            let mapping = catalog.resolve(path).expect("all DB paths are loaded");
            assert_eq!(
                mapping.location, db_zone,
                "Details source differs at {path}"
            );
            assert_eq!(
                mapping.world.as_deref(),
                (db_world != "Unknown").then_some(db_world),
                "State source differs at {path}"
            );
            if db_world == "Unknown" {
                unknown_worlds += 1;
            } else {
                known_worlds += 1;
            }
            if db_world == "House" {
                house_worlds += 1;
            }
            if db_zone == "Unknown" || db_zone.is_empty() {
                missing_details += 1;
                if db_zone == "Unknown" {
                    unknown_zones += 1;
                }
            }

            let expected_art = if db_world == "House" {
                assets.worlds.get("House")
            } else if db_world != "Unknown" {
                assets.worlds.get(db_world)
            } else {
                None
            };
            if expected_art.is_some() {
                world_art_matches += 1;
            } else {
                generic_art_fallbacks += 1;
            }

            let now = Instant::now();
            let mut state = GameState::default();
            state.apply(
                GameEvent::ZoneChanged {
                    raw_zone_id: path.clone(),
                },
                &catalog,
                now,
            );
            let presence = Presence::from_game_state(
                &state,
                &presence_config,
                now,
                std::time::SystemTime::now(),
            )
            .expect("every DB zone remains displayable");
            assert_eq!(
                presence.details.as_deref(),
                (db_zone != "Unknown").then_some(db_zone),
                "Details at {path}"
            );
            assert_eq!(
                presence.state.as_deref(),
                (db_world != "Unknown" && db_world != "House").then_some(db_world),
                "State at {path}"
            );
            assert_eq!(
                presence.large_image.as_deref(),
                Some(expected_art.unwrap_or(&assets.fallback).as_str()),
                "large art at {path}"
            );
            assert_eq!(presence.small_image.as_deref(), Some("wizrust101_rpc"));
        }

        assert_eq!(catalog.zones.len(), 3346);
        assert_eq!(known_worlds, 3346);
        assert_eq!(unknown_worlds, 0);
        assert_eq!(house_worlds, 495);
        assert_eq!(unknown_zones, 0);
        assert_eq!(world_art_matches, 3005);
        assert_eq!(generic_art_fallbacks, 341);
        assert_eq!(world_art_matches + generic_art_fallbacks, 3346);
        assert_eq!(missing_details, 0);
        assert!(generic_art_fallbacks > 0);

        // Representative DB values include major worlds, housing, and raw
        // fallback values. Values are deliberately passed through unchanged.
        let representatives = [
            (
                "WizardCity/Interiors/WC_Headmistress_House",
                "The Commons",
                "Wizard City",
            ),
            ("Krokotopia/KT_Hub", "The Oasis", "Krokotopia"),
            (
                "Marleybone/Interiors/MB_AdasLab",
                "Regent's Square",
                "Marleybone",
            ),
            (
                "MooShu/Interiors/MS_Emperor_Palace",
                "Jade Palace",
                "MooShu",
            ),
            (
                "DragonSpire/DS_A1_Knowledge/DS_A1Hub_Library",
                "The Atheneum",
                "Dragonspyre",
            ),
            ("Celestia/CL_Hub", "Celestia Base Camp", "Celestia"),
            (
                "Zafaria/Interiors/ZF_Z07_HallOfFury",
                "Stone Town",
                "Zafaria",
            ),
            ("Avalon/AV_Z00_Hub", "Caliburn", "Avalon"),
            ("Azteca/AZ_Z00_Zocalo", "The Zocalo", "Azteca"),
            (
                "Khrysalis/Interiors/Dungeon_Rematch_KR_Morganthe",
                "Morganthe Rematch",
                "Khrysalis",
            ),
            ("Aquila/AQ_Z01_MountOlympus", "Mount Olympus", "Aquila"),
            (
                "Wysteria/Interiors/PA_Classroom_Chaos",
                "Pigswick Academy",
                "Wysteria",
            ),
            (
                "Grizzleheim/GH_AbandCity/GH_EntranceHall",
                "Nidavellir",
                "Grizzleheim",
            ),
        ];
        for (path, expected_zone, expected_world) in representatives {
            let entry = &db[path];
            assert_eq!(entry["zone"].as_str(), Some(expected_zone), "{path}");
            assert_eq!(entry["world"].as_str(), Some(expected_world), "{path}");
        }
        let housing = db.get("Housing/CardPromo/GS_Fantasy_Castle").unwrap();
        assert_eq!(housing["world"], "House");
        assert_eq!(housing["zone"], "Massive Fantasy Palace");
        let raw_fallback_path = "Test/Court_Test";
        assert_eq!(db[raw_fallback_path]["world"], "Test");
        assert_eq!(db[raw_fallback_path]["zone"], "Court_Test");
        for (path, zone, world) in [
            (
                "DragonSpire/DS_A1_Knowledge/Interiors/DS_Chasm_Gauntlet_3Room",
                "Kyanite Tower",
                "Dragonspyre",
            ),
            (
                "Marleybone/G14_Gauntlet/MB_G14_ThroneRoomB_Challenge",
                "Barkingham Palace",
                "Marleybone",
            ),
        ] {
            assert_eq!(db[path]["zone"], zone, "{path}");
            assert_eq!(db[path]["world"], world, "{path}");
        }

        let now = Instant::now();
        let mut state = GameState::default();
        state.apply(
            GameEvent::ZoneChanged {
                raw_zone_id: raw_fallback_path.into(),
            },
            &catalog,
            now,
        );
        let presence =
            Presence::from_game_state(&state, &presence_config, now, std::time::SystemTime::now())
                .unwrap();
        assert_eq!(presence.details.as_deref(), Some("Court_Test"));
        assert_eq!(presence.state.as_deref(), Some("Test"));
        assert_eq!(presence.large_image.as_deref(), Some("wizard101"));
    }
}
