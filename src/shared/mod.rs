/*!
Common modules used by the actions defined in the [`actions`](../actions/index.html) module.

* Models
  * **package sets** - the things you keep in your repository
  * **installers** - the things that install package sets
* Logging
  * **install log** - the place we record what the installers did
* Command Execution
  * **shell command** - the way we execute installers
  * **editor** - when we need to edit things
  * **env** - the environment variables we set for executing installers
*/

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration captures the set of supported platforms.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    #[allow(missing_docs)]
    Macos,
    #[allow(missing_docs)]
    Linux,
}

///
/// This enumeration captures the set of support package types.
///
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum PackageKind {
    Application,
    Default,
    Language(String),
}

///
/// A Trait that is used by model elements that have file-system backed persistence.
///
pub trait FileSystemResource {
    /// The assumed default path for this resource.
    fn default_path() -> PathBuf;

    /// Open the resource from it's default location.
    fn open() -> Result<Self>
    where
        Self: Sized,
    {
        Self::open_from(Self::default_path())
    }

    /// Open the resource from the provided location.
    fn open_from(non_default_path: PathBuf) -> Result<Self>
    where
        Self: Sized;

    /// Returns `true` if the resource exist as a directory at it's default location, else `false`.
    fn is_dir() -> bool {
        Self::default_path().is_dir()
    }

    /// Returns `true` if the resource exist as a file at it's default location, else `false`.
    fn is_file() -> bool {
        Self::default_path().is_file()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Platform {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Macos => "macos",
                Self::Linux => "linux",
            },
        )
    }
}

impl Platform {
    /// The platform you are running on.
    #[cfg(target_os = "macos")]
    pub const CURRENT: Platform = Platform::Macos;
    #[cfg(target_os = "linux")]
    pub const CURRENT: Platform = Platform::Linux;

    /// Returns `true` if the provided platform `other` a match with `Self::Current`, else `false`.
    pub fn is_current(other: &Option<Platform>) -> bool {
        Self::CURRENT.is_match(other)
    }

    /// Returns `true` if the two Platform values are equal, **or** if `other` is `None`, else
    /// `false`.
    pub fn is_match(&self, other: &Option<Platform>) -> bool {
        *self == other.as_ref().cloned().unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for PackageKind {
    fn default() -> Self {
        PackageKind::Default
    }
}

impl PackageKind {
    /// Returns `true` if the two Platform values are equal, **or** if `other` is `None`, else
    /// `false`.
    pub fn is_match(&self, other: &Option<PackageKind>) -> bool {
        *self == other.as_ref().cloned().unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod command;
pub use command::{
    edit_file, execute_interactive_shell, execute_shell_command, user_editor, user_shell,
};

#[doc(hidden)]
mod counter;
pub use counter::StepCounter;

#[doc(hidden)]
pub mod env;
pub use env::{
    add_action_vars, add_other_vars, add_package_action_vars, add_package_set_action_vars,
    default_vars,
};

#[doc(hidden)]
pub mod installer;
pub use installer::{InstallActionKind, Installer, InstallerRegistry};

#[doc(hidden)]
pub mod install_log;
pub use install_log::{InstalledPackage, PackageLog};

#[doc(hidden)]
pub mod packages;
pub use packages::{Package, PackageRepository, PackageSet, PackageSetActions, PackageSetGroup};

///
/// Builder implementations to construct package and installer related struct types.
///
pub mod builders {
    pub use super::installer::builders::InstallerBuilder;
    pub use super::packages::builders::{
        PackageBuilder, PackageSetBuilder, PackageSetGroupBuilder,
    };
}
