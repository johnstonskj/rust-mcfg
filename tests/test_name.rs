use mcfg::shared::Name;
use std::str::FromStr;

const CASK_NAMES: &str = include_str!("brew-casks.txt");

const FORMULAE_NAMES: &str = include_str!("brew-formulae.txt");

#[test]
fn test_valid_names() {
    assert!(Name::from_str("hello_world").is_ok());
    assert!(Name::from_str("hello-world").is_ok());
    assert!(Name::from_str("hello/world").is_ok());
    assert!(Name::from_str("hello_world@1.2").is_ok());
    assert!(Name::from_str("99balloons").is_ok());
    assert!(Name::from_str("9").is_ok());
    assert!(Name::from_str("a").is_ok());
}

#[test]
fn test_invalid_names() {
    assert!(Name::from_str("").is_err());
    assert!(Name::from_str("hello world").is_err());
}

#[test]
fn test_all_cask_names_are_valid() {
    for name in CASK_NAMES.lines() {
        if !Name::is_valid(name) {
            panic!("cask {:?} is not a valid Name value", name);
        }
    }
}

#[test]
fn test_all_formulae_names_are_valid() {
    for name in FORMULAE_NAMES.lines() {
        if !Name::is_valid(name) {
            panic!("formulae {:?} is not a valid Name value", name);
        }
    }
}
