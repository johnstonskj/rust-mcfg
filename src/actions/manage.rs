use crate::actions::Action;
use crate::error::Result;
use crate::shared::command::edit_file;
use crate::shared::{FileSystemResource, Name, PackageRepository};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action performs simple management actions on the package repository -- to add, edit, or
/// remove package sets.
///
#[derive(Debug)]
pub struct ManageAction {
    kind: ManageActionKind,
    group: Name,
    package_set: Name,
    package_set_is_file: bool,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ManageActionKind {
    Add,
    Edit,
    Remove,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const EMPTY_PACKAGE_SET: &str = r##"---
name: pset
description: my new pset package set.
packages:
  - name: pset"##;

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
                            EMPTY_PACKAGE_SET.replace("pset", &self.package_set.to_string()),
                        )?;
                        edit_file(&direct_path)?;
                    } else {
                        create_dir_all(indirect_path.parent().unwrap())?;
                        write(
                            &indirect_path,
                            EMPTY_PACKAGE_SET.replace("pset", &self.package_set.to_string()),
                        )?;
                        edit_file(&indirect_path)?;
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
                    edit_file(&direct_path)?;
                }
                (false, true) => {
                    edit_file(&indirect_path)?;
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
    pub fn add_action(
        group: Name,
        package_set: Name,
        package_set_is_file: bool,
    ) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Add,
            group,
            package_set,
            package_set_is_file,
        }))
    }
    pub fn edit_action(group: Name, package_set: Name) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Edit,
            group,
            package_set,
            package_set_is_file: true,
        }))
    }
    pub fn remove_action(group: Name, package_set: Name) -> Result<Box<dyn Action>> {
        Ok(Box::new(ManageAction {
            kind: ManageActionKind::Remove,
            group,
            package_set,
            package_set_is_file: true,
        }))
    }

    fn make_package_set_path(&self, package_set_is_file: bool) -> PathBuf {
        let group_path = PackageRepository::default_path().join(&self.group.as_path());
        if package_set_is_file {
            group_path.join(&format!("{}.yml", self.package_set))
        } else {
            group_path
                .join(&self.package_set.as_path())
                .join("package-set.yml")
        }
    }
}
