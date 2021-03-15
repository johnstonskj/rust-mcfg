use mcfg::shared::installer::builders::InstallerBuilder;
use mcfg::shared::Installer;

#[test]
fn test_parse() {
    let config_str = r##"
        - name: homebrew
          platform: macos
          kind: default
          commands:
            install: "brew install {{package}}"
            update: "brew update {{package}}"
"##;
    let configs: Vec<Installer> = serde_yaml::from_str(config_str).unwrap();
    println!("{:?}", configs);
}

#[test]
fn test_write() {
    let installers = vec![InstallerBuilder::named("homebrew")
        .for_macos_only()
        .for_default_packages()
        .add_install_command("brew install {{package_name}}")
        .add_update_command("brew update {{package_name}}")
        .add_update_self_command("brew upgrade")
        .installer()];

    let installers_str = serde_yaml::to_string(&installers).unwrap();
    println!("{:?}", installers_str);
}
