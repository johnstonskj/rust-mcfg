# Package Set Groups


## Naming

`^([0-9]+\-)?(.*)$`


1. A set of package-set groups, these are basically all the directories under the repository
   directory. Certain directories, such as `.git`, will be ignored.
1. A set of package-sets, these are within the group directories and are of one of two forms:
    1. a directory containing a file with the name `package-set.yml`, or
    1. a file in the group directory with the `.yml` extension.


```text
$HOME/
└─ .local/
   └─ share/
      └─ mcfg/
         └─ repository/
            ├─ .config/
            ├─ .git/  
            ├─ .local/
            ├─ 01-operating-system/
            ├─ 02-developer-stuff/
            ├─ 03-productivity-stuff/
            └─ 04-work-stuff/
```
