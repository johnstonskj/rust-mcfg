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
    edit           Add an existing package-set in the local repository
    help           Prints this message or the help of the given subcommand(s)
    history        Show a history of install actions on the local machine
    init           Initialize a repository to manage package-set installs
    install        Install package-sets as described in the local repository
    installers     Edit the current installer registry file
    link-files     Link any files specified in package-sets as described in the 
                   local repository
    list           List package-sets in the local repository
    paths          Show current path locations
    refresh        Refresh the current repository
    remove         Remove an existing package-set from the local repository
    uninstall      Uninstall package-sets as described in the local repository
    update         Update package-sets as described in the local repository
    update-self    Show the current configuration
```

## Package Repository Commands

### Init

```text
$ mcfg init -l ~/dotfiles-2
1. Creating local directory for repository
2. Initializing Git repository
3. Creating repository link "/Users/simonjo/dotfiles-2" -> "/Users/simonjo/Library/Application Support/mcfg/repository"
4. Creating repository config/local directories
5. Creating 'example/hello world' package set
6. Creating initial installer registry file
Done.
```

### Add, Edit, Remove

TBD

### List

TBD

### Refresh

TBD

## Package Commands

All take -p and -g

* **install** install package sets.
* **link-files**
* **uninstall** uninstall package sets.
* **update** update package sets.

## Installer Commands

### History

```text
+-------------------------------+--------+---------+---------+-----------+
| Date                          | Group  | Set     | Package | Installer |
+===============================+========+=========+=========+===========+
| 2021-03-11 20:28:04.339072 +0 | system | gnu-sed | gnu-sed | homebrew  |
+-------------------------------+--------+---------+---------+-----------+
```

### Update Self

TBD

## Information Commands

* **installers** edit the current installer registry.

### Paths

```text
$ mcfg paths
Package Repository path:
	"/Users/simon/Library/Application Support/mcfg/repository"
Package Repository config file path:
	"/Users/simon/Library/Application Support/mcfg/repository/.config"
Package Repository local file path:
	"/Users/simon/Library/Application Support/mcfg/repository/.local"
Installer Registry path:
	"/Users/simon/Library/Application Support/mcfg/installers.yml"
Package Installer log file path:
	"/Users/simon/Library/Logs/mcfg/install-log.sql"
```