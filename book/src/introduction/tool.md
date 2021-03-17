# The CLI tool

```text
$ mcfg help
mcfg 0.1.0
Machine configurator.

USAGE:
    mcfg [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    The level of logging to perform; from off to trace

SUBCOMMANDS:
    add            Add a new package-set to the local repository
    edit           Add an existing package-set in the local repository
    help           Prints this message or the help of the given subcommand(s)
    history        Show a history of install actions on the local machine
    init           Initialize a repository to manage package-set installs
    install        Install package-sets as described in the local repository
    installers     Edit the current installer registry file
    link-files     Link any files specified in package-sets as described in the local repository
    list           List package-sets in the local repository
    paths          Show current path locations
    refresh        Refresh the current repository
    remove         Remove an existing package-set from the local repository
    shell          Run a shell in the repository directory, with a basic script environment
    uninstall      Uninstall package-sets as described in the local repository
    update         Update package-sets as described in the local repository
    update-self    Show the current configuration
```

These can be grouped into those that 1) act on the package repository, 2) those that act on package sets, and 3) those
that act on the installer registry.

## Package repository commands

**add** a new package set to the repository; either creating a directory for the package set with a single file named 
`package-set.yml`, *or* if the `-a/--as-file` flag is set the file will be named for the package-set in the group 
directory.

**edit** an existing package set in the repository, this will look for a file in the following order:

1. `{{group}}/{{package_set}}/package-set.yml`
2. `{{group}}/{{package_set}}.yml`

**init**-ialize the repository, creating the repository, adding the default installer registry, and log file.

**list** the repository contents, as a hierarchy with groups and package sets. By default it will list all groups, the
`-g/--group` argument can be set to list only the contents of the named group.

show the configured **paths** for the current package repository, installer registry, and log file. 

**remove** an existing package set from the repository.

**refresh** the Git repository.

Run a **shell** within the package repository directory, with the default set of script environment variables set. This
is useful for testing scripts and doing repository edit/Git actions.

## Package set commands

All the following commands take both `-g/--group` and `-p/--package-set` arguments, resulting in the following behavior:

1. If neither is set the tool attempts to act on all groups, and all required package sets in each group.
1. If only the group is specified the tool attempts to act on all required package sets in thee specified group.
1. If both are specified, the tool attempts to act on the specified package set in the specified group and will also
   act even if the package set is marked as optional.

**install** the package set(s); this will attempt to install even if previously installed, and the behavior of such is
dependent on the installer.

**link-files** specified in the package set(s), performing no other actions - specifically neither the `run-before` or 
`run-after` script strings are run.

**uninstall** package set(s) from the repository; the behavior of this if the package is not previously installed is
dependent on the installer.

**update** package set(s) to their latest version; the behavior of this if the package is not previously installed is
dependent on the installer.

## Installer commands

Show a **history** of all package install actions. The `-l/--limit` argument can be used to return only a number of most 
recent entries from the log.

Edit the **installers** in the registry file.

Ask all installers in the registry to **update-self**.
