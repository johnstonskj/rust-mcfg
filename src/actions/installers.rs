/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::editor::SystemEditor;
use crate::shared::installer::InstallerRegistry;
use crate::shared::FileSystemResource;

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
        let editor = SystemEditor::default();
        let _ = editor.edit(&registry_path)?;
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
