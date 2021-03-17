use crate::actions::Action;
use crate::error::Result;
use crate::shared::editor::SystemEditor;
use crate::shared::installer::InstallerRegistry;
use crate::shared::FileSystemResource;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action will invoke the system text editor to edit the installer registry file.
///
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
