use mcfg::shared::environment::Environment;
use mcfg::shared::installer::InstallerRegistry;
use mcfg::shared::packages::PackageRepository;
use std::path::PathBuf;

#[test]
fn test_environment_config() {
    let env = Environment::with_roots(
        PathBuf::from("tests/root/config"),
        PathBuf::from("tests/root/logs"),
        PathBuf::from("tests/root/data"),
    );
    println!("{:?}", env);
    assert!(env.has_config_path());
    assert!(env.has_repository_path());
    assert!(env.has_installer_file());
}

#[test]
fn test_environment_installer_file() {
    let env = Environment::with_roots(
        PathBuf::from("tests/root/config"),
        PathBuf::from("tests/root/logs"),
        PathBuf::from("tests/root/data"),
    );
    let registry = InstallerRegistry::read(&env).unwrap();
    assert_eq!(registry.installers().count(), 4);
}

#[test]
fn test_environment_repository() {
    let env = Environment::with_roots(
        PathBuf::from("tests/root/config"),
        PathBuf::from("tests/root/logs"),
        PathBuf::from("tests/root/data"),
    );
    let repository = PackageRepository::open(&env).unwrap();
    assert!(!repository.is_empty());
}
