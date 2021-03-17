# The Installer Registry

## Example Installer Registry file

```yaml
- name: homebrew
  platform: macos
  kind: default
  commands:
    install: "brew install {{package}}"
    uninstall: "brew uninstall {{package}}"
    update-self: "brew upgrade"
    update: "brew update {{package}}"

- name: homebrew apps
  platform: macos
  kind: application
  commands:
    install: "brew cask install {{package}}"
    uninstall: "brew cask uninstall {{package}}"
    update-self: "brew upgrade"
    update: "brew cask update {{package}}"

- name: cargo
  kind:
    language: rust
  commands:
    install: "cargo install {{package}}"
    uninstall: "cargo uninstall {{package}}"
```

## Example InstallerRegistry API

```rust,no_run
use mcfg::shared::InstallerRegistry;

let installer_registry = InstallerRegistry::open().unwrap();
```

