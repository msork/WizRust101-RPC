//! Steam/Wizard101 discovery inside CrossOver bottles.

use std::{
    cmp::Reverse,
    collections::HashSet,
    fs,
    path::{Component, Path, PathBuf},
};

use steamlocate::{Library, SteamDir};

use crate::discovery::{LogCandidate, WIZARD101_STEAM_APP_ID};

const WIZARD101_LOG_RELATIVE_PATH: &str = "Bin/WizardClient.log";

/// Returns the standard per-user and system published CrossOver bottle roots.
/// Additional preference-derived roots are passed in separately by the macOS adapter.
pub fn standard_bottle_roots(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join("Library/Application Support/CrossOver/Bottles"),
        PathBuf::from("/Library/Application Support/CrossOver/Bottles"),
    ]
}

/// Reads the user's CrossOver bottle preferences without requiring the user to
/// launch CrossOver from Terminal or export its environment variables.
#[cfg(target_os = "macos")]
pub fn configured_bottle_roots(home: &Path) -> Vec<PathBuf> {
    use std::process::Command;

    fn read_preference(key: &str) -> Option<String> {
        Command::new("/usr/bin/defaults")
            .args(["read", "com.codeweavers.CrossOver", key])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
    }

    let mut roots = standard_bottle_roots(home);
    if let Some(path) = read_preference("BottleDir") {
        let path = path.trim();
        if !path.is_empty() {
            roots.push(PathBuf::from(path));
        }
    }
    if let Some(paths) = read_preference("ManagedBottleDirs") {
        roots.extend(paths.lines().filter_map(|line| {
            let path = line.trim().trim_matches(['(', ')', ',', '"', '\'']);
            (!path.is_empty()).then(|| PathBuf::from(path))
        }));
    }
    let mut seen = HashSet::new();
    roots.retain(|root| seen.insert(fs::canonicalize(root).unwrap_or_else(|_| root.clone())));
    roots
}

/// Enumerates one level of bottle directories and finds installed Wizard101 logs.
/// The discovery is read-only and uses each bottle's Windows drive mappings.
pub fn discover_candidates(bottle_roots: &[PathBuf]) -> Vec<LogCandidate> {
    let mut bottles = Vec::new();
    for root in bottle_roots {
        if is_bottle(root) {
            bottles.push(root.clone());
        }
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        bottles.extend(
            entries
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| is_bottle(path)),
        );
    }

    let mut seen_bottles = HashSet::new();
    let mut candidates = Vec::new();
    for bottle in bottles {
        let identity = fs::canonicalize(&bottle).unwrap_or_else(|_| bottle.clone());
        if !seen_bottles.insert(identity) {
            continue;
        }
        for steam_root in steam_roots_in_bottle(&bottle) {
            let mut seen_libraries = HashSet::new();
            for library_root in steam_library_roots(&bottle, &steam_root) {
                let identity =
                    fs::canonicalize(&library_root).unwrap_or_else(|_| library_root.clone());
                if !seen_libraries.insert(identity) {
                    continue;
                }
                let Ok(library) = Library::from_dir(&library_root) else {
                    continue;
                };
                let Some(app) = library.app(WIZARD101_STEAM_APP_ID).and_then(Result::ok) else {
                    continue;
                };
                let log = library
                    .resolve_app_dir(&app)
                    .join(WIZARD101_LOG_RELATIVE_PATH);
                if let Ok(metadata) = fs::metadata(&log)
                    && metadata.is_file()
                {
                    candidates.push(LogCandidate {
                        path: log,
                        modified: metadata.modified().ok(),
                    });
                }
            }
        }
    }
    candidates.sort_by_key(|candidate| Reverse(candidate.modified));
    candidates
}

/// Checks that a folder is a CrossOver bottle with a Steam installation containing
/// the Wizard101 app manifest. The log itself may not exist until the game is run.
pub fn contains_wizard101_install(bottle: &Path) -> bool {
    steam_roots_in_bottle(bottle).into_iter().any(|steam_root| {
        steam_library_roots(bottle, &steam_root)
            .into_iter()
            .any(|root| {
                Library::from_dir(&root)
                    .ok()
                    .and_then(|library| library.app(WIZARD101_STEAM_APP_ID).and_then(Result::ok))
                    .is_some()
            })
    })
}

pub fn is_bottle(path: &Path) -> bool {
    path.is_dir() && path.join("drive_c").is_dir()
}

fn steam_roots_in_bottle(bottle: &Path) -> Vec<PathBuf> {
    [
        bottle.join("drive_c/Program Files (x86)/Steam"),
        bottle.join("drive_c/Program Files/Steam"),
        bottle.join("drive_c/Steam"),
    ]
    .into_iter()
    .filter(|root| {
        root.join("steamapps/appmanifest_799960.acf").is_file()
            || root.join("steamapps/libraryfolders.vdf").is_file()
            || root.join("steam.exe").is_file()
    })
    .collect()
}

