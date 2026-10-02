use std::{collections::BTreeMap, io::Read};

use serde::{Deserialize, Serialize};

pub const MAPPING_SCHEMA_VERSION: u32 = 1;
pub const BACON_SOURCE: &str = "Bacon1661/Wizard101-RPC";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ZoneCatalog {
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<CatalogProvenance>,
    #[serde(default)]
    pub zones: BTreeMap<String, ZoneMapping>,
}

impl Default for ZoneCatalog {
    fn default() -> Self {
        Self {
            schema_version: MAPPING_SCHEMA_VERSION,
            provenance: None,
            zones: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogProvenance {
    pub source: String,
    pub source_file: String,
    pub imported_at_unix_seconds: u64,
    pub review_status: ReviewStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ZoneMapping {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world: Option<WorldMapping>,
    pub provenance: MappingProvenance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorldMapping {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStatus {
    UnverifiedLegacy,
    Verified,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MappingProvenance {
    pub source: String,
    pub review_status: ReviewStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<MappingEvidence>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MappingEvidence {
    pub kind: MappingEvidenceKind,
    pub description: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MappingEvidenceKind {
    CurrentClientLog,
    ProjectOwnerManual,
    Wizard101CentralManual,
    BaconCandidate,
}

#[derive(Debug, thiserror::Error)]
pub enum MappingError {
    #[error("could not read mapping catalog: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid mapping catalog: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported mapping schema version {0}")]
    UnsupportedSchema(u32),
    #[error("legacy mapping must be a JSON object")]
    LegacyRootNotObject,
}

impl ZoneCatalog {
    pub fn from_reader(reader: impl Read) -> Result<Self, MappingError> {
        let catalog: Self = serde_json::from_reader(reader)?;
        if catalog.schema_version != MAPPING_SCHEMA_VERSION {
            return Err(MappingError::UnsupportedSchema(catalog.schema_version));
        }
        Ok(catalog)
    }

    pub fn resolve(&self, raw_zone_id: &str) -> Option<&ZoneMapping> {
        self.zones.get(raw_zone_id)
    }

    pub fn import_bacon_json(
        reader: impl Read,
        source_file: impl Into<String>,
        imported_at_unix_seconds: u64,
    ) -> Result<Self, MappingError> {
        let legacy: serde_json::Value = serde_json::from_reader(reader)?;
        let root = legacy
            .as_object()
            .ok_or(MappingError::LegacyRootNotObject)?;

        let world_names = root
            .get("zoneNames")
            .and_then(serde_json::Value::as_object)
            .map(|worlds| {
                worlds
                    .iter()
                    .filter_map(|(id, name)| name.as_str().map(|name| (id.as_str(), name)))
                    .collect::<BTreeMap<_, _>>()
            })
            .unwrap_or_default();
        let row_provenance = MappingProvenance {
            source: BACON_SOURCE.to_owned(),
            review_status: ReviewStatus::UnverifiedLegacy,
            verified_at: None,
            evidence: vec![MappingEvidence {
                kind: MappingEvidenceKind::BaconCandidate,
                description: "Imported display-name candidate; not verified against current-client zone evidence.".to_owned(),
            }],
        };

        let mut zones = BTreeMap::new();
        for (raw_zone_id, value) in root {
            if raw_zone_id == "zoneNames" {
                continue;
            }
            let Some(location) = value.as_str() else {
                continue;
            };
            let world = raw_zone_id.split_once('/').and_then(|(world_id, _)| {
                world_names.get(world_id).map(|world_name| WorldMapping {
                    id: world_id.to_owned(),
                    name: (*world_name).to_owned(),
                })
            });
            zones.insert(
                raw_zone_id.clone(),
                ZoneMapping {
                    location: location.to_owned(),
                    world,
                    provenance: row_provenance.clone(),
                },
            );
        }

        Ok(Self {
            schema_version: MAPPING_SCHEMA_VERSION,
            provenance: Some(CatalogProvenance {
                source: BACON_SOURCE.to_owned(),
                source_file: source_file.into(),
                imported_at_unix_seconds,
                review_status: ReviewStatus::UnverifiedLegacy,
            }),
            zones,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const LEGACY_CATALOG: &str = include_str!("../tests/fixtures/bacon-zones-excerpt.json");

    #[test]
    fn imports_legacy_entries_and_only_assigns_evidenced_worlds() {
        let catalog = ZoneCatalog::import_bacon_json(
            Cursor::new(LEGACY_CATALOG),
            "tests/fixtures/bacon-zones-excerpt.json",
            1_800_000_000,
        )
        .expect("import legacy catalog");

        let ravenwood = catalog
            .resolve("WizardCity/WC_Ravenwood")
            .expect("known legacy location");
        assert_eq!(ravenwood.location, "Ravenwood");
        assert_eq!(
            ravenwood.world,
            Some(WorldMapping {
                id: "WizardCity".to_owned(),
                name: "Wizard City".to_owned(),
            })
        );
        assert_eq!(
            ravenwood.provenance.review_status,
            ReviewStatus::UnverifiedLegacy
        );
        assert_eq!(
            ravenwood.provenance.evidence[0].kind,
            MappingEvidenceKind::BaconCandidate
        );
        assert_eq!(ravenwood.provenance.verified_at, None);

        let short_id = catalog
            .resolve("G14_HS/HS_Z01_ZigazagUpper")
            .expect("legacy location without world prefix");
        assert_eq!(short_id.location, "Upper Zigazag");
        assert_eq!(short_id.world, None);

        assert!(
            catalog.resolve("not-in-catalog").is_none(),
            "unknown zone must remain unresolved"
        );
    }

    #[test]
    fn catalog_round_trips_and_rejects_future_schema_versions() {
        let catalog = ZoneCatalog::import_bacon_json(Cursor::new(LEGACY_CATALOG), "fixture", 7)
            .expect("import");
        let json = serde_json::to_vec(&catalog).expect("serialize");
        let restored = ZoneCatalog::from_reader(Cursor::new(json)).expect("deserialize");
        assert_eq!(restored, catalog);

        let error = ZoneCatalog::from_reader(Cursor::new(r#"{"schema_version":99,"zones":{}}"#))
            .expect_err("future version must fail");
        assert!(matches!(error, MappingError::UnsupportedSchema(99)));
    }

    #[test]
    fn stone_town_runtime_row_has_separate_owner_and_current_log_evidence() {
        let catalog = ZoneCatalog::from_reader(include_bytes!("../data/zones.json").as_slice())
            .expect("runtime catalog");
        let mapping = catalog
            .resolve("Zafaria/ZF_Z07_Stone_Town")
            .expect("captured raw ID");
        assert_eq!(mapping.location, "Stone Town");
        assert_eq!(
            mapping.world,
            Some(WorldMapping {
                id: "Zafaria".into(),
                name: "Zafaria".into()
            })
        );
        assert_eq!(mapping.provenance.review_status, ReviewStatus::Verified);
        assert_eq!(
            mapping.provenance.verified_at.as_deref(),
            Some("2026-10-01")
        );
        assert!(
            mapping
                .provenance
                .evidence
                .iter()
                .any(|item| item.kind == MappingEvidenceKind::CurrentClientLog)
        );
        assert!(
            mapping
                .provenance
                .evidence
                .iter()
                .any(|item| item.kind == MappingEvidenceKind::ProjectOwnerManual)
        );
        assert!(
            !mapping
                .provenance
                .evidence
                .iter()
                .any(|item| item.kind == MappingEvidenceKind::Wizard101CentralManual)
        );
        assert!(catalog.resolve("Zafaria/ZF_Z06_Unknown").is_none());
    }
}
