# Variables


All of these variables are also set as environment variables to be used inside any running script. Each variable name
is upper-cased and prefixed with "MCFG_", so `command_action` becomes `MCFG_COMMAND_ACTION`.

## Default variables

* `home` - the current user's home directory, usually equivalent to `$HOME`.
* `command_log_level` - the name of the current log level, if a command wishes to do any logging of it's own.
* `command_shell` - the name of the command shell used to execute script strings.
* `local_download_path` - the name of the user's local download directory.
* `platform` - the value of the `Platform` enum.
* `platform_family` - the operating system family, defined by Rust.
* `platform_os` - the operating system ID, defined by Rust.
* `platform_arch` - the system architecture ID, defined by Rust.
* `repo_config_path` - the path within the package repository for config files.
* `repo_local_path` - the path within the package repository for local files, including the `bin` directory.

## Action variables

 * `command_action` - the kind of action being performed; one of `install`, `link-files`,
   `update`, or `uninstall`.

## Package set variables

* `package_set_name` - the name of the package set being actioned.
* `package_set_file` - the name of the package set file, this is within `package_set_path`
* `package_set_path` - the directory containing the package set file.

## Package variables

* `package_name` - the name of the package being actioned.
* `package_config_path` - the current user's local configuration path for this package.
* `package_data_local_path` - the current user's local data path for this package.
* `package_log_path` - the full path to the installer log file.

## User-defined variables

