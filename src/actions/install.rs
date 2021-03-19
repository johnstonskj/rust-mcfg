use crate::actions::Action;
use crate::error::Result;
use crate::shared::installer::{InstallActionKind, InstallerRegistry};
use crate::shared::packages::PackageRepository;
use crate::shared::{FileSystemResource, Name};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action performs one of the core install, update, link-files, or uninstall actions.
///
#[derive(Debug)]
pub struct InstallAction {
    kind: InstallActionKind,
    group: Option<Name>,
    package_set: Option<Name>,
}

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
    pub fn install_action(group: Option<Name>, package_set: Option<Name>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Install,
            group,
            package_set,
        }))
    }
    pub fn update_action(group: Option<Name>, package_set: Option<Name>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Update,
            group,
            package_set,
        }))
    }
    pub fn uninstall_action(group: Option<Name>, package_set: Option<Name>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::Uninstall,
            group,
            package_set,
        }))
    }
    pub fn link_files_action(group: Option<Name>, package_set: Option<Name>) -> Result<Box<dyn Action>> {
        Ok(Box::from(InstallAction {
            kind: InstallActionKind::LinkFiles,
            group,
            package_set,
        }))
    }
}
