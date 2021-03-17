use mcfg::shared::installer::builders::InstallerBuilder;
use mcfg::shared::{Installer, PackageKind, Platform};
use pretty_assertions::assert_eq;

#[test]
fn test_parse() {
    let installers_str = r##"
        - name: homebrew
          platform: macos
          kind: default
          commands:
            install: "brew install {{package}}"
            update: "brew update {{package}}"
"##;
    let installers: Vec<Installer> = serde_yaml::from_str(installers_str).unwrap();
    println!("{:?}", installers);
    assert_eq!(installers.len(), 1);
    let installer = installers.first().unwrap();
    assert_eq!(installer.name(), "homebrew");
    assert_eq!(installer.platform(), Platform::Macos);
    assert_eq!(installer.kind(), PackageKind::Default);
    assert_eq!(installer.commands().len(), 2);
}

#[test]
fn test_write() {
    let installers = vec![InstallerBuilder::named("homebrew")
        .for_macos_only()
        .for_default_packages()
        .add_install_command("brew install {{package_name}}")
        .add_update_command("brew update {{package_name}}")
        .update_self_command("brew upgrade")
        .installer()];

    let installers_str = serde_yaml::to_string(&installers).unwrap();
    println!("{:?}", installers_str);

    let new_installers: Vec<Installer> = serde_yaml::from_str(&installers_str).unwrap();
    assert_eq!(installers, new_installers);
}
