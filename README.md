# Crate rust-mcfg

Machine configurator, a cross-platform meta-package manager.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/mcfg.svg)](https://crates.io/crates/mcfg)
[![docs.rs](https://docs.rs/mcfg/badge.svg)](https://docs.rs/mcfg)
![Build](https://github.com/johnstonskj/rust-mcfg/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-mcfg/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-mcfg.svg)](https://github.com/johnstonskj/rust-mcfg/stargazers)

The `mcfg` crate and command-line tool, implements a simple *Machine Configurator* or meta-package manager to keep
desktop environments the same across machines and wherever possible across operating systems. The tool makes use of
existing package managers such as [homebrew](https://brew.sh/), [apt](https://en.wikipedia.org/wiki/APT_(software)),
or [yum](https://en.wikipedia.org/wiki/Yum_(software)). It allows for packages to be grouped into package sets which
are the units of management and then package sets into groups for simple organization.

The tool keeps all of it's package sets organized in a repository which just happens to be a Git repo and so can
be versioned and easily shared between machines. It allows for the specification of different installer tools that
will be used to do actual package management, so the user doesn't need to remember specific command-lines or other
details. This repo can also include any additional scripts or tools the user needs, and the execution of the package
set includes a set of environment variables to allow scripts to run without knowing any O/S or machine specific
paths or other details.

More information is found in the [User Guide](https://simonkjohnston.life/rust-mcfg/).
  
## The tool

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

## Changes

**Version 0.1.1**

* Close to ready, a lot of refactoring completed.
* Added crate documentation, and some of the book.

**Version 0.1.0**

* Initial commit.

## TODO

* More tests!
