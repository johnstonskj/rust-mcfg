/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::packages::{PackageRepository, PackageSet, PackageSetGroup};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ListAction {
    env: Environment,
    group: Option<String>,
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

impl Action for ListAction {
    fn run(&self) -> Result<()> {
        info!("ListAction::run {:?}", self);
        let package_repository = PackageRepository::open(&self.env)?;
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
    pub fn new(env: Environment, group: Option<String>) -> Result<Box<dyn Action>> {
        Ok(Box::from(ListAction { env, group }))
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
