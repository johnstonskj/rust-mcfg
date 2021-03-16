/*!
One-line description.

More detailed description, with

# Example

*/

use crate::shared::{
    InstallActionKind, Package, PackageRepository, PackageSet, Platform, ShellCommand,
};
use dirs_next::home_dir;
use regex::Regex;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn default_vars() -> HashMap<String, String> {
    let mut replacements: HashMap<String, String> = Default::default();
    let _ = replacements.insert(
        "home".to_string(),
        home_dir().unwrap().to_string_lossy().to_string(),
    );
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

pub fn add_action_vars(
    action: &InstallActionKind,
    default_vars: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut replacements = default_vars.clone();
    let _ = replacements.insert("command_action".to_string(), action.to_string());
    debug!("add_action_vars: {:?}", &replacements);
    replacements
}

pub fn add_package_set_action_vars(
    package_set: &PackageSet,
    action_vars: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut replacements = action_vars.clone();
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

pub fn add_package_action_vars(
    package: &Package,
    package_set_vars: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut replacements = package_set_vars.clone();
    let _ = replacements.insert("package_name".to_string(), package.name().clone());
    let _ = replacements.insert(
        "package_config_path".to_string(),
        xdirs::config_dir_for(package.name())
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_string(),
    );
    let _ = replacements.insert(
        "package_data_local_path".to_string(),
        xdirs::data_local_dir_for(package.name())
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_string(),
    );
    let _ = replacements.insert(
        "package_log_path".to_string(),
        xdirs::log_dir_for(package.name())
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .to_string(),
    );

    debug!("add_package_action_vars: {:?}", &replacements);
    replacements
}

pub fn add_other_vars(
    existing_vars: &HashMap<String, String>,
    other_vars: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut replacements = existing_vars.clone();

    for (key, value) in other_vars {
        let _ = replacements.insert(
            var_string_replace(key, &replacements),
            var_string_replace(value, &replacements),
        );
    }

    debug!("add_other_vars: {:?}", &replacements);
    replacements
}

pub fn vars_to_env_vars(
    variables: &HashMap<String, String>,
    prefix: &str,
) -> HashMap<String, String> {
    let mut env_vars: HashMap<String, String> = variables
        .iter()
        .map(|(k, v)| (format!("{}_{}", prefix, k.to_uppercase()), v.clone()))
        .collect();
    if let Ok(current_path) = std::env::var("PATH") {
        let _ = env_vars.insert(
            "PATH".to_string(),
            format!(
                "{}:{:?}/bin",
                current_path,
                PackageRepository::default_local_path()
            ),
        );
    }
    env_vars
}

lazy_static! {
    static ref VARIABLES: Regex = Regex::new(r#"(\{\{[a-zA-Z0-9\-_:]+\}\})"#).unwrap();
}

pub fn var_string_replace(string: &str, vars: &HashMap<String, String>) -> String {
    let mut out_string = String::new();

    let mut from: usize = 0;
    for capture in VARIABLES.captures_iter(string) {
        let var = capture.get(1).unwrap();
        out_string.push_str(&string[from..var.start()]);
        let var_name = var.as_str();
        let var_name = &var_name[2..var_name.len() - 2];
        if let Some(replacement) = vars.get(var_name) {
            out_string.push_str(replacement)
        } else {
            warn!("No variable named {:?} in replacements", var_name);
            out_string.push_str(var_name);
        }
        from = var.end();
    }
    out_string.push_str(&string[from..]);

    out_string
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_replace_variables_in_string() {
        let replacements: HashMap<String, String> = vec![("name", "wallace")]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        assert_eq!(var_string_replace("{{name}}", &replacements), "wallace");

        assert_eq!(
            var_string_replace("hello {{name}}!", &replacements),
            "hello wallace!"
        );

        assert_eq!(
            var_string_replace("{{salutation}} {{name}}!", &replacements),
            "salutation wallace!"
        );
    }

    #[test]
    fn test_replace_variables_in_variables() {
        let replacements = default_vars();

        let test_vars: HashMap<String, String> = vec![
            (
                "platform-path",
                "{{home}}/x-data/{{platform_os}}/{{platform_arch}}",
            ),
            ("for-{{platform_os}}", "this is my platform"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let new_replacements = add_other_vars(&replacements, &test_vars);
        println!("{:?}", new_replacements);

        assert_eq!(
            new_replacements.get("platform-path").unwrap(),
            &format!(
                "{}/x-data/{}/{}",
                home_dir().unwrap().to_string_lossy().to_string(),
                std::env::consts::OS,
                std::env::consts::ARCH
            )
        );

        assert_eq!(
            new_replacements
                .get(&format!("for-{}", std::env::consts::OS))
                .unwrap(),
            "this is my platform"
        );
    }
}
