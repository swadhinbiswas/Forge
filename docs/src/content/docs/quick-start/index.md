---
title: Quick Start
description: Get up and running with ForgeDesk in seconds.
---

Welcome to **ForgeDesk**! This guide will walk you through scaffolding your very first Python-powered desktop application.

## Scaffold Your Project

We provide a robust scaffolding utility to quickly set up your project with your favorite frontend framework (React, Vue, Astro, vanilla, etc.) already pre-configured to communicate with the Python backend.

Because of how NPM's `create` command parses package names natively, it is important to execute the scaffold using the following commands to avoid the `create-create` 404 registry error.

Run one of the following commands in your terminal:

### Using NPM
```bash
npx @forgedesk/create-forge-app@latest my-app
```

### Using PNPM
```bash
pnpm dlx @forgedesk/create-forge-app@latest my-app
```

### Using Yarn
```bash
yarn dlx @forgedesk/create-forge-app@latest my-app
```

### Using Bun
```bash
bunx @forgedesk/create-forge-app@latest my-app
```
---

## 2. Installation

Once the CLI finishes generating your application, navigate into the project directory and install the dependencies. The template automatically includes the standard `package.json` for your frontend and `requirements.txt` (or similar) for the backend.

```bash
cd my-app

# Install frontend dependencies
npm install

# Create a virtual environment for Python (highly recommended)
python -m venv .venv

# Activate the venv (Mac/Linux)
source .venv/bin/activate
# Activate the venv (Windows)
# .venv\Scripts\activate

# Install the Python dependencies (Forge core)
pip install -r requirements.txt
```

---

## 3. Run the Development Server

With dependencies installed, simply boot up the Forge development environment. This automatically starts your frontend development server, watches your Python code, and opens your native desktop window!

```bash
forge dev
```

You are now running a blazing-fast native application securely bridged to a fully-typed Python backend.

## Next Steps

- Want to see how the IPC works? Head over to the [Develop](/develop/) section.
- Need native filesystem or window controls? Check out our [Plugins](/plugins/).
