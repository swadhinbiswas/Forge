# ForgeDesk: The Tauri Parity & Enhancement Plan

To elevate ForgeDesk from a "Python Web-Wrapper" to a **Tauri-level Native Framework**, we must aggressively mitigate Python's performance penalties while leaning heavily into its data-science and AI capabilities. 

This document outlines the architectural guidelines and concrete roadmap to achieve feature parity with Tauri.

---

## 🏎️ Phase 1: Eradicating the IPC Bottleneck (Raw Binary IPC)

Currently, sending a 50MB Pandas DataFrame or a 4K image from Python to the WebView via JSON-RPC + Base64 encoding will cause massive freezes and memory spikes.

### The Solution: Mixed-Mode IPC Protocol
We will split our Inter-Process Communication (IPC) into two distinct channels:

1. **Control Channel (JSON over HTTP/WS):** Keeps the existing `@router.command` structure. Used for small, typed JSON payloads (e.g., triggering a function, returning a boolean, updating configuration).
2. **Data Channel (Zero-Copy or Raw Sockets):** 
    * **Approach A: Local TCP/UDS Sockets.** Python opens a specialized raw socket, and the frontend fetches binary chunks via standard JavaScript `fetch()` -> `ReadableStream` or WebSockets configured for `ArrayBuffer`.
    * **Approach B: Protocol Buffers / MessagePack.** Serialize complex Python objects (like ML tensors) directly into binary arrays instead of formatting them as JSON strings.
    * **Approach C: Virtual Custom Scheme (The Tauri Way).** Implement a custom protocol handler in our native window wrapper (e.g. `forgedesk://asset/images/dataset.png`). When the frontend requests this URL, the native layer intercepts it and streams the local bytes without ever hitting a web server.

**Action Item:** Implement the Custom Protocol Handler in `src/native_window.rs` (if using Rust for the window) or `bridge.py` so standard `<img src="forgedesk://...">` tags can load massive local files instantly.

---

## 🪟 Phase 2: Core Platform Upgrades

Tauri feels "native" because it deeply integrates with the OS window manager and packaging systems. ForgeDesk must do the same.

### 1. Multi-Window Support
* **The Problem:** ForgeDesk currently supports a 1:1 Backend-to-WebView model.
* **The Fix:** Upgrade `forge/window.py` to act as a **Window Manager**. Allow Python or JS to spawn sub-windows dynamically (`app.windows.create("settings", url="/settings")`). The backend must route IPC messages to the correct window ID.

### 2. Native Multi-Platform Bundler
* **The Problem:** `forge build` builds an executable, but users expect true installers.
* **The Fix:** Integrate native installers into the CLI:
    * Windows: Auto-generate `.NSIS` or `.MSI` utilizing WiX Toolset.
    * macOS: Auto-generate `.DMG` and integrate `codesign` + notarization workflows (so macOS doesn't block the app).
    * Linux: Auto-generate `.AppImage` or `.deb`.

### 3. Native OS Menus & Systray
* **The Problem:** Recreating top-bar menus (File, Edit, View) in HTML feels like a fake web app.
* **The Fix:** Create a `@forgedesk/api/menu` and `@forgedesk/api/tray` plugin to hook into the OS native GUI APIs. Let developers configure native menus in `forge.toml` or dynamically via Python/JS.

---

## 🧩 Phase 3: Plugin Ecosystem Parity

To prevent developers from abandoning ForgeDesk due to missing OS features, we must build a robust suite of native plugins.

**The "Must-Have Before 1.0" Plugin List:**

* [ ] `api/dialog`: Bind to native OS file pickers (Open File, Save Document, Select Folder). Blocking UI thread alerts.
* [ ] `api/shell`: Spawn child OS commands (like running `git` or `ffmpeg`) securely from the frontend, with live `stdout` streaming over the IPC.
* [ ] `api/clipboard`: Read/write text, HTML, and *images* directly to the native clipboard.
* [ ] `api/notification`: Trigger OS-level toast notifications (Windows Action Center, macOS Notifications).
* [ ] `api/global-shortcut`: Register system-wide keybinds that trigger Python/JS events even when the app is not focused.

---

## 🤖 Phase 4: The "Python Advantage" (Out-competing Tauri)

Tauri wins on binary size and speed. ForgeDesk must win on **Capabilities**. We need built-in plugins that Tauri developers would kill for:

* **AI Out-of-the-Box:** A `@forgedesk/plugin-llm` that automatically downloads and runs local HuggingFace/Llama.cpp models without the developer needing to configure complex C++ build chains.
* **Pandas / Jupyter UI Integrations:** A pre-built frontend data-grid component that natively binds to a Python Pandas DataFrame over the high-speed binary IPC.
* **System Automation:** True bindings for `pyautogui` injected directly into the frontend, enabling desktop automation scripts instantly.

---

## Execution Priority (What to code first)

1. **Immediate Priority:** The Custom Scheme Handler / Binary IPC (Phase 1). This is the foundation; without it, AI or Data Science apps will crash the UI.
2. **High Priority:** The `dialog` and `shell` native plugins. These are the most commonly requested features for basic desktop apps.
3. **Medium Priority:** Multi-window orchestration. 

Are we ready to start coding Phase 1?
