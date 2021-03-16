/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::shared::{FileSystemResource, InstallActionKind, PackageKind, Platform};
use crate::APP_NAME;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::Write;
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
    package_sets: Vec<PackageSet>,
}

#[derive(Clone, Debug)]
pub struct PackageRepository {
    path: PathBuf,
    package_set_groups: Vec<PackageSetGroup>,
}

pub const REPOSITORY_DIR: &str = "repository";

pub trait Readable {
    fn read(path: &PathBuf) -> Result<Self>
    where
        Self: Sized;
}

pub trait Writeable<W: Write>: Serialize {
    fn write(&self, w: &mut W) -> Result<()> {
        serde_yaml::to_writer(w, self)?;
        Ok(())
    }
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

impl<W: Write> Writeable<W> for Package {}

impl Package {
    pub fn new(name: String, platform: Option<Platform>, kind: PackageKind) -> Self {
        Self {
            name,
            platform,
            kind,
        }
    }

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

impl Readable for PackageSet {
    fn read(path: &PathBuf) -> Result<Self> {
        debug!("PackageSet::read: reading package set file {:?}", path);
        let value = std::fs::read_to_string(path)?;
        let mut result: PackageSet = serde_yaml::from_str(&value)?;
        result.path = path.clone();
        trace!("read package_set: {:?}", result);
        Ok(result)
    }
}

impl<W: Write> Writeable<W> for PackageSet {}

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

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn has_actions(&self) -> bool {
        !match &self.actions {
            PackageSetActions::Packages { packages } => packages.is_empty(),
            PackageSetActions::Scripts { scripts } => scripts.is_empty(),
        }
    }

    pub fn actions(&self) -> &PackageSetActions {
        &self.actions
    }

    pub fn has_package_actions(&self) -> bool {
        match &self.actions {
            PackageSetActions::Packages { .. } => true,
            PackageSetActions::Scripts { .. } => false,
        }
    }

    pub fn packages(&self) -> Option<impl Iterator<Item = &Package>> {
        match &self.actions {
            PackageSetActions::Packages { packages } => Some(packages.iter()),
            PackageSetActions::Scripts { .. } => None,
        }
    }

    pub fn has_script_actions(&self) -> bool {
        match &self.actions {
            PackageSetActions::Packages { .. } => false,
            PackageSetActions::Scripts { .. } => true,
        }
    }

    pub fn scripts(&self) -> Option<&HashMap<InstallActionKind, String>> {
        match &self.actions {
            PackageSetActions::Packages { .. } => None,
            PackageSetActions::Scripts { scripts } => Some(scripts),
        }
    }

    pub fn env_file(&self) -> &Option<String> {
        &self.env_file
    }

    pub fn env_file_path(&self) -> Option<PathBuf> {
        self.env_file.as_ref().map(|f| PathBuf::from(f))
    }

    pub fn link_files(&self) -> &HashMap<String, String> {
        &self.link_files
    }

