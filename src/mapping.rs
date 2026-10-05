use std::{
    collections::{BTreeMap, HashMap},
    io::Read,
};

use serde::Deserialize;

pub const WORLD_ASSOCIATION_SCHEMA_VERSION: u32 = 1;

/// DB confidence is kept separate from the flat display string. Independent
/// evidence can verify a readable DB value; a value equal to the raw internal
/// filename is never promoted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewStatus {
    Verified,
    UnverifiedFallback,
    Unknown,
    MissingDiagnostic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneMapping {
    pub location: String,
    /// Verified by DB diagnostics or independent exact-value evidence. Raw
    /// internal-filename fallbacks are never promoted.
    pub location_name_verified: bool,
    pub world: Option<WorldMapping>,
    pub provenance: MappingProvenance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct WorldMapping {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingProvenance {
    pub review_status: ReviewStatus,
    /// The upstream diagnostic record is retained verbatim, including its
    /// selected candidate, source, and provenance fields.
    pub diagnostic: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct WorldAssociationCatalog {
    schema_version: u32,
    #[serde(default)]
    zones: BTreeMap<String, WorldAssociation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct WorldAssociation {
    world: WorldMapping,
    #[serde(default)]
    location_name_evidence: Option<LocationNameEvidence>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct LocationNameEvidence {
    source: String,
    verified_at: String,
    confirms_db_value: bool,
    expected_db_value: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MappingError {
    #[error("could not read mapping source: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid mapping source JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported world-association schema version {0}")]
    UnsupportedSchema(u32),
    #[error("DB zone names must be a flat JSON object of strings")]
    InvalidNames,
    #[error("DB diagnostics must be a JSON array")]
    InvalidDiagnostics,
    #[error("DB diagnostics contain a record without a unique canonical path")]
    InvalidDiagnosticPath,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ZoneCatalog {
    pub zones: BTreeMap<String, ZoneMapping>,
}

impl ZoneCatalog {
    pub fn from_sources(
        names: impl Read,
        diagnostics: impl Read,
        world_associations: impl Read,
    ) -> Result<Self, MappingError> {
        let names: serde_json::Value = serde_json::from_reader(names)?;
        let names = names.as_object().ok_or(MappingError::InvalidNames)?;
        if names.values().any(|value| !value.is_string()) {
            return Err(MappingError::InvalidNames);
        }

        let diagnostics: serde_json::Value = serde_json::from_reader(diagnostics)?;
        let diagnostics = diagnostics
            .as_array()
            .ok_or(MappingError::InvalidDiagnostics)?;
        let mut diagnostics_by_path = HashMap::with_capacity(diagnostics.len());
        for diagnostic in diagnostics {
            let path = diagnostic
                .get("path")
                .and_then(serde_json::Value::as_str)
                .ok_or(MappingError::InvalidDiagnosticPath)?;
            if diagnostics_by_path
                .insert(path.to_owned(), diagnostic.clone())
                .is_some()
            {
                return Err(MappingError::InvalidDiagnosticPath);
            }
        }

        let world_associations: WorldAssociationCatalog =
            serde_json::from_reader(world_associations)?;
        if world_associations.schema_version != WORLD_ASSOCIATION_SCHEMA_VERSION {
            return Err(MappingError::UnsupportedSchema(
                world_associations.schema_version,
            ));
        }

        let zones = names
            .iter()
            .filter_map(|(path, value)| {
                let location = value.as_str()?;
                let diagnostic = diagnostics_by_path.get(path).cloned();
                let review_status = diagnostic
                    .as_ref()
                    .and_then(|value| value.get("confidence"))
                    .and_then(serde_json::Value::as_str)
                    .map(|confidence| match confidence {
                        "verified" => ReviewStatus::Verified,
                        "unverified_fallback" => ReviewStatus::UnverifiedFallback,
                        _ => ReviewStatus::Unknown,
                    })
                    .unwrap_or(ReviewStatus::MissingDiagnostic);
                let association = world_associations.zones.get(path);
                let raw_filename = path.rsplit('/').next().unwrap_or(path);
                let externally_verified = association
                    .and_then(|association| association.location_name_evidence.as_ref())
                    .is_some_and(|evidence| {
                        evidence.confirms_db_value
                            && !evidence.source.trim().is_empty()
                            && !evidence.verified_at.trim().is_empty()
                            && evidence.expected_db_value == location
                            && location != raw_filename
                    });
                let location_name_verified =
                    review_status == ReviewStatus::Verified || externally_verified;
                let world = association.map(|association| association.world.clone());

                Some((
                    path.clone(),
                    ZoneMapping {
                        location: location.to_owned(),
                        location_name_verified,
                        world,
                        provenance: MappingProvenance {
                            review_status,
                            diagnostic,
                        },
                    },
                ))
            })
            .collect();

        Ok(Self { zones })
    }

    pub fn resolve(&self, canonical_zone_path: &str) -> Option<&ZoneMapping> {
        self.zones.get(canonical_zone_path)
    }
}

/// Builds from the exact upstream DB files checked in through the Git
/// submodule. Runtime never reaches the network.
pub fn runtime_catalog() -> Result<ZoneCatalog, MappingError> {
    ZoneCatalog::from_sources(
        include_bytes!("../vendor/WizRust101-DB/out/zones.json").as_slice(),
        include_bytes!("../vendor/WizRust101-DB/out/zones.diagnostics.json").as_slice(),
        include_bytes!("../data/zone-worlds.json").as_slice(),
    )
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const STONE_NAMES: &str = r#"{
        "Zafaria/ZF_Z07_Stone_Town": "Stone Town",
        "World/RawZoneFile": "RawZoneFile",
        "World/ChangedValue": "New Candidate",
        "World/Verified": "Verified Place"
    }"#;
    const STONE_DIAGNOSTICS: &str = r#"[
        {"path":"Zafaria/ZF_Z07_Stone_Town","confidence":"unverified_fallback","selected":{"name":"Stone Town","source":"WizardZone","provenance":"fallback"}},
        {"path":"World/RawZoneFile","confidence":"unverified_fallback","selected":{"name":"RawZoneFile","source":"filename","provenance":"raw internal filename"}},
        {"path":"World/ChangedValue","confidence":"unverified_fallback","selected":{"name":"New Candidate","source":"WizardZone","provenance":"fallback"}},
        {"path":"World/Verified","confidence":"verified","selected":{"name":"Verified Place","source":"SharedMap","provenance":"verified resource link"}}
    ]"#;
    const WORLDS: &str = r#"{
        "schema_version": 1,
        "zones": {
            "Zafaria/ZF_Z07_Stone_Town": {"world":{"id":"Zafaria","name":"Zafaria"},"location_name_evidence":{"source":"project-owner-manual","verified_at":"2026-10-01","confirms_db_value":true,"expected_db_value":"Stone Town"}},
            "World/RawZoneFile": {"world":{"id":"Separate","name":"Separate World"},"location_name_evidence":{"source":"project-owner-manual","verified_at":"2026-10-01","confirms_db_value":true,"expected_db_value":"RawZoneFile"}},
            "World/ChangedValue": {"world":{"id":"Separate","name":"Separate World"},"location_name_evidence":{"source":"project-owner-manual","verified_at":"2026-10-01","confirms_db_value":true,"expected_db_value":"Old Candidate"}}
        }
    }"#;

    #[test]
    fn database_names_are_gated_by_diagnostics_and_keep_fallback_provenance() {
        let catalog = ZoneCatalog::from_sources(
            Cursor::new(STONE_NAMES),
            Cursor::new(STONE_DIAGNOSTICS),
            Cursor::new(WORLDS),
        )
        .expect("DB files load");

        let stone = catalog.resolve("Zafaria/ZF_Z07_Stone_Town").unwrap();
        assert_eq!(stone.location, "Stone Town");
        assert_eq!(
            stone.provenance.review_status,
            ReviewStatus::UnverifiedFallback
        );
        assert!(stone.location_name_verified);
        assert_eq!(stone.world.as_ref().unwrap().name, "Zafaria");
        assert_eq!(
            stone.provenance.diagnostic.as_ref().unwrap()["selected"]["source"],
            "WizardZone"
        );

        let raw_fallback = catalog.resolve("World/RawZoneFile").unwrap();
        assert_eq!(raw_fallback.location, "RawZoneFile");
        assert_eq!(
            raw_fallback.provenance.review_status,
            ReviewStatus::UnverifiedFallback
        );
        assert!(!raw_fallback.location_name_verified);
        assert_eq!(
            raw_fallback.provenance.diagnostic.as_ref().unwrap()["selected"]["provenance"],
            "raw internal filename"
        );

        assert!(
            !catalog
                .resolve("World/ChangedValue")
                .unwrap()
                .location_name_verified
        );

        let verified = catalog.resolve("World/Verified").unwrap();
        assert_eq!(verified.provenance.review_status, ReviewStatus::Verified);
        assert!(verified.location_name_verified);
        assert!(verified.world.is_none());
        assert!(catalog.resolve("Unknown/Absent").is_none());
    }

    #[test]
    fn bundled_database_has_expected_size_and_preserves_current_stone_town_confidence() {
        let catalog = runtime_catalog().expect("bundled source loads");
        assert_eq!(catalog.zones.len(), 3346);
        assert!(
            catalog
                .zones
                .values()
                .all(|zone| zone.provenance.diagnostic.is_some())
        );
        let stone = catalog
            .resolve("Zafaria/ZF_Z07_Stone_Town")
            .expect("DB contains Stone Town entry");
        assert_eq!(stone.location, "Stone Town");
        assert_eq!(
            stone.provenance.review_status,
            ReviewStatus::UnverifiedFallback
        );
        assert!(stone.location_name_verified);
        assert_eq!(
            stone.world.as_ref().map(|world| world.name.as_str()),
            Some("Zafaria")
        );
        assert!(
            catalog
                .resolve("Zafaria/ZF_Z07_Stone_Town")
                .unwrap()
                .provenance
                .diagnostic
                .is_some()
        );
        let raw_filename_fallback = catalog
            .zones
            .iter()
            .find(|(path, zone)| {
                path.rsplit('/')
                    .next()
                    .is_some_and(|name| name == zone.location)
            })
            .expect("upstream contains a raw-filename fallback");
        assert!(!raw_filename_fallback.1.location_name_verified);
    }
}
