use std::{fs, path::Path, time::SystemTime};

use tempfile::tempdir;
use wizrust101_rpc::{
    config::{
        AppConfig, EnvironmentOverrides, LogLevel, config_path_in, default_config_path,
        resolve_log_candidates,
    },
    discovery::{DiscoveryError, LogCandidate},
    presence::DisplayStat,
};

fn candidate(path: &Path) -> LogCandidate {
    LogCandidate {
        path: path.to_owned(),
        modified: Some(SystemTime::UNIX_EPOCH),
    }
}

#[test]
fn reads_user_file_and_resolves_environment_precedence() {
    let root = tempdir().expect("temporary user config");
    let config_path = config_path_in(root.path());
    fs::write(
        &config_path,
        r#"{"schema_version":1,"game_log_path":null,"display_stat":"none","log_level":"info"}"#,
    )
    .expect("write v1 config");

    let loaded = AppConfig::load(&config_path);
    assert!(loaded.warnings.is_empty());
    let settings = loaded.config.resolve(&EnvironmentOverrides {
        display_stat: Some("health".into()),
        log_level: None,
    });
    assert_eq!(settings.display_stat, DisplayStat::Health);
    assert_eq!(settings.log_level, LogLevel::Info);
}

#[test]
fn missing_user_file_uses_defaults_without_creating_or_rewriting_anything() {
    let root = tempdir().expect("temporary user config");
    let config_path = config_path_in(root.path());
    let loaded = AppConfig::load(&config_path);

    assert_eq!(loaded.config, AppConfig::default());
    assert!(loaded.warnings.is_empty());
    assert!(!config_path.exists());
}

#[test]
fn invalid_or_missing_override_falls_back_to_discovered_candidates() {
    let root = tempdir().expect("temporary install");
    let log_path = root.path().join("WizardClient.log");
    fs::write(&log_path, "").expect("create log");
    let expected = candidate(&log_path);

    let (candidates, warning) = resolve_log_candidates(
        Some(&root.path().join("missing").join("WizardClient.log")),
        || Ok(vec![expected.clone()]),
    );
    assert!(warning.is_some());
    assert_eq!(
        candidates.expect("fallback discovery"),
        vec![expected.clone()]
    );

    let (candidates, warning) = resolve_log_candidates(None, || Ok(vec![expected.clone()]));
    assert!(warning.is_none());
    assert_eq!(candidates.expect("automatic discovery"), vec![expected]);
}

#[test]
fn valid_override_takes_priority_without_running_automatic_discovery() {
    let root = tempdir().expect("temporary install");
    let path = root.path().join("WizardClient.log");
    fs::write(&path, "").expect("create log");
    let (candidates, warning) = resolve_log_candidates(Some(&path), || {
        panic!("discovery must not replace a valid explicit override")
    });
    assert!(warning.is_none());
    assert_eq!(candidates.expect("valid override")[0].path, path);
}

#[test]
fn config_path_is_in_the_native_project_config_location() {
    let path = default_config_path().expect("native per-user config directory");
    assert!(path.is_absolute());
    assert!(path.ends_with("config.json"));

    #[cfg(target_os = "linux")]
    assert!(
        path.to_string_lossy().contains("/.config/")
            || std::env::var_os("XDG_CONFIG_HOME")
                .is_some_and(|base| { path.starts_with(Path::new(&base)) })
    );

    #[cfg(target_os = "windows")]
    assert!(
        path.to_string_lossy()
            .to_ascii_lowercase()
            .contains("\\appdata\\roaming\\")
    );
}

#[test]
fn automatic_discovery_error_is_preserved_when_override_is_invalid() {
    let root = tempdir().expect("temporary install");
    let error_path = root.path().join("locked").join("WizardClient.log");
    let io_error = || {
        Err(DiscoveryError::Io {
            path: error_path.clone(),
            source: std::io::Error::from(std::io::ErrorKind::PermissionDenied),
        })
    };
    let (candidates, warning) = resolve_log_candidates(Some(&error_path), io_error);

    assert!(warning.is_some());
    assert!(matches!(candidates, Err(DiscoveryError::Io { .. })));
}