    pub fn link_file_paths(&self) -> Vec<(PathBuf, PathBuf)> {
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
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref PSG_NAME: Regex = Regex::new(r#"^([0-9]+\-)?(.*)$"#).unwrap();
}

impl Readable for PackageSetGroup {
    fn read(path: &PathBuf) -> Result<Self> {
        debug!("PackageSetGroup::read: reading dir {:?}", path);
        let mut group = PackageSetGroup {
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
        group.package_sets.sort_by_key(|ps| ps.name().clone());
        Ok(group)
    }
}

impl PackageSetGroup {
    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_string_lossy().to_string()
    }

    pub fn display_name(&self) -> String {
        let name = self.name();
        let captures = PSG_NAME.captures(&name);
        if let Some(captures) = captures {
            let name = captures.get(2).unwrap();
            name.as_str().to_string()
        } else {
            name.clone()
        }
        .replace('-', " ")
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
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RESERVED_REPO_NAMES: Vec<&'static str> = vec![".git", ".config", ".local"];
}

impl FileSystemResource for PackageRepository {
    fn default_path() -> PathBuf {
        xdirs::config_dir_for(APP_NAME)
            .unwrap()
            .join(REPOSITORY_DIR)
    }

    fn open_from(repository_path: PathBuf) -> Result<Self> {
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
        package_set_groups.sort_by_key(|psg| psg.name());
        Ok(PackageRepository {
            path: repository_path.clone(),
            package_set_groups,
        })
    }
}

impl PackageRepository {
    pub fn default_config_path() -> PathBuf {
        Self::default_path().join(".config")
    }

    pub fn default_local_path() -> PathBuf {
        Self::default_path().join(".local")
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
        self.package_set_groups
            .iter()
            .find(|psg| psg.name() == name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub mod builders {
    use crate::error::{ErrorKind, Result};
    use crate::shared::packages::PackageSetActions;
    use crate::shared::{
        InstallActionKind, Package, PackageKind, PackageSet, PackageSetGroup, Platform,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    // ------------------------------------------------------------------------------------------------
    // Public Types
    // ------------------------------------------------------------------------------------------------

    #[derive(Clone, Debug)]
    pub struct PackageBuilder(Package);

    #[derive(Clone, Debug)]
    pub struct PackageSetBuilder(PackageSet);

    #[derive(Clone, Debug)]
    pub struct PackageSetGroupBuilder(PackageSetGroup);

    // ------------------------------------------------------------------------------------------------
    // Private Types
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Public Functions
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Implementations
    // ------------------------------------------------------------------------------------------------

    impl From<Package> for PackageBuilder {
        fn from(package: Package) -> Self {
            Self(package)
        }
    }

    impl From<PackageBuilder> for Package {
        fn from(builder: PackageBuilder) -> Self {
            builder.0
        }
    }

    impl PackageBuilder {
        pub fn named(name: &str) -> Self {
            Self(Package {
                name: name.to_string(),
                platform: None,
                kind: Default::default(),
            })
        }

        pub fn for_platform(&mut self, platform: Platform) -> &mut Self {
            self.0.platform = Some(platform);
            self
        }

        pub fn for_macos_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        pub fn for_linux_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        pub fn for_any_platform(&mut self) -> &mut Self {
            self.0.platform = None;
            self
        }

        pub fn of_kind(&mut self, kind: PackageKind) -> &mut Self {
            self.0.kind = kind;
            self
        }

        pub fn using_default_installer(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Default)
        }

        pub fn using_application_installer(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Application)
        }

        pub fn using_language_installer(&mut self, language: &str) -> &mut Self {
            self.of_kind(PackageKind::Language(language.to_string()))
        }

        pub fn build(&self) -> Package {
            self.0.clone()
        }
    }

    // ------------------------------------------------------------------------------------------------

    impl From<PackageSet> for PackageSetBuilder {
        fn from(package: PackageSet) -> Self {
            Self(package)
        }
    }

    impl From<PackageSetBuilder> for PackageSet {
        fn from(builder: PackageSetBuilder) -> Self {
            builder.0
        }
    }

    impl PackageSetBuilder {
        pub fn named(name: &str) -> Self {
            Self(PackageSet {
                path: Default::default(),
                name: name.to_string(),
                description: None,
                optional: false,
                run_before: None,
                actions: Default::default(),
                env_file: None,
                link_files: Default::default(),
                run_after: None,
            })
        }

        pub fn path(&mut self, path: PathBuf) -> &mut Self {
            self.0.path = path;
            self
        }

        pub fn description(&mut self, description: &str) -> &mut Self {
            self.0.description = Some(description.to_string());
            self
        }

        pub fn optional(&mut self) -> &mut Self {
            self.0.optional = true;
            self
        }

        pub fn required(&mut self) -> &mut Self {
            self.0.optional = false;
            self
        }

        pub fn run_before(&mut self, script_string: &str) -> &mut Self {
            self.0.run_before = Some(script_string.to_string());
            self
        }

        pub fn actions(&mut self, actions: PackageSetActions) -> &mut Self {
            self.0.actions = actions;
            self
        }

        pub fn with_package_actions(&mut self) -> &mut Self {
            self.actions(PackageSetActions::Packages {
                packages: Default::default(),
            })
        }

        pub fn package_actions(&mut self, packages: &[Package]) -> &mut Self {
            self.actions(PackageSetActions::Packages {
                packages: packages.to_vec(),
            })
        }

        pub fn add_package_action(&mut self, package: Package) -> Result<&mut Self> {
            match &mut self.0.actions {
                PackageSetActions::Packages { packages } => {
                    packages.push(package);
                    Ok(self)
                }
                PackageSetActions::Scripts { .. } => Err(ErrorKind::InvalidBuilderState.into()),
            }
        }

        pub fn with_script_actions(&mut self) -> &mut Self {
            self.actions(PackageSetActions::Scripts {
                scripts: Default::default(),
            })
        }

        pub fn script_actions_list(
            &mut self,
            scripts: &[(InstallActionKind, String)],
        ) -> &mut Self {
            self.script_actions(scripts.into_iter().cloned().collect())
        }

        pub fn script_actions(&mut self, scripts: HashMap<InstallActionKind, String>) -> &mut Self {
            self.actions(PackageSetActions::Scripts { scripts })
        }

        pub fn add_script_action(
            &mut self,
            kind: InstallActionKind,
            script_string: &str,
        ) -> Result<&mut Self> {
            match &mut self.0.actions {
                PackageSetActions::Packages { .. } => Err(ErrorKind::InvalidBuilderState.into()),
                PackageSetActions::Scripts { scripts } => {
                    let _ = scripts.insert(kind, script_string.to_string());
                    Ok(self)
                }
            }
        }

        pub fn add_install_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Install, script_string)
        }

        pub fn add_update_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Update, script_string)
        }

        pub fn add_uninstall_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Uninstall, script_string)
        }

