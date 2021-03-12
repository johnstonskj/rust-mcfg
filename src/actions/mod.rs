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

mod installers;
pub use installers::EditInstallersAction;

mod init;
pub use init::InitAction;

mod history;
pub use history::HistoryAction;

mod install;
pub use install::InstallAction;

mod list;
pub use list::ListAction;

mod manage;
pub use manage::ManageAction;

mod paths;
pub use paths::ShowPathsAction;

mod refresh;
pub use refresh::RefreshAction;

mod upgrade;
pub use upgrade::UpdateSelfAction;
