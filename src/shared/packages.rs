use crate::error::Result;
use crate::shared::{FileSystemResource, InstallActionKind, Name, PackageKind, Platform};
use crate::APP_NAME;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Package is the unit of installation, provided by a configured `Installer`. It therefore has
/// a name, platform match, and package kind.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Package {
    #[serde(deserialize_with = "Name::deserialize")]
    name: Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    #[serde(default, skip_serializing_if = "is_default")]
    kind: PackageKind,
}

///
/// The kinds of actions a package set can perform; either a list of packages to install, *or* a
/// map of actions to script strings.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields, untagged, rename_all = "kebab-case")]
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

///
/// A Package set brings together a set of package actions, with additional actions such as linking
/// files, adding an env-file, and run before/after script strings.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct PackageSet {
    #[serde(skip)]
    path: PathBuf,
    #[serde(deserialize_with = "Name::deserialize")]
    name: Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    #[serde(default, skip_serializing_if = "is_default")]
    optional: bool,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    env_vars: HashMap<String, String>,
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

///
/// Package set groups are simply directories in the package repository.
///
#[derive(Clone, Debug)]
pub struct PackageSetGroup {
    path: PathBuf,
    package_sets: Vec<PackageSet>,
}

///
/// The package repository is a directory that contains package groups, which in turn contain
/// package sets.
#[derive(Clone, Debug)]
pub struct PackageRepository {
    path: PathBuf,
    package_set_groups: Vec<PackageSetGroup>,
}

///
/// The name of the repository directory.
///
pub const REPOSITORY_DIR: &str = "repository";

///
/// A trait implemented by things read from the file system.
pub trait Readable {
    fn read(path: &PathBuf) -> Result<Self>
    where
        Self: Sized;
}

