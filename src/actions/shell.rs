use crate::actions::Action;
use crate::error::Result;
use crate::shared::{execute_interactive_shell, FileSystemResource, PackageRepository};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action will refresh the package repository, basically a Git pull.
///
#[derive(Debug)]
pub struct ShellAction {
    shell: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for ShellAction {
    fn run(&self) -> Result<()> {
        info!("ShellAction::run");
        execute_interactive_shell(PackageRepository::default_path())?;
        Ok(())
    }
}

impl ShellAction {
    pub fn new(shell: &str) -> Result<Box<dyn Action>> {
        Ok(Box::from(ShellAction {
            shell: shell.to_string(),
        }))
    }
}
