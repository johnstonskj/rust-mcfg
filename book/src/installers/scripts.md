# Scripts

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

A script to be run using the system shell, or rather a command than can only be interpreted using the system shell.

```yaml
name: hello world
run-before: echo 'error!!' 1>&2
```

In this case the command is re-written to be called by the system default shell, equivalent to the following.

```yaml
run-before: {{shell}} -c "echo 'error!!' 1>&2"
```

## Script Syntax

* **Plain** text
* **String**
* **Variable**
* **Special**

## Script Variables

All of these variables are also set as environment variables to be used inside any running script. Each variable name
is upper-cased and prefixed with "MCFG_", so `command_action` becomes `MCFG_COMMAND_ACTION`.

* `command_action` - the current tool action; one of "install", "uninstall", "update".
* `command_log_level` - the current tool max log level; one of "error", "warn", "info", "debug", "trace".
* `local_download_path` - the user's local download directory; e.g. "/home/simon/Downloads".
* `opsys` - the current operating system; one of "linux" or "macos".
* `repo_config_path` - ; e.g. "/home/simon/.config/mcfg/repository/.config".
* `repo_local_path` - ; e.g. "/home/simon/.config/mcfg/repository/.local".
* `shell` - the default system shell; e.g. "bash".
* `package_set_name` - the name of the package-set being operated on.
* `package_set_file` - the file name of the package-set description; e.g. "package-set.yml".
* `package_set_path` - the path to the package-set enclosing directory | "/home/simon/.config/mcfg/repository/work-tools/productivity".
