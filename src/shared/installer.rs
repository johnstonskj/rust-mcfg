/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{ErrorKind, Result};
use crate::shared::command::Tokens;
use crate::shared::install_log::{InstalledPackage, PackageLog};
use crate::shared::packages::{Package, PackageRepository, PackageSet, PackageSetGroup};
use crate::shared::{PackageKind, Platform};
use crate::APP_NAME;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum InstallActionKind {
    Install,
    Update,
    Uninstall,
    LinkFiles,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum InstallerCommandKind {
    Install,
    Update,
    Uninstall,
    UpdateSelf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Installer {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    platform: Option<Platform>,
    kind: PackageKind,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    commands: HashMap<InstallerCommandKind, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

impl InstallerRegistry {
    pub fn default_path() -> PathBuf {
        xdirs::config_dir_for(APP_NAME).unwrap().join(REGISTRY_FILE)
    }

    pub fn open() -> Result<Self> {
        Self::actual_open(Self::default_path())
    }

    pub fn open_from(config_root: PathBuf) -> Result<Self> {
        let base = current_dir().unwrap();
        Self::actual_open(base.join(config_root))
    }

    fn actual_open(registry_file: PathBuf) -> Result<Self> {
        info!("InstallerRegistry::read loading from {:?}", registry_file);
        let registry_data = read_to_string(registry_file)?;
        let installers: Vec<Installer> = serde_yaml::from_str(&registry_data)?;
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
            debug!(
                "InstallerRegistry::read: Read config for installer {:?}",
                key
            );
            let result = registry.installers.insert(key, installer);
            if result.is_some() {
                debug!(
                    "InstallerRegistry::read: key is a duplicate, previous value was overwritten"
                );
            }
        }
        Ok(registry)
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
                    self.package_variable_replacements(None, &InstallActionKind::Update);
                execute_shell_command(cmd_str, &variable_replacements)?;
            }
        }
        println!("Done.");
        Ok(())
    }

    pub fn execute(
        &self,
        repository: &PackageRepository,
        action: &InstallActionKind,
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

        let mut variable_replacements =
            self.package_variable_replacements(Some(package_set), action);

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
                        let _ = variable_replacements
                            .insert("package_name".to_string(), package.name().to_string());
                        installer.package_action(action, package, &variable_replacements)?;
                        log_db.log_installed_package(&InstalledPackage::new(
                            package_set_group.name(),
                            package_set.name(),
                            package.name(),
                            installer.name(),
                        ))?;
                    }
                }
            }
        }

        if let Some(scripts) = package_set.scripts() {
            if let Some(cmd_str) = scripts.get(action) {
                trace!("executing {:?} script", action);
                execute_shell_command(cmd_str, &variable_replacements)?;
            }
        }

        trace!("executing all env-file actions");
        if let Some(original) = package_set.env_file() {
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
        for (link, original) in package_set.link_files() {
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

    fn package_variable_replacements(
        &self,
        package_set: Option<&PackageSet>,
        action: &InstallActionKind,
    ) -> HashMap<String, String> {
        let mut replacements: HashMap<String, String> = Default::default();
        let _ = replacements.insert("command_action".to_string(), action.to_string());
        let _ = replacements.insert(
            "command_log_level".to_string(),
            log::max_level().to_string().to_lowercase(),
        );
        let _ = replacements.insert(
            "local_download_path".to_string(),
            dirs_next::download_dir()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );
        let _ = replacements.insert("opsys".to_string(), Platform::CURRENT.to_string());
        let _ = replacements.insert(
            "repo_config_path".to_string(),
            PackageRepository::default_config_path()
                .to_string_lossy()
                .to_string(),
        );
        let _ = replacements.insert(
            "repo_local_path".to_string(),
            PackageRepository::default_local_path()
                .to_string_lossy()
                .to_string(),
        );
        let _ = replacements.insert("shell".to_string(), "bash".to_string());

        if let Some(package_set) = package_set {
            let _ = replacements.insert(
                "package_set_name".to_string(),
                package_set.name().to_string(),
            );
            let _ = replacements.insert(
                "package_set_file".to_string(),
                package_set
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
                    .to_string(),
            );
            let _ = replacements.insert(
                "package_set_path".to_string(),
                package_set
                    .path()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
                    .to_string(),
            );
        }

        debug!("package_variable_replacements: {:?}", &replacements);
        replacements
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
    let mut tokenized = Tokens::from_str(cmd_str)?;
    tokenized.execute_command(variable_replacements)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let config_str = r##"
        - name: homebrew
          platform: macos
          kind: default
          commands:
            install: "brew install {{package}}"
            update: "brew update {{package}}"
"##;
        let configs: Vec<Installer> = serde_yaml::from_str(config_str).unwrap();
        println!("{:?}", configs);
    }

    #[test]
    fn test_write() {
        let configs = vec![Installer {
            name: "homebrew".to_string(),
            platform: Some(Platform::Macos),
            kind: PackageKind::Default,
            commands: vec![
                (
                    InstallerCommandKind::Install,
                    "brew install {{package}}".to_string(),
                ),
                (
                    InstallerCommandKind::Update,
                    "brew update {{package}}".to_string(),
                ),
                (InstallerCommandKind::UpdateSelf, "brew upgrade".to_string()),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<InstallerCommandKind, String>>(),
        }];

        let config_str = serde_yaml::to_string(&configs).unwrap();
        println!("{:?}", config_str);
    }
}
