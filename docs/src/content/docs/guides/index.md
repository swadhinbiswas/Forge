---
title: Welcome to Guides
description: Comprehensive guides, tutorials, and recipes for building modern desktop apps with Forge.
---

Welcome to the **Forge Framework Guides**! Whether you are just getting started with Desktop app development or you're a seasoned Electron veteran looking for a lighter, faster alternative, these guides are designed to help you build, optimize, and ship extraordinary cross-platform apps using Python and Web technologies.

## 🧭 Navigating the Guides

These guides are broken down into logical categories to help you find the exact information you need as quickly as possible.

### 📚 1. Core Concepts
Master the fundamental architecture of Forge.
- **[App Architecture](/architecture)**: Understand the multi-process design, IPC bridge, and webview integration.
- **[Lifecycle Management](/guides/lifecycle)**: Learn how Forge handles startup, background execution, and graceful teardown.
- **[Routing & IPC](/guides/routing)**: Deep dive into `@forgedesk/api` and Python `@app.command()` routing.

### 🎨 2. Frontend Frameworks
Learn how to use Forge with your favorite web framework. Forge is completely agnostic, but we have tailored guides for the most popular options:
- **[React & Next.js](/guides/react)**: Bootstrapping SPAs, using React 19, and SSG with standard hooks.
- **[Vue & Nuxt](/guides/vue)**: Reactivity, pinia state, and Vue Router integration.
- **[Svelte](/guides/svelte)**: Best practices for Svelte 5 and the SvelteKit adapter.
- **[Vanilla JS / HTML](/guides/vanilla)**: Sticking to the brilliant basics.

### 🔌 3. Backend & Python
Combine the vast ecosystem of Python with your application.
- **[Managing State](/guides/state)**: Keep your Python backend state perfectly synchronized with your UI.
- **[System Access & Native Dialogs](/guides/native-api)**: Using standard OS features like native file pickers, tray icons, and notifications.
- **[Database Integration](/guides/database)**: Connecting SQLite, SQLAlchemy, or Postgres to your desktop app.

### 🛠️ 4. Build & Distribute
Your app is done—now it's time to get it into your users' hands!
- **[Packaging for Production](/distribute)**: Creating standalone macOS `.app` / `dmg`, Windows `.exe`, and Linux `AppImage` files.
- **[Code Signing & Notarization](/guides/code-signing)**: Prevent "Unknown Developer" warnings.
- **[Auto-Updating](/guides/auto-updates)**: Distribute over-the-air (OTA) patches seamlessly.

### 🚀 5. Advanced Recipes
Pushing Forge to its absolute limit!
- **[Writing Custom Plugins](/plugins)**: Creating shareable and reusable Rust/Python C-extensions.
- **[Error Recovery Dynamics](/guides/error-recovery)**: Setting up fallback UI and crash-dump reporting.
- **[Migrating from Electron](/migration-from-electron)**: A step-by-step translation map of Electron APIs to Forge calls.

---

## Need Help?

Can't find what you're looking for? 
* Search the **[API Reference](/api-reference)** for specific method signatures.
* Join the discussion over on our **[GitHub Discussions](https://github.com/forgedesk/forge/discussions)** or our Discord community. Happy forging!
