use crate::actions::Action;
use crate::error::Result;
use crate::shared::install_log::PackageLog;
use crate::shared::FileSystemResource;
use prettytable::Table;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This action displays, in a table, the history of installer actions from the log file.
///
#[derive(Debug)]
pub struct HistoryAction {
    limit: u32,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action for HistoryAction {
    fn run(&self) -> Result<()> {
        info!("HistoryAction::run {:?}", self);

        let mut log_db = PackageLog::open()?;
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
    pub fn new(limit: Option<u32>) -> Result<Box<dyn Action>> {
        Ok(Box::from(HistoryAction {
            limit: limit.unwrap_or_default(),
        }))
    }
}
