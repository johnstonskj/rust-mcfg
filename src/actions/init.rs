/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::install_log::PackageLog;
use git2::Repository;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InitAction {
    env: Environment,
    local_dir: Option<String>,
    repository_url: Option<String>,
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

impl Action for InitAction {
    fn run(&self) -> Result<()> {
        info!("InitAction::run {:?}", self);
        let (link_required, local_dir) = match &self.local_dir {
            None => (false, self.env.repository_path().clone()),
            Some(path) => (true, PathBuf::from(path)),
        };
        std::fs::create_dir_all(local_dir.clone())?;
        let git_repo = local_dir.clone().join(".git");
        if !git_repo.is_dir() {
            match &self.repository_url {
                None => {
                    debug!("InitAction::run git init in {:?}", local_dir.clone());
                    let _ = Repository::init(local_dir.clone())?;
                }
                Some(repo_url) => {
                    debug!("InitAction::run git clone: {}", repo_url);
                    let _ = Repository::clone(repo_url, local_dir.clone())?;
                }
            }
        } else {
            debug!("InitAction::run git repo exists, ignoring init/clone");
        }
        if link_required {
            debug!(
                "InitAction::run creating symlink to local dir: {:?} -> {:?}",
                local_dir,
                self.env.repository_path()
            );
            std::fs::create_dir_all(self.env.repository_path())?;
            std::os::unix::fs::symlink(local_dir, self.env.repository_path())?;
        }
        if !self.env.has_config_path() {
            debug!(
                "InitAction::run creating config path: {:?}",
                self.env.config_path()
            );
            std::fs::create_dir_all(self.env.config_path())?;
        } else {
            debug!(
                "InitAction::run config path {:?} exists",
                self.env.config_path()
            );
        }
        if !self.env.has_installer_file() {
            debug!(
                "InitAction::run creating default installer config file: {:?}",
                self.env.installer_file_path()
            );
            std::fs::write(
                self.env.installer_file_path(),
                r##"---
- name: homebrew
  platform: macos
  kind: default
  commands:
    install: "brew install {{package}}"
    update-self: brew upgrade
    update: "brew update {{package}}"

- name: homebrew apps
  platform: macos
  kind: application
  commands:
    install: "brew cask install {{package}}"
    update-self: brew upgrade
    update: "brew cask update {{package}}"

- name: cargo
  kind:
    language: rust
  commands:
    install: "cargo install {{package}}"
    update: "cargo update {{package}}"
"##,
            )?;
        } else {
            debug!(
                "InitAction::run installer config file {:?} exists",
                self.env.installer_file_path()
            );
        }
        let _ = PackageLog::open(&self.env.log_file_path());
        Ok(())
    }
}

impl InitAction {
    pub fn new(
        env: Environment,
        local_dir: Option<String>,
        repository_url: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InitAction {
            env,
            local_dir,
            repository_url,
        }))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