///
/// A trait implemented by things that may be serialized to Writers.
///
pub trait Writeable<W: Write>: Serialize {
    fn write(&self, w: &mut W) -> Result<()> {
        serde_yaml::to_writer(w, self)?;
        Ok(())
    }
}

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
    /// Construct a new package instance.
    pub fn new(name: Name, platform: Option<Platform>, kind: PackageKind) -> Self {
        Self {
            name,
            platform,
            kind,
        }
    }

    /// Return this package's name.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return `true` if this package is intended for the current platform, else `false`.
    pub fn is_platform_match(&self) -> bool {
        Platform::CURRENT.is_match(&self.platform)
    }

    /// Return the platform this package is intended for, `None` implies all.
    pub fn platform(&self) -> Platform {
        self.platform.as_ref().cloned().unwrap_or_default()
    }

    /// Return the kind of installer required for this package.
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
    /// Return this package set's name.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return the path from which this package set was loaded.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Return the description of this package set, if one has been provided.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Return `true` if this package is intended for the current platform, else `false`.
    pub fn is_platform_match(&self) -> bool {
        Platform::CURRENT.is_match(&self.platform)
    }

    /// Return the platform this package is intended for, `None` implies all.
    pub fn platform(&self) -> Platform {
        self.platform.as_ref().cloned().unwrap_or_default()
    }

    /// Return `true` if this package set is optional, else `false`.
    pub fn is_optional(&self) -> bool {
        self.optional
    }

    /// Return any environment variables the package set has declared for use in script strings.
    pub fn env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
    }

    /// Return `true` if this package set has any actions, either package or script string.
    pub fn has_actions(&self) -> bool {
        !match &self.actions {
            PackageSetActions::Packages { packages } => packages.is_empty(),
            PackageSetActions::Scripts { scripts } => scripts.is_empty(),
        }
    }

    /// Return the actions required by this package set.
    pub fn actions(&self) -> &PackageSetActions {
        &self.actions
    }

    /// Return all the packages to install for this package set, or `None` if script strings have
    /// been provided instead.
    pub fn packages(&self) -> Option<impl Iterator<Item = &Package>> {
        match &self.actions {
            PackageSetActions::Packages { packages } => Some(packages.iter()),
            PackageSetActions::Scripts { .. } => None,
        }
    }

    /// Return all the script strings to execute for this package set, or `None` if packages have
    /// been provided instead.
    pub fn scripts(&self) -> Option<&HashMap<InstallActionKind, String>> {
        match &self.actions {
            PackageSetActions::Packages { .. } => None,
            PackageSetActions::Scripts { scripts } => Some(scripts),
        }
    }

    /// Return the name of an environment file to link, if one was provided.
    pub fn env_file(&self) -> &Option<String> {
        &self.env_file
    }

    /// Return the path to the environment file to link, if one was provided.
    pub fn env_file_path(&self) -> Option<PathBuf> {
        self.env_file.as_ref().map(PathBuf::from)
    }

    /// Return a map of file names to link.
    pub fn link_files(&self) -> &HashMap<String, String> {
        &self.link_files
    }

    /// Return a map of file path s to link.
    pub fn link_file_paths(&self) -> Vec<(PathBuf, PathBuf)> {
        self.link_files
            .iter()
            .map(|(src, tgt)| (self.path.join(src), PathBuf::from(tgt)))
            .collect()
    }

    /// Return the script string to run before any other action, if one was provided.
    pub fn run_before(&self) -> &Option<String> {
        &self.run_before
    }

    /// Return the script string to run after any other action, if one was provided.
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
    /// Return the name of this package set group, this is derived from the path of the group's
    /// directory.
    pub fn name(&self) -> Name {
        Name::from_str(&*self.path.file_name().unwrap().to_string_lossy())
            .expect("Invalid name format!")
    }

    /// Return a display name for this package set group, this is derived from the path of the
    /// group's directory with any numerif prefix removed and any '-' characters replaced with
    /// spaces.
    pub fn display_name(&self) -> String {
        let name = self.name().to_string();
        let captures = PSG_NAME.captures(&name);
        if let Some(captures) = captures {
            let name = captures.get(2).unwrap();
            name.as_str().to_string()
        } else {
            name.to_string()
        }
        .replace('-', " ")
    }

    /// Return the path to this package set group.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Return an iterator over all the package sets in this group.
    pub fn package_sets(&self) -> impl Iterator<Item = &PackageSet> {
        self.package_sets.iter()
    }

    /// Return `true` if this group has a package set named `name`, else `false`.
    pub fn has_package_set(&self, name: &Name) -> bool {
        self.package_set(name).is_some()
    }

    /// Return the package set named `name`, if one is present.
    pub fn package_set(&self, name: &Name) -> Option<&PackageSet> {
        self.package_sets.iter().find(|ps| &ps.name == name)
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
            path: repository_path,
            package_set_groups,
        })
    }
}

impl PackageRepository {
    /// Return the path to the configuration directory included in the repository.
    pub fn default_config_path() -> PathBuf {
        Self::default_path().join(".config")
    }

    /// Return the path to the local content directory included in the repository.
    pub fn default_local_path() -> PathBuf {
        Self::default_path().join(".local")
    }

    /// Return the path to the repository root directory.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Return `true` if the repository has no groups, else `false`.
    pub fn is_empty(&self) -> bool {
        self.package_set_groups.is_empty()
    }

    /// Return an iterator over all groups in this repository.
    pub fn groups(&self) -> impl Iterator<Item = &PackageSetGroup> {
        self.package_set_groups.iter()
    }

    /// Return `true` if this repository has a group named `name`, else `false`.
    pub fn has_group(&self, name: &Name) -> bool {
        self.group(name).is_some()
    }

