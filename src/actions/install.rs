/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::installer::{InstallActionKind, InstallerRegistry};
use crate::shared::packages::PackageRepository;
use crate::shared::FileSystemResource;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InstallAction {
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

        let repository = PackageRepository::open()?;
        if repository.is_empty() {
            println!("No package sets found in repository");
        } else {
            let installer_registry = InstallerRegistry::open()?;
            installer_registry.execute(&self.kind, &repository, &self.group, &self.package_set)?;
        }
        Ok(())
    }
}

impl InstallAction {
    pub fn install(group: Option<String>, package_set: Option<String>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Install,
            group,
            package_set,
        }))
    }
    pub fn update(group: Option<String>, package_set: Option<String>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Update,
            group,
            package_set,
        }))
    }
    pub fn uninstall(
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Uninstall,
            group,
            package_set,
        }))
    }
    pub fn link_files(
        group: Option<String>,
        package_set: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
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
