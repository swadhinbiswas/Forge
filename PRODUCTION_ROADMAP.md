# ForgeDesk - Robust Production Roadmap (Path to v1.0)

This document outlines the sequential, prioritized roadmap to bring ForgeDesk from its current state to a fully production-ready, enterprise-grade desktop framework (Tauri parity and beyond).

## 🛡️ Phase 1: Security & Core Hardening (CRITICAL)

_Security must be guaranteed before any production application is shipped._

- **[ ] 1.1 Filesystem Scope Hardening (`forge/builtins/fs.py`)**
  - Implement strict path resolution checks.
  - Block path traversal (`../../`) and absolute paths outside the explicitly allowed `$APPDIR`, `$DATADIR`, etc. scopes.
- **[ ] 1.2 Capability Auditing & Enforcement (`forge/runtime.py` / `bridge.py`)**
  - Enforce `forge.toml` permission scopes at the runtime level.
  - Implement a Deny-by-Default policy for all native API commands (e.g., shell execution, fs, network).
- **[ ] 1.3 Thread-Safe Event Emitters (`forge/events.py`)**
  - Wrap event listeners and emitters in thread-safe locks (`threading.Lock` or NoGIL equivalents) to guarantee safety under multi-threading and Python 3.14+ free-threading.

## ⚡ Phase 2: Performance & Architecture (Scale & Speed)

_Optimizing the bridge and foundational state logic to remove bottlenecks._

- **[ ] 2.1 Zero-Copy Binary IPC (`src/lib.rs` & `forge/bridge.py`)**
  - Replace JSON payload serialization with `msgpack`.
  - Implement zero-copy byte buffers for massive data transfer (e.g., sending large files or datasets directly to the frontend).
- **[ ] 2.2 Dependency Injection for State (`forge/state.py`)**
  - Refactor Python state management from closure/singleton-heavy logic to a strict Injection System (similar to Tauri's `State<T>`).
  - Ensure type-safe, thread-safe dependency injection on all `@forge.command` routes.

## 📦 Phase 3: Build, Distribution & Updates (DX)

_Ensuring application bundles are small, builds are reproducible, and updates are seamless._

- **[ ] 3.1 Build Optimization & Shrinking (`forge_cli/bundler.py`)**
  - Optimize Nuitka configurations to minimize bundle sizes (target < 15MB).
  - Implement post-build asset stripping/tree-shaking for omitted dynamic libraries.
- **[ ] 3.2 Hermetic Toolchains**
  - Automate downloading of isolated, version-pinned build tools (WiX, appimagetool) into `.forge/toolchains`.
  - Eliminate reliance on user host environment setups.
- **[ ] 3.3 Sidecar Delta Updaters (`src/updater.rs` & `forge/recovery.py`)**
  - Implement a lightweight binary diff patcher (e.g., `bsdiff`) to fetch small delta updates instead of pulling entire 50MB+ bundles.

## 🪟 Phase 4: User Experience (Polish)

_Features that make apps feel like high-quality native software._

- **[ ] 4.1 Window State Persistency (`forge/window.py` & `src/native_window.rs`)**
  - Automatically save and load window size, position, and maximized state to disk.
  - Restore window state seamlessly on application launch avoiding "window jumping" or resetting to the center.
- **[ ] 4.2 Comprehensive Error Dialogs**
  - Catch silent Python crashes (e.g., segfaults or unhandled exceptions in `__main__`) and display native Rust UI panic dialogs to the end-user.
