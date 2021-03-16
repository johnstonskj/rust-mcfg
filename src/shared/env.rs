/*!
One-line description.

More detailed description, with

# Example

*/

use crate::shared::{InstallActionKind, PackageRepository, PackageSet, Platform, ShellCommand};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn default_vars() -> HashMap<String, String> {
    let mut replacements: HashMap<String, String> = Default::default();
    let _ = replacements.insert(
        "command_log_level".to_string(),
        log::max_level().to_string().to_lowercase(),
    );
    let _ = replacements.insert(
        "command_shell".to_string(),
        ShellCommand::SHELL_CMD.to_string(),
    );
    let _ = replacements.insert(
        "local_download_path".to_string(),
        dirs_next::download_dir()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    );
    let _ = replacements.insert("platform".to_string(), Platform::CURRENT.to_string());
    let _ = replacements.insert(
        "platform_family".to_string(),
        std::env::consts::FAMILY.to_string(),
    );
    let _ = replacements.insert("platform_os".to_string(), std::env::consts::OS.to_string());
    let _ = replacements.insert(
        "platform_arch".to_string(),
        std::env::consts::ARCH.to_string(),
    );
    let _ = replacements.insert(
        "repo_config_path".to_string(),
        PackageRepository::default_config_path()
            .to_string_lossy()
            .to_string(),
    );
    let _ = replacements.insert(
        "repo_local_path".to_string(),
        PackageRepository::default_local_path()
            .to_string_lossy()
            .to_string(),
    );

    debug!("default_vars: {:?}", &replacements);
    replacements
}

pub fn action_vars(action: &InstallActionKind) -> HashMap<String, String> {
    let mut replacements = default_vars();
    let _ = replacements.insert("command_action".to_string(), action.to_string());
    debug!("action_vars: {:?}", &replacements);
    replacements
}

pub fn package_set_action_vars(
    package_set: &PackageSet,
    action: &InstallActionKind,
) -> HashMap<String, String> {
    let mut replacements = action_vars(action);
    let _ = replacements.insert(
        "package_set_name".to_string(),
        package_set.name().to_string(),
    );
    let _ = replacements.insert(
        "package_set_file".to_string(),
        package_set
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_string(),
    );
    let _ = replacements.insert(
        "package_set_path".to_string(),
        package_set
            .path()
            .parent()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_string(),
    );

    debug!("package_set_action_vars: {:?}", &replacements);
    replacements
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
