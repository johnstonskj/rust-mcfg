use crate::error::Result;
use crate::shared::{FileSystemResource, Name};
use crate::APP_NAME;
use rusqlite::{params, Connection, Row};
use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This is the log where installer actions are recorded, primarily the successful installation
/// of packages within a package set.
///
/// This file is a SQLite3 file, each log entry is a row in the table `installed`.
///
#[derive(Debug)]
pub struct PackageLog(Connection);

///
/// This represents a single log entry in `PackageLog`.
///
#[derive(Debug)]
pub struct InstalledPackage {
    date_time: Option<time::OffsetDateTime>,
    package_set_group_name: Name,
    package_set_name: Name,
    package_name: Name,
    installer_name: Name,
}

///
/// The file name of the installer log.
///
pub const LOG_FILE: &str = "install-log.sql";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FileSystemResource for PackageLog {
    fn default_path() -> PathBuf {
        xdirs::log_dir_for(APP_NAME).unwrap().join(LOG_FILE)
    }

    fn open_from(log_file_path: PathBuf) -> Result<Self> {
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
    /// Add this installed package to the log file. Currently this only logs successful
    /// execution of the associated package installer.
    pub fn log_installed_package(&mut self, package: &InstalledPackage) -> Result<()> {
        trace!("Logging package installation success");
        let date_time = time::OffsetDateTime::now_utc();
        let _ = self.0.execute(
            "INSERT INTO installed (date_time, package_set_group, package_set, package, installer) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                date_time,
                package.package_set_group_name.to_string(),
                package.package_set_name.to_string(),
                package.package_name.to_string(),
                package.installer_name.to_string()],
        )?;
        Ok(())
    }

    /// Return up to `limit` number of rows from the installation history.
    pub fn installed_package_history(&mut self, limit: u32) -> Result<Vec<InstalledPackage>> {
        let mut stmt = self.0.prepare(&format!(
            "SELECT * FROM installed ORDER BY date_time DESC{}",
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
        fn get_name_from_row(row: &Row<'_>, idx: usize) -> rusqlite::Result<Name, rusqlite::Error> {
            let value_string: String = row.get(idx)?;
            let name: Name = Name::from_str(&value_string).unwrap();
            Ok(name)
        }

        Ok(InstalledPackage {
            date_time: row.get(0)?,
            package_set_group_name: get_name_from_row(row, 1)?,
            package_set_name: get_name_from_row(row, 2)?,
            package_name: get_name_from_row(row, 3)?,
            installer_name: get_name_from_row(row, 4)?,
        })
    }
}

impl InstalledPackage {
    /// Create a new record for the install history log.
    pub fn new(
        package_set_group_name: Name,
        package_set_name: Name,
        package_name: Name,
        installer_name: Name,
    ) -> Self {
        Self {
            date_time: None,
            package_set_group_name,
            package_set_name,
            package_name,
            installer_name,
        }
    }

    /// Return the date and time of the installation.
    pub fn date_time(&self) -> &Option<time::OffsetDateTime> {
        &self.date_time
    }

    /// Return the date and time, as a string, of the installation.
    pub fn date_time_str(&self) -> String {
        self.date_time.unwrap().to_string()
    }

    /// Return the name of the package set group that contained the package set.
    pub fn package_set_group_name(&self) -> &Name {
        &self.package_set_group_name
    }

    /// Return the name of the package set that contained the package.
    pub fn package_set_name(&self) -> &Name {
        &self.package_set_name
    }

    /// Return the name of the package that was installed.
    pub fn package_name(&self) -> &Name {
        &self.package_name
    }

    /// Return the name of the installer that acted on the package.
    pub fn installer_name(&self) -> &Name {
        &self.installer_name
    }
}
