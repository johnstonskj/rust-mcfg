/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::installer::InstallerRegistry;
use crate::shared::{PackageLog, PackageRepository};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ShowPathsAction {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for ShowPathsAction {
    fn run(&self) -> Result<()> {
        let repository_location = PackageRepository::default_path();
        println!("Package Repository path:\n\t{:?}", &repository_location);
        let metadata = std::fs::symlink_metadata(&repository_location)?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            let local_location = std::fs::read_link(repository_location)?;
            println!("Package Repository symlinked to:\n\t{:?}", &local_location);
        }
        println!(
            "Package Repository config file path:\n\t{:?}",
            &PackageRepository::default_config_path()
        );
        println!(
            "Package Repository local file path:\n\t{:?}",
            &PackageRepository::default_local_path()
        );
        println!(
            "Installer Registry path:\n\t{:?}",
            InstallerRegistry::default_path()
        );
        println!(
            "Package Installer log file path:\n\t{:?}",
            PackageLog::default_path()
        );
        Ok(())
    }
}

impl ShowPathsAction {
    pub fn new() -> Result<Box<dyn Action>> {
        Ok(Box::from(ShowPathsAction {}))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
