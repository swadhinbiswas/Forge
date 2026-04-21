---
title: Core Concepts
description: Understand the fundamental architecture and philosophy behind ForgeDesk.
---

Welcome to the **Core Concepts** section of ForgeDesk! To build fast, secure, and maintainable native desktop applications with ForgeDesk, it's essential to understand its underlying architecture.

## 🏗️ The Dual-Process Architecture

ForgeDesk utilizes a **Dual-Process Architecture**. Instead of packing an entire web browser and a Node.js runtime into the application (like Electron), Forge operates natively using two distinct areas:

1. **The Python Backend (Core Process)**
2. **The Frontend Webview (Render Process)**

### 🐍 1. Python Backend
The backbone of your application is a fully-typed Python ASGI runtime.
- **System Access:** Has full access to native OS components, file systems, and databases.
- **Window Management:** Spawns and manages the native application window (size, state, titlebar).
- **Event Loop:** Runs an asynchronous event loop that processes events, routes API calls, and hosts local networking/database connections seamlessly.

### 🌐 2. Frontend Webview
Your user interface is rendered using the operating system's native webview component (e.g., WebView2 on Windows, WKWebView on macOS/Linux).
- **Framework Agnostic:** You can use React, Vue, Svelte, Astro, Next.js, or plain HTML/JavaScript.
- **Lightweight:** No bundled chromium engine. The installer size stays tiny and RAM usage drops to a fraction of traditional web-wrapper apps.
- **Sandboxed:** The frontend doesn't have direct access to your hard drive. It must pass messages to the Python backend to accomplish secure native tasks.

---

## ⚡ Inter-Process Communication (IPC)

Because the frontend is sandboxed, Forge uses a blazing-fast, asynchronous JSON-RPC protocol to allow the webview and Python to communicate seamlessly.

### 🗣️ Invoking Commands (`invoke`)
The frontend calls Python functionality via the `@forgedesk/api` library's `invoke` method.
When a call is sent, Forge's router executes the corresponding decorated `@forge.router.command()` Python function, awaits its result, and instantly resolves the JavaScript promise.

### 🎧 Listening to Events (`listen`)
For push-based communication (such as download progress or a native shortcut keypress), Python can emit events directly to the frontend. JavaScript components use `listen("event-name", callback)` to react instantly.

---

## 🛡️ Security by Default

ForgeDesk treats webviews as untrusted by default.
- **Context Isolation:** The frontend JavaScript context is completely separated from the native bindings.
- **Explicit Permissions:** Plugins like `@forgedesk/fs` (filesystem access) require explicit configurations in your `forge.toml` to ensure the frontend can only read/write out of explicit approved directories.
- **No Eval:** Harmful executions and arbitrary protocol fetches are cleanly intercepted and denied.

---

## 📦 Plugins

You aren't strictly limited to writing all native code yourself. The **Forge Plugins** ecosystem provides pre-written Rust/Python packages that add explicit native API functions (like keychain management, deeper filesystem utilities, or printer access) straight to your frontend.

### Next Steps...

Ready to start building? 
- Check out the **[Guides](/guides/)** to dive into code.
- Read through the **[API References](/references/)** if you're looking for specific functions.
