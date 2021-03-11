/*!
One-line description.

More detailed description, with

# Example

*/

use crate::actions::Action;
use crate::error::Result;
use crate::shared::environment::Environment;
use crate::shared::install_log::PackageLog;
use prettytable::Table;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct HistoryAction {
    env: Environment,
    limit: u32,
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

impl Action for HistoryAction {
    fn run(&self) -> Result<()> {
        info!("HistoryAction::run {:?}", self);

        let mut log_db = PackageLog::open(&self.env.log_file_path())?;
        let history = log_db.installed_package_history(self.limit)?;

        let mut table = Table::new();
        table.set_titles(row!["Date", "Group", "Set", "Package", "Installer"]);

        for db_row in history {
            let _ = table.add_row(row![
                db_row.date_time_str(),
                db_row.package_set_group_name(),
                db_row.package_set_name(),
                db_row.package_name(),
                db_row.installer_name()
            ]);
        }

        let _ = table.printstd();

        Ok(())
    }
}

impl HistoryAction {
    pub fn new(env: Environment, limit: Option<u32>) -> Result<Box<dyn Action>> {
        Ok(Box::from(HistoryAction {
            env,
            limit: limit.unwrap_or_default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
