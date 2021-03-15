/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::install_log::PackageLog;
use crate::shared::{FileSystemResource, InstallerRegistry, PackageRepository};
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
        let mut steps = 1..;
        info!("InitAction::run {:?}", self);
        let (link_required, local_dir) = match &self.local_dir {
            None => (false, PackageRepository::default_path()),
            Some(path) => (true, PathBuf::from(path)),
        };
        println!(
            "{}. Creating local directory for repository",
            steps.next().unwrap()
        );
        debug!("InitAction::run local_dir={:?}", local_dir);
        std::fs::create_dir_all(&local_dir)?;
        let git_repo = local_dir.clone().join(".git");
        if !git_repo.is_dir() {
            match &self.repository_url {
                None => {
                    println!("{}. Initializing Git repository", steps.next().unwrap());
                    let _ = Repository::init(local_dir.clone())?;
                }
                Some(repo_url) => {
                    println!(
                        "{}. Cloning <{}> into repository",
                        steps.next().unwrap(),
                        &repo_url
                    );
                    debug!("InitAction::run repo_url={:?}", repo_url);
                    let _ = Repository::clone(repo_url, local_dir.clone())?;
                }
            }
        } else {
            debug!("InitAction::run git repo exists, ignoring init/clone");
        }

        let repository_path = PackageRepository::default_path();
        if link_required {
            println!(
                "{}. Creating repository link {:?} -> {:?}",
                steps.next().unwrap(),
                local_dir,
                &repository_path
            );
            debug!("InitAction::run repository_path={:?}", repository_path);
            std::fs::create_dir_all(repository_path.parent().unwrap())?;
            std::os::unix::fs::symlink(local_dir, &repository_path)?;
        }
        println!(
            "{}. Creating repository config/local directories",
            steps.next().unwrap()
        );
        std::fs::create_dir_all(PackageRepository::default_config_path())?;
        std::fs::create_dir_all(PackageRepository::default_local_path())?;

        println!(
            "{}. Creating 'example/hello world' package set",
            steps.next().unwrap()
        );
        let example_group_path = repository_path.join("example");
        debug!(
            " InitAction::run example_group_path={:?}",
            example_group_path
        );
        std::fs::create_dir_all(&example_group_path)?;
        let example_set_file = example_group_path.join("hello-world.yml");
        debug!(" InitAction::run example_set_file={:?}", example_set_file);
        std::fs::write(
            example_set_file,
            r##"---
        name: hello world
        description: just a test to make sure things work
        run-before: cargo --version"##,
        )?;

        let installer_registry = InstallerRegistry::default_path();
        if !installer_registry.is_file() {
            println!(
                "{}. Creating initial installer registry file",
                steps.next().unwrap()
            );
            debug!(
                " InitAction::run installer_registry={:?}",
                installer_registry
            );
            std::fs::create_dir_all(installer_registry.parent().unwrap())?;
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
        println!("Done.");
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
