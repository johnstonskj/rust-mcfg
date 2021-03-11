/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::{PackageKind, Platform};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kind: Option<PackageKind>,
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
    env_file: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    packages: Vec<Package>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    link_files: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    run_before: Option<String>,
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

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
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

    pub fn kind(&self) -> PackageKind {
        self.kind.as_ref().cloned().unwrap_or_default()
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

    pub fn packages(&self) -> impl Iterator<Item = &Package> {
        self.packages.iter()
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
        let mut group = PackageSetGroup {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
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

impl PackageRepository {
    pub fn open(env: &Environment) -> Result<Self> {
        info!(
            "PackageRepository::open: reading all package data from {:?}",
            env.repository_path()
        );
        let mut package_set_groups: Vec<PackageSetGroup> = Default::default();
        for dir_entry in read_dir(env.repository_path())? {
            let group_path = dir_entry?.path();
            if group_path.is_dir() {
                if group_path.to_string_lossy().to_string().ends_with("/.git") {
                    debug!(
                        "PackageRepository::open: some files are always ignored ({:?}).",
                        group_path
                    );
                } else {
                    package_set_groups.push(PackageSetGroup::read(&group_path)?);
                }
            }
        }
        package_set_groups.sort_by_key(|psg| psg.name.clone());
        Ok(PackageRepository {
            path: env.repository_path().clone(),
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
    fn test_parse() {
        let config_str = r##"
        name: lux
        env-file: sample.env
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
    fn test_to_string() {
        let config = PackageSet {
            path: PathBuf::default(),
            name: "lux".to_string(),
            description: None,
            optional: false,
            env_file: Some("sample.env".to_string()),
            packages: vec![Package {
                name: "lux".to_string(),
                platform: None,
                kind: Some(PackageKind::Language("python".to_string())),
            }],
            link_files: vec![("set-lux".to_string(), "{{local-bin}}/set-lux".to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
            run_before: None,
            run_after: None,
        };

        let config_str = serde_yaml::to_string(&config).unwrap();
        println!("{:?}", config_str);
    }
}
