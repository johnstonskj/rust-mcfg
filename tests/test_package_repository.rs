use mcfg::shared::packages::PackageRepository;
use mcfg::shared::FileSystemResource;
use std::env::current_dir;
use std::path::PathBuf;

#[test]
fn test_parse_repository() {
    let _ = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init();
    let repository =
        PackageRepository::open_from(PathBuf::from("tests/root/data/repository")).unwrap();
    println!("{:#?}", repository);
    assert_eq!(
        repository.path(),
        &current_dir().unwrap().join("tests/root/data/repository")
    );
    assert_eq!(repository.is_empty(), false);
    assert_eq!(repository.groups().count(), 1);
    let system_group = repository.group("system").unwrap();
    assert_eq!(system_group.package_sets().count(), 2);
}
