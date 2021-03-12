# Script Environments

## Script Syntax

## Script Variables

| registry variable     | environment variable       | usage | example |
| --------------------- | -------------------------- | ----- | ------- |
| `command_action`      | `MCFG_COMMAND_ACTION`      | The current tool action | "install" |
| `command_log_level`   | `MCFG_COMMAND_LOG_LEVEL`   | The current tool max log level | "info" |
| `local_download_path` | `MCFG_LOCAL_DOWNLOAD_PATH` | The user's local download directory | "/home/alice/Downloads" |
| `local_bin_path`      | `MCFG_LOCAL_BIN_PATH`      | The user's local executable directory | "/home/alice/.local/bin" |
| `opsys`               | `MCFG_OPSYS`               | The current operating system | "macos" |
| `shell`               | `MCFG_SHELL`               | The default system shell | "bash" |
| `package_set_name`    | `MCFG_PACKAGE_SET_NAME`    | The name of the package-set being operated on | "lux" |
| `package_set_file`    | `MCFG_PACKAGE_SET_FILE`    | The file name of the package-set description | "package-set.yml" |
| `package_set_path`    | `MCFG_PACKAGE_SET_PATH`    | The path to the package-set enclsing directory | "/home/alice/.config/mcfg/repository/work-tools/productivity" |
