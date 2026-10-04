use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::crossover;

pub const REGISTRY_FILE_NAME: &str = "crossover-bottles.json";
const REGISTRY_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("cannot read CrossOver bottle registry ({0})")]
    Read(std::io::ErrorKind),
    #[error("CrossOver bottle registry is malformed or has an unsupported version")]
    Invalid,
    #[error("selected folder is not a CrossOver bottle with Steam Wizard101 app 799960")]
    InvalidBottle,
    #[error("cannot save CrossOver bottle registry ({0})")]
    Write(std::io::ErrorKind),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct Registry {
    version: u32,
    bottle_roots: Vec<PathBuf>,
}

pub fn default_registry_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "msork", "WizRust101-RPC")
        .map(|dirs| dirs.data_dir().join(REGISTRY_FILE_NAME))
}

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
        .bottle_roots
        .into_iter()
        .filter(|root| crossover::is_bottle(root) && crossover::contains_wizard101_install(root))
        .collect())
}

pub fn add_root(path: &Path, root: &Path) -> Result<bool, RegistryError> {
    if !crossover::is_bottle(root) || !crossover::contains_wizard101_install(root) {
        return Err(RegistryError::InvalidBottle);
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
    let data = Registry {
        version: REGISTRY_VERSION,
        bottle_roots: roots,
    };
    let contents = serde_json::to_vec_pretty(&data).map_err(|_| RegistryError::Invalid)?;
    let parent = path.parent().ok_or(RegistryError::Invalid)?;
    fs::create_dir_all(parent).map_err(|error| RegistryError::Write(error.kind()))?;
    let temporary = path.with_extension(format!("json.{}.tmp", std::process::id()));
    fs::write(&temporary, contents).map_err(|error| RegistryError::Write(error.kind()))?;
    fs::rename(&temporary, path).map_err(|error| RegistryError::Write(error.kind()))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;

    use super::*;

    fn wizard_bottle(root: &Path) -> PathBuf {
        let bottle = root.join("Bottle");
        let steamapps = bottle.join("drive_c/Program Files (x86)/Steam/steamapps");
        fs::create_dir_all(&steamapps).unwrap();
        fs::write(
            steamapps.join("appmanifest_799960.acf"),
            "\"AppState\" { \"appid\" \"799960\" \"installdir\" \"Wizard101\" }",
        )
        .unwrap();
        bottle
    }

    #[test]
    fn persists_only_a_valid_cross_over_bottle_and_deduplicates_it() {
        let temp = tempdir().unwrap();
        let bottle = wizard_bottle(temp.path());
        let registry = temp.path().join(REGISTRY_FILE_NAME);

        assert!(add_root(&registry, &bottle).unwrap());
        assert!(!add_root(&registry, &bottle).unwrap());
        assert_eq!(load_roots(&registry).unwrap(), [bottle]);
    }

    #[test]
    fn rejects_a_folder_without_a_cross_over_wizard101_install() {
        let temp = tempdir().unwrap();
        let folder = temp.path().join("Documents");
        fs::create_dir_all(&folder).unwrap();

        assert!(matches!(
            add_root(&temp.path().join(REGISTRY_FILE_NAME), &folder),
            Err(RegistryError::InvalidBottle)
        ));
    }

    #[test]
    fn malformed_registry_is_left_unchanged_on_add() {
        let temp = tempdir().unwrap();
        let bottle = wizard_bottle(temp.path());
        let registry = temp.path().join(REGISTRY_FILE_NAME);
        let original = br#"{"version":2,"bottle_roots":[]}"#;
        fs::write(&registry, original).unwrap();

        assert!(matches!(
            add_root(&registry, &bottle),
            Err(RegistryError::Invalid)
        ));
        assert_eq!(fs::read(registry).unwrap(), original);
    }
}
