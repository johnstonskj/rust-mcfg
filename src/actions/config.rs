/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::installer::{Installer, InstallerRegistry};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ConfigAction {
    env: Environment,
}

#[derive(Serialize, Deserialize, Debug)]
struct CombinedConfig {
    root_paths: Environment,
    installers: Vec<Installer>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for ConfigAction {
    fn run(&self) -> Result<()> {
        let registry = InstallerRegistry::read(&self.env)?;
        let combined = CombinedConfig {
            root_paths: self.env.clone(),
            installers: registry.into(),
        };
        serde_yaml::to_writer(std::io::stdout(), &combined)?;
        Ok(())
    }
}

impl ConfigAction {
    pub fn new(env: Environment) -> Result<Box<dyn Action>> {
        Ok(Box::from(ConfigAction { env }))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
