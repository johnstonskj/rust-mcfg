use crate::error::{ErrorKind, Result};
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

#[inline]
pub fn user_shell() -> String {
    env::var("SHELL").unwrap_or("bash".to_string())
}

pub fn execute_shell_command(
    script_string: &str,
    variable_replacements: &HashMap<String, String>,
) -> Result<()> {
    debug!("execute_shell_command ({:?}, ...)", script_string);
    let mut command = prepare(script_string, variable_replacements);
    execute(&mut command)
}

pub fn user_editor() -> String {
    match (env::var("VISUAL"), env::var("EDITOR")) {
        (Ok(cmd), _) => cmd,
        (Err(_), Ok(cmd)) => cmd,
        (_, _) => "vi".to_string(),
    }
}

pub fn edit_file(file_path: &PathBuf) -> Result<()> {
    let editor_command = user_editor();
    let result = Command::new(&editor_command).arg(file_path).status();
    if result.is_err() {
        error!(
            "Could not start editor {} for file {:?}",
            editor_command, file_path
        );
        Err(ErrorKind::CommandExecutionFailed(editor_command, None).into())
    } else {
        let exit_status = result.unwrap();
        if exit_status.success() {
            Ok(())
        } else {
            Err(ErrorKind::CommandExecutionFailed(editor_command, Some(exit_status)).into())
        }
    }
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

fn execute(command: &mut Command) -> Result<()> {
    debug!("execute({:?})", command);
    let out = command.output()?;

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
