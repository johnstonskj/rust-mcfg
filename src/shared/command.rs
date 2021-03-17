use crate::error::{ErrorKind, Result};
use crate::shared::env::{var_string_replace, vars_to_env_vars};
use crate::APP_NAME;
use log::LevelFilter;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::process::Command;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A ShellCommand represents a wrapper around `std::process::Command` for commands that are
/// executed via a shell. It also clearly provides a prepare/execute model which is a little
/// cleaner than the reuse model provided by `std::process::Command`.
///
/// The actual shell used to execute the command is taken from the `SHELL` environment variable
/// if it exists, or `bash` if it does not.
///
#[derive(Clone, Debug, PartialEq)]
pub struct ShellCommand {
    variables: HashMap<String, String>,
}

///
/// The result of `ShellCommand::prepare` this may be executed any number of times, however it's
/// details (arguments, environment, etc.) are fixed.
///
#[derive(Debug)]
pub struct ShellCommandPlan {
    command: Command,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref UNQUOTED: Regex = Regex::new(r#"((^|[^\\])")"#).unwrap();
}

impl ShellCommandPlan {
    const SHELL_ARG: &'static str = "-c";

    fn new(script_string: &str, variables: &HashMap<String, String>) -> Self {
        debug!("ShellCommandPlan::new({:?})", script_string);
        let safe_script = make_safe(&var_string_replace(script_string, variables));

        let mut command = Command::new(ShellCommand::run_shell());
        let _ = command
            .envs(vars_to_env_vars(variables, &APP_NAME.to_uppercase()))
            .args(vec![Self::SHELL_ARG, &safe_script]);
        trace!("ShellCommandPlan::new: command={:?}", command);
        Self { command }
    }

    ///
    /// Execute the prepared command.
    ///
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
    /// Determine the shell command to run, this will use the value of the `SHELL` environment
    /// variable if set, or fall back to `bash`.
    pub fn run_shell() -> String {
        env::var("SHELL").unwrap_or("bash".to_string())
    }

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
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_make_safe() {
        assert_eq!(make_safe("hello simon"), r#"hello simon"#);
        assert_eq!(make_safe("hello \"simon\""), r#"hello \"simon\""#);
        assert_eq!(make_safe("\"hello\" simon"), r#"\"hello\" simon"#);
    }
}