    /// Return the group named `name`, if one is present.
    pub fn group(&self, name: &Name) -> Option<&PackageSetGroup> {
        self.package_set_groups
            .iter()
            .find(|psg| &psg.name() == name)
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
    use crate::shared::builders::Builder;
    use crate::shared::packages::PackageSetActions;
    use crate::shared::{
        InstallActionKind, Name, Package, PackageKind, PackageSet, PackageSetGroup, Platform,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    // ---------------------------------------------------------------------------------------------
    // Public Types
    // --------------------------------------------------------------------------------------------

    ///
    /// Provides a fluent interface for programmatic creation of [`Package`](../struct.package.html)
    /// instances.
    ///
    #[derive(Clone, Debug)]
    pub struct PackageBuilder(Package);

    ///
    /// Provides a fluent interface for programmatic creation of
    /// [`PackageSet`](../struct.packageset.html) instances.
    ///
    #[derive(Clone, Debug)]
    pub struct PackageSetBuilder(PackageSet);

    ///
    /// Provides a fluent interface for programmatic creation of
    /// [`PackageSetGroup`](../struct.packagesetgroup.html) instances.
    ///
    #[derive(Clone, Debug)]
    pub struct PackageSetGroupBuilder(PackageSetGroup);

    // --------------------------------------------------------------------------------------------
    // Implementations
    // --------------------------------------------------------------------------------------------

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

    impl Builder for PackageBuilder {
        type Inner = Package;

        fn build(&mut self) -> Self::Inner {
            self.0.clone()
        }
    }

    impl PackageBuilder {
        /// Create a new instance, all instances must be named.
        pub fn named(name: Name) -> Self {
            Self(Package {
                name,
                platform: None,
                kind: Default::default(),
            })
        }

        /// Adds a platform constraint, this package is only installed on the provided platform.
        pub fn for_platform(&mut self, platform: Platform) -> &mut Self {
            self.0.platform = Some(platform);
            self
        }

        /// Adds a platform constraint, this package is only installed on macos.
        pub fn for_macos_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// Adds a platform constraint, this package is only installed on linux.
        pub fn for_linux_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// This package has no platform constraint, it should install anywhere.
        pub fn for_any_platform(&mut self) -> &mut Self {
            self.0.platform = None;
            self
        }

        /// Sets the kind of package installer to use.
        pub fn of_kind(&mut self, kind: PackageKind) -> &mut Self {
            self.0.kind = kind;
            self
        }

        /// This package uses the platform's default installer
        pub fn using_default_installer(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Default)
        }

        /// This package uses the platform's application installer
        pub fn using_application_installer(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Application)
        }

        /// This package uses the specified language's installer
        pub fn using_language_installer(&mut self, language: &Name) -> &mut Self {
            self.of_kind(PackageKind::Language(language.clone()))
        }
    }

    // --------------------------------------------------------------------------------------------

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

    impl Builder for PackageSetBuilder {
        type Inner = PackageSet;

        fn build(&mut self) -> Self::Inner {
            self.0.clone()
        }
    }

    impl PackageSetBuilder {
        /// Create a new instance, all instances must be named.
        pub fn named(name: Name) -> Self {
            Self(PackageSet {
                path: Default::default(),
                name,
                description: None,
                platform: None,
                optional: false,
                env_vars: Default::default(),
                run_before: None,
                actions: Default::default(),
                env_file: None,
                link_files: Default::default(),
                run_after: None,
            })
        }

        /// Set the path that this package set was loaded from.
        pub fn path(&mut self, path: PathBuf) -> &mut Self {
            self.0.path = path;
            self
        }

        /// Add a description of this package set.
        pub fn description(&mut self, description: &str) -> &mut Self {
            self.0.description = Some(description.to_string());
            self
        }

        /// Adds a platform constraint, this package is only installed on the provided platform.
        pub fn for_platform(&mut self, platform: Platform) -> &mut Self {
            self.0.platform = Some(platform);
            self
        }

        /// Adds a platform constraint, this package is only installed on macos.
        pub fn for_macos_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// Adds a platform constraint, this package is only installed on linux.
        pub fn for_linux_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// This package has no platform constraint, it should install anywhere.
        pub fn for_any_platform(&mut self) -> &mut Self {
            self.0.platform = None;
            self
        }

        /// Make this an optional package set.
        pub fn optional(&mut self) -> &mut Self {
            self.0.optional = true;
            self
        }

        /// Make this a required (the default) package set.
        pub fn required(&mut self) -> &mut Self {
            self.0.optional = false;
            self
        }

        /// Set the key/values to use as additional tool/environment variables.
        pub fn env_vars(&mut self, env_vars: HashMap<String, String>) -> &mut Self {
            self.0.env_vars = env_vars;
            self
        }

        /// Add a new tool/environment variable.
        pub fn env_var(&mut self, key: &str, value: &str) -> &mut Self {
            let _ = self.0.env_vars.insert(key.to_string(), value.to_string());
            self
        }

