use mcfg::shared::environment::Environment;
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
