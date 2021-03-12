# Packages, Sets, and Groups


1. A set of package-set groups, these are basically all the directories under the repository
   directory. Certain directories, such as `.git`, will be ignored.
1. A set of package-sets, these are within the group directories and are of one of two forms:
    1. a directory containing a file with the name `package-set.yml`, or
    1. a file in the group directory with the `.yml` extension.

## Example PackageRepository API

```rust,no_run
use mcfg::shared::PackageRepository;

let package_repository = PackageRepository::open().unwrap();
```

