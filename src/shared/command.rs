use crate::error::{ErrorKind, Result};
use crate::shared::default_vars;
use crate::shared::env::{var_string_replace, vars_to_env_vars};
use crate::APP_NAME;
use log::LevelFilter;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return the currently selected shell for this terminal session.
///
#[inline]
pub fn user_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
}

///
/// Execute a shell interactively, the shell to run is taken from `user_shell`.
///
pub fn execute_interactive_shell(in_dir: PathBuf) -> Result<()> {
    debug!("execute_interactive_shell ({:?}", in_dir);
    let program = user_shell();
    let mut command = Command::new(&program);
    let _ = command
        .envs(vars_to_env_vars(&default_vars(), &APP_NAME.to_uppercase()))
        .current_dir(in_dir);
    execute(&mut command, &program)
}

///
/// Execute a script string using a shell, the shell to run is taken from `user_shell`.
pub fn execute_shell_command(
    script_string: &str,
    variable_replacements: &HashMap<String, String>,
) -> Result<()> {
    debug!("execute_shell_command ({:?}, ...)", script_string);
    let program = user_shell();
    let mut command = prepare(script_string, variable_replacements);
    execute(&mut command, &program)
}

///
/// Return the currently selected editor for this terminal session.
///
pub fn user_editor() -> String {
    match (env::var("VISUAL"), env::var("EDITOR")) {
        (Ok(cmd), _) => cmd,
        (Err(_), Ok(cmd)) => cmd,
        (_, _) => "vi".to_string(),
    }
}

///
/// Edit the provided file, the editor to run is taken from `user_editor`.
///
pub fn edit_file(file_path: &PathBuf) -> Result<()> {
    debug!("edit_file ({:?})", file_path);
    let program = user_editor();
    let mut command = Command::new(&program);
    let _ = command.arg(file_path);
    execute(&mut command, &program)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref UNQUOTED: Regex = Regex::new(r#"((^|[^\\])")"#).unwrap();
}

const SHELL_ARG: &str = "-c";

fn prepare(script_string: &str, variables: &HashMap<String, String>) -> Command {
    debug!("prepare({:?}, ...)", script_string);
    let safe_script = make_safe(&var_string_replace(script_string, variables));

    let mut command = Command::new(user_shell());
    let _ = command
        .envs(vars_to_env_vars(variables, &APP_NAME.to_uppercase()))
        .args(vec![SHELL_ARG, &safe_script]);
    command
}

fn execute(command: &mut Command, program: &str) -> Result<()> {
    debug!("execute({:?})", command);
    let result = command.output();

    match result {
        Ok(output) => {
            if log::max_level() >= LevelFilter::Debug {
                for line in String::from_utf8(output.stdout).unwrap().split('\n') {
                    if !line.is_empty() {
                        debug!("stdout: {}", line);
                    }
                }
            }

            let exit_status = output.status;
            if exit_status.success() {
                if log::max_level() >= LevelFilter::Debug {
                    for line in String::from_utf8(output.stderr).unwrap().split('\n') {
                        if !line.is_empty() {
                            warn!("stderr: {}", line);
                        }
                    }
                }
                Ok(())
            } else {
                error!(
                    "Error executing command {}, status: {:?}",
                    program, exit_status
                );
                for line in String::from_utf8(output.stderr).unwrap().split('\n') {
                    if !line.is_empty() {
                        error!("stderr: {}", line);
                    }
                }
                Err(
                    ErrorKind::CommandExecutionFailed(program.to_string(), Some(exit_status))
                        .into(),
                )
            }
        }
        Err(err) => {
            error!("Error executing command {}, err: {:?}", program, err);
            Err(ErrorKind::CommandExecutionFailed(program.to_string(), None).into())
        }
    }
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
