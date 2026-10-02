use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::discovery::WIZARD101_STEAM_APP_ID;

pub const REGISTRY_FILE_NAME: &str = "steam-libraries.json";
const REGISTRY_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("cannot read Steam library access registry ({0})")]
    Read(std::io::ErrorKind),
    #[error("Steam library access registry is malformed or has an unsupported version")]
    Invalid,
    #[error(
        "selected folder is not a Steam library containing Wizard101 app {WIZARD101_STEAM_APP_ID}"
    )]
    InvalidLibrary,
    #[error("cannot save Steam library access registry ({0})")]
    Write(std::io::ErrorKind),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct Registry {
    version: u32,
    library_roots: Vec<PathBuf>,
}

pub fn default_registry_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "msork", "WizRust101-RPC")
        .map(|dirs| dirs.data_dir().join(REGISTRY_FILE_NAME))
}

/// Loads persisted portal-granted roots. Invalid files are left untouched.
pub fn load_roots(path: &Path) -> Result<Vec<PathBuf>, RegistryError> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(RegistryError::Read(error.kind())),
    };
    let registry: Registry = serde_json::from_str(&contents).map_err(|_| RegistryError::Invalid)?;
    if registry.version != REGISTRY_VERSION {
        return Err(RegistryError::Invalid);
    }
    Ok(registry
        .library_roots
        .into_iter()
        .filter(|root| is_authorized_steam_library(root))
        .collect())
}

/// Adds a portal-returned root, preserving a malformed existing registry.
pub fn add_root(path: &Path, root: &Path) -> Result<bool, RegistryError> {
    if !is_authorized_steam_library(root) {
        return Err(RegistryError::InvalidLibrary);
    }
    let mut roots = load_roots(path)?;
    let identity = fs::canonicalize(root).unwrap_or_else(|_| root.to_owned());
    if roots
        .iter()
        .any(|existing| fs::canonicalize(existing).unwrap_or_else(|_| existing.clone()) == identity)
    {
        return Ok(false);
    }
    roots.push(root.to_owned());
    roots.sort();
    let registry = Registry {
        version: REGISTRY_VERSION,
        library_roots: roots,
    };
    let contents = serde_json::to_vec_pretty(&registry).map_err(|_| RegistryError::Invalid)?;
    let parent = path.parent().ok_or(RegistryError::Invalid)?;
    fs::create_dir_all(parent).map_err(|error| RegistryError::Write(error.kind()))?;
    let temporary = path.with_extension(format!("json.{}.tmp", std::process::id()));
    fs::write(&temporary, contents).map_err(|error| RegistryError::Write(error.kind()))?;
    fs::rename(&temporary, path).map_err(|error| RegistryError::Write(error.kind()))?;
    Ok(true)
}

fn is_authorized_steam_library(root: &Path) -> bool {
    if !root.is_dir() {
        return false;
    }
    let Ok(library) = steamlocate::Library::from_dir(root) else {
        return false;
    };
    library
        .app(WIZARD101_STEAM_APP_ID)
        .is_some_and(|app| app.is_ok())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn library(root: &Path) {
        let manifest = root.join("steamapps/appmanifest_799960.acf");
        fs::create_dir_all(manifest.parent().expect("steamapps parent")).unwrap();
        fs::write(
            manifest,
            "\"AppState\" { \"appid\" \"799960\" \"installdir\" \"Wizard101\" }",
        )
        .unwrap();
    }

    #[test]
    fn absent_registry_uses_no_external_roots() {
        let root = tempdir().unwrap();
        assert!(
            load_roots(&root.path().join(REGISTRY_FILE_NAME))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn validates_and_deduplicates_portal_granted_steam_libraries() {
        let root = tempdir().unwrap();
        let library_root = root.path().join("SteamLibrary");
        library(&library_root);
        let registry = root.path().join(REGISTRY_FILE_NAME);

        assert!(add_root(&registry, &library_root).unwrap());
        assert!(!add_root(&registry, &library_root).unwrap());
        assert_eq!(load_roots(&registry).unwrap(), [library_root]);
    }

    #[test]
    fn unsupported_or_malformed_registry_is_preserved_on_add_attempt() {
        let root = tempdir().unwrap();
        let library_root = root.path().join("SteamLibrary");
        library(&library_root);
        let registry = root.path().join(REGISTRY_FILE_NAME);
        let original = br#"{"version":2,"library_roots":[]}"#;
        fs::write(&registry, original).unwrap();

        assert!(matches!(
            add_root(&registry, &library_root),
            Err(RegistryError::Invalid)
        ));
        assert_eq!(fs::read(registry).unwrap(), original);
    }

    #[test]
    fn rejects_non_steam_folders() {
        let root = tempdir().unwrap();
        let other = root.path().join("Documents");
        fs::create_dir_all(&other).unwrap();

        assert!(matches!(
            add_root(&root.path().join(REGISTRY_FILE_NAME), &other),
            Err(RegistryError::InvalidLibrary)
        ));
    }
}
