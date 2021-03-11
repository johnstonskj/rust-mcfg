/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::installer::{InstallActionKind, InstallerRegistry};
use crate::shared::packages::PackageRepository;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InstallAction {
    env: Environment,
    kind: InstallActionKind,
    group: Option<String>,
    package_set: Option<String>,
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

impl Action for InstallAction {
    fn run(&self) -> Result<()> {
        info!("InstallAction::run {:?}", self);

        let repository = PackageRepository::open(&self.env)?;
        if repository.is_empty() {
            println!("No package sets found in repository");
        } else {
            let installer_registry = InstallerRegistry::read(&self.env)?;
            installer_registry.execute(&repository, &self.kind, &self.group, &self.package_set)?;
        }
        Ok(())
    }
}

impl InstallAction {
    pub fn install(
        env: Environment,
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            env,
            kind: InstallActionKind::Install,
            group,
            package_set,
        }))
    }
    pub fn update(
        env: Environment,
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            env,
            kind: InstallActionKind::Update,
            group,
            package_set,
        }))
    }
    pub fn uninstall(
        env: Environment,
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            env,
            kind: InstallActionKind::Uninstall,
            group,
            package_set,
        }))
    }
    pub fn link_files(
        env: Environment,
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            env,
            kind: InstallActionKind::LinkFiles,
            group,
            package_set,
        }))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
