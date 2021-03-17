# Appendix: Example package sets

The following are package-sets that can be useful starting places.

## Example: installing homebrew via curl

```yaml
---
name: homebrew
platform: macos
description: macOS homebrew package manager
actions:
  scripts:
    install: "curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | bash"
```

## Example: setting macOS experience defaults

```yaml
---
name: macos defaults
platform: macos
actions:
  scripts:
    install: >-
      defaults write com.apple.dashboard devmode YES &&
      defaults write com.apple.finder _FXShowPosixPathInTitle -bool YES &&
      defaults write com.apple.Dock showhidden -bool YES
```

## Example: a long list of packages

```yaml
name: fonts
description: all those missing fonts!
actions:
  packages:
    - name: homebrew/cask-fonts/font-fira-code
      platform: macos
      kind: application
    - name: homebrew/cask-fonts/font-fira-code-nerd-font
      platform: macos
      kind: application
    - name: font-meslo-lg
      platform: macos
      kind: application
    - name: font-meslo-lg-nerd-font
      platform: macos
      kind: application
    - name: font-linux-libertine
      platform: macos
      kind: application
    - name: fonts-powerline
      platform: linux
```

## Example: linking files and run-after

```yaml
---
name: zsh
description: the Z shell
actions:
  packages:
    - name: zsh
    - name: zsh-completions
    - name: zsh-navigation-tools
      platform: macos
link-files:
  dot-zlogin: "{{home}}/.zlogin"
  dot-zshenv: "{{home}}/.zshenv"
  dot-zshrc: "{{home}}/.zshrc"
run-after: "{{package_set_path}}/run-after"
```

## Example: custom variables

```yaml
---
name: gpg
description: Gnu Privacy Guard
env-vars:
  gpg_home: "{{home}}/.gnupg"
actions:
  packages:
    - name: gpg
    - name: pinentry-gnome3
      platform: linux
    - name: pinentry-mac
      platform: macos
link-files:
  gpg.conf: "{{gpg_home}}/gpg.conf"
  "gpg-agent-{{platform_os}}.conf": "{{gpg_home}}/gpg-agent.conf"
run-after: gpg --list-keys
```
