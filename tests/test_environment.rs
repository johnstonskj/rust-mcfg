use mcfg::shared::installer::InstallerRegistry;
use mcfg::shared::packages::PackageRepository;
use std::path::PathBuf;
use pretty_assertions{assert_eq};

#[test]
fn test_environment_installer_file() {
    let registry =
        InstallerRegistry::open_from(PathBuf::from("tests/root/config/installers.yml")).unwrap();
    assert_eq!(registry.installers().count(), 5);
}

#[test]
fn test_environment_repository() {
    let repository =
        PackageRepository::open_from(PathBuf::from("tests/root/data/repository")).unwrap();
    assert!(!repository.is_empty());
}
