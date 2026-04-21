---
title: Command Line Interface (CLI)
description: Master the ForgeDesk CLI to scaffold, develop, and build your applications.
---

Welcome to the **ForgeDesk CLI** guide! The command-line interface is your primary tool for managing the entire lifecycle of a ForgeDesk application, from scaffolding a new project to compiling the final standalone executable.

---

## ⌨️ Overview

The `forge` command becomes available once you install the Python package (`pip install forgedesk` or similar) in your virtual environment. 

To see all available commands and flags at any time, you can run:

```bash
forge --help
```

---

## 🛠️ Core Commands

Here is a breakdown of the primary CLI commands and how you should use them during your development workflow.

### `forge create`

Scaffolds a brand new ForgeDesk project. This is equivalent to running the `npx @forgedesk/create-forge-app` wizard, but run directly using the Python CLI.

```bash
forge create my-app
```

**Options:**
- `--template <name>`: Bypass the interactive keyboard wizard and directly generate a specific template (e.g., `plain`, `complex`, `astro`, `nextjs`).
- `--backend-only`: Skip generating the frontend folder, useful if you already have an existing web project you just want to wrap.

### `forge dev`

Starts the local development environment. 

```bash
forge dev
```

**What it does internally:**
1. Parses your `forge.toml` configuration.
2. Spawns the frontend development server (like `vite`, `next dev`, or `astro dev`) as a background process.
3. Boots up the Python ASGI backend.
4. Opens the native desktop window and automatically points it to your local frontend server.
5. Enables **Hot Module Replacement (HMR)** for the frontend, and hot-reloads the Python window whenever backend code is edited.

### `forge build`

Compiles your project into a production-ready natively compiled binary.

```bash
forge build
```

**What it does internally:**
1. Commands the frontend tooling to run its production build script (e.g., `npm run build`).
2. Takes the outputted HTML/JS/CSS assets and bundles them securely alongside your Python code.
3. Packages the Python runtime, plugins, and your source code into a standalone directory using PyInstaller or Nuitka (depending on your configuration).
4. Generates the final output artifact (a `.exe` on Windows, a `.app` on macOS, or an executable binary on Linux) in the `dist/` directory.

**Options:**
- `--target <os>`: Attempt cross-compilation (if supported by your toolchain). Defaults to your current operating system.
- `--no-upx`: Disable UPX compression to speed up the bundling process during testing.

---

## ⚙️ Project Configuration (`forge.toml`)

The CLI reads all project metadata, build instructions, and plugin permissions from the `forge.toml` file at the root of your project.

When running `forge build` or `forge dev`, the CLI automatically uses the settings defined here. For a deep dive into every available configuration option, check out our **[API References](/references/)**.

---

## 🚀 Advanced Interface Usage

For users integrating ForgeDesk into larger CI/CD pipelines or automated workflows, the CLI commands support headless execution. 

Ensure that prompts are suppressed by providing the required flags explicitely (like passing `--template` to `create`) to prevent blocking during GitHub Actions or GitLab CI builds.

### Next Steps

Now that you understand the tooling, it's time to build something!
- Head to the **[Learning Hub](/learn/)** to start your first tutorial.
- Read through the **[Guides](/guides/)** to explore specific concepts.
