# Getting Started

## Install

## Initialize Repository

### Paths

```text
$ mcfg paths
Package Repository path:
	"/Users/simonjo/Library/Application Support/mcfg/repository"
Installer Registry path:
	"/Users/simonjo/Library/Application Support/mcfg/installers.yml"
Package Installer log file path:
	"/Users/simonjo/Library/Logs/mcfg/install-log.sql"
```

`config_path` - the path to the user's operating-system specific configuration directory for this
app.
`installer_file_path` the path to the installer registry file; by default this is
`{{config_path}}/installers.yml`.
`repository_path` - the path to the user's package-set repository; by default this is in the user's
operating-system specific *local data* directory, but is often a symbolic link to the repository
elsewhere.
`log_file_path` - the path to the log file which is a SQLite3 database, it is held in the user's
operating-system specific log directory.

## Add Package Sets

### Install Order and Naming