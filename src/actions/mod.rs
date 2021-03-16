/*!
These actions are complete stand-alone entry points for the command-line tools. These may be
invoked by other tools, however they may have side-effects such as writing to stdout.

# Example

The following is an example `Action` implementation that does very little.

```rust
use mcfg::actions::Action;
use mcfg::error::Result;

#[derive(Debug)]
pub struct ExampleAction {}

impl Action for ExampleAction {
    fn run(&self) -> Result<()> {
        println!("ListAction::run {:?}", self);
        Ok(())
    }
}

impl ExampleAction {
    pub fn new() -> Result<Box<dyn Action>> {
        Ok(Box::from(ExampleAction {}))
    }
}

```

*/

use crate::error::Result;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Action {
    fn run(&self) -> Result<()>;
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod installers;
pub use installers::EditInstallersAction;

#[doc(hidden)]
mod init;
pub use init::InitAction;

#[doc(hidden)]
mod history;
pub use history::HistoryAction;

#[doc(hidden)]
mod install;
pub use install::InstallAction;

#[doc(hidden)]
mod list;
pub use list::ListAction;

#[doc(hidden)]
mod manage;
pub use manage::ManageAction;

#[doc(hidden)]
mod paths;
pub use paths::ShowPathsAction;

#[cfg(feature = "remove-self")]
#[doc(hidden)]
mod remove_self;
#[cfg(feature = "remove-self")]
pub use remove_self::RemoveSelfAction;

#[doc(hidden)]
mod refresh;
pub use refresh::RefreshAction;

#[doc(hidden)]
mod upgrade;
pub use upgrade::UpdateSelfAction;
