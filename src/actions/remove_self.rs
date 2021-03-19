use crate::actions::Action;
use crate::error::Result;
use crate::shared::installer::InstallerRegistry;
use crate::shared::{FileSystemResource, PackageLog, PackageRepository};
use std::fs;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action will remove the package repository and installer registry. This is only included in
/// the CLI if the feature "remove-self" is included.
///
/// If the package repository directory is a symlink, only the link is removed and the directory
/// contents are left intact.
///
#[derive(Debug)]
pub struct RemoveSelfAction {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for RemoveSelfAction {
    fn run(&self) -> Result<()> {
        let repository_location = PackageRepository::default_path();
        let metadata = std::fs::symlink_metadata(&repository_location)?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            println!(
                "Removing Package Repository symlink:\n\t{:?}",
                &repository_location
            );
            fs::remove_file(repository_location)?;
        } else {
            println!("Removing Package Repository:\n\t{:?}", &repository_location);
        }

        println!(
            "Removing Installer Registry file:\n\t{:?}",
            InstallerRegistry::default_path()
        );
        fs::remove_file(InstallerRegistry::default_path())?;

        println!(
            "Removing Package Installer log file:\n\t{:?}",
            PackageLog::default_path()
        );
        fs::remove_file(PackageLog::default_path())?;
        Ok(())
    }
}

impl RemoveSelfAction {
    pub fn new_action() -> Result<Box<dyn Action>> {
        Ok(Box::from(RemoveSelfAction {}))
    }
}
