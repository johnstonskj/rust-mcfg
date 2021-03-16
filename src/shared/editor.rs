/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{ErrorKind, Result};
use std::env::var;
use std::path::PathBuf;
use std::process::Command;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SystemEditor(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SystemEditor {
    fn default() -> Self {
        Self(match (var("VISUAL"), var("EDITOR")) {
            (Ok(cmd), _) => cmd,
            (Err(_), Ok(cmd)) => cmd,
            (_, _) => "vi".to_string(),
        })
    }
}

impl SystemEditor {
    pub fn new(editor_command: String) -> Self {
        Self(editor_command)
    }

    pub fn command(&self) -> &String {
        &self.0
    }

    pub fn edit(&self, file_path: &PathBuf) -> Result<()> {
        let result = Command::new(&self.0).arg(file_path).status();
        if result.is_err() {
            error!("Could not start editor for file {:?}", file_path);
            Err(ErrorKind::CommandExecutionFailed(self.0.clone(), None).into())
        } else {
            let exit_status = result.unwrap();
            if exit_status.success() {
                Ok(())
            } else {
                Err(ErrorKind::CommandExecutionFailed(self.0.clone(), Some(exit_status)).into())
            }
        }
    }
}
