# Getting Started

To use the `mcfg` tool you need a package repository and an installer registry. Your package repository is a directory
containing the configuration you want to use for your machine, the installer registry provides details of the tools
used to actually install individual packages. The tool expects that the package repository is also a Git repository,
this being the mechanism to share your repository across machines. For first use, the tool has an **init** command 
that we will demonstrate below.

## Install

TBD

## Initialize your repository

To initialize a new repository, on a new machine, using the system defaults for configuration and data paths the 
following is all that is necessary.

```text
$ mcfg init
1. Creating local directory for repository
2. Initializing Git repository
3. Creating repository '.config' directory
4. Creating repository '.local' directory
5. Creating '00-installers/homebrew' package set
6. Creating '00-installers/homebrew-services' package set
7. Creating 'example/hello world' package set
8. Creating standard installer registry file
9. Creating package install log file
Done.
```

Step number 2 is important, after creating the repository directory it will perform the equivalent of a `git init` 
command. This sets up the versioning for the repository but obviously as this repository has no upstream origin 
we can't push changes until we make that connection.

Alternatively you can provide the URL to an existing Git repository which will be cloned into the package repository
directory.

```text
$ mcfg init -r https://github.com/simon/mcfg-repo.git
1. Creating local directory for repository
2. Cloning <https://github.com/simon/mcfg-repo.git> into repository
3. Creating repository '.config' directory
4. Creating repository '.local' directory
5. Creating '00-installers/homebrew' package set
6. Creating '00-installers/homebrew-services' package set
7. Creating 'example/hello world' package set
8. Creating standard installer registry file
9. Creating package install log file
```

Finally, if you would rather keep your actual Git repository in a known place outside the standard directory 
structure you can specify a local directory (with the `-l` argument) and a symlink will be created from the standard
repository location to your local Git directory.

```text
$ mcfg init -r https://github.com/fakeuser/mcfg-repo.git -l $HOME/mcfg-repo
```

### Paths

To show all the paths that the tool uses, the `paths` command will list them all.

```text
$ mcfg paths
Package Repository path:
	"/Users/simon/Library/Application Support/mcfg/repository"
Package Repository symlinked to:
	"/Users/simon/dotfiles-2"
Package Repository config file path:
	"/Users/simon/Library/Application Support/mcfg/repository/.config"
Package Repository local file path:
	"/Users/simon/Library/Application Support/mcfg/repository/.local"
Installer Registry path:
	"/Users/simon/Library/Application Support/mcfg/installers.yml"
Package Installer log file path:
	"/Users/simon/Library/Logs/mcfg/install-log.sql"
```

## Add Package Sets

The command `add <group> <package-set>` will create a new group if one doesn't already exist, a directory for the 
package set name and finally a `package-set.yml` file within this. If the `-a/--as-file` flag is set the tool will 
create a new group if one doesn't already exist, and a file name + `.yml`. After creating the package set the tool will 
execute an editor to edit the template package set file it created. 

Alternatively the `shell` command can be used to start an interactive shell in the repository document so you can
create groups and package set files more manually.


## Install!

Once you

```text
$ mcfg install
```