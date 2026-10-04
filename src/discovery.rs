use std::{cmp::Reverse, collections::HashSet, env, fs, path::PathBuf, time::SystemTime};

use steamlocate::{Library, SteamDir};

pub const WIZARD101_STEAM_APP_ID: u32 = 799_960;
const WIZARD101_LOG_RELATIVE_PATH: &str = "Bin/WizardClient.log";

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

/// Finds Wizard101 Steam logs in every readable library listed by each Steam root.
///
/// `additional_library_roots` contains folders explicitly authorized through the
/// desktop document portal when Flatpak cannot read a library automatically.
pub fn discover_candidates(
    steam_roots: &[PathBuf],
    additional_library_roots: &[PathBuf],
) -> Result<Vec<LogCandidate>, DiscoveryError> {
    let mut library_roots = Vec::new();
    for steam_root in steam_roots {
        library_roots.push(steam_root.clone());
        if let Ok(steam_dir) = SteamDir::from_dir(steam_root)
            && let Ok(paths) = steam_dir.library_paths()
        {
            library_roots.extend(paths);
        }
    }
    library_roots.extend(additional_library_roots.iter().cloned());

    let mut seen_roots = HashSet::new();
    let mut paths = Vec::new();
    for root in library_roots {
        let identity = fs::canonicalize(&root).unwrap_or_else(|_| root.clone());
        if !seen_roots.insert(identity) {
            continue;
        }
        let Ok(library) = Library::from_dir(&root) else {
            continue;
        };
        let Some(app) = library.app(WIZARD101_STEAM_APP_ID).and_then(Result::ok) else {
            continue;
        };
        paths.push(
            library
                .resolve_app_dir(&app)
                .join(WIZARD101_LOG_RELATIVE_PATH),
        );
    }

    let mut candidates = Vec::new();
    for path in paths {
        match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => candidates.push(LogCandidate {
                path,
                modified: metadata.modified().ok(),
            }),
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::PermissionDenied
                ) => {}
            Err(source) => return Err(DiscoveryError::Io { path, source }),
        }
    }
    candidates.sort_by_key(|candidate| Reverse(candidate.modified));
    Ok(candidates)
}

/// Finds standard Steam roots for the current OS and searches their libraries.
pub fn discover_system() -> Result<Vec<LogCandidate>, DiscoveryError> {
    discover_system_with_roots(&[])
}

pub fn discover_system_with_roots(
    additional_library_roots: &[PathBuf],
) -> Result<Vec<LogCandidate>, DiscoveryError> {
    let mut steam_roots = steamlocate::locate_all()
        .unwrap_or_default()
        .into_iter()
        .map(|steam| steam.path().to_owned())
        .collect::<Vec<_>>();
    steam_roots.extend(standard_steam_roots());
    deduplicate_roots(&mut steam_roots);
    discover_candidates(&steam_roots, additional_library_roots)
}

fn standard_steam_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    #[cfg(target_os = "linux")]
    if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
        roots.extend(linux_steam_roots(&home));
    }
    #[cfg(target_os = "windows")]
    roots.extend(windows_steam_roots(
        env::var_os("PROGRAMFILES(X86)").as_deref(),
    ));
    #[cfg(target_os = "macos")]
    if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
        roots.push(home.join("Library/Application Support/Steam"));
    }
    roots
}