fn steam_library_roots(bottle: &Path, steam_root: &Path) -> Vec<PathBuf> {
    let mut roots = vec![steam_root.to_owned()];
    if let Ok(steam) = SteamDir::from_dir(steam_root)
        && let Ok(paths) = steam.library_paths()
    {
        roots.extend(
            paths
                .into_iter()
                .filter_map(|path| map_windows_path(bottle, &path)),
        );
    }
    roots
}

/// Maps an absolute Windows path stored in Steam metadata through the bottle's
/// `dosdevices` directory. Native absolute paths are accepted only when already
/// readable, which supports explicitly mapped macOS volumes in Steam metadata.
pub fn map_windows_path(bottle: &Path, path: &Path) -> Option<PathBuf> {
    if path.is_absolute() && path.exists() {
        return Some(path.to_owned());
    }
    let value = path.to_string_lossy();
    let bytes = value.as_bytes();
    if bytes.len() < 3 || !bytes[0].is_ascii_alphabetic() || bytes[1] != b':' {
        return None;
    }
    let drive = (bytes[0] as char).to_ascii_lowercase();
    let tail = value[2..].trim_start_matches(['\\', '/']);
    let components = tail
        .split(['\\', '/'])
        .filter(|part| !part.is_empty())
        .map(Path::new)
        .collect::<Vec<_>>();
    if components.iter().any(|component| {
        component
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    }) {
        return None;
    }

    let drive_root = if drive == 'c' {
        bottle.join("drive_c")
    } else {
        let mapping = bottle.join("dosdevices").join(format!("{drive}:"));
        if mapping.exists() {
            mapping
        } else {
            return None;
        }
    };
    Some(
        components
            .iter()
            .fold(drive_root, |root, part| root.join(part)),
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn steam_library(root: &Path, install_dir: &str) -> PathBuf {
        let steamapps = root.join("steamapps");
        fs::create_dir_all(&steamapps).expect("steamapps");
        fs::write(
            steamapps.join("appmanifest_799960.acf"),
            format!("\"AppState\" {{ \"appid\" \"799960\" \"installdir\" \"{install_dir}\" }}"),
        )
        .expect("Wizard101 manifest");
        let log = root
            .join("steamapps/common")
            .join(install_dir)
            .join("Bin/WizardClient.log");
        fs::create_dir_all(log.parent().expect("log parent")).expect("game path");
        fs::write(&log, "zone = ZF_Z07_Stone_Town\n").expect("log");
        log
    }

    fn bottle(root: &Path) -> PathBuf {
        let bottle = root.join("CrossOver/Bottles/Steam");
        fs::create_dir_all(bottle.join("drive_c")).expect("C drive");
        bottle
    }

    #[test]
    fn locates_wizard101_from_steam_app_manifest_inside_bottle() {
        let temp = tempdir().expect("temp directory");
        let bottle = bottle(temp.path());
        let steam = bottle.join("drive_c/Program Files (x86)/Steam");
        let expected = steam_library(&steam, "Wizard101");

        let found = discover_candidates(std::slice::from_ref(&bottle));

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
        assert!(contains_wizard101_install(&bottle));
    }

    #[test]
    #[cfg(unix)]
    fn translates_extra_steam_library_from_bottle_drive_mapping() {
        let temp = tempdir().expect("temp directory");
        let bottle = bottle(temp.path());
        let steam = bottle.join("drive_c/Program Files (x86)/Steam");
        fs::create_dir_all(steam.join("steamapps")).expect("Steam root");
        let extra = bottle.join("dosdevices/d:");
        let expected = steam_library(&extra, "Wizard Client");
        fs::write(
            steam.join("steamapps/libraryfolders.vdf"),
            r#""libraryfolders" { "0" { "path" "C:\\Program Files (x86)\\Steam" } "1" { "path" "D:\\" } }"#,
        )
        .expect("library metadata");

        let found = discover_candidates(&[bottle]);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, expected);
        assert!(contains_wizard101_install(&bottle));
    }

    #[test]
    fn path_mapping_rejects_relative_paths_and_parent_traversal() {
        let temp = tempdir().expect("temp directory");
        let bottle = bottle(temp.path());

        assert_eq!(map_windows_path(&bottle, Path::new("relative/path")), None);
        assert_eq!(map_windows_path(&bottle, Path::new(r"C:\..\outside")), None);
    }

    #[test]
    fn enumerates_custom_bottles_and_uses_documented_default_roots() {
        assert_eq!(
            standard_bottle_roots(Path::new("/Users/example")),
            [
                PathBuf::from("/Users/example/Library/Application Support/CrossOver/Bottles"),
                PathBuf::from("/Library/Application Support/CrossOver/Bottles"),
            ]
        );
        let temp = tempdir().expect("temp directory");
        let bottle = bottle(temp.path());
        let steam = bottle.join("drive_c/Program Files/Steam");
        steam_library(&steam, "Wizard101");
        assert_eq!(
            discover_candidates(&[bottle.parent().unwrap().to_owned()]).len(),
            1
        );
    }
}
