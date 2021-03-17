# Introduction

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

## Goals and non-goals

The intent is to provide a light-weigh way to describe the installation of a lot of related packages, scripts, and
customizations that comprise a machine environment. Specifically this was developed for keeping developer desktops
as close as possible between different laptop or desktop machines and between Linux and macOS systems.

The intent is also to *not*:

* Be a package manager itself; package sets are simply logical higher-level grouping of packages but rely solely on the 
  underlying package manager.
* Be a sync mechanism; the CLI does not sync automatically, refreshing the package repository and running update and
  install actions are still manual steps.
* Be atomic; there is no roll-back mechanism on failure.

## Current status

* Basic operations working, not yet ready for more use than that.