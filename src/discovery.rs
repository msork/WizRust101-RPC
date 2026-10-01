use std::{
    cmp::Reverse,
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use steamlocate::SteamDir;

pub const WIZARD101_STEAM_APP_ID: u32 = 799_960;
const WIZARD101_LOG_RELATIVE_PATH: &str = "Bin/WizardClient.log";
const DEFAULT_STEAM_ROOT: &str = r"C:\Program Files (x86)\Steam";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogCandidate {
    pub path: PathBuf,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("failed to inspect {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

/// Returns valid WizardClient.log candidates ordered newest-first.
///
/// The standalone ProgramData path, each supplied Steam root's app manifest,
/// and the historical default Steam install path are checked. Invalid or
/// missing Steam metadata is ignored so one broken install cannot block the
/// other candidates.
pub fn discover_candidates(
    program_data: Option<&Path>,
    steam_roots: &[PathBuf],
    default_steam_root: Option<&Path>,
) -> Result<Vec<LogCandidate>, DiscoveryError> {
    let mut paths = Vec::new();
    if let Some(program_data) = program_data {
        paths.push(
            program_data
                .join("KingsIsle Entertainment")
                .join("Wizard101")
                .join(WIZARD101_LOG_RELATIVE_PATH),
        );
    }

    for steam_root in steam_roots {
        if let Ok(steam_dir) = SteamDir::from_dir(steam_root) {
            if let Ok(Some((app, library))) = steam_dir.find_app(WIZARD101_STEAM_APP_ID) {
                paths.push(
                    library
                        .resolve_app_dir(&app)
                        .join(WIZARD101_LOG_RELATIVE_PATH),
                );
            }
        }

        paths.push(
            steam_root
                .join("steamapps")
                .join("common")
                .join("Wizard101")
                .join(WIZARD101_LOG_RELATIVE_PATH),
        );
    }

    if let Some(default_steam_root) = default_steam_root {
        paths.push(
            default_steam_root
                .join("steamapps")
                .join("common")
                .join("Wizard101")
                .join(WIZARD101_LOG_RELATIVE_PATH),
        );
    }

    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    for path in paths {
        if !seen.insert(path.clone()) {
            continue;
        }
        match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => candidates.push(LogCandidate {
                path,
                modified: metadata.modified().ok(),
            }),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(source) => return Err(DiscoveryError::Io { path, source }),
        }
    }

    candidates.sort_by_key(|candidate| Reverse(candidate.modified));
    Ok(candidates)
}

/// Discovers the current machine's standard ProgramData and Steam locations.
pub fn discover_system() -> Result<Vec<LogCandidate>, DiscoveryError> {
    let program_data = env::var_os("PROGRAMDATA").map(PathBuf::from);
    let steam_roots = steamlocate::locate_all()
        .unwrap_or_default()
        .into_iter()
        .map(|steam| steam.path().to_owned())
        .collect::<Vec<_>>();
    let default_steam_root = Path::new(DEFAULT_STEAM_ROOT);
    discover_candidates(
        program_data.as_deref(),
        &steam_roots,
        Some(default_steam_root),
    )
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;

    use super::*;

    fn create_log(root: &Path, relative: &str) -> PathBuf {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("parent directory")).expect("create directories");
        fs::write(&path, "").expect("create log");
        path
    }

    #[test]
    fn discovers_standalone_program_data_log() {
        let root = tempdir().expect("temp directory");
        let expected = create_log(
            root.path(),
            "KingsIsle Entertainment/Wizard101/Bin/WizardClient.log",
        );

        let found = discover_candidates(Some(root.path()), &[], None).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn discovers_steam_install_in_a_second_non_default_library_from_manifest() {
        let root = tempdir().expect("temp directory");
        let steam = root.path().join("Steam");
        let first_library = root.path().join("E Drive Games");
        let second_library = root.path().join("F Games");
        fs::create_dir_all(steam.join("steamapps")).expect("steam root");
        fs::create_dir_all(first_library.join("steamapps")).expect("first library");
        fs::create_dir_all(second_library.join("steamapps/common/Wiz Client/Bin"))
            .expect("library install");
        fs::write(
            steam.join("steamapps/libraryfolders.vdf"),
            format!(
                "\"libraryfolders\" {{ \"0\" {{ \"path\" \"{}\" }} \"1\" {{ \"path\" \"{}\" }} }}",
                first_library.display(),
                second_library.display()
            ),
        )
        .expect("write library metadata");
        fs::write(
            second_library.join("steamapps/appmanifest_799960.acf"),
            "\"AppState\" { \"appid\" \"799960\" \"installdir\" \"Wiz Client\" }",
        )
        .expect("write app manifest");
        let expected = create_log(
            &second_library,
            "steamapps/common/Wiz Client/Bin/WizardClient.log",
        );

        let found = discover_candidates(None, &[steam], None).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn ignores_malformed_steam_metadata_without_blocking_standalone_discovery() {
        let root = tempdir().expect("temp directory");
        let program_data = root.path().join("ProgramData");
        let steam = root.path().join("Steam");
        let expected = create_log(
            &program_data,
            "KingsIsle Entertainment/Wizard101/Bin/WizardClient.log",
        );
        fs::create_dir_all(steam.join("steamapps")).expect("steam root");
        fs::write(steam.join("steamapps/libraryfolders.vdf"), "not vdf").expect("metadata");

        let found = discover_candidates(Some(&program_data), &[steam], None).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn ignores_missing_files_and_deduplicates_default_steam_path() {
        let root = tempdir().expect("temp directory");
        let steam = root.path().join("Steam");
        let expected = create_log(&steam, "steamapps/common/Wizard101/Bin/WizardClient.log");

        let found = discover_candidates(None, std::slice::from_ref(&steam), Some(&steam))
            .expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }
}
