/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::install_log::PackageLog;
use crate::shared::{FileSystemResource, InstallerRegistry, PackageRepository, StepCounter};
use git2::Repository;
use std::fs;
use std::os::unix::fs as unix_fs;
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

const DEFAULT_INSTALLER_REGISTRY: &str = include_str!("default-installers.yml");

const HOMEBREW_PACKAGE_SET: &str = include_str!("macos-homebrew.yml");
const HOMEBREW_SERVICES_PACKAGE_SET: &str = include_str!("macos-homebrew-services.yml");

impl Action for InitAction {
    fn run(&self) -> Result<()> {
        let steps = StepCounter::from_one();
        info!("InitAction::run {:?}", self);

        let (link_required, local_dir) = match &self.local_dir {
            None => (false, PackageRepository::default_path()),
            Some(path) => (true, PathBuf::from(path)),
        };

        init_create_dir(&steps, &local_dir, "local directory for repository")?;

        let git_repo = local_dir.clone().join(".git");
        if !git_repo.is_dir() {
            match &self.repository_url {
                None => {
                    println!("{}. Initializing Git repository", steps.step());
                    let _ = Repository::init(local_dir.clone())?;
                }
                Some(repo_url) => {
                    println!("{}. Cloning <{}> into repository", steps.step(), &repo_url);
                    debug!("InitAction::run repo_url={:?}", repo_url);
                    let _ = Repository::clone(repo_url, local_dir.clone())?;
                }
            }
        } else {
            warn!("InitAction::run git repo exists, ignoring init/clone");
        }

        let repository_path = PackageRepository::default_path();
        if link_required {
            println!(
                "{}. Creating repository link {:?} -> {:?}",
                steps.step(),
                local_dir,
                &repository_path
            );
            debug!("InitAction::run repository_path={:?}", repository_path);
            fs::create_dir_all(repository_path.parent().unwrap())?;
            unix_fs::symlink(local_dir, &repository_path)?;
        }

        if matches!(&self.repository_url, None) {
            init_create_dir(
                &steps,
                &PackageRepository::default_config_path(),
                "repository '.config' directory",
            )?;

            init_create_dir(
                &steps,
                &PackageRepository::default_local_path(),
                "repository '.local' directory",
            )?;

            init_create_file(
                &steps,
                &repository_path.join("00-installers/macos-homebrew.yml"),
                "'00-installers/homebrew' package set",
                HOMEBREW_PACKAGE_SET,
            )?;

            init_create_file(
                &steps,
                &repository_path.join("00-installers/macos-homebrew-services.yml"),
                "'00-installers/homebrew-services' package set",
                HOMEBREW_SERVICES_PACKAGE_SET,
            )?;

            init_create_file(
                &steps,
                &repository_path.join("example/hello-world.yml"),
                "'example/hello world' package set",
                r##"---
        name: hello world
        description: just a test to make sure things work
        run-before: cargo --version"##,
            )?;
        } else {
            warn!("InitAction::run no examples added to cloned repository");
        }

        init_create_file(
            &steps,
            &InstallerRegistry::default_path(),
            "standard installer registry file",
            DEFAULT_INSTALLER_REGISTRY,
        )?;

        let log_file = PackageLog::default_path();
        if !log_file.is_file() {
            println!("{}. Creating package install log file", steps.step(),);
            let _ = PackageLog::open();
        } else {
            warn!("InitAction::run log file {:?} exists", log_file)
        }

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

fn init_create_dir(steps: &StepCounter, dir_path: &PathBuf, message: &str) -> Result<()> {
    if !dir_path.is_dir() {
        println!("{}. Creating {}", steps.step(), message);
        fs::create_dir_all(PackageRepository::default_config_path())?;
    } else {
        warn!("Directory {} ({:?}) exists", message, dir_path);
    }
    Ok(())
}

fn init_create_file(
    steps: &StepCounter,
    file_path: &PathBuf,
    message: &str,
    content: &str,
) -> Result<()> {
    if !file_path.is_file() {
        println!("{}. Creating {}", steps.step(), message,);
        fs::create_dir_all(file_path.parent().unwrap())?;
        fs::write(file_path, content)?;
    } else {
        warn!("File {} ({:?}) exists", message, file_path);
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
