# Introduction

The `mcfg` crate and command-line tool, implements a simple *Machine Configurator* or meta-package manager to keep 
desktop environments the same across machines and wherever possible across operating systems. The tool makes use of 
existing package managers such as [homebrew](https://brew.sh/), [apt](https://en.wikipedia.org/wiki/APT_(software)), 
or [yum](https://en.wikipedia.org/wiki/Yum_(software)). It allows for packages to be grouped into package sets which
are the units of management and then package sets into groups for simple organization.

The description of package sets are in YAML files and 