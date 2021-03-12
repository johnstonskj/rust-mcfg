/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::editor::run_editor;
use crate::shared::installer::InstallerRegistry;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct EditInstallersAction {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for EditInstallersAction {
    fn run(&self) -> Result<()> {
        let registry_path = InstallerRegistry::default_path();
        debug!("EditInstallersAction::run editing file {:?}", registry_path);
        run_editor(&registry_path);
        Ok(())
    }
}

impl EditInstallersAction {
    pub fn new() -> Result<Box<dyn Action>> {
        Ok(Box::from(EditInstallersAction {}))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------