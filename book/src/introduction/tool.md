# The Tool


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
    config         Show the current configuration
    edit           Add an existing package-set in the local repository
    help           Prints this message or the help of the given subcommand(s)
    history        Show a history of install actions on the local machine
    init           Initialize a repository to manage package-set installs
    install        Install package-sets as described in the local repository
    link-files     Link any files specified in package-sets as described in the 
                   local repository
    list           List package-sets in the local repository
    refresh        Refresh the current repository
    remove         Remove an existing package-set from the local repository
    uninstall      Uninstall package-sets as described in the local repository
    update         Update package-sets as described in the local repository
    update-self    Show the current configuration
```

## Package Repository Commands

* **add** add a new package set to your repository. Requires a group name and package set name.
* **edit** edit an existing package set in your repository. Requires a group name and package set name.
* **list** list the groups and package sets in your repository. If given a group name it only lists packages in that group.
* **refresh** refresh the repository by performing a Git pull.
* **remove** remove an existing package set from your repository. Requires a group name and package set name.

## Package Commands

All take -p and -g

* **install** install package sets.
* **link-files**
* **uninstall** uninstall package sets.
* **update** update package sets.

## Installer Commands

* **update-self** update any installer in the registry with an `update-self` script property.

## Information Commands

* **config** show the current configuration details, including paths and installers.
* **help** show the help text above.
