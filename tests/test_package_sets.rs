use mcfg::shared::packages::builders::{PackageBuilder, PackageSetBuilder};
use mcfg::shared::{InstallActionKind, PackageSet};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_minimal_package_set() {
    let package_set = PackageSetBuilder::named("example").build();
    assert_eq!(package_set.name(), &String::from("example"));
    assert_eq!(package_set.path(), &PathBuf::default());
    assert_eq!(package_set.description(), &None);
    assert_eq!(package_set.is_optional(), false);
    assert_eq!(package_set.run_before(), &None);
    assert_eq!(package_set.has_actions(), false);
    assert_eq!(package_set.env_file(), &None);
    assert_eq!(package_set.link_files(), &HashMap::default());
    assert_eq!(package_set.run_after(), &None);
}

#[test]
fn test_package_set_with_packages() {
    let package_set = PackageSetBuilder::named("example")
        .path(PathBuf::from("repo/group/package-set.yml"))
        .description("an example package set, with package actions")
        .optional()
        .run_before("{{local-bin}}/ex-pre-install")
        .with_package_actions()
        .add_package_action(PackageBuilder::named("expackage").build())
        .unwrap()
        .env_file("example.env")
        .run_after("{{local-bin}}/ex-post-install")
        .build();
    assert_eq!(package_set.name(), &String::from("example"));
    assert_eq!(
        package_set.path(),
        &PathBuf::from("repo/group/package-set.yml")
    );
    assert_eq!(
        package_set.description(),
        &Some("an example package set, with package actions".to_string())
    );
    assert_eq!(package_set.is_optional(), true);
    assert_eq!(
        package_set.run_before(),
        &Some("{{local-bin}}/ex-pre-install".to_string())
    );
    assert_eq!(package_set.has_actions(), true);
    assert_eq!(package_set.packages().unwrap().count(), 1);
    assert_eq!(package_set.env_file(), &Some("example.env".to_string()));
    assert_eq!(package_set.link_files(), &HashMap::default());
    assert_eq!(
        package_set.run_after(),
        &Some("{{local-bin}}/ex-post-install".to_string())
    );
}

#[test]
fn test_package_set_with_scripts() {
    let package_set = PackageSetBuilder::named("example")
        .path(PathBuf::from("repo/group/package-set.yml"))
        .description("an example package set, with package actions")
        .optional()
        .run_before("{{local-bin}}/ex-pre-install")
        .with_script_actions()
        .add_install_script_action("{{local-bin}}/ex-installer")
        .unwrap()
        .add_uninstall_script_action("{{local-bin}}/ex-uninstaller")
        .unwrap()
        .env_file("example.env")
        .run_after("{{local-bin}}/ex-post-install")
        .build();
    assert_eq!(package_set.name(), &String::from("example"));
    assert_eq!(
        package_set.path(),
        &PathBuf::from("repo/group/package-set.yml")
    );
    assert_eq!(
        package_set.description(),
        &Some("an example package set, with package actions".to_string())
    );
    assert_eq!(package_set.is_optional(), true);
    assert_eq!(
        package_set.run_before(),
        &Some("{{local-bin}}/ex-pre-install".to_string())
    );
    assert_eq!(package_set.has_actions(), true);
    assert_eq!(package_set.scripts().unwrap().len(), 2);
    assert_eq!(package_set.env_file(), &Some("example.env".to_string()));
    assert_eq!(package_set.link_files(), &HashMap::default());
    assert_eq!(
        package_set.run_after(),
        &Some("{{local-bin}}/ex-post-install".to_string())
    );
}

#[test]
fn test_parse_package_set_with_packages() {
    let config_str = r##"
        name: lux
        env-file: sample.env
        actions:
          packages:
            - name: lux
              kind:
                language: python
        link-files:
          set-lux: "{{local-bin}}/set-lux"
        "##;

    let config: PackageSet = serde_yaml::from_str(config_str).unwrap();
    println!("{:?}", config);
}

#[test]
fn test_parse_package_set_with_scripts() {
    let config_str = r##"
        name: lux
        env-file: sample.env
        actions:
          scripts:
            install: install-lux
            uninstall: uninstall-lux
        link-files:
          set-lux: "{{local-bin}}/set-lux"
        "##;

    let config: PackageSet = serde_yaml::from_str(config_str).unwrap();
    println!("{:?}", config);
}

#[test]
fn test_package_set_with_packages_to_string() {
    let package_set = PackageSetBuilder::named("lux")
        .env_file("sample.env")
        .with_package_actions()
        .add_package_action(
            PackageBuilder::named("lux")
                .for_any_platform()
                .using_language_installer("python")
                .build(),
        )
        .unwrap()
        .add_link_file("set-lux", "{{local-bin}}/set-lux")
        .build();

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);
}

#[test]
fn test_package_set_with_scripts_to_string() {
    let package_set = PackageSetBuilder::named("lux")
        .env_file("sample.env")
        .with_script_actions()
        .add_script_action(InstallActionKind::Install, "install-lux")
        .unwrap()
        .add_script_action(InstallActionKind::Uninstall, "uninstall-lux")
        .unwrap()
        .add_link_file("set-lux", "{{local-bin}}/set-lux")
        .build();

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);
}
