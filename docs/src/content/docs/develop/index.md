---
title: Developing with ForgeDesk
description: Comprehensive guide on how to build, compile, and structure your Python-powered desktop applications.
---

Welcome to the development core of **ForgeDesk**. This section covers the fundamental concepts, structural architecture, and practical steps for building robust, secure, and performant desktop applications using Python and native webviews.

## Workspace Architecture

A standard ForgeDesk project fuses two primary environments communicating in real-time over a highly optimized, strictly typed Inter-Process Communication (IPC) bridge:

1. **The Webview (Frontend)** - Powered by your framework of choice (React, Vue, Astro, vanilla JS). 
2. **The Core (Backend)** - Powered by Python (ASGI, custom routers, states), acting as the native OS bridge.

---

## 1. The Development Server

To see your application in action with instant feedback, rely on the Forge CLI. The CLI handles hot-module replacement (HMR) for both the frontend development server and the Python runtime simultaneously.

```bash
forge dev
```

### What happens under the hood?
1. Spawns your framework's local dev server (e.g., Vite on `localhost:5173`).
2. Starts the Forge Python core (`forge-framework.asgi`) in watch mode.
3. Automatically opens the native Rust OS window pointing to the dev URL.
4. Securely bridges standard `stdout`/`stderr` logging directly to your terminal.

---

## 2. Frontend Integration

Your frontend operates in a secure webview. To interact with the host OS, ForgeDesk safely injects the global asynchronous `window.forge` object at window creation time.

### Invoking Native Python Commands
You can trigger backend logic smoothly using Promises:

```javascript
// A simple payload passed to Python
const result = await window.forge.invoke('database_query', { 
    table: 'users',
    limit: 10 
});

console.log("Users fetched natively: ", result);
```

### Subscribing to System Events
Push-based notifications from Python are handled via strictly typed event listeners:

```javascript
// Reacting to a file download progression emitted from Python
const unlisten = window.forge.events.listen('download-progress', (payload) => {
    updateProgressBar(payload.percent);
});

// Clean up when the component unmounts
unlisten();
```

---

## 3. The Python Backend

All heavy lifting—file system access, database queries, cryptographic signing, or machine learning model execution—should happen in your Python layer. 

### Defining Commands
Commands act as your native endpoints. Map them natively to your frontend promises using the `@app.command` decorator.

```python
# src/app.py
from forge import app

@app.command("database_query")
def fetch_users(table: str, limit: int):
    # Perform heavy OS/DB tasks here securely
    # Only return JSON-serializable payloads
    return get_db_records(table, limit=limit)

if __name__ == "__main__":
    app.run()
```

### State Management
A global, mutable backend state allows you to persist data cleanly without exposing it to webview memory environments (preventing XSS data scraping).

```python
from forge import app
from forge.state import AppState

@app.command("increment_counter")
def increment(state: AppState):
    state.counter += 1
    return {"current_value": state.counter}
```

---

## 4. Debugging & Developer Tools

Building a solid application requires top-tier visibility into both the frontend rendering and the backend executions.

### The Web Inspector
When running `forge dev`, **Inspect Element** is automatically enabled for the frontend native window. You have absolute access to the standard Chromium/WebKit console, Network tabs, and framework DevTools (React/Vue).

*Note: The Inspector is automatically disabled when you compile via `forge build` to ensure production security.*

### Cross-Boundary Error Handling
The `forge.diagnostics` layer naturally catches unhandled Python exceptions. If a command fails natively, the frontend `invoke` Promise throws a formatted JavaScript `Error` with the stack trace safely stripped in production but fully verbose in development.

```javascript
try {
    await window.forge.invoke('unstable_task');
} catch (error) {
    // Graceful native error handling
    console.error("Python task failed:", error.message);
}
```

## Next Steps

Now that you understand the fundamental loop, explore our deep dives into extending capabilities:
- Check out the [Plugins](/plugins/) directory to utilize the File System, Keychain, or Notifications.
- Read through the [Security](/security/) guide to understand isolation concepts.
