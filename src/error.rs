/*!
The common `ErrorKind`, `Error`, and `Result` types used throughout.
*/

#![allow(missing_docs)]

use crate::shared::{PackageKind, Platform};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        #[doc("Invalid configuration value")]
        InvalidConfigValue(field: String, value: String) {
            description("Invalid configuration value")
            display("Invalid value for configuration field '{}': '{}'", field, value)
        }

        #[doc("No package set found in group")]
        NoPackageSet(group: String, package_set: String) {
            description("No package set found in group")
            display("No package set '{}' found in group '{}'", package_set, group)
        }

        #[doc("No package set found in group")]
        PackagePlatformError(package: String) {
            description("The package cannot be installed on this platform")
            display("The package '{}' cannot be installed on platform {:?}", package, Platform::CURRENT)
        }

        #[doc("No installer found for package kind")]
        NoInstallerForKind(kind: PackageKind) {
            description("No installer found for package kind")
            display("No installer found for platform '{:?}' and package kind '{:?}'", Platform::CURRENT, kind)
        }

        #[doc("Wrong installer used for package kind")]
        WrongInstallerForKind(kind: PackageKind) {
            description("Wrong installer used for package kind")
            display("Wrong installer used for package kind '{:?}'", kind)
        }

        #[doc("Invalid command string for installer action")]
        InvalidCommandString(cmd_str: String) {
            description("Invalid command string for installer action")
            display("Invalid command string for installer action: {:?}", cmd_str)
        }

        #[doc("Command string for install action failed to run")]
        InstallerCommandFailed {
            description("Command string for install action failed to run")
            display("Command string for install action failed to run")
        }
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Git(::git2::Error);
        Io(::std::io::Error);
        Serialization(::serde_yaml::Error);
        Sql(::rusqlite::Error);
    }
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
