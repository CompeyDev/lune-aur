This repository houses the required manifests and build scripts for lune packaging. 
For more information on lune, see [filiptibell/lune](https://github.com/filiptibell/lune).

## Packaging Statuses

| Platform | Status                                                                                                                                                                    |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| AUR      | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/aur_test.yaml?logo=archlinux&label=%20&color=black) |
| Scoop    | ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/CompeyDev/lune-packaging/scoop_test.yaml?logo=windows&logoColor=blue&label=%20&color=black) |


## Installation
### AUR
```console
yay -S lune
### OR ###
yay -S lune-git
### OR ###
yay -S lune-bin
```

### Scoop
```ps
scoop bucket add lune https://github.com/CompeyDev/lune-packaging.git
scoop install lune
```
