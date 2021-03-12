/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::install_log::PackageLog;
use crate::shared::installer::REGISTRY_FILE;
use crate::shared::{InstallerRegistry, PackageRepository};
use git2::Repository;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InitAction {
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
            None => (false, PackageRepository::default_path()),
            Some(path) => (true, PathBuf::from(path)),
        };
        std::fs::create_dir_all(local_dir.clone())?;
        let git_repo = local_dir.clone().join(".git");
        if !git_repo.is_dir() {
            match &self.repository_url {
                None => {
                    debug!("InitAction::run git init in {:?}", local_dir.clone());
                    let _ = Repository::init(local_dir.clone())?;
                    // TODO: add a hello-world example package-set
                }
                Some(repo_url) => {
                    debug!("InitAction::run git clone: {}", repo_url);
                    let _ = Repository::clone(repo_url, local_dir.clone())?;
                }
            }
        } else {
            debug!("InitAction::run git repo exists, ignoring init/clone");
        }
        let repository_path = PackageRepository::default_path();
        if link_required {
            debug!(
                "InitAction::run creating symlink to local dir: {:?} -> {:?}",
                local_dir, &repository_path
            );
            std::fs::create_dir_all(&repository_path)?;
            std::os::unix::fs::symlink(local_dir, &repository_path)?;
        }
        let config_path = InstallerRegistry::default_path();
        if !config_path.is_dir() {
            debug!("InitAction::run creating config path: {:?}", &config_path);
            std::fs::create_dir_all(&config_path)?;
        } else {
            debug!("InitAction::run config path {:?} exists", &config_path);
        }
        let installer_registry = config_path.join(REGISTRY_FILE);
        if !installer_registry.is_file() {
            debug!(
                "InitAction::run creating default installer config file: {:?}",
                &installer_registry
            );
            std::fs::write(
                installer_registry,
                r##"---
- name: homebrew
  platform: macos
  kind: default
  commands:
    install: "brew install {{package_name}}"
    uninstall: "brew uninstall {{package_name}}"
    update-self: "brew update"
    update: "brew upgrade {{package_name}}"

- name: homebrew apps
  platform: macos
  kind: application
  commands:
    install: "brew cask install {{package_name}}"
    uninstall: "brew cask uninstall {{package_name}}"
    update-self: "brew update"
    update: "brew cask upgrade {{package_name}}"

- name: cargo
  kind:
    language: rust
  commands:
    install: "cargo install {{package_name}}"
    uninstall: "cargo uninstall {{package_name}}"

- name: conda
  kind:
    language: python
  commands:
    install: "conda install {{package_name}}"
    uninstall: "conda remove {{package_name}}"
    update: "conda update {{package_name}}"

- name: gem
  kind:
    language: ruby
  commands:
    install: "gem install {{package_name}}"
    uninstall: "gem uninstall {{package_name}}"
    update: "gem update {{package_name}}"
"##,
            )?;
        } else {
            debug!(
                "InitAction::run installer config file {:?} exists",
                installer_registry
            );
        }
        let _ = PackageLog::open();
        Ok(())
    }
}

impl InitAction {
    pub fn new(
        local_dir: Option<String>,
        repository_url: Option<String>,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::from(InitAction {
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
