use crate::actions::Action;
use crate::error::Result;
use crate::shared::env::vars_to_env_vars;
use crate::shared::{default_vars, FileSystemResource, PackageRepository};
use crate::APP_NAME;
use std::process::Command;

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

        match Command::new(&self.shell)
            .envs(vars_to_env_vars(&default_vars(), &APP_NAME.to_uppercase()))
            .current_dir(PackageRepository::default_path())
            .status()
        {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Could not execute shell");
            }
        }
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
