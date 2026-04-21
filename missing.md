# Forge Framework API Analysis

Based on a deep and rigorous analysis of the `forge-framework` codebase, here is an assessment of what APIs are present and what essential APIs are entirely missing to consider this a fully complete desktop application framework on par with industry standards like Electron or Tauri.

## Currently Implemented APIs

The framework provides a strong foundation with the following modules:
1. **Window / Webview / Setup** (`app.py`): Control single/multi window positions, sizes, states, devtools, ipc, capabilities.
2. **File System** (`fs.py`): Basic file interactions.
3. **Dialogs** (`dialog.py`): Native open, save, message box dialogs.
4. **Notifications** (`notification.py`): Desktop notifications.
5. **Deep Linking** (`deep_link.py`): Protocol schema bindings (`myapp://`).
6. **Clipboard** (`clipboard.py`): Read/write text to system clipboard.
7. **Menu** (`menu.py`): Native application menus and context menus.
8. **Tray** (`tray.py`): System tray/menu bar icons and simple menus.
9. **System** (`system.py`): Environment vars, OS info, opening standard URLs/files, checking basic state.
10. **Updater** (`updater.py`): OTA and app update management.

---

## 🛑 MISSING APIs

To build *any* kind of professional desktop application, the framework currently lacks several fundamental desktop APIs.

### 1. Global Shortcuts / Hotkeys API (`global_shortcut.py`)
✅ **STATUS: IMPLEMENTED**
- Support for registering complex shortcuts like `CmdOrCtrl+Shift+X` globally in Python and directly exposed to the Frontend SDK.

### 2. Screen & Displays API (`screen.py`)
✅ **STATUS: IMPLEMENTED**
- Full query capabilities for resolving primary, connected bounding coordinates, DPI scales, and native OS cursor position.

### 3. Native Network & Custom Protocols API (`protocol.py` or native Fetch)
✅ **STATUS: IMPLEMENTED**
- Secure Zero-Copy `forge-asset://` file proxy for large files with strictly enforced path sandboxing parsing `$APPDATA` paths correctly to bypass CORS securely.
- Memory pointer passing `forge-memory://` for passing raw memory dicts directly over WebKit URL hooks.

### 4. Power & Hardware State Monitor API (`power.py`)
✅ **STATUS: IMPLEMENTED**
- Native bindings for reading battery info, percentages, and global system suspend/resume sleep cycles.

### 5. Secure Storage / Keychain API (`keychain.py`)
✅ **STATUS: IMPLEMENTED**
- The frontend can now access the OS's native secure credential managers (macOS Keychain, Windows Credential Vault, Linux Secret Service).

### 6. App Lifecycle & Single Instance API (`app_lifecycle.py`)
✅ **STATUS: IMPLEMENTED**
- Strict OS-level named single-instance mutex locks (`requestSingleInstanceLock()`). Prevents users from accidentally splitting memory footprints. Includes automatic app restart behavior natively.

### 7. OS Integration & Taskbar API (`os_integration.py`)
✅ **STATUS: IMPLEMENTED**
- Taskbar/dock icon visual management: macOS dock bouncing, taskbar/dock progress bars to grab user attention natively.

### 8. Autostart / Login Items API (`autostart.py`)
✅ **STATUS: IMPLEMENTED**
- Programmatic registration to launch the application automatically when the OS boots/user logs in via JS API.

### 9. Hardware & Device Interfaces (WebUSB/WebSerial emulation or Native pass-through)
✅ **STATUS: IMPLEMENTED**
- **Feature integrated via pyserial proxy:** Exposing native hardware interfaces (USB, Serial ports, Bluetooth) safely to the front-end, or providing a structured backend wrapper.
- **Use case:** IoT flashed tools, keyboard configuration software, local hardware controllers.

### 10. Native Drag & Drop API
✅ **STATUS: IMPLEMENTED**
- Frontend can listen to `drag_drop`, `drag_hover`, and `drag_cancelled` events to capture absolute native file paths gracefully.

### 11. Printing and PDF Generation API
✅ **STATUS: IMPLEMENTED**
- Frontend Webview execution via `window.__forge__.printing.printPage()`.

---

## 🚀 The "Tauri Gap" (What it takes to be on par with Tauri)

