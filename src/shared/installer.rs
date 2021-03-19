use crate::error::{ErrorKind, Result};
use crate::shared::command::execute_shell_command;
use crate::shared::env::{
    add_action_vars, add_package_action_vars, add_package_set_action_vars, default_vars,
};
use crate::shared::install_log::{InstalledPackage, PackageLog};
use crate::shared::packages::{Package, PackageRepository, PackageSet, PackageSetGroup};
use crate::shared::{FileSystemResource, Name, PackageKind, Platform};
use crate::APP_NAME;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An action that may be taken by an installer. These are set and passed through by a client such
/// as the CLI to denote the action to take.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum InstallActionKind {
    #[allow(missing_docs)]
    Install,
    #[allow(missing_docs)]
    Update,
    #[allow(missing_docs)]
    Uninstall,
    #[allow(missing_docs)]
    LinkFiles,
}

///
/// This holds the configuration regarding a single installer type, these can be platform-specific
/// or not, and are defined to handle one kind of `PackageKind`. These instances are a part of the
/// `InstallerRegistry` and loaded from a single file.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Installer {
    #[serde(deserialize_with = "Name::deserialize")]
    name: Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    kind: PackageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    if_exists: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    commands: HashMap<InstallActionKind, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    update_self: Option<String>,
}

///
/// The installer registry is a file that contains a list of `Installer` configurations. This is
/// also the interface for installer actions such as install, update, uninstall.
///
#[derive(Clone, Debug)]
pub struct InstallerRegistry {
    installers: HashMap<(Platform, PackageKind), Installer>,
}

