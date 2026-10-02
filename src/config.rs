use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use directories::ProjectDirs;
use serde_json::{Map, Number, Value};

use crate::discovery::LogCandidate;

pub const CONFIG_SCHEMA_VERSION: u64 = 1;
pub const CONFIG_FILE_NAME: &str = "config.json";
const PROJECT_QUALIFIER: &str = "com";
const PROJECT_ORGANIZATION: &str = "msork";
const PROJECT_APPLICATION: &str = "WizRust101-RPC";

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    #[default]
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn permits(self, message_level: Self) -> bool {
        message_level <= self
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
        }
    }
}

impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            _ => Err("expected error, warn, info, or debug"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppConfig {
    pub game_log_path: Option<PathBuf>,
    pub log_level: LogLevel,
    extra_fields: Map<String, Value>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            game_log_path: None,
            log_level: LogLevel::Warn,
            extra_fields: Map::new(),
        }
    }
}

impl AppConfig {
    pub fn from_json(input: &str) -> ConfigLoad {
        let value: Value = match serde_json::from_str(input) {
            Ok(value) => value,
            Err(error) => {
                return ConfigLoad {
                    config: Self::default(),
                    warnings: vec![format!(
                        "configuration JSON is malformed at line {}, column {}; using defaults",
                        error.line(),
                        error.column()
                    )],
                };
            }
        };

        let Some(mut fields) = value.as_object().cloned() else {
            return ConfigLoad {
                config: Self::default(),
                warnings: vec!["configuration root must be a JSON object; using defaults".into()],
            };
        };

        let Some(version) = fields.get("schema_version").and_then(Value::as_u64) else {
            return ConfigLoad {
                config: Self::default(),
                warnings: vec![
                    "configuration schema_version is missing or invalid; using defaults".into(),
                ],
            };
        };
        if version != CONFIG_SCHEMA_VERSION {
            return ConfigLoad {
                config: Self::default(),
                warnings: vec![format!(
                    "unsupported configuration schema_version {version}; file left unchanged and defaults used"
                )],
            };
        }

        let mut config = Self::default();
        let mut warnings = Vec::new();

        if let Some(value) = fields.get("game_log_path") {
            match value {
                Value::Null => {}
                Value::String(path) if !path.trim().is_empty() => {
                    config.game_log_path = Some(PathBuf::from(path));
                }
                _ => warnings.push("invalid game_log_path; using automatic discovery".into()),
            }
        }

        if let Some(value) = fields.get("log_level") {
            match value.as_str().and_then(|value| value.parse().ok()) {
                Some(level) => config.log_level = level,
                None => warnings.push("invalid log_level; using warn".into()),
            }
        }

        fields.remove("schema_version");
        fields.remove("game_log_path");
        fields.remove("log_level");
        config.extra_fields = fields;

        ConfigLoad { config, warnings }
    }

