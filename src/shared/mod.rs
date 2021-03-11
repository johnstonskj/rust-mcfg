/*!
One-line description.

More detailed description, with

# Example

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
    Default,
    Application,
    Script(String),
    Language(String),
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

pub mod command;

pub mod environment;

pub mod installer;

pub mod install_log;

pub mod packages;
