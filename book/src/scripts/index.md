# Script strings

Package installers, and package sets, have a number of properties that are intended to execute commands. These are
simple YAML strings, but some parsing has to be done to ensure correct handling of a number of possibilities.

These parsing rules *should* allow the following patterns of usage:

A simple self-contained command, no shell interpretation or variable substitution.

```yaml
- name: homebrew
  commands:
    update-self: "brew upgrade"
```

A simple command, but parameterized using one or more of the standard script variables.

```yaml
- name: homebrew
  commands:
    install: "brew install {{package}}"
```

## Usage in installers

## Usage in package sets