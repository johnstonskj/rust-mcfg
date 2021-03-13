# The Package Set Repository


```text
$HOME/
├─ .config/
│  └─ mcfg/
│     └─ installers.yml
└─ .local/
   └─ share/
      └─ mcfg/
         ├─ logs/
         │  └─ install-log.sql
         └─ repository/
            ├─ .config/
            ├─ .git/  
            └─ .local/
```

```text
$HOME/
├─ .config/
│  └─ mcfg/
│     └─ installers.yml
├─ .local/
│  └─ share/
│     └─ mcfg/
│        ├─ logs/
│        │  └─ install-log.sql
│        └─ repository/  ->  $HOME/mcfg-repo-simon/
└─ mcfg-repo-simon/
   ├─ .config/
   ├─ .git/  
   └─ .local/
```

```text
$HOME/
└─ Library/
   ├─ Application Support/
   │  └─ mcfg/
   │     ├─ installers.yml
   │     └─ repository/
   │        ├─ .config/
   │        ├─ .git/  
   │        └─ .local/
   └─ Logs/
      └─ mcfg/
         └─ install-log.sql
```

## Example PackageRepository API

```rust,no_run
use mcfg::shared::PackageRepository;

let package_repository = PackageRepository::open().unwrap();
```
