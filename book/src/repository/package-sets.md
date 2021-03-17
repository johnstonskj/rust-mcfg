# Packages Sets

A package set is described in a YAML file, usually named `package-set.yml` and which contains the following properties.

* A name, and optional description.
* A flag denoting whether the package set is optional. 
* An optional script line to run before any other action.
* **Either**:
  * A list of [packages](./packages.md) to be installed by their respective installers.
  * A set of keyed [scripts](#scripts) executed during different installer actions.
* An optional name for an *env file* to link into the user's configuration space.
* An optional map of files to be symbolically linked into the user's file system.  
* An optional script line to run after all other actions.

## Example Package Set file

```yaml
name: lux
env-file: sample.env
actions:
  packages:
    - name: lux
      kind:
        language: python
link-files:
  set-lux: "{{local-bin}}/set-lux"
```

## Scripts

## Env Files

## Link Files 
