---
title: Welcome to References
description: Complete API References for ForgeDesk backend, frontend, plugins, and CLI.
---

Welcome to the **ForgeDesk API References**! Here you will find the complete, exhaustive documentation for every class, method, function, and configuration option available in the Forge framework.

## 📖 Available References

Browse through the references to dig deeper into the technical specifications of Forge components.

### 🐍 1. Python API (`forge`)
The backend Python library that controls the desktop window, IPC channels, and lifecycle.
- **[forge.App](/api-reference#forgeapp)**: The main application instance.
- **[forge.window](/api-reference#forgewindow)**: Window manipulation, bounds, and state.
- **[forge.events](/api-reference#forgeevents)**: Event emitting and listening.
- **[forge.dialogs](/api-reference#forgedialogs)**: System file pickers, alerts, and message boxes.

### ⚡ 2. Frontend API (`@forgedesk/api`)
The TypeScript/JavaScript API to interact with the Python backend from the webview.
- **[invoke](/api-reference#invoke)**: Call Python commands from JavaScript.
- **[listen](/api-reference#listen)**: React to events emitted from the Python side.
- **[Clipboard](/api-reference#clipboard)**: Native copy and paste operations.
- **[Notifications](/api-reference#notifications)**: Trigger native OS notifications.

### 🛠️ 3. Configuration (`forge.toml`)
The central configuration file for building and running your Forge applications.
- **[App Metadata](/api-reference#app-metadata)**: Name, version, description.
- **[Build Settings](/api-reference#build-settings)**: Output formats, binary naming, icon paths.
- **[Permissions](/api-reference#permissions)**: Security and isolation settings for what the frontend can access.

### 🧩 4. Native Plugins
Official plugins that extend Forge's capabilities.
- **[@forgedesk/fs](/plugins/fs)**: Native filesystem access from the frontend.
- **[@forgedesk/keychain](/plugins/keychain)**: Secure storage for passwords and tokens.

---

## Need a Guide Instead?

If you're looking for step-by-step instructions or tutorials on how to use these APIs, check out the **[Guides](/guides/)** section!
