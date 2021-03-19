use mcfg::shared::builders::Builder;
use mcfg::shared::packages::builders::{PackageBuilder, PackageSetBuilder};
use mcfg::shared::{Name, PackageSet};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn test_minimal_package_set() {
    let package_set = PackageSetBuilder::named(Name::from_str("example").unwrap()).build();
    assert_eq!(package_set.name(), &String::from("example"));
    assert_eq!(package_set.path(), &PathBuf::default());
    assert_eq!(package_set.description(), &None);
    assert_eq!(package_set.is_optional(), false);
    assert_eq!(package_set.run_before(), &None);
    assert_eq!(package_set.has_actions(), false);
    assert_eq!(package_set.env_file(), &None);
    assert_eq!(package_set.link_files(), &HashMap::default());
    assert_eq!(package_set.run_after(), &None);

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);

    let new_package_set = serde_yaml::from_str(&package_set_str).unwrap();
    assert_eq!(package_set, new_package_set);
}

#[test]
fn test_package_set_with_packages() {
    let package_set = PackageSetBuilder::named(Name::from_str("example").unwrap())
        .description("an example package set, with package actions")
        .optional()
        .run_before("{{local-bin}}/ex-pre-install")
        .with_package_actions()
        .add_package_action(PackageBuilder::named(Name::from_str("expackage").unwrap()).build())
        .unwrap()
        .env_file("example.env")
        .run_after("{{local-bin}}/ex-post-install")
        .build();
    assert_eq!(package_set.name(), &String::from("example"));
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

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);

    let new_package_set = serde_yaml::from_str(&package_set_str).unwrap();
    assert_eq!(package_set, new_package_set);
}

#[test]
fn test_package_set_with_scripts() {
    let package_set = PackageSetBuilder::named(Name::from_str("example").unwrap())
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

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);

    let new_package_set = serde_yaml::from_str(&package_set_str).unwrap();
    assert_eq!(package_set, new_package_set);
}

#[test]
fn test_package_set_with_a_lot() {
    let package_set = PackageSetBuilder::named(Name::from_str("gpg").unwrap())
        .description("Gnu Privacy Guard")
        .env_var("gpg_home", "{{home}}/.gnupg")
        .package_actions(&[
            PackageBuilder::named(Name::from_str("gpg").unwrap()).build(),
            PackageBuilder::named(Name::from_str("pinentry-gnome3").unwrap())
                .for_linux_only()
                .build(),
            PackageBuilder::named(Name::from_str("pinentry-mac").unwrap())
                .for_macos_only()
                .build(),
        ])
        .add_link_file("gpg.conf", "{{gpg_home}}/gpg.conf")
        .add_link_file(
            "gpg-agent-{{platform_os}}.conf",
            "{{gpg_home}}/gpg-agent.conf",
        )
        .run_after("gpg --list-keys")
        .build();

    assert_eq!(package_set.name(), &String::from("gpg"));
    assert_eq!(
        package_set.description(),
        &Some("Gnu Privacy Guard".to_string())
    );
    assert_eq!(package_set.has_actions(), true);
    assert_eq!(package_set.packages().unwrap().count(), 3);
    assert_eq!(package_set.link_files().len(), 2);
    assert_eq!(
        package_set.run_after(),
        &Some("gpg --list-keys".to_string())
    );

    let package_set_str = serde_yaml::to_string(&package_set).unwrap();
    println!("{}", package_set_str);

    let new_package_set = serde_yaml::from_str(&package_set_str).unwrap();
    assert_eq!(package_set, new_package_set);
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

    let package_set: PackageSet = serde_yaml::from_str(config_str).unwrap();
    println!("{:?}", package_set);
    assert_eq!(package_set.name(), "lux");
    assert_eq!(package_set.env_file(), &Some("sample.env".to_string()));
    assert_eq!(package_set.packages().unwrap().count(), 1);
    assert!(package_set.scripts().is_none());
    assert_eq!(package_set.link_files().len(), 1)
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

    let package_set: PackageSet = serde_yaml::from_str(config_str).unwrap();
    println!("{:?}", package_set);
    assert_eq!(package_set.name(), "lux");
    assert_eq!(package_set.env_file(), &Some("sample.env".to_string()));
    assert!(package_set.packages().is_none());
    assert_eq!(package_set.scripts().unwrap().len(), 2);
    assert_eq!(package_set.link_files().len(), 1)
}
