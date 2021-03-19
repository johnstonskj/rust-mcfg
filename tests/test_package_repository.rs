use mcfg::shared::packages::PackageRepository;
use mcfg::shared::{FileSystemResource, Name};
use std::env::current_dir;
use std::str::FromStr;

#[test]
fn test_parse_repository() {
    let _ = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init();
    let repository =
        PackageRepository::open_from(current_dir().unwrap().join("tests/root/data/repository"))
            .unwrap();
    println!("{:#?}", repository);
    assert_eq!(
        repository.path(),
        &current_dir().unwrap().join("tests/root/data/repository")
    );
    assert_eq!(repository.is_empty(), false);
    assert_eq!(repository.groups().count(), 1);
    let system_group = repository
        .group(&Name::from_str("system").unwrap())
        .unwrap();
    assert_eq!(system_group.package_sets().count(), 5);
}
