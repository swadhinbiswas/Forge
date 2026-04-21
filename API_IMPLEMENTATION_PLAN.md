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

## 🧱 Phase 5: Production Hardening & Architectural Audits
These items address the glaring gaps that separate Forge from a "perfect, production-grade" framework like Tauri.

### 1. ~~IPC Security & Strict Payload Routing (The XSS/RCE Mitigation)~~
- **Gap:** IPC payload routing currently parses standard JSON with minimal structural enforcement before reaching Python limits. An attacker injecting bad payloads or prototype manipulation attributes could execute arbitrary actions if arguments aren't strictly type-checked.
- **Action Plan:**
  - **Adopt `msgspec` Validation:** Change `json.loads(raw_message)` and dictionary passing to `msgspec.json.decode(raw_message, type=CommandEnvelope)` within `forge/bridge.py` for massive speedups and structural immunity.
  - **Parameter Schema Enforcement:** Validate incoming `args` specifically for each registered command using `inspect` metadata and `msgspec` structures to guarantee types at the boundary.

### 2. Build & Distribution Asset Pipeline (The Native OS Installer Problem)
- **Gap:** `forge_cli/bundler.py` generates installers via crude string templating (e.g., XML for Windows WiX, Plist strings for macOS).
- **Action Plan:**
  - Introduce AST-based generation for `.plist`, `AndroidManifest.xml`, and `.wxs`.
  - Download Hermetic Toolchains (e.g. WiX, appimagetool) automatically mapped to `.forge/toolchains` instead of depending on the user's host environment.
  - Handle complex code signing entitlements natively (macOS Hardened Runtime, Windows Authenticode EV limits).

### 3. Application Footprint Minimization
- **Gap:** Python + Rust + Webview = bloated footprint. Nuitka module trimming leaves behind heavy dependencies.
- **Action Plan:**
  - Default optimization switches to `pyoxidizer` configurations leveraging zero-copy, mem-mapped imports for Python environments to dramatically reduce execution overhead.
  - Shrink the bundled asset directories post-build.

### 4. Zero-Copy & Binary IPC
- **Gap:** Base JSON IPC serialization causes UI studders with high-frequency updates or large files.
- **Action Plan:**
  - Add native IPC streams / ArrayBuffer passing over `window.__forge__.invoke_buffer()` that bypasses UTF-8 string encoding limits.
  - Implement zero-copy byte serialization directly from `src/lib.rs` to Python `bridge.py` using `msgpack` under the hood.

### 5. Backend Reloading (DX Evolution)
- **Gap:** Modifying Python logic currently requires killing the Rust host window and restarting the full stack.
- **Action Plan:**
  - Integrate an `os.execv` or `importlib.reload` based backend watcher that re-initializes Python state while the Rust Webview instance holds state and reconnects the IPC pipe.

### 6. Delta Updater
- **Gap:** Updater pulls full 50MB+ bundles.
- **Action Plan:**
  - Create a Sidecar Updater binary.
  - Ship delta binary patches (`courgette/bsdiff` equivalent) directly executed by the isolated sidecar instead of replacing monolithic bundles mid-flight.

---

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
