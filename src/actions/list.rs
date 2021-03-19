use crate::actions::Action;
use crate::error::Result;
use crate::shared::packages::{PackageRepository, PackageSet, PackageSetGroup};
use crate::shared::{FileSystemResource, Name};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action will list, hierarchically, the package set groups and package sets.
///
#[derive(Debug)]
pub struct ListAction {
    group: Option<Name>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for ListAction {
    fn run(&self) -> Result<()> {
        info!("ListAction::run {:?}", self);
        let package_repository = PackageRepository::open()?;
        if package_repository.is_empty() {
            println!("No package sets found in repository");
        } else {
            match &self.group {
                None => {
                    for group in package_repository.groups() {
                        list_group(&group);
                    }
                }
                Some(group) => {
                    if let Some(found) = package_repository.group(group) {
                        list_group(found);
                    } else {
                        println!("No group found in repository named '{}'", group);
                    }
                }
            }
        }
        Ok(())
    }
}

impl ListAction {
    pub fn new_action(group: Option<Name>) -> Result<Box<dyn Action>> {
        Ok(Box::from(ListAction { group }))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn list_group(group: &PackageSetGroup) {
    println!("* {}", group.name());
    for set in group.package_sets() {
        list_set(set);
    }
}

fn list_set(set: &PackageSet) {
    match set.description() {
        None => {
            println!("  * {}", set.name());
        }
        Some(description) => {
            println!("  * {}: {}", set.name(), description);
        }
    }
}