Beyond just a list of raw APIs, reaching **Tauri's level of maturity** requires a fundamental shift in architecture, Developer Experience (DX), and security modeling. Here is the analysis of the framework's shortcomings compared to Tauri:

### 1. The Isolation Pattern & IPC Security
- **Tauri's Strength:** Tauri uses an "Isolation Pattern" where an injected iframe intercepts all IPC messages to prevent the frontend from directly invoking arbitrary backend code if the webview is compromised (XSS attacks). Messages are cryptographically signed. Furthermore, Tauri implements **Strict Scopes** (e.g., allowing `$FS/read` *only* in `C:\Users\John\AppData\Local\MyApp\data`).
- **Forge's Gap:** Forge's capability model (`has_capability`) is strictly boolean—it turns an API on or off entirely, without sandboxing paths or URLs.

### 2. State Management & Dependency Injection
- **Tauri's Strength:** You can register a native `Mutex<State>` in Rust, and Tauri will automatically inject it into any command handler that asks for it, heavily optimizing thread-safe state sharing.
- **Forge's Gap:** You are forced to manually manage global/singleton state in Python (`app.state()` or custom singletons). Handlers rely on closures or classes rather than elegant Dependency Injection of strongly-typed state pointers.

### 3. Zero-Overhead ABI / Memory passing
- **Tauri's Strength:** Moving a 50MB array of bytes from Rust to JS can be done via raw buffers or highly optimized WRY streams with near-zero copy overhead.
- **Forge's Gap:** Data must be serialized in Rust, deserialized into Python via PyO3, transformed by backend logic, serialized to JSON (or MessagePack), and sent to JS. This double-hop architecture introduces performance bottlenecks for heavy applications.

### 4. Multi-Window State Persistency
- **Tauri's Strength:** A first-party plugin saves window size, position, maximization state, and monitor bindings securely, ensuring that when the app restarts, it looks exactly identical to when it closed, preventing "window teleporting".
- **Forge's Gap:** Developers have to manually listen to resize/move events, write to the filesystem, and load those coordinates dynamically on process start.

### 5. Transparent & Vibrancy Window Styling
✅ **STATUS: IMPLEMENTED**
- Support for `vibrancy` (macOS visual materials like `hudWindow`, `popover`) and Windows Mica / Acrylic API blending directly via `forge.js` `window.__forge__.window.setVibrancy()`.

### 6. Built-in Auto-Updater
- **Tauri's Strength:** Native auto-updater with cryptographic signature verification built-in.
- **Forge's Gap:** Manual distribution or piecing together third-party Python update libraries.

### 7. App Bundle Size Optimization
- **Tauri's Strength:** App bundles can be incredibly small (~3MB) because the Rust binary has nearly zero runtime overhead and leverages the OS webview.
- **Forge's Gap:** Bundling Python via Nuitka inherently includes the Python interpreter and standard library, leading to minimum bundle sizes around 30-50MB.

### 8. Mobile Support (iOS / Android)
- **Tauri's Strength:** Tauri 2.0 treats mobile as a first-class citizen, wrapping WKWebView on iOS and WebView on Android.
- **Forge's Gap:** Strictly desktop/web-bound.

### Conclusion: Top Priorities to match Tauri
To become the "Python equivalent of Tauri", Forge must instantly prioritize:

**1. The Custom Protocol (`forge-memory://`) & Zero-Overhead Memory Passing.**
✅ **STATUS: IMPLEMENTED IN RUST CORE.** 
We just successfully bypassed the JSON serialization bottleneck.
- **Python side:** Developers can place byte-streams into `from forge.memory import buffers`.
- **Rust side:** `src/window/builder.rs` intercepts `forge-memory://` network requests using WebKit/WebView2's internal URI interceptors.
- **Execution:** Rust grabs the `PyBytes` reference using `pyo3` and streams the inner byte slice directly into an `wry::http::Response`. 
- **JS Native Fetch:** The frontend securely calls `const bytes = await fetch('forge-memory://huge_data.bin').then(r => r.arrayBuffer());`. No IPC WebSockets needed!

**2. Granular FS/Network Scopes** (Path-based sandboxing, not just booleans).
**3. Transparent / Mac-Mica Windowing APIs** (To appeal to UI designers).
