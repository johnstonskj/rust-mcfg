/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::{FileSystemResource, PackageRepository};
use git2::{ErrorClass, ErrorCode, Repository};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct RefreshAction {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for RefreshAction {
    fn run(&self) -> Result<()> {
        info!("RefreshAction::run refreshing local git");
        // TODO: check if it exists!
        match Repository::open(PackageRepository::default_path()) {
            Err(e) => {
                if e.code() == ErrorCode::NotFound && e.class() == ErrorClass::Repository {
                    debug!("Local dir does not contain a Git repo, ignoring refresh");
                    Ok(())
                } else {
                    Err(e.into())
                }
            }
            Ok(repo) => {
                let head_ref = repo.head();
                let head_ref = head_ref.unwrap();
                let head_ref = head_ref.name().unwrap();
                debug!("fetching remote reference {}", head_ref);

                repo.find_remote("origin")?.fetch(&[head_ref], None, None)?;
                // TODO: stop if it is not remote

                let fetch_head = repo.find_reference("FETCH_HEAD")?;
                let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
                let analysis = repo.merge_analysis(&[&fetch_commit])?;
                if analysis.0.is_up_to_date() {
                    debug!("No remote changes, repository untouched");
                    Ok(())
                } else if analysis.0.is_fast_forward() {
                    debug!("fast-forwarding changes from remote");
                    let mut reference = repo.find_reference(head_ref)?;
                    // returns another reference, we can ignore it.
                    let _ = reference.set_target(fetch_commit.id(), "Fast-Forward")?;
                    repo.set_head(head_ref)?;
                    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
                    Ok(())
                } else {
                    panic!("Fast-Forward only");
                }
            }
        }
    }
}

impl RefreshAction {
    pub fn new() -> Result<Box<dyn Action>> {
        Ok(Box::from(RefreshAction {}))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
