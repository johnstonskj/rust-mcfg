/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{ErrorKind, Result};
use crate::shared::PackageRepository;
use log::LevelFilter;
use regex::Regex;
use std::collections::HashMap;
use std::env::var;
use std::process::Command;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct ShellCommand {
    variables: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ShellCommandPlan {
    command: Command,
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

lazy_static! {
    static ref VARIABLES: Regex = Regex::new(r#"(\{\{[a-zA-Z0-9\-_:]+\}\})"#).unwrap();
    static ref UNQUOTED: Regex = Regex::new(r#"((^|[^\\])")"#).unwrap();
}

const SHELL_CMD: &'static str = "bash";
const SHELL_ARG: &'static str = "-c";

impl ShellCommandPlan {
    pub fn new(script_string: &str, variables: &HashMap<String, String>) -> Self {
        debug!("ShellCommandPlan::new({:?})", script_string);
        let safe_script = make_safe(&replace_variables(script_string, variables));

        let mut command = Command::new(SHELL_CMD);
        let _ = command
            .envs(variables_to_environment(variables))
            .args(vec![SHELL_ARG, &safe_script]);
        trace!("ShellCommandPlan::new: command={:?}", command);
        Self { command }
    }

    pub fn execute(&mut self) -> Result<()> {
        debug!("ShellCommandPlan::execute()");
        let out = self.command.output()?;

        if log::max_level() >= LevelFilter::Debug {
            for line in String::from_utf8(out.stdout).unwrap().split('\n') {
                if !line.is_empty() {
                    debug!("stdout: {}", line);
                }
            }
        }

        if out.status.success() {
            if log::max_level() >= LevelFilter::Debug {
                for line in String::from_utf8(out.stderr).unwrap().split('\n') {
                    if !line.is_empty() {
                        warn!("stderr: {}", line);
                    }
                }
            }
            Ok(())
        } else {
            for line in String::from_utf8(out.stderr).unwrap().split('\n') {
                if !line.is_empty() {
                    error!("stderr: {}", line);
                }
            }
            Err(ErrorKind::InstallerCommandFailed.into())
        }
    }
}

impl Default for ShellCommand {
    fn default() -> Self {
        Self {
            variables: Default::default(),
        }
    }
}

impl ShellCommand {
    pub fn new(variables: HashMap<String, String>) -> Self {
        debug!("ShellCommand::new(..)");
        Self { variables }
    }

    pub fn prepare(&self, script_string: &str) -> ShellCommandPlan {
        debug!("ShellCommand::prepare({:?})", script_string);
        ShellCommandPlan::new(script_string, &self.variables)
    }

    pub fn execute(&self, script_string: &str) -> Result<()> {
        debug!("ShellCommand::execute({:?})", script_string);
        self.prepare(script_string).execute()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn variables_to_environment(variables: &HashMap<String, String>) -> HashMap<String, String> {
    let mut env_vars: HashMap<String, String> = variables
        .iter()
        .map(|(k, v)| (format!("MCFG_{}", k.to_uppercase()), v.clone()))
        .collect();
    if let Ok(current_path) = var("PATH") {
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

fn replace_variables(script_string: &str, variables: &HashMap<String, String>) -> String {
    let mut out_string = String::new();

    let mut from: usize = 0;
    for capture in VARIABLES.captures_iter(script_string) {
        let var = capture.get(1).unwrap();
        out_string.push_str(&script_string[from..var.start()]);
        let var_name = var.as_str();
        let var_name = &var_name[2..var_name.len() - 2];
        if let Some(replacement) = variables.get(var_name) {
            out_string.push_str(replacement)
        } else {
            warn!("No variable named {:?} in replacements", var_name);
            out_string.push_str(var_name);
        }
        from = var.end();
    }
    out_string.push_str(&script_string[from..]);

    out_string
}

fn make_safe(script_string: &str) -> String {
    let mut out_string = String::new();

    let mut from: usize = 0;
    for capture in UNQUOTED.captures_iter(script_string) {
        let var = capture.get(1).unwrap();
        out_string.push_str(&script_string[from..var.start()]);
        let unquoted = var.as_str();
        let unquoted = &unquoted[..unquoted.len() - 1];
        out_string.push_str(unquoted);
        out_string.push_str("\\\"");
        from = var.end();
    }
    out_string.push_str(&script_string[from..]);

    out_string
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_replace_variables() {
        let replacements: HashMap<String, String> = vec![("name", "wallace")]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        assert_eq!(replace_variables("{{name}}", &replacements), "wallace");

        assert_eq!(
            replace_variables("hello {{name}}!", &replacements),
            "hello wallace!"
        );

        assert_eq!(
            replace_variables("{{salutation}} {{name}}!", &replacements),
            "salutation wallace!"
        );
    }

    #[test]
    fn test_make_safe() {
        assert_eq!(make_safe("hello simon"), r#"hello simon"#);
        assert_eq!(make_safe("hello \"simon\""), r#"hello \"simon\""#);
        assert_eq!(make_safe("\"hello\" simon"), r#"\"hello\" simon"#);
    }
}
