---
title: "Forge Architecture"
---

Forge is a "Tauri for Python" framework. It bridges web technologies (HTML/CSS/JS) with a high-performance Rust native core and a Python 3.14+ (NoGIL) backend to create secure, lightweight desktop applications.

## Layer Overview

![Forge Architecture Overview](../../assets/layer-overview.svg)

## 1. Native Core (Rust)
The lowest layer handles tight OS integration, bypassing massive UI bloat frameworks (like Electron/CEF).
- **Multi-Window System**: Built on `tao` and `wry`, maintaining a Rust-owned registry of active WebView windows with unique UUIDs. The Python and JS layers only see the Window labels.
- **Native OS GUI Consolidation**: Features like System Trays (`tray-icon`), context menus (`muda`), and File Pickers (`rfd`) run directly in Rust, interacting safely with native OS C APIs instead of relying on Python shims or browser WebAPIs, ensuring pure native event-loops.
- **Auto-Updater**: Background downloaded `Reqwest` payloads are strictly matched to an `Ed25519` key, swappable via `self-replace` without ever blocking the UI/event loop.

## 2. Python 3.14 (Free-Threading / NoGIL)
Because Forge uses **Python 3.14+ Free-Threading**, it removes the typical asynchronous UI bottleneck.
- Commands from the WebView immediately jump into a concurrent thread pool (`ThreadPoolExecutor`) executed safely bypassing the Global Interpreter Lock (GIL). 
- Python hot-reloads instantly during development (`forge dev` via `watchfiles` and `os.execv`) retaining active Window processes or cleanly spinning them back up.

## 3. IPC Bridge (Strict Type Validation)
Commands traverse from JS to Python through a tightly guarded bridge:
- Signatures derived via Python 3.14 type hints (`typing.get_type_hints`) instantly synthesize into exact **Pydantic schemas**.
- Unexpected kwargs, un-allowed execution scopes, or spoofed commands are rejected *before* Python evaluates the core function payload.

## 4. Zero-Copy Architecture (forge-memory:// and forge-asset://)

Unlike Electron, which streams large files via asynchronous websockets causing heavy JSON serialization/deserialization overhead, Forge implements a strict Zero-Copy Architecture routing through custom network HTTP schemes:
- **`forge-asset://`**: A secure local file protocol allowing the frontend to natively fetch local paths (`fetch('forge-asset://PATH')`) bypassing the serialization layer. This is strictly gated by the Python backend's `ScopeValidator` synchronizing path authorization between Rust and Python safely using PyO3 `Python::attach`.
- **`forge-memory://`**: A memory pointer protocol passing raw byte payloads between Python (`forge.memory.buffers`) and Rust cleanly over WebKit/WebView2 URL fetch interception with near-zero copy overhead.

## 5. Strict Permission Scopes

Capabilities are not just booleans—they are locked down to exact filesystem patterns (`allow` / `deny`), exact origins, and URL scopes in `forge.toml`. The `app.fs` and `app.shell` modules validate against matching globs (like `$APPDATA/my-app/**`) directly within the IPC Bridge before any system IO operates.