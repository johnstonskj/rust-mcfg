/*!
These actions are complete stand-alone entry points for the command-line tools. These may be
invoked by other tools, however they may have side-effects such as writing to stdout.

# Example

The following is an example [`Action`](trait.action.html) implementation that does very little. To understand how to
use existing actions, or create new ones, see the [User Guide](https://simonkjohnston.life/rust-mcfg/).

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

///
/// Implemented by the actions exposed by the CLI.
///
pub trait Action: Debug {
    /// Run this action, this assumes all information was passed to the action during creation.
    fn run(&self) -> Result<()>;
}

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
use std::fmt::Debug;
pub use upgrade::UpdateSelfAction;
