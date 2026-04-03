# Forge Framework Implementation Plan: Elevating to Tauri Equivalence

This document outlines the priority-based rollout strategy to implement the missing native APIs identified in `missing.md`.

## 🏗️ Phase 1: Core Desktop Foundations
These are non-negotiable for a professional desktop app experience.

1. **Screen & Displays API (`forge/api/screen.py`)** **(STARTING HERE)**
   - **Rust Backend:** Use `tao`'s `MonitorHandle` to get active monitors, bounding boxes, and scale factors.
   - **Python API:** `app.screen.get_monitors()`, `app.screen.primary()`.
2. **Global Shortcuts (`forge/api/shortcuts.py`)**
   - **Rust Backend:** Integrate `global-hotkey` crate.
   - **Python API:** `app.shortcuts.register("Cmd+Shift+X", callback)`.
3. **App Lifecycle & Single Instance (`forge/api/lifecycle.py`)**
   - **Rust Backend:** Platform-specific named pipes or `single-instance` crate.
   - **Python API:** `app.lifecycle.request_single_instance()`.

## 🛠️ Phase 2: OS Integration & UX
These APIs make the application feel truly native to the host operating system.

4. ~~Taskbar & Dock Integration (`forge/api/os_integration.py`)**
   - **Features:** Progress bars, badges, macOS dock bouncing.
5. ~~Autostart & Login Daemons (`forge/api/autostart.py`)**
   - **Features:** Register app to launch at boot/login.
6. ~~Power Monitor API~~ (`forge/api/power.py`)**
   - **Features:** Suspend/resume events, AC/battery state.

## 🔐 Phase 3: Tauri-Level Security & High-Performance IPC
Implementing the "Tauri Gap" architectural differences.

7. ~~Strict FS & Capability Scopes~~
   - **Features:** Change `permissions.fs = true` to `permissions.fs.read = ["$APPDATA/myapp"]`.
8. ~~**Secure Keychain Storage API (`forge/api/keychain.py`)**
   - **Features:** Encrypted credential vault using `keyring` crate.
9. ~~**Native Custom Protocol Streams (`forge://`)**
   - **Features:** Intercept web requests natively to stream fast buffers.

---

## 🚀 Execution: Progressing to Phase 3
Phase 1, 2, and 3 are complete. All native APIs implemented.

## 🎨 Phase 4: UI Vibrancy, Persistency & Native Feel
Focusing on making the framework feel like a native, robust, and highly *Pythonic* experience for developers.

10. **Transparent & Vibrancy Window Styling**
    - **Rust Backend:** Integrate `window-vibrancy` for native macOS Mica/Windows Acrylic.
    - **Python API:** `app.config.window.vibrancy = "sidebar"` (Abstracting the complexity into Python settings).
11. **Multi-Window State Persistency (`forge/api/window_state.py`)**
    - **Features:** Auto-save/restore window positions and sizes to prevent "window teleporting" using a pure Python persistent state manager.
12. ~~Native Drag & Drop API (`forge/api/drag_drop.py`)**
    - **Features:** Intercept heavy files natively and emit clean Python events (`@app.events.on("drag_drop")`) with native absolute paths.
13. ~~Printing and PDF Generation~~
    - **Features:** Expose silent printing and PDF generation to the Python backend with zero overhead.
14. **Unified Build Tooling (`forge_cli/build.py`)**
    - **Features:** A seamless `forge build` command handling Vite, Python bundling, and native installer generation orchestrations in one Python-centric step.

---
## 🚀 Execution: Progressing to Phase 4
Starting Phase 4 (UI Vibrancy, Persistency & Native Feel).

## 🧱 Production Hardening Backlog
These items are now the focus for making the framework feel production-grade:

1. **Capability audit and enforcement pass**
   - Ensure every native API that reaches the bridge has a clear capability boundary.
   - Align `forge.toml` permissions with runtime checks and add tests for deny-by-default paths.

2. **Runtime wiring cleanup**
   - Remove duplicate imports and duplicate initialization blocks in `forge/app.py`.
   - Keep the app bootstrap path deterministic and easy to reason about.

3. **Event emitter compatibility**
   - Support both decorator-style and callback-style listeners so APIs stay ergonomic.
   - Keep the emitter thread-safe and test-covered.

4. **Filesystem scope hardening**
   - Support explicit allowed directories.
   - Reject absolute path access outside the configured scope.

5. **CLI and example parity**
   - Keep templates, docs, and examples aligned with the runtime API surface.
   - Add regression coverage for scaffolded apps and example projects.

6. **Build and release tooling**
   - Finish the unified build pipeline.
   - Ensure packaging, signing, and release-manifest generation remain reproducible.
