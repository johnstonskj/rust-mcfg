# Packages

Packages are described within a [package set](./package-sets.md), they have the following p properties:

* A name.
* An optional platform specification.
* An optional package kind specification.

## Platforms

The platform value, typed as `Option<mcfg::shared::Platform>`, specifies whether a package is only applicable for one 
of the supported operating system and where `None` implies no restriction, it should be installed for all.

## Package Kinds

The package kind value, typed as `mcfg::shared::PackageKind`, specifies the kind of installer to use for this
package.

* application
* default
* language
* script