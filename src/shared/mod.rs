/*!
Common modules used by the actions defined in [`crate::actions`](../actions/index.html).
*/

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    Macos,
    Linux,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum PackageKind {
    Application,
    Default,
    Language(String),
    Script(String),
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
    #[cfg(target_os = "macos")]
    pub const CURRENT: Platform = Platform::Macos;
    #[cfg(target_os = "linux")]
    pub const CURRENT: Platform = Platform::Linux;

    pub fn is_current(other: &Option<Platform>) -> bool {
        Self::CURRENT.is_match(other)
    }

    pub fn is_match(&self, other: &Option<Platform>) -> bool {
        *self == other.as_ref().cloned().unwrap_or(Self::CURRENT)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for PackageKind {
    fn default() -> Self {
        PackageKind::Default
    }
}

impl PackageKind {
    pub fn is_match(&self, other: &Option<PackageKind>) -> bool {
        *self == other.as_ref().cloned().unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod command;
pub use command::Tokens;

#[doc(hidden)]
pub mod editor;

#[doc(hidden)]
pub mod installer;
pub use installer::{InstallActionKind, Installer, InstallerCommandKind, InstallerRegistry};

#[doc(hidden)]
pub mod install_log;
pub use install_log::{InstalledPackage, PackageLog};

#[doc(hidden)]
pub mod packages;
pub use packages::{Package, PackageRepository, PackageSet, PackageSetGroup};
