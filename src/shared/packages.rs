/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::shared::{InstallActionKind, PackageKind, Platform};
use crate::APP_NAME;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    #[serde(default, skip_serializing_if = "is_default")]
    kind: PackageKind,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum PackageSetActions {
    Packages {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        packages: Vec<Package>,
    },
    Scripts {
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        scripts: HashMap<InstallActionKind, String>,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackageSet {
    #[serde(skip)]
    path: PathBuf,
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    optional: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    run_before: Option<String>,
    #[serde(default, skip_serializing_if = "PackageSetActions::is_empty")]
    actions: PackageSetActions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    env_file: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    link_files: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    run_after: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PackageSetGroup {
    path: PathBuf,
    name: String,
    package_sets: Vec<PackageSet>,
}

#[derive(Clone, Debug)]
pub struct PackageRepository {
    path: PathBuf,
    package_set_groups: Vec<PackageSetGroup>,
}

pub const REPOSITORY_DIR: &str = "repository";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for PackageSetActions {
    fn default() -> Self {
        Self::Packages {
            packages: Default::default(),
        }
    }
}

impl From<Vec<Package>> for PackageSetActions {
    fn from(packages: Vec<Package>) -> Self {
        Self::Packages { packages }
    }
}

impl PackageSetActions {
    pub fn is_empty(&self) -> bool {
        match self {
            PackageSetActions::Packages { packages } => packages.is_empty(),
            PackageSetActions::Scripts { scripts } => scripts.is_empty(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Package {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_platform_match(&self) -> bool {
        Platform::CURRENT.is_match(&self.platform)
    }

    pub fn platform(&self) -> Platform {
        self.platform.as_ref().cloned().unwrap_or_default()
    }

    pub fn kind(&self) -> &PackageKind {
        &self.kind
    }
}

// ------------------------------------------------------------------------------------------------

impl PackageSet {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn packages(&self) -> Option<impl Iterator<Item = &Package>> {
        match &self.actions {
            PackageSetActions::Packages { packages } => Some(packages.iter()),
            PackageSetActions::Scripts { .. } => None,
        }
    }

    pub fn scripts(&self) -> Option<&HashMap<InstallActionKind, String>> {
        match &self.actions {
            PackageSetActions::Packages { .. } => None,
            PackageSetActions::Scripts { scripts } => Some(scripts),
        }
    }

    pub fn env_file(&self) -> Option<PathBuf> {
        self.env_file.as_ref().map(|f| PathBuf::from(f))
    }

    pub fn link_files(&self) -> Vec<(PathBuf, PathBuf)> {
        self.link_files
            .iter()
            .map(|(src, tgt)| (self.path.join(src), PathBuf::from(tgt)))
            .collect()
    }

    pub fn run_before(&self) -> &Option<String> {
        &self.run_before
    }

    pub fn run_after(&self) -> &Option<String> {
        &self.run_after
    }

    pub fn read(path: &PathBuf) -> Result<Self> {
        debug!("PackageSet::read: reading package set file {:?}", path);
        let value = std::fs::read_to_string(path)?;
        let mut result: PackageSet = serde_yaml::from_str(&value)?;
        result.path = path.clone();
        trace!("read package_set: {:?}", result);
        Ok(result)
    }
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref PSG_NAME: Regex = Regex::new(r#"^([0-9]+\-)?(.*)$"#).unwrap();
}

impl PackageSetGroup {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn package_sets(&self) -> impl Iterator<Item = &PackageSet> {
        self.package_sets.iter()
    }

    pub fn has_package_set(&self, name: &str) -> bool {
        self.package_set(name).is_some()
    }

    pub fn package_set(&self, name: &str) -> Option<&PackageSet> {
        self.package_sets.iter().find(|ps| ps.name == name)
    }

    pub fn read(path: &PathBuf) -> Result<Self> {
        debug!("PackageSetGroup::read: reading dir {:?}", path);
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let captures = PSG_NAME.captures(&name);
        let name = if let Some(captures) = captures {
            let name = captures.get(2).unwrap();
            name.as_str().to_string()
        } else {
            name
        }
        .replace('-', " ");
        let mut group = PackageSetGroup {
            name,
            path: path.clone(),
            package_sets: Default::default(),
        };
        let yaml_extension = OsStr::new("yml");

        for dir_entry in read_dir(path)? {
            let set_path = dir_entry?.path();
            // Option 1. Any file in this directory, "*.yml" that is package-set itself.
            if set_path.is_file() && set_path.extension() == Some(yaml_extension) {
                let _ = group.package_sets.push(PackageSet::read(&set_path)?);
            }
            // Option 2. A directory, which contains a file named "package-set.yml"
            else if set_path.is_dir() {
                let set_path = set_path.join("package-set.yml");
                if set_path.is_file() {
                    let _ = group.package_sets.push(PackageSet::read(&set_path)?);
                }
            } else {
                debug!("PackageSetGroup::read: ignoring {:?}", set_path);
            }
        }
        group.package_sets.sort_by_key(|ps| ps.name.clone());
        Ok(group)
    }
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RESERVED_REPO_NAMES: Vec<&'static str> = vec![".git", ".config", ".local"];
}

impl PackageRepository {
    pub fn default_path() -> PathBuf {
        xdirs::config_dir_for(APP_NAME)
            .unwrap()
            .join(REPOSITORY_DIR)
    }

    pub fn default_config_path() -> PathBuf {
        Self::default_path().join(".config")
    }

    pub fn default_local_path() -> PathBuf {
        Self::default_path().join(".local")
    }

    pub fn open() -> Result<Self> {
        Self::actual_open(Self::default_path())
    }

    pub fn open_from(repository_root: PathBuf) -> Result<Self> {
        let base = current_dir().unwrap();
        Self::actual_open(base.join(repository_root))
    }

    fn actual_open(repository_path: PathBuf) -> Result<Self> {
        info!(
            "PackageRepository::actual_open: reading all package data from {:?}",
            &repository_path
        );
        let mut package_set_groups: Vec<PackageSetGroup> = Default::default();
        for dir_entry in read_dir(&repository_path)? {
            let group_path = dir_entry?.path();
            if group_path.is_dir() {
                trace!(
                    "PackageRepository::actual_open: found possible group dir {:?} -> {:?}",
                    &group_path,
                    group_path.file_name(),
                );
                let dir_name = group_path.file_name().unwrap().to_str().unwrap();
                if RESERVED_REPO_NAMES.contains(&dir_name) {
                    debug!(
                        "PackageRepository::actual_open: some files are always ignored ({:?}).",
                        group_path
                    );
                } else {
                    package_set_groups.push(PackageSetGroup::read(&group_path)?);
                }
            }
        }
        package_set_groups.sort_by_key(|psg| psg.name.clone());
        Ok(PackageRepository {
            path: repository_path.clone(),
            package_set_groups,
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_empty(&self) -> bool {
        self.package_set_groups.is_empty()
    }

    pub fn groups(&self) -> impl Iterator<Item = &PackageSetGroup> {
        self.package_set_groups.iter()
    }

    pub fn has_group(&self, name: &str) -> bool {
        self.group(name).is_some()
    }

    pub fn group(&self, name: &str) -> Option<&PackageSetGroup> {
        self.package_set_groups.iter().find(|psg| psg.name == name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packages() {
        let config_str = r##"
        name: lux
        env-file: sample.env
        actions:
          packages:
            - name: lux
              kind: 
                language: python
        link-files:
          set-lux: "{{local-bin}}/set-lux"
        "##;

        let config: PackageSet = serde_yaml::from_str(config_str).unwrap();
        println!("{:?}", config);
    }

    #[test]
    fn test_parse_scripts() {
        let config_str = r##"
        name: lux
        env-file: sample.env
        actions:
          scripts:
            install: install-lux
            uninstall: uninstall-lux 
        link-files:
          set-lux: "{{local-bin}}/set-lux"
        "##;

        let config: PackageSet = serde_yaml::from_str(config_str).unwrap();
        println!("{:?}", config);
    }

    #[test]
    fn test_packages_to_string() {
        let config = PackageSet {
            path: PathBuf::default(),
            name: "lux".to_string(),
            description: None,
            optional: false,
            env_file: Some("sample.env".to_string()),
            actions: PackageSetActions::Packages {
                packages: vec![Package {
                    name: "lux".to_string(),
                    platform: None,
                    kind: PackageKind::Language("python".to_string()),
                }],
            },
            link_files: vec![("set-lux".to_string(), "{{local-bin}}/set-lux".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            run_before: None,
            run_after: None,
        };
        let config_str = serde_yaml::to_string(&config).unwrap();
        println!("{}", config_str);
    }

    #[test]
    fn test_scripts_to_string() {
        let config = PackageSet {
            path: PathBuf::default(),
            name: "lux".to_string(),
            description: None,
            optional: false,
            env_file: Some("sample.env".to_string()),
            actions: PackageSetActions::Scripts {
                scripts: vec![
                    (InstallActionKind::Install, "install-lux".to_string()),
                    (InstallActionKind::Uninstall, "uninstall-lux".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            link_files: vec![("set-lux".to_string(), "{{local-bin}}/set-lux".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            run_before: None,
            run_after: None,
        };
        let config_str = serde_yaml::to_string(&config).unwrap();
        println!("{}", config_str);
    }
}