        pub fn add_link_files_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::LinkFiles, script_string)
        }

        pub fn env_file(&mut self, file_name: &str) -> &mut Self {
            self.0.env_file = Some(file_name.to_string());
            self
        }

        pub fn link_files(&mut self, link_files: HashMap<String, String>) -> &mut Self {
            self.0.link_files = link_files;
            self
        }

        pub fn add_link_file(&mut self, repo_file_name: &str, local_fs_name: &str) -> &mut Self {
            let _ = self
                .0
                .link_files
                .insert(repo_file_name.to_string(), local_fs_name.to_string());
            self
        }

        pub fn run_after(&mut self, script_string: &str) -> &mut Self {
            self.0.run_after = Some(script_string.to_string());
            self
        }

        pub fn build(&mut self) -> PackageSet {
            self.0.clone()
        }
    }
    // ------------------------------------------------------------------------------------------------

    impl Default for PackageSetGroupBuilder {
        fn default() -> Self {
            Self(PackageSetGroup {
                path: Default::default(),
                package_sets: Default::default(),
            })
        }
    }

    impl From<PackageSetGroup> for PackageSetGroupBuilder {
        fn from(package: PackageSetGroup) -> Self {
            Self(package)
        }
    }

    impl From<PackageSetGroupBuilder> for PackageSetGroup {
        fn from(builder: PackageSetGroupBuilder) -> Self {
            builder.0
        }
    }

    impl PackageSetGroupBuilder {
        pub fn new_in(path: PathBuf) -> Self {
            Self(PackageSetGroup {
                path,
                package_sets: vec![],
            })
        }

        pub fn path(&mut self, path: PathBuf) -> &mut Self {
            self.0.path = path;
            self
        }

        pub fn package_sets(&mut self, package_sets: &[PackageSet]) {
            self.0.package_sets = package_sets.to_vec()
        }

        pub fn add_package_set(&mut self, package_set: PackageSet) {
            self.0.package_sets.push(package_set)
        }
    }

    // ------------------------------------------------------------------------------------------------
    // Private Functions
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Modules
    // ------------------------------------------------------------------------------------------------
}
