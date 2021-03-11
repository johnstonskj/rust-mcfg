/*!
One-line description.

More detailed description, with

# Example

*/

use crate::APP_NAME;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    config: PathBuf,
    log: PathBuf,
    repository: PathBuf,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const INSTALLER_CONFIG_FILE: &str = "installers.yml";

const USER_LOG_FILE: &str = "install-log.sql";

const USER_REPOSITORY_DIR: &str = "repository";

impl Default for Environment {
    fn default() -> Self {
        Self::with_roots(
            xdirs::config_dir_for(APP_NAME).unwrap(),
            xdirs::log_dir_for(APP_NAME).unwrap(),
            xdirs::data_dir_for(APP_NAME).unwrap(),
        )
    }
}

impl Environment {
    pub fn with_roots(config_root: PathBuf, log_root: PathBuf, data_root: PathBuf) -> Self {
        let base = current_dir().unwrap();
        let config = base.join(config_root);
        let log = base.join(log_root);
        let repository = base.join(data_root).join(USER_REPOSITORY_DIR);
        debug!("Environment::with_roots config dir: {:?}", &config);
        debug!("Environment::with_roots log dir: {:?}", &log);
        debug!("Environment::with_roots repository dir: {:?}", &repository);
        Self {
            config,
            log,
            repository,
        }
    }

    pub fn config_path(&self) -> &PathBuf {
        &self.config
    }

    pub fn has_config_path(&self) -> bool {
        self.config_path().is_dir()
    }

    pub fn repository_path(&self) -> &PathBuf {
        &self.repository
    }

    pub fn has_repository_path(&self) -> bool {
        self.repository_path().is_dir()
    }

    pub fn installer_file_path(&self) -> PathBuf {
        self.config.join(INSTALLER_CONFIG_FILE)
    }

    pub fn has_installer_file(&self) -> bool {
        self.installer_file_path().is_file()
    }

    pub fn log_file_path(&self) -> PathBuf {
        self.log.join(USER_LOG_FILE)
    }

    pub fn has_log_file(&self) -> bool {
        self.log_file_path().is_file()
    }

    pub fn is_initialized(&self) -> bool {
        self.has_config_path() && self.has_repository_path()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
