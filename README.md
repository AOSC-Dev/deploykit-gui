DeployKit GUI
===

Graphical installer for AOSC OS, inspired by Microsoft® Windows® XP and early Loonghorn installers. DeployKit GUI provides a wizard-like interface for users to install and configure AOSC OS:

- Selecting a system version.
- Selecting a download source.
- Configuration of...
  - Partitioning.
  - User accounts.
  - Time and language settings.
  - Swap space.

Dependencies
---

- Rust >= 1.75
- WebKitGTK+ (4.1)
- Yarn

Building
---

Building DeployKit GUI is simple.

### Building for release deployment

```bash
yarn
yarn tauri build
```
### Building for development/debugging

```bash
yarn
yarn tauri build --debug
```

or

```bash
yarn
yarn tauri dev
```
