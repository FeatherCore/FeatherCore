# Feather

Feather is the platform super-project for FeatherCore NuttX work.

It keeps the matching `nuttx` and `apps` repositories together as Git
submodules:

```text
Feather/
├── docs/   -> workflow and board porting notes
├── build/  -> local firmware output directory
├── nuttx/  -> https://github.com/FeatherCore/nuttx
└── apps/   -> https://github.com/FeatherCore/nuttx-apps
```

Both submodules track the `develop` branch and are pinned by this repository
to known-good commits.

## Clone

```bash
git clone --recurse-submodules ssh://git@ssh.github.com:443/FeatherCore/Feather.git
```

If the repository was cloned without submodules:

```bash
git submodule update --init --recursive
```

## Update Submodules

To move both submodules to the latest `origin/develop` commits:

```bash
git submodule update --remote --merge
git status
git add nuttx apps .gitmodules
git commit -s -m "Update NuttX submodules"
```
