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

**add**

**edit**

**init**

**list**

**paths**

**remove**

**refresh**

**shell**

## Package set commands

All take -p and -g

**install** install package sets.

**link-files**

**uninstall** uninstall package sets.

**update** update package sets.

## Installer commands

**history**

**installers**

**update-self**