///
/// The registry file name.
///
pub const REGISTRY_FILE: &str = "installers.yml";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for InstallActionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InstallActionKind::Install => "install",
                InstallActionKind::Update => "update",
                InstallActionKind::Uninstall => "uninstall",
                InstallActionKind::LinkFiles => "link",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Installer {
    /// Return the name of this installer.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Return `true` if this installer is a match for the current platform, else `false`.
    pub fn is_platform_match(&self) -> bool {
        Platform::CURRENT.is_match(&self.platform)
    }

    /// Return `true` if the installer has a specified `if_exists` value, and if that path exists.
    pub fn if_exists_match(&self) -> bool {
        match &self.if_exists {
            None => true,
            Some(path) => PathBuf::from(path).exists(),
        }
    }

    /// Return the platform specification for this installer.
    pub fn platform(&self) -> Platform {
        self.platform.as_ref().cloned().unwrap_or_default()
    }

    /// Return the package kind specification for this installer.
    pub fn kind(&self) -> PackageKind {
        self.kind.clone()
    }

    /// Return the map of commands for this installer.
    pub fn commands(&self) -> &HashMap<InstallActionKind, String> {
        &self.commands
    }

    /// Return the command for the specific action kind.
    pub fn command_for(&self, kind: &InstallActionKind) -> Option<&String> {
        self.commands.get(kind)
    }

    /// Return `true` if this installer supports updating itself.
    pub fn has_update_self(&self) -> bool {
        self.update_self.is_some()
    }

    /// Return the update self command for this installer.
    pub fn update_self(&self) -> &Option<String> {
        &self.update_self
    }

    fn package_action(
        &self,
        action: &InstallActionKind,
        package: &Package,
        variable_replacements: &HashMap<String, String>,
    ) -> Result<()> {
        if self.is_platform_match() && package.is_platform_match() {
            if self.kind() == *package.kind() {
                let cmd = self.commands.get(&action);
                if let Some(cmd_str) = cmd {
                    reportln!(
                        "* performing {} on {} package {}",
                        action,
                        &self.name,
                        package.name()
                    );
                    execute_shell_command(cmd_str, variable_replacements)?;
                } else {
                    info!("installer has no command for action {:?}", action);
                }
                Ok(())
            } else {
                // One hopes we don't get here.
                error!("Installer::install: the package isn't meant for this installer.");
                Err(ErrorKind::WrongInstallerForKind(self.kind.clone()).into())
            }
        } else {
            // It is not an error as a package set may include different packages per platform.
            warn!(
                "Installer::install: ignoring package '{}', not applicable for platform '{:?}'",
                package.name(),
                Platform::CURRENT
            );
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Into<Vec<Installer>> for InstallerRegistry {
    fn into(self) -> Vec<Installer> {
        let mut inner = self.installers;
        inner.drain().map(|(_, v)| v).collect()
    }
}

impl From<Vec<Installer>> for InstallerRegistry {
    fn from(installers: Vec<Installer>) -> Self {
        let mut registry = Self {
            installers: Default::default(),
        };
        for installer in installers {
            let key = (
                installer
                    .platform
                    .as_ref()
                    .cloned()
                    .unwrap_or(Platform::Macos),
                installer.kind.clone(),
            );
            debug!("InstallerRegistry::from: config for installer {:?}", key);
            let result = registry.installers.insert(key, installer);
            if result.is_some() {
                debug!(
                    "InstallerRegistry::from: key is a duplicate, previous value was overwritten"
                );
            }
        }
        registry
    }
}

impl FileSystemResource for InstallerRegistry {
    fn default_path() -> PathBuf {
        xdirs::config_dir_for(APP_NAME).unwrap().join(REGISTRY_FILE)
    }

    fn open_from(registry_file: PathBuf) -> Result<Self> {
        info!("InstallerRegistry::read loading from {:?}", registry_file);
        let registry_data = read_to_string(registry_file)?;
        let installers: Vec<Installer> = serde_yaml::from_str(&registry_data)?;
        debug!(
            "InstallerRegistry::read: fetched {} installers from registry",
            installers.len()
        );

        let (keep, discard): (Vec<Installer>, Vec<Installer>) = installers
            .into_iter()
            .partition(|i| i.is_platform_match() && i.if_exists_match());
        for discarded in discard {
            info!(
                "InstallerRegistry::read: discarding installer {}, not a platform match, or 'if_exist' check failed",
                discarded.name()
            )
        }

        Ok(Self::from(keep))
    }
}

impl InstallerRegistry {
    /// Return `true` if this registry contains no installer specifications, else `false`..
    pub fn is_empty(&self) -> bool {
        self.installers.is_empty()
    }

    /// Return an iterator over all the installer specifications in this registry.
    pub fn installers(&self) -> impl Iterator<Item = &Installer> {
        self.installers.values()
    }

    /// Return a matching installer for the platform/package kind pair.
    pub fn installer_for(&self, platform: Platform, kind: PackageKind) -> Option<&Installer> {
        self.installers.get(&(platform, kind))
    }

    /// Update all installers, at least all those that support update-self.
    pub fn update_self(&self) -> Result<()> {
        debug!("InstallerRegistry::update_self");

        for installer in self.installers() {
            if installer.is_platform_match() && installer.has_update_self() {
                reportln!("Updating installer {}", installer.name);
                let cmd_str = installer.update_self().as_ref().unwrap();
                let variable_replacements =
                    add_action_vars(&InstallActionKind::Update, &default_vars());
                execute_shell_command(cmd_str, &variable_replacements)?;
            }
        }
        reportln!("Done.");
        Ok(())
    }

    /// Execute the `action`, against some package set (or all), in some package set group (or all)
    /// in the provided repository.
    pub fn execute(
        &self,
        action: &InstallActionKind,
        repository: &PackageRepository,
        package_set_group_name: &Option<Name>,
        package_set_name: &Option<Name>,
    ) -> Result<()> {
        debug!(
            "InstallerRegistry::execute (.., {}, {:?}, {:?})",
            &action, &package_set_group_name, &package_set_name
        );
        let mut log_db = PackageLog::open()?;
        if let Some(package_set_group_name) = package_set_group_name {
            if let Some(package_set_group) = repository.group(package_set_group_name) {
                self.execute_package_set_group(
                    action,
                    package_set_group,
                    package_set_name,
                    &mut log_db,
                )?;
            } else {
                warn!(
                    "No package set group found named {:?}",
                    package_set_group_name
                )
            }
        } else {
            trace!("executing for all package groups in repository");
            for package_set_group in repository.groups() {
                self.execute_package_set_group(
                    action,
                    package_set_group,
                    package_set_name,
                    &mut log_db,
                )?;
            }
        }
        reportln!("Done.");
        Ok(())
    }

    fn execute_package_set_group(
        &self,
        action: &InstallActionKind,
        package_set_group: &PackageSetGroup,
        package_set_name: &Option<Name>,
        log_db: &mut PackageLog,
    ) -> Result<()> {
        debug!(
            "Installer::execute_package_set_group ({}, {:?}, {:?})",
            action,
            package_set_group.name(),
            package_set_name,
        );
        if let Some(package_set_name) = package_set_name {
            if let Some(package_set) = package_set_group.package_set(package_set_name) {
                self.execute_package_set(action, package_set_group, package_set, log_db)?;
            } else {
                warn!("No package set found named {:?}", package_set_name)
            }
        } else {
            trace!("executing for all package sets in group");
            for package_set in package_set_group.package_sets() {
                self.execute_package_set(action, package_set_group, &package_set, log_db)?;
            }
        }
        Ok(())
    }

    fn execute_package_set(
        &self,
        action: &InstallActionKind,
        package_set_group: &PackageSetGroup,
        package_set: &PackageSet,
        log_db: &mut PackageLog,
    ) -> Result<()> {
        reportln!(
            "Performing {} on package-set {} (in group {})",
            action,
            package_set.name(),
            package_set_group.name()
        );

        let mut variable_replacements =
            add_package_set_action_vars(package_set, &add_action_vars(action, &default_vars()));

        variable_replacements.extend(package_set.env_vars().clone());

        if let Some(cmd_str) = package_set.run_before() {
            trace!("executing `run_before` script");
            execute_shell_command(cmd_str, &variable_replacements)?;
        }

        if let Some(packages) = package_set.packages() {
            trace!("executing all package actions");
            for package in packages {
                match self.installer_for(package.platform(), package.kind().clone()) {
                    None => {
                        return Err(ErrorKind::NoInstallerForKind(package.kind().clone()).into())
                    }
                    Some(installer) => {
                        let variable_replacements =
                            add_package_action_vars(package, &variable_replacements);
                        installer.package_action(action, package, &variable_replacements)?;
                        log_db.log_installed_package(&InstalledPackage::new(
                            package_set_group.name(),
                            package_set.name().clone(),
                            package.name().clone(),
                            installer.name().clone(),
                        ))?;
                    }
                }
            }
        }

        if let Some(scripts) = package_set.scripts() {
            trace!("executing scripts? {:?}", scripts);
            if let Some(cmd_str) = scripts.get(action) {
                trace!("executing {:?} script", action);
                execute_shell_command(cmd_str, &variable_replacements)?;
            }
        }

        trace!("executing all env-file actions");
        if let Some(original) = package_set.env_file_path() {
            let link = package_set
                .path()
                .parent()
                .unwrap()
                .join(package_set.name().as_path())
                .join(original.file_name().unwrap());
            match action {
                InstallActionKind::Install => {
                    self.link_file(&link, &original)?;
                }
                InstallActionKind::Update => {
                    self.unlink_file(&link)?;
                }
                _ => {}
            };
        }

        trace!("executing all link-file actions");
        for (link, original) in package_set.link_file_paths() {
            match action {
                InstallActionKind::Install => {
                    self.link_file(&link, &original)?;
                }
                InstallActionKind::Update => {
                    self.unlink_file(&link)?;
                }
                _ => {}
            };
        }

        if let Some(cmd_str) = package_set.run_after() {
            let _ = variable_replacements.remove("package_name");
            trace!("executing `run_after` script");
            execute_shell_command(cmd_str, &variable_replacements)?;
        }

        Ok(())
    }

    fn link_file(&self, link: &PathBuf, original: &PathBuf) -> Result<()> {
        debug!("InstallerRegistry::link_file ({:?}, {:?})", link, original);
        std::os::unix::fs::symlink(original, link)?;
        Ok(())
    }

    fn unlink_file(&self, link: &PathBuf) -> Result<()> {
        debug!("InstallerRegistry::unlink_file ({:?})", link);
        std::fs::remove_file(link)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod builders {
    use crate::shared::builders::Builder;
    use crate::shared::{InstallActionKind, Installer, Name, PackageKind, Platform};
    use std::collections::HashMap;
    use std::path::PathBuf;

    // --------------------------------------------------------------------------------------------
    // Public Types
    // --------------------------------------------------------------------------------------------

    ///
    /// Provides a fluent interface for programmatic creation of [`Installer`](../struct.installer.html)
    /// instances.
    ///
    #[derive(Clone, Debug)]
    pub struct InstallerBuilder(Installer);

    // --------------------------------------------------------------------------------------------
    // Implementations
    // --------------------------------------------------------------------------------------------

    impl From<Installer> for InstallerBuilder {
        fn from(installer: Installer) -> Self {
            Self(installer)
        }
    }

    impl From<InstallerBuilder> for Installer {
        fn from(builder: InstallerBuilder) -> Self {
            builder.0
        }
    }

    impl Builder for InstallerBuilder {
        type Inner = Installer;

        fn build(&mut self) -> Self::Inner {
            self.0.clone()
        }
    }

    impl InstallerBuilder {
        /// Create a new instance, all instances must be named.
        pub fn named(name: Name) -> Self {
            Self(Installer {
                name,
                platform: None,
                kind: Default::default(),
                if_exists: None,
                commands: Default::default(),
                update_self: None,
            })
        }

        /// Add a file or directory path that determines whether this installer is enabled.
        pub fn if_exists(&mut self, path: &str) -> &mut Self {
            self.0.if_exists = Some(path.to_string());
            self
        }

        /// Add a file or directory path that determines whether this installer is enabled.
        pub fn if_exists_path(&mut self, path: &PathBuf) -> &mut Self {
            self.0.if_exists = Some(path.to_string_lossy().to_owned().to_string());
            self
        }

        /// Adds a platform constraint, the installer only works on the provided platform.
        pub fn for_platform(&mut self, platform: Platform) -> &mut Self {
            self.0.platform = Some(platform);
            self
        }

        /// Adds a platform constraint, the installer only works on macos.
        pub fn for_macos_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// Adds a platform constraint, the installer only works on linux.
        pub fn for_linux_only(&mut self) -> &mut Self {
            self.for_platform(Platform::Macos)
        }

        /// This installer has no platform constraint, it should work anywhere.
        pub fn for_any_platform(&mut self) -> &mut Self {
            self.0.platform = None;
            self
        }

        /// Sets the kind of packages this installer operates on.
        pub fn of_kind(&mut self, kind: PackageKind) -> &mut Self {
            self.0.kind = kind;
            self
        }

        /// This is an installer for default packages.
        pub fn for_default_packages(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Default)
        }

        /// This is an installer for application packages.
        pub fn for_application_packages(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Application)
        }

        /// This is an installer for language packages.
        pub fn for_language_packages(&mut self, language: &Name) -> &mut Self {
            self.of_kind(PackageKind::Language(language.clone()))
        }

        /// Set the map of script strings from the array of tuples, this is not additive.
        pub fn commands_list(&mut self, commands: &[(InstallActionKind, String)]) -> &mut Self {
            self.commands(commands.iter().cloned().collect())
        }

        /// Set the map of script strings, this is not additive.
        pub fn commands(&mut self, commands: HashMap<InstallActionKind, String>) -> &mut Self {
            self.0.commands = commands;
            self
        }

        /// Add a specific script string for the given command.
        pub fn add_command(&mut self, kind: InstallActionKind, script_string: &str) -> &mut Self {
            let _ = self.0.commands.insert(kind, script_string.to_string());
            self
        }

        /// Add a specific script string for the install command.
        pub fn add_install_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallActionKind::Install, script_string)
        }

        /// Add a specific script string for the update command.
        pub fn add_update_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallActionKind::Update, script_string)
        }

        /// Add a specific script string for the uninstall command.
        pub fn add_uninstall_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallActionKind::Uninstall, script_string)
        }

        /// Add a specific script string for the link-files command.
        pub fn add_link_files_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallActionKind::LinkFiles, script_string)
        }

        /// Add a specific script string for the update-self command.
        pub fn update_self_command(&mut self, script_string: &str) -> &mut Self {
            self.0.update_self = Some(script_string.to_string());
            self
        }
    }
}
