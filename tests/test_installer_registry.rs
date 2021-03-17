use mcfg::shared::installer::InstallerRegistry;
use mcfg::shared::FileSystemResource;
use pretty_assertions::assert_eq;
use std::env::current_dir;

#[test]
fn test_parse_installer_file() {
    let registry = InstallerRegistry::open_from(
        current_dir()
            .unwrap()
            .join("tests/root/config/installers.yml"),
    )
    .unwrap();

    #[cfg(target_os = "macos")]
    assert_eq!(registry.installers().count(), 5);

    #[cfg(target_os = "linux")]
    assert_eq!(registry.installers().count(), 3);
}
