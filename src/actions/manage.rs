/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::editor::run_editor;
use crate::shared::{FileSystemResource, PackageRepository};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ManageActionKind {
    Add,
    Edit,
    Remove,
}

#[derive(Debug)]
pub struct ManageAction {
    kind: ManageActionKind,
    group: String,
    package_set: String,
    package_set_is_file: bool,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const EMPTY_PACKAGE_SET: &str = r##"---
name: pset
description: my new pset package set.
packages:
  - name: pset"##;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for ManageAction {
    fn run(&self) -> Result<()> {
        let direct_path = self.make_package_set_path(true);
        let indirect_path = self.make_package_set_path(false);
        debug!(
            "ManageAction::run: Taking action {:?} on path {:?} or {:?}",
            self.kind, direct_path, indirect_path
        );
        match self.kind {
            ManageActionKind::Add => {
                if !direct_path.exists() && !indirect_path.exists() {
                    if self.package_set_is_file {
                        create_dir_all(direct_path.parent().unwrap())?;
                        write(
                            &direct_path,
                            EMPTY_PACKAGE_SET.replace("pset", &self.package_set),
                        )?;
                        run_editor(&direct_path);
                    } else {
                        create_dir_all(indirect_path.parent().unwrap())?;
                        write(
                            &indirect_path,
                            EMPTY_PACKAGE_SET.replace("pset", &self.package_set),
                        )?;
                        run_editor(&indirect_path);
                    }
                } else {
                    eprintln!(
                        "Error: a package set file {:?} or {:?} already exists, cannot add",
                        direct_path, indirect_path
                    );
                }
            }
            ManageActionKind::Edit => match (direct_path.exists(), indirect_path.exists()) {
                (true, false) => {
                    run_editor(&direct_path);
                }
                (false, true) => {
                    run_editor(&indirect_path);
                }
                (true, true) => {
                    eprintln!(
                        "Error: both the package set files {:?} and {:?} exist, I don't know which to edit",
                        direct_path, indirect_path
                    );
                }
                (false, false) => {
                    eprintln!(
                        "Error: neither the package set file {:?} or {:?} exist, making it hard to edit them",
                        direct_path, indirect_path
                    );
                }
            },
            ManageActionKind::Remove => {
                if direct_path.exists() {
                    debug!("ManageAction::run: removing file {:?}", direct_path);
                    std::fs::remove_file(direct_path)?;
                } else if indirect_path.exists() {
                    debug!("ManageAction::run: removing file {:?}", indirect_path);
                    std::fs::remove_file(indirect_path)?;
                } else {
                    eprintln!(
                        "Error: neither the package set file {:?}, or {:?} exist, making it hard to remove them",
                        direct_path, indirect_path
                    );
                }
            }
        }
        Ok(())
    }
}

impl ManageAction {
    pub fn add(
        group: String,
        package_set: String,
        package_set_is_file: bool,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Add,
            group,
            package_set,
            package_set_is_file,
        }))
    }
    pub fn edit(group: String, package_set: String) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Edit,
            group,
            package_set,
            package_set_is_file: true,
        }))
    }
    pub fn remove(group: String, package_set: String) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Remove,
            group,
            package_set,
            package_set_is_file: true,
        }))
    }

    fn make_package_set_path(&self, package_set_is_file: bool) -> PathBuf {
        let group_path = PackageRepository::default_path().join(&self.group);
        if package_set_is_file {
            group_path.join(&format!("{}.yml", self.package_set))
        } else {
            group_path.join(&self.package_set).join("package-set.yml")
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