#[cfg(any(target_os = "windows", test))]
fn windows_steam_roots(program_files_x86: Option<&std::ffi::OsStr>) -> Vec<PathBuf> {
    program_files_x86
        .map(|root| vec![PathBuf::from(root).join("Steam")])
        .unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn linux_steam_roots(home: &std::path::Path) -> Vec<PathBuf> {
    vec![
        home.join(".local/share/Steam"),
        home.join(".steam/steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
    ]
}

fn deduplicate_roots(roots: &mut Vec<PathBuf>) {
    let mut seen = HashSet::new();
    roots.retain(|root| {
        let identity = fs::canonicalize(root).unwrap_or_else(|_| root.clone());
        seen.insert(identity)
    });
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;

    use super::*;

    fn create_library(root: &Path, install_dir: &str) -> PathBuf {
        let manifest = root.join("steamapps/appmanifest_799960.acf");
        fs::create_dir_all(manifest.parent().expect("steamapps parent")).expect("create steamapps");
        fs::write(
            manifest,
            format!("\"AppState\" {{ \"appid\" \"799960\" \"installdir\" \"{install_dir}\" }}"),
        )
        .expect("write manifest");
        root.join("steamapps/common")
            .join(install_dir)
            .join(WIZARD101_LOG_RELATIVE_PATH)
    }

    fn create_log(root: &Path, install_dir: &str) -> PathBuf {
        let log = create_library(root, install_dir);
        fs::create_dir_all(log.parent().expect("log parent")).expect("create game directory");
        fs::write(&log, "").expect("create log");
        log
    }

    #[test]
    fn standalone_program_data_is_not_an_active_discovery_source() {
        let root = tempdir().expect("temp directory");
        let standalone = root
            .path()
            .join("ProgramData/KingsIsle Entertainment/Wizard101/Bin");
        fs::create_dir_all(&standalone).expect("create standalone tree");
        fs::write(standalone.join("WizardClient.log"), "").expect("standalone log");

        let found = discover_candidates(&[], &[]).expect("discover");

        assert!(found.is_empty());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_roots_include_native_aliases_and_researched_steam_flatpak_locations() {
        let home = Path::new("/home/example");
        assert_eq!(
            linux_steam_roots(home),
            [
                home.join(".local/share/Steam"),
                home.join(".steam/steam"),
                home.join(".steam/root"),
                home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
                home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
            ]
        );
    }

    #[test]
    fn windows_fallback_is_only_the_standard_32_bit_program_files_steam_root() {
        use std::ffi::OsStr;

        assert_eq!(
            windows_steam_roots(Some(OsStr::new(r"C:\Program Files (x86)"))),
            [PathBuf::from(r"C:\Program Files (x86)").join("Steam")]
        );
        assert!(windows_steam_roots(None).is_empty());
    }

    #[test]
    fn discovers_steam_install_in_a_second_non_default_library_from_manifest() {
        let root = tempdir().expect("temp directory");
        let steam = root.path().join("Steam");
        let first_library = root.path().join("E Drive Games");
        let second_library = root.path().join("F Games");
        fs::create_dir_all(steam.join("steamapps")).expect("steam root");
        fs::create_dir_all(first_library.join("steamapps")).expect("first library");
        let first_path = first_library.to_string_lossy().replace('\\', "\\\\");
        let second_path = second_library.to_string_lossy().replace('\\', "\\\\");
        fs::write(
            steam.join("steamapps/libraryfolders.vdf"),
            format!(
                "\"libraryfolders\" {{ \"0\" {{ \"path\" \"{}\" }} \"1\" {{ \"path\" \"{}\" }} }}",
                first_path, second_path
            ),
        )
        .expect("write library metadata");
        let expected = create_log(&second_library, "Wiz Client");

        let found = discover_candidates(&[steam], &[]).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn discovers_a_portal_granted_additional_library_directly() {
        let root = tempdir().expect("temp directory");
        let external_library = root.path().join("Mounted Games/SteamLibrary");
        let expected = create_log(&external_library, "Wizard101");

        let found = discover_candidates(&[], &[external_library]).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn ignores_missing_and_inaccessible_additional_libraries_without_blocking_other_roots() {
        let root = tempdir().expect("temp directory");
        let steam = root.path().join("Steam");
        let expected = create_log(&steam, "Wizard101");

        let found = discover_candidates(
            &[steam],
            &[root.path().join("not authorized or no longer mounted")],
        )
        .expect("available Steam library remains discoverable");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }

    #[test]
    fn deduplicates_steam_root_and_additional_root_aliases() {
        let root = tempdir().expect("temp directory");
        let steam = root.path().join("Steam");
        let expected = create_log(&steam, "Wizard101");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&steam, root.path().join("steam-alias")).expect("create alias");
        #[cfg(not(unix))]
        let alias = steam.clone();
        #[cfg(unix)]
        let alias = root.path().join("steam-alias");

        let found = discover_candidates(&[steam], &[alias]).expect("discover");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
    }
}
