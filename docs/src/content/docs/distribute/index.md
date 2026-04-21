---
title: Distributing ForgeDesk Apps
description: Learn how to build, package, sign, and distribute your ForgeDesk applications across Windows, macOS, and Linux.
---

Welcome to the distribution guide for **ForgeDesk**. Once you have finished developing your application, you need to compile the native Rust window, bundle your Python core, package the frontend assets, and generate standard installers for your users.

ForgeDesk simplifies this deeply complex process into a single command, automating cross-platform builds and asset bundling.

---

## The Build Pipeline

To compile your project for production, run the Forge CLI build command:

```bash
forge build
```

### What happens during a build?
1. **Frontend Compilation:** Hooks into your frontend bundler (e.g., `vite build`, `next build`) to generate static HTML/JS/CSS assets.
2. **Python Bundling:** Compiles and isolates your Python environment, dependencies, and source code into an embedded, self-contained runtime.
3. **Native Compilation:** Compiles the underlying Rust core in `--release` mode, embedding the packaged frontend and Python engine directly into a highly optimized binary.
4. **Installer Generation:** Wraps the resulting executable in OS-native installer formats (e.g., `.msi`, `.dmg`, `.AppImage`).

---

## Supported Target Formats

ForgeDesk relies on native OS-level packaging to ensure your app feels right at home on any operating system.

### Windows
- `.msi` (Windows Installer)
- `.nsis` (Nullsoft Scriptable Install System)
- Portable `.exe`

### macOS
- `.dmg` (Disk Image)
- `.app` (macOS Application Bundle)

### Linux
- `.deb` (Debian/Ubuntu)
- `.AppImage` (Universal Linux package)

*Note: Cross-compilation is partially supported, but for native installers like `.dmg` or `.msi`, it is highly recommended to run `forge build` directly on the target operating system (or via a CI provider).*

---

## Configuring Application Metadata

Before distributing your app, ensure its metadata, icons, and versioning are properly configured in your `forge.toml` file at the root of your project:

```toml
[package]
name = "my-awesome-app"
version = "1.0.0"
description = "A powerful desktop tool built with Python."
authors = ["Your Name <hello@example.com>"]

[build]
icon = "src-forge/icons/app-icon.png" # Forge desk will automatically generate .ico and .icns 
target = ["msi", "nsis"] # Specify default installer output formats
```

When you place a high-resolution PNG (e.g., `1024x1024`) at the specified `icon` path, ForgeDesk's CLI automatically scales and creates the `.ico` (Windows) and `.icns` (macOS) required by the target OS.

---

## Code Signing & Notarization

Modern operating systems aggressively warn users when they open software from unknown developers. To bypass Windows SmartScreen or macOS Gatekeeper, you must digitally sign your application.

### macOS Code Signing
To distribute on macOS, you need an Apple Developer ID. ForgeDesk reads standard environment variables to sign the `.app` bundle automatically during `forge build`:

```bash
export APPLE_CERTIFICATE="Developer ID Application: Team Name (TEAM_ID)"
export APPLE_CERTIFICATE_PASSWORD="your-secure-password"
export APPLE_TEAM_ID="TEAM_ID"

forge build
```

### Windows Authenticode
To sign Windows binaries, you require a valid Authenticode certificate (PFX):

```bash
export WIN_CERTIFICATE="path/to/certificate.pfx"
export WIN_CERTIFICATE_PASSWORD="your-secure-password"

forge build
```

---

## Continuous Integration (CI)

Manually building applications across three different operating systems is tedious. ForgeDesk is designed to thrive within CI/CD environments like **GitHub Actions**.

Here is an example snippet of a GitHub Actions configuration that automatically builds your application for macOS, Windows, and Linux on every release tag:

```yaml
name: Release ForgeDesk App

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [ubuntu-latest, macos-latest, windows-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install Forge CLI
        run: pip install forge-cli

      - name: Build Application
        run: forge build

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: app-installers-${{ matrix.platform }}
          path: src-forge/target/release/bundle/
```

## Next Steps

With your app compiled, signed, and ready for end-users, you have successfully completed the core ForgeDesk lifecycle!

- Need to handle automatic background updates? See our [Auto-Updater](/guides/auto-updating/) guide.
- Review [Security Best Practices](/security/) before a major public release.
