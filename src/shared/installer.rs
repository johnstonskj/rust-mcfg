/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{ErrorKind, Result};
use crate::shared::command::ShellCommand;
use crate::shared::env::{
    add_action_vars, add_package_action_vars, add_package_set_action_vars, default_vars,
};
use crate::shared::install_log::{InstalledPackage, PackageLog};
use crate::shared::packages::{Package, PackageRepository, PackageSet, PackageSetGroup};
use crate::shared::{FileSystemResource, PackageKind, Platform};
use crate::APP_NAME;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum InstallActionKind {
    Install,
    Update,
    Uninstall,
    LinkFiles,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum InstallerCommandKind {
    Install,
    Update,
    Uninstall,
    UpdateSelf,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Installer {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    kind: PackageKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    if_exists: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    commands: HashMap<InstallerCommandKind, String>,
}

#[derive(Clone, Debug)]
pub struct InstallerRegistry {
    installers: HashMap<(Platform, PackageKind), Installer>,
}

pub const REGISTRY_FILE: &str = "installers.yml";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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

impl Default for InstallerCommandKind {
    fn default() -> Self {
        Self::Install
    }
}

// ------------------------------------------------------------------------------------------------

impl Installer {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_platform_match(&self) -> bool {
        Platform::CURRENT.is_match(&self.platform)
    }

    pub fn if_exists_match(&self) -> bool {
        match &self.if_exists {
            None => true,
            Some(path) => PathBuf::from(path).exists(),
        }
    }

    pub fn has_upgrade_self(&self) -> bool {
        self.commands
            .keys()
            .any(|kind| kind == &InstallerCommandKind::UpdateSelf)
    }

    pub fn platform(&self) -> Platform {
        self.platform.as_ref().cloned().unwrap_or_default()
    }

    pub fn kind(&self) -> PackageKind {
        self.kind.clone()
    }

    pub fn commands(&self) -> &HashMap<InstallerCommandKind, String> {
        &self.commands
    }

    fn package_action(
        &self,
        action: &InstallActionKind,
        package: &Package,
        variable_replacements: &HashMap<String, String>,
    ) -> Result<()> {
        if self.is_platform_match() && package.is_platform_match() {
            if self.kind() == *package.kind() {
                let cmd = match action {
                    InstallActionKind::Install => self.commands.get(&InstallerCommandKind::Install),
                    InstallActionKind::Update => self.commands.get(&InstallerCommandKind::Update),
                    InstallActionKind::Uninstall => {
                        self.commands.get(&InstallerCommandKind::Uninstall)
                    }
                    _ => return Ok(()),
                };
                if let Some(cmd_str) = cmd {
                    println!(
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
    pub fn is_empty(&self) -> bool {
        self.installers.is_empty()
    }

    pub fn installers(&self) -> impl Iterator<Item = &Installer> {
        self.installers.values()
    }

    pub fn installer_for(&self, platform: Platform, kind: PackageKind) -> Option<&Installer> {
        self.installers.get(&(platform, kind))
    }

    pub fn update_self(&self) -> Result<()> {
        debug!("InstallerRegistry::update_self");

        for installer in self.installers() {
            if installer.is_platform_match() && installer.has_upgrade_self() {
                println!("Updating installer {}", installer.name);
                let cmd_str = installer
                    .commands
                    .get(&InstallerCommandKind::UpdateSelf)
                    .unwrap();
                let variable_replacements =
                    add_action_vars(&InstallActionKind::Update, &default_vars());
                execute_shell_command(cmd_str, &variable_replacements)?;
            }
        }
        println!("Done.");
        Ok(())
    }

    pub fn execute(
        &self,
        action: &InstallActionKind,
        repository: &PackageRepository,
        package_set_group_name: &Option<String>,
        package_set_name: &Option<String>,
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
        println!("Done.");
        Ok(())
    }

    fn execute_package_set_group(
        &self,
        action: &InstallActionKind,
        package_set_group: &PackageSetGroup,
        package_set_name: &Option<String>,
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
        println!(
            "Performing {} on package-set {} (in group {})",
            action,
            package_set.name(),
            package_set_group.name()
        );

        trace!("package-set {:?}", package_set);

        let mut variable_replacements =
            add_package_set_action_vars(package_set, &add_action_vars(action, &default_vars()));

        variable_replacements.extend(package_set.more_env_vars().clone());

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
                            &package_set_group.name(),
                            package_set.name(),
                            package.name(),
                            installer.name(),
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
                .join(package_set.name())
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
// Private Functions
// ------------------------------------------------------------------------------------------------

fn execute_shell_command(
    cmd_str: &str,
    variable_replacements: &HashMap<String, String>,
) -> Result<()> {
    debug!("execute_shell_command ({:?}", cmd_str);
    let shell_command = ShellCommand::new(variable_replacements.clone());
    shell_command.execute(cmd_str)
}

pub mod builders {
    use crate::shared::{Installer, InstallerCommandKind, PackageKind, Platform};
    use std::collections::HashMap;

    // ------------------------------------------------------------------------------------------------
    // Public Types
    // ------------------------------------------------------------------------------------------------

    #[derive(Clone, Debug)]
    pub struct InstallerBuilder(Installer);

    // ------------------------------------------------------------------------------------------------
    // Private Types
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Public Functions
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Implementations
    // ------------------------------------------------------------------------------------------------

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

    impl InstallerBuilder {
        pub fn named(name: &str) -> Self {
            Self(Installer {
                name: name.to_string(),
                platform: None,
                kind: Default::default(),
                if_exists: None,
                commands: Default::default(),
            })
        }

        pub fn if_exists(&mut self, cmd_path: &str) -> &mut Self {
            self.0.if_exists = Some(cmd_path.to_string());
            self
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

        pub fn for_default_packages(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Default)
        }

        pub fn for_application_packages(&mut self) -> &mut Self {
            self.of_kind(PackageKind::Application)
        }

        pub fn for_language_packages(&mut self, language: &str) -> &mut Self {
            self.of_kind(PackageKind::Language(language.to_string()))
        }

        pub fn commands_list(&mut self, commands: &[(InstallerCommandKind, String)]) -> &mut Self {
            self.commands(commands.into_iter().cloned().collect())
        }

        pub fn commands(&mut self, commands: HashMap<InstallerCommandKind, String>) -> &mut Self {
            self.0.commands = commands;
            self
        }

        pub fn add_command(
            &mut self,
            kind: InstallerCommandKind,
            script_string: &str,
        ) -> &mut Self {
            let _ = self.0.commands.insert(kind, script_string.to_string());
            self
        }

        pub fn add_install_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallerCommandKind::Install, script_string)
        }

        pub fn add_update_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallerCommandKind::Update, script_string)
        }

        pub fn add_uninstall_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallerCommandKind::Uninstall, script_string)
        }

        pub fn add_update_self_command(&mut self, script_string: &str) -> &mut Self {
            self.add_command(InstallerCommandKind::UpdateSelf, script_string)
        }

        pub fn installer(&self) -> Installer {
            self.0.clone()
        }
    }

    // ------------------------------------------------------------------------------------------------
    // Private Functions
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // Modules
    // ------------------------------------------------------------------------------------------------
}
