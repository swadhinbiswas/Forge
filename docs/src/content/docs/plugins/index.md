---
title: Plugins
description: Extend ForgeDesk capabilities with native system plugins.
---

Welcome to the **ForgeDesk Plugins** section! While the core ForgeDesk package focuses on window management and secure Inter-Process Communication (IPC), **Plugins** are what give your webview powerful native capabilities.

---

## 🧩 What are Plugins?

Plugins are pre-written libraries (usually implemented in Rust or Python) that connect directly to native operating system APIs and expose them cleanly to your frontend JavaScript over our secure JSON-RPC bridge.

Because we believe in **Security by Design**, plugins are never enabled by default. To use a plugin, you must explicitly enable and configure its permissions in your `forge.toml` file.

---

## 📦 Official Plugins

Here is a list of the official native plugins maintained by the ForgeDesk team:

*   **[🗂️ Filesystem (fs)](/plugins/fs)**  
    Safely read, write, move, and copy files from the frontend. Supports strict sandbox scopes (e.g., `["$APP_DATA", "$DOWNLOADS"]`).
*   **[🔐 Keychain](/plugins/keychain)**  
    Securely store and retrieve passwords, tokens, and sensitive data using the native OS credential manager (Credential Manager on Windows, Keychain on macOS, Secret Service on Linux).
*   **[🔔 Notifications](/plugins/notification)**  
    Trigger rich native push notifications to notify the user of background tasks, messages, or alerts.
*   **[🪟 Window Management](/plugins/window)**  
    Take granular control over secondary windows, frameless settings, transparency, system tray integration, and custom title bars directly from CSS/JS.

---

## 🛠️ Adding a New Plugin

Using a plugin usually involves two steps: installing the package and configuring its permissions.

### 1. Install the Plugin

Install the frontend and backend counterparts using your package manager.

```bash
# Frontend
npm install @forgedesk/plugin-name

# Backend (if applicable)
pip install forgedesk-plugin-name
```

### 2. Configure `forge.toml`

Every plugin that interacts with the system must be declared in your configuration file.

```toml
# forge.toml

[plugins]
# Enable the plugin
name = true

[permissions.name]
# Explicitly authorize what it can do
read = true
write = false
```

---

## 🔧 Building Your Own Plugins

Didn't find what you need? ForgeDesk makes it easy to build your own plugins to expose custom Rust, C++, or Python libraries. 

Check out the **[Build a Custom Plugin](/learn/custom-plugins)** tutorial to learn how to hook into the Forge application lifecycle, register IPC endpoints, and package it for other developers.