    pub fn load(path: &Path) -> ConfigLoad {
        match fs::read_to_string(path) {
            Ok(contents) => Self::from_json(&contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => ConfigLoad {
                config: Self::default(),
                warnings: Vec::new(),
            },
            Err(error) => ConfigLoad {
                config: Self::default(),
                warnings: vec![format!(
                    "cannot read configuration file ({}); using defaults",
                    error.kind()
                )],
            },
        }
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        let mut fields = self.extra_fields.clone();
        fields.insert(
            "schema_version".into(),
            Value::Number(Number::from(CONFIG_SCHEMA_VERSION)),
        );
        fields.insert(
            "game_log_path".into(),
            self.game_log_path
                .as_ref()
                .map(|path| Value::String(path.to_string_lossy().into_owned()))
                .unwrap_or(Value::Null),
        );
        fields.insert(
            "log_level".into(),
            Value::String(self.log_level.as_str().into()),
        );
        serde_json::to_string_pretty(&Value::Object(fields))
    }

    pub fn resolve(&self, overrides: &EnvironmentOverrides) -> ResolvedConfig {
        let mut result = ResolvedConfig {
            game_log_path: self.game_log_path.clone(),
            log_level: self.log_level,
            warnings: Vec::new(),
        };

        if let Some(value) = &overrides.log_level {
            match value.parse() {
                Ok(level) => result.log_level = level,
                Err(_) => result
                    .warnings
                    .push("invalid WIZRUST101_LOG_LEVEL; using config value or warn".into()),
            }
        }

        result
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConfigLoad {
    pub config: AppConfig,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EnvironmentOverrides {
    pub log_level: Option<String>,
}

impl EnvironmentOverrides {
    pub fn from_process() -> Self {
        Self {
            log_level: environment_value("WIZRUST101_LOG_LEVEL"),
        }
    }
}

fn environment_value(name: &str) -> Option<String> {
    env::var_os(name).map(|value| value.into_string().unwrap_or_default())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedConfig {
    pub game_log_path: Option<PathBuf>,
    pub log_level: LogLevel,
    pub warnings: Vec<String>,
}

pub fn default_config_path() -> Option<PathBuf> {
    ProjectDirs::from(PROJECT_QUALIFIER, PROJECT_ORGANIZATION, PROJECT_APPLICATION)
        .map(|directories| config_path_in(directories.config_dir()))
}

pub fn config_path_in(directory: &Path) -> PathBuf {
    directory.join(CONFIG_FILE_NAME)
}

/// Converts a configured override into a log candidate. Failure is recoverable:
/// callers should warn and continue with automatic discovery.
pub fn candidate_from_override(path: &Path) -> Result<LogCandidate, String> {
    let is_expected_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("WizardClient.log"));
    if !is_expected_name {
        return Err("configured game_log_path must name WizardClient.log".into());
    }

    let metadata = fs::metadata(path).map_err(|error| {
        format!(
            "configured game_log_path is unavailable ({}); automatic discovery will be used",
            error.kind()
        )
    })?;
    if !metadata.is_file() {
        return Err(
            "configured game_log_path is not a regular file; automatic discovery will be used"
                .into(),
        );
    }

    Ok(LogCandidate {
        path: path.to_owned(),
        modified: metadata.modified().ok(),
    })
}

pub fn resolve_log_candidates<F>(
    override_path: Option<&Path>,
    discover: F,
) -> (
    Result<Vec<LogCandidate>, crate::discovery::DiscoveryError>,
    Option<String>,
)
where
    F: FnOnce() -> Result<Vec<LogCandidate>, crate::discovery::DiscoveryError>,
{
    match override_path {
        Some(path) => match candidate_from_override(path) {
            Ok(candidate) => (Ok(vec![candidate]), None),
            Err(warning) => (discover(), Some(warning)),
        },
        None => (discover(), None),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn defaults_are_safe_and_need_no_config_file() {
        let loaded = AppConfig::load(Path::new("a-file-that-does-not-exist/config.json"));
        assert_eq!(loaded.config, AppConfig::default());
        assert!(loaded.warnings.is_empty());
        assert_eq!(loaded.config.log_level, LogLevel::Warn);
        assert!(loaded.config.game_log_path.is_none());
    }

    #[test]
    fn v1_config_loads_supported_fields_and_preserves_removed_or_unknown_fields() {
        let loaded = AppConfig::from_json(
            r#"{"schema_version":1,"game_log_path":"/tmp/WizardClient.log","display_stat":"none","log_level":"debug","future_option":{"kept":true}}"#,
        );

        assert!(loaded.warnings.is_empty());
        assert_eq!(
            loaded.config.game_log_path.as_deref(),
            Some(Path::new("/tmp/WizardClient.log"))
        );
        assert_eq!(loaded.config.log_level, LogLevel::Debug);
        let serialized = loaded.config.to_json_pretty().expect("serialize config");
        let value: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(value["display_stat"], "none");
        assert_eq!(value["future_option"]["kept"], true);
        let round_trip = AppConfig::from_json(&serialized);
        assert!(round_trip.warnings.is_empty());
        assert_eq!(round_trip.config, loaded.config);
    }

    #[test]
    fn invalid_fields_fall_back_independently_and_keep_valid_siblings() {
        let loaded =
            AppConfig::from_json(r#"{"schema_version":1,"game_log_path":22,"log_level":"trace"}"#);
        assert_eq!(loaded.config, AppConfig::default());
        assert_eq!(loaded.warnings.len(), 2);

        let loaded = AppConfig::from_json(
            r#"{"schema_version":1,"display_stat":"none","log_level":"bogus"}"#,
        );
        assert_eq!(loaded.config.log_level, LogLevel::Warn);
        assert_eq!(loaded.warnings.len(), 1);
    }

    #[test]
    fn removed_stat_selector_is_inert_but_preserved_for_existing_v1_configs() {
        for stat in ["health", "none", "level", "school", "character_name"] {
            let input = format!(r#"{{"schema_version":1,"display_stat":"{stat}"}}"#);
            let loaded = AppConfig::from_json(&input);
            assert!(loaded.config.game_log_path.is_none());
            assert_eq!(loaded.config.log_level, LogLevel::Warn);
            assert!(loaded.warnings.is_empty());
            let serialized = loaded.config.to_json_pretty().unwrap();
            let value: Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(value["display_stat"], stat);
        }
    }

    #[test]
    fn malformed_and_unsupported_configs_fall_back_without_rewriting_source() {
        let root = tempdir().expect("temp directory");
        let path = root.path().join(CONFIG_FILE_NAME);
        let original = b"{not json";
        fs::write(&path, original).expect("write malformed config");

        let loaded = AppConfig::load(&path);
        assert_eq!(loaded.config, AppConfig::default());
        assert_eq!(fs::read(&path).expect("read untouched config"), original);
        assert_eq!(loaded.warnings.len(), 1);

        let original = br#"{"schema_version":99,"display_stat":"none"}"#;
        fs::write(&path, original).expect("write future schema");
        let loaded = AppConfig::load(&path);
        assert_eq!(loaded.config, AppConfig::default());
        assert_eq!(fs::read(&path).expect("read untouched config"), original);
        assert!(loaded.warnings[0].contains("unsupported configuration schema_version 99"));
    }

    #[test]
    fn config_read_errors_use_defaults_and_report_a_warning() {
        let root = tempdir().expect("temporary directory");
        let loaded = AppConfig::load(root.path());
        assert_eq!(loaded.config, AppConfig::default());
        assert_eq!(loaded.warnings.len(), 1);
    }

    #[test]
    fn environment_overrides_valid_file_values_but_invalid_overrides_do_not_erase_them() {
        let config = AppConfig::from_json(
            r#"{"schema_version":1,"display_stat":"none","log_level":"debug"}"#,
        )
        .config;
        let resolved = config.resolve(&EnvironmentOverrides {
            log_level: Some("error".into()),
        });
        assert_eq!(resolved.log_level, LogLevel::Error);

        let resolved = config.resolve(&EnvironmentOverrides {
            log_level: Some("trace".into()),
        });
        assert_eq!(resolved.log_level, LogLevel::Debug);
        assert_eq!(resolved.warnings.len(), 1);
    }

    #[test]
    fn native_config_path_is_project_scoped_and_never_current_directory_relative() {
        let path = default_config_path().expect("native user config directory");
        assert!(path.is_absolute());
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("config.json")
        );
        assert_ne!(path.parent(), Some(Path::new(".")));
        assert_eq!(
            config_path_in(Path::new("/user-config/app")),
            Path::new("/user-config/app/config.json")
        );
    }

    #[test]
    fn configured_override_requires_existing_wizard_client_log_file() {
        let root = tempdir().expect("temp directory");
        let path = root.path().join("WizardClient.log");
        fs::write(&path, "").expect("create log");
        let candidate = candidate_from_override(&path).expect("valid override");
        assert_eq!(candidate.path, path);

        assert!(candidate_from_override(&root.path().join("other.log")).is_err());
        let missing = root.path().join("missing").join("WizardClient.log");
        assert!(candidate_from_override(&missing).is_err());
        assert!(candidate_from_override(root.path()).is_err());
    }

    #[test]
    fn log_verbosity_filters_lower_priority_messages() {
        assert!(LogLevel::Warn.permits(LogLevel::Error));
        assert!(LogLevel::Warn.permits(LogLevel::Warn));
        assert!(!LogLevel::Warn.permits(LogLevel::Info));
        assert!(LogLevel::Debug.permits(LogLevel::Debug));
        assert!(!LogLevel::Error.permits(LogLevel::Warn));
    }
}
