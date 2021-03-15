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
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::Command;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    Plain,
    String,
    Variable,
    Special,
}

#[derive(Clone, Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    value: String,
}

#[derive(Debug)]
pub struct Tokens(Vec<Token>);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.kind {
                TokenKind::Plain => self.value.to_string(),
                TokenKind::String => format!("'{}'", self.value),
                TokenKind::Variable => format!("{{{{{}}}}}", self.value),
                TokenKind::Special => self.value.to_string(),
            }
        )
    }
}

impl Token {
    fn plain(value: &str) -> Self {
        Self {
            kind: TokenKind::Plain,
            value: value.to_string(),
        }
    }
    fn string(value: &str) -> Self {
        Self {
            kind: TokenKind::String,
            value: value.to_string(),
        }
    }
    fn variable(value: &str) -> Self {
        Self {
            kind: TokenKind::Variable,
            value: value.to_string(),
        }
    }
    fn special(value: &str) -> Self {
        Self {
            kind: TokenKind::Special,
            value: value.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref TOKEN: Regex =
        Regex::new(r#"(".*")|('.*')|(\{\{\S+\}\})|([\|\&<>]+)|([^\\"\{ ]\S+)"#).unwrap();
}

impl Default for Tokens {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl FromStr for Tokens {
    type Err = crate::error::Error;

    fn from_str(cmd_str: &str) -> std::result::Result<Self, Self::Err> {
        let mut result_tokens: Vec<Token> = Default::default();
        for token_capture in TOKEN.captures_iter(&cmd_str) {
            if let Some(token) = token_capture.get(1) {
                let token_str = token.as_str();
                let token_str = &token_str[1..token_str.len() - 1];
                result_tokens.push(Token::string(token_str));
            } else if let Some(token) = token_capture.get(2) {
                let token_str = token.as_str();
                let token_str = &token_str[1..token_str.len() - 1];
                result_tokens.push(Token::string(token_str));
            } else if let Some(token) = token_capture.get(3) {
                let token_str = token.as_str();
                let token_str = &token_str[2..token_str.len() - 2];
                result_tokens.push(Token::variable(token_str));
            } else if let Some(token) = token_capture.get(4) {
                result_tokens.push(Token::special(token.as_str()));
            } else if let Some(token) = token_capture.get(5) {
                result_tokens.push(Token::plain(token.as_str()));
            }
        }
        if result_tokens.len() > 0 {
            Ok(Self(result_tokens))
        } else {
            Err(ErrorKind::InvalidCommandString(cmd_str.to_string()).into())
        }
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Tokens {
    pub fn execute_command(
        &mut self,
        variable_replacements: &HashMap<String, String>,
    ) -> Result<()> {
        self.replace_variables(variable_replacements);

        if self.0.iter().any(|t| t.kind == TokenKind::Special) {
            // Let bash deal with pipes and redirects
            self.0 = vec![
                Token::plain(variable_replacements.get("shell").unwrap()),
                Token::plain("-c"),
                Token::plain(&self.to_string()),
            ];
        }
        trace!("variable replacements: {:?}", variable_replacements);

        // turn command variables into environment variables
        let mut env_vars: HashMap<String, String> = variable_replacements
            .iter()
            .map(|(k, v)| (format!("MCFG_{}", k.to_uppercase()), v.clone()))
            .collect();

        // add to the system path
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
        trace!("environment variables: {:?}", env_vars);

        let mut token_iter = self.tokens();
        let first = token_iter.next().unwrap();
        let mut command = Command::new(first.to_string());
        let command = command
            .envs(env_vars)
            .args(token_iter.map(|t| t.value.to_string()));
        trace!("executing Command: {:?}", &command);

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
                        debug!("stderr: {}", line);
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

    pub fn replace_variables(&mut self, variable_replacements: &HashMap<String, String>) {
        if self.tokens().any(|t| t.kind == TokenKind::Variable) {
            for token in self.0.iter_mut() {
                if token.kind == TokenKind::Variable {
                    if let Some(value) = variable_replacements.get(&token.value) {
                        token.kind = TokenKind::Plain;
                        token.value = value.to_string();
                    } else {
                        warn!("No variable replacement for name {:?}", &token.value);
                    }
                }
            }
        }
    }

    fn tokens(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens_eq(
        tokenize: &str,
        tokens: Vec<Token>,
        replacement_values: Option<&HashMap<String, String>>,
    ) {
        println!(">>> {}", tokenize);
        let mut tokenized = Tokens::from_str(tokenize).unwrap();
        println!("<<< {:?}", tokenized);
        if let Some(replacement_values) = replacement_values {
            tokenized.replace_variables(replacement_values);
        }
        assert_eq!(tokenized.0, tokens);
    }

    fn test_replacement_values() -> HashMap<String, String> {
        let mut replacement_values: HashMap<String, String> = Default::default();
        let _ = replacement_values.insert("package".to_string(), "lux".to_string());
        replacement_values
    }

    #[test]
    fn test_tokenizer() {
        assert_tokens_eq(
            "brew cask install {{package}}",
            vec![
                Token::plain("brew"),
                Token::plain("cask"),
                Token::plain("install"),
                Token::variable("package"),
            ],
            None,
        );

        assert_tokens_eq(
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
            vec![
                Token::plain("curl"),
                Token::plain("--proto"),
                Token::string("=https"),
                Token::plain("--tlsv1.2"),
                Token::plain("-sSf"),
                Token::plain("https://sh.rustup.rs"),
                Token::special("|"),
                Token::plain("sh"),
            ],
            None,
        );

        assert_tokens_eq(
            "curl --proto \"=https\" --tlsv1.2 -sSf https://sh.rustup.rs | sh",
            vec![
                Token::plain("curl"),
                Token::plain("--proto"),
                Token::string("=https"),
                Token::plain("--tlsv1.2"),
                Token::plain("-sSf"),
                Token::plain("https://sh.rustup.rs"),
                Token::special("|"),
                Token::plain("sh"),
            ],
            None,
        );
    }

    #[test]
    fn test_empty_string_failure() {
        let result = Tokens::from_str("");
        assert!(result.is_err())
    }

    #[test]
    fn test_normalize_tokens() {
        assert_tokens_eq(
            "brew cask install {{package}}",
            vec![
                Token::plain("brew"),
                Token::plain("cask"),
                Token::plain("install"),
                Token::plain("lux"),
            ],
            Some(&test_replacement_values()),
        );
    }
}