        /// Add a run-before script string.
        pub fn run_before(&mut self, script_string: &str) -> &mut Self {
            self.0.run_before = Some(script_string.to_string());
            self
        }

        /// Set the set of actions, whether package or script.
        pub fn actions(&mut self, actions: PackageSetActions) -> &mut Self {
            self.0.actions = actions;
            self
        }

        /// This sets the internal actions to expect packages, not script strings.
        pub fn with_package_actions(&mut self) -> &mut Self {
            self.actions(PackageSetActions::Packages {
                packages: Default::default(),
            })
        }

        /// Set the list of packages, this is not additive.
        pub fn package_actions(&mut self, packages: &[Package]) -> &mut Self {
            self.actions(PackageSetActions::Packages {
                packages: packages.to_vec(),
            })
        }

        /// Add a package to the list of packages, this is additive.
        pub fn add_package_action(&mut self, package: Package) -> Result<&mut Self> {
            match &mut self.0.actions {
                PackageSetActions::Packages { packages } => {
                    packages.push(package);
                    Ok(self)
                }
                PackageSetActions::Scripts { .. } => Err(ErrorKind::InvalidBuilderState.into()),
            }
        }

        /// This sets the internal actions to expect script strings, not packages.
        pub fn with_script_actions(&mut self) -> &mut Self {
            self.actions(PackageSetActions::Scripts {
                scripts: Default::default(),
            })
        }

        /// Set the map of script strings from the array of tuples, this is not additive.
        pub fn script_actions_list(
            &mut self,
            scripts: &[(InstallActionKind, String)],
        ) -> &mut Self {
            self.script_actions(scripts.iter().cloned().collect())
        }

        /// Set the map of script strings, this is not additive.
        pub fn script_actions(&mut self, scripts: HashMap<InstallActionKind, String>) -> &mut Self {
            self.actions(PackageSetActions::Scripts { scripts })
        }

        /// Add a specific script string for the given action.
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

        /// Add a specific script string for the install action.
        pub fn add_install_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Install, script_string)
        }

        /// Add a specific script string for the update action.
        pub fn add_update_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Update, script_string)
        }

        /// Add a specific script string for the uninstall action.
        pub fn add_uninstall_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::Uninstall, script_string)
        }

        /// Add a specific script string for the link-files action.
        pub fn add_link_files_script_action(&mut self, script_string: &str) -> Result<&mut Self> {
            self.add_script_action(InstallActionKind::LinkFiles, script_string)
        }

        /// Set the name of a file to be treated as an 'env-file'.
        pub fn env_file(&mut self, file_name: &str) -> &mut Self {
            self.0.env_file = Some(file_name.to_string());
            self
        }

        /// Set the map of source to target link files.
        pub fn link_files(&mut self, link_files: HashMap<String, String>) -> &mut Self {
            self.0.link_files = link_files;
            self
        }

        /// Add a source and target to the map of link files
        pub fn add_link_file(&mut self, repo_file_name: &str, local_fs_name: &str) -> &mut Self {
            let _ = self
                .0
                .link_files
                .insert(repo_file_name.to_string(), local_fs_name.to_string());
            self
        }

        /// Add a run-after script string.
        pub fn run_after(&mut self, script_string: &str) -> &mut Self {
            self.0.run_after = Some(script_string.to_string());
            self
        }
    }
    // --------------------------------------------------------------------------------------------

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

    impl Builder for PackageSetGroupBuilder {
        type Inner = PackageSetGroup;

        fn build(&mut self) -> Self::Inner {
            self.0.clone()
        }
    }

    impl PackageSetGroupBuilder {
        /// Create a new instance with the given source path.
        pub fn new_in(path: PathBuf) -> Self {
            Self(PackageSetGroup {
                path,
                package_sets: vec![],
            })
        }

        /// Add all package sets to the group, this is not additive.
        pub fn package_sets(&mut self, package_sets: &[PackageSet]) {
            self.0.package_sets = package_sets.to_vec()
        }

        /// Add a package set to the group, this is additive.
        pub fn add_package_set(&mut self, package_set: PackageSet) {
            self.0.package_sets.push(package_set)
        }
    }
}
