use crate::actions::Action;
use crate::error::Result;
use crate::shared::installer::InstallerRegistry;
use crate::shared::FileSystemResource;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action asks all installers that support the operation to update themselves.
///
#[derive(Debug)]
pub struct UpdateSelfAction {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for UpdateSelfAction {
    fn run(&self) -> Result<()> {
        let installer_registry = InstallerRegistry::open()?;
        installer_registry.update_self()?;
        Ok(())
    }
}

impl UpdateSelfAction {
    pub fn new_action() -> Result<Box<dyn Action>> {
        Ok(Box::from(UpdateSelfAction {}))
    }
}
