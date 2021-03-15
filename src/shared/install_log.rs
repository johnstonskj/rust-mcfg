/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::Result;
use crate::shared::FileSystemResource;
use crate::APP_NAME;
use rusqlite::{params, Connection, Row};
use std::convert::TryFrom;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PackageLog(Connection);

#[derive(Debug)]
pub struct InstalledPackage {
    date_time: Option<time::OffsetDateTime>,
    package_set_group_name: String,
    package_set_name: String,
    package_name: String,
    installer_name: String,
}

pub const LOG_FILE: &str = "install-log.sql";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FileSystemResource for PackageLog {
    fn default_path() -> PathBuf {
        xdirs::log_dir_for(APP_NAME).unwrap().join(LOG_FILE)
    }

    fn actual_open(log_file_path: PathBuf) -> Result<Self> {
        let connection = if !log_file_path.is_file() {
            debug!(
                "PackageLog::open creating new log file: {:?}",
                log_file_path
            );
            std::fs::create_dir_all(log_file_path.parent().unwrap()).unwrap();
            let db = Connection::open(log_file_path).unwrap();
            let _ = db.execute(
                r##"CREATE TABLE installed (
    date_time DATETIME NOT NULL,
    package_set_group TEXT NOT NULL,
    package_set TEXT NOT NULL,
    package TEXT NOT NULL,
    installer TEXT NOT NULL
)"##,
                params![],
            )?;
            db
        } else {
            debug!(
                "PackageLog::open opening existing log file {:?}",
                log_file_path
            );
            Connection::open(log_file_path)?
        };
        Ok(PackageLog(connection))
    }
}

impl PackageLog {
    pub fn log_installed_package(&mut self, package: &InstalledPackage) -> Result<()> {
        trace!("Logging package installation success");
        let date_time = time::OffsetDateTime::now_utc();
        let _ = self.0.execute(
            "INSERT INTO installed (date_time, package_set_group, package_set, package, installer) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![date_time, package.package_set_group_name, package.package_set_name, package.package_name, package.installer_name],
        )?;
        Ok(())
    }

    pub fn installed_package_history(&mut self, limit: u32) -> Result<Vec<InstalledPackage>> {
        let mut stmt = self.0.prepare(&format!(
            "SELECT * FROM installed{}",
            if limit > 0 {
                format!(" LIMIT {}", limit)
            } else {
                String::new()
            }
        ))?;
        let result_iter = stmt.query_map(params![], |row| InstalledPackage::try_from(row))?;
        Ok(result_iter.map(|ip| ip.unwrap()).collect())
    }
}

// ------------------------------------------------------------------------------------------------

impl<'stmt> TryFrom<&Row<'stmt>> for InstalledPackage {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'stmt>) -> rusqlite::Result<Self, Self::Error> {
        Ok(InstalledPackage {
            date_time: row.get(0)?,
            package_set_group_name: row.get(1)?,
            package_set_name: row.get(2)?,
            package_name: row.get(3)?,
            installer_name: row.get(4)?,
        })
    }
}

impl InstalledPackage {
    pub fn new(
        package_set_group_name: &str,
        package_set_name: &str,
        package_name: &str,
        installer_name: &str,
    ) -> Self {
        Self {
            date_time: None,
            package_set_group_name: package_set_group_name.to_string(),
            package_set_name: package_set_name.to_string(),
            package_name: package_name.to_string(),
            installer_name: installer_name.to_string(),
        }
    }

    pub fn date_time(&self) -> &Option<time::OffsetDateTime> {
        &self.date_time
    }

    pub fn date_time_str(&self) -> String {
        self.date_time.unwrap().to_string()
    }

    pub fn package_set_group_name(&self) -> &String {
        &self.package_set_group_name
    }

    pub fn package_set_name(&self) -> &String {
        &self.package_set_name
    }

    pub fn package_name(&self) -> &String {
        &self.package_name
    }

    pub fn installer_name(&self) -> &String {
        &self.installer_name
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
