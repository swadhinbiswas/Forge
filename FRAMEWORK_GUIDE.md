# Forge Framework Developer Guide

**Version: 3.0.0** | Python 3.14+ (Free-Threaded) | Cross-Platform Desktop Applications

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Core Concepts](#core-concepts)
4. [API Reference](#api-reference)
5. [Configuration Guide](#configuration-guide)
6. [Security & Best Practices](#security--best-practices)
7. [Advanced Topics](#advanced-topics)
8. [Code Examples](#code-examples)
9. [Troubleshooting](#troubleshooting)

---

## Introduction

### What is Forge Framework?

Forge is a **production-ready desktop application framework** that bridges Python 3.14+ (free-threaded/NoGIL) backend logic with modern web technologies (React, Vue, Svelte, or vanilla HTML/CSS/JS) in the frontend. It enables developers to:

- **Write backend logic in Python** with full access to the operating system
- **Build UI with modern web tools** (Vite, React, TypeScript, Tailwind CSS, etc.)
- **Communicate bidirectionally** via a high-performance, security-hardened IPC bridge
- **Deploy as native executables** for Windows, macOS, and Linux

### Key Features

| Feature | Description |
|---------|-------------|
| **NoGIL Python 3.14+** | True multi-threaded parallelism without GIL contention |
| **Zero-Cost IPC** | msgspec-based JSON serialization for minimal overhead |
| **Command Decorators** | Simple `@app.command` API for exposing Python functions to JavaScript |
| **Event System** | Thread-safe bidirectional event emitter for real-time updates |
| **Built-in APIs** | 20+ production APIs: File System, Shell, Dialogs, Clipboard, Notifications, etc. |
| **Window Management** | Single and multi-window apps with full native integration |
| **Security First** | Origin validation, capability-based permissions, rate limiting |
| **Hot Reload** | Both Python backend and JavaScript frontend support hot reloading |
| **State Management** | Tauri-like `AppState<T>` for type-safe shared state |

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Native OS                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Rust Runtime (forge_core)                   │   │
│  │  • NativeWindow (WebKit/WPE)                        │   │
│  │  • Event Loop                                       │   │
│  │  • Native API Bindings                              │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
               ↑                                ↓
     IPC Bridge (msgspec)                JavaScript
               ↑                                ↓
┌─────────────────────────────────────────────────────────────┐
│                    Python Backend                           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  ForgeApp                                            │   │
│  │  • Command Registry & Dispatcher                     │   │
│  │  • Event Emitter                                     │   │
│  │  • Built-in APIs (FS, Shell, etc.)                  │   │
│  │  • Background Threads                               │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## Getting Started

### Installation

1. **Create a new Forge application** using the scaffolding tool:

```bash
create-forge-app my-app --template react
cd my-app
```

2. **Install dependencies**:

```bash
npm install                    # Frontend dependencies
pip install -r requirements.txt  # Python backend
```

3. **View your `forge.toml` configuration**:

```toml
[app]
name = "My App"
version = "1.0.0"
description = "A beautiful Forge desktop application"

[window]
title = "My App"
width = 1200
height = 800
resizable = true
```

4. **Start development**:

```bash
python -m forge dev
```

This launches both the Vite dev server and the Python backend with hot reload enabled.

### Your First Command

**Backend (`src/main.py`):**

```python
from forge import ForgeApp, command

app = ForgeApp()

@app.command
def greet(name: str) -> str:
    """A simple IPC command."""
    return f"Hello, {name}!"

if __name__ == "__main__":
    app.run()
```

**Frontend (`src/App.tsx`):**

```jsx
import { invoke } from './forge-api.js'

export default function App() {
  const [greeting, setGreeting] = React.useState("")

  const handleGreet = async () => {
    const result = await invoke("greet", { name: "World" })
    setGreeting(result)
  }

  return (
    <div>
      <button onClick={handleGreet}>Greet</button>
      {greeting && <p>{greeting}</p>}
    </div>
  )
}
```

### Project Structure

```
my-app/
├── forge.toml                    # Configuration
├── package.json                  # Frontend dependencies
├── requirements.txt              # Python dependencies
├── src/
│   ├── main.py                  # Python entry point
│   ├── frontend/
│   │   ├── index.html           # HTML entry
│   │   ├── index.css            # Global styles
│   │   └── App.tsx              # React root
│   └── components/              # React components
├── vite.config.mjs              # Vite configuration
└── tailwind.config.js           # Tailwind CSS
```

---

## Core Concepts

### ForgeApp

The `ForgeApp` class is the central hub of your application. It manages:
- Window creation and lifecycle
- IPC bridge initialization
- Built-in API registration
- Event emitter
- Command registration

**Basic initialization:**

```python
from forge import ForgeApp

app = ForgeApp()  # Loads config from forge.toml

# Customize settings programmatically
app.config.window.title = "Custom Title"

# Start the event loop
app.run()
```

**Lifecycle hooks:**

```python
@app.on_ready
def startup():
    """Called after the window is created."""
    print("App is ready!")
    app.notifications.show(
        title="Welcome",
        body="Application started successfully"
    )

@app.on_close
def cleanup():
    """Called before the window closes."""
    print("Cleaning up...")
```

### IPC (Inter-Process Communication) System

IPC enables secure bidirectional communication between Python and JavaScript.

#### Commands (Python → JS callable)

Commands are Python functions exposed to JavaScript via the `@app.command` decorator.

**Command registration:**

```python
@app.command
def sync_operation(value: int) -> int:
    """Synchronous command."""
    return value * 2

@app.command("custom_name")
def my_handler() -> str:
    """Custom command name."""
    return "result"

@app.command
async def async_operation(name: str) -> str:
    """Asynchronous command (optional)."""
    await asyncio.sleep(1)
    return f"Processed {name}"
```

**Invoking from JavaScript:**

```javascript
// forge-api.js provides a safe wrapper
import { invoke } from './forge-api.js'

// Invoke synchronous command
const result = await invoke("sync_operation", { value: 10 })
console.log(result)  // 20

// Invoke async command
const name_result = await invoke("async_operation", { name: "Alice" })
console.log(name_result)  // "Processed Alice"

// Error handling
try {
  await invoke("failing_command")
} catch (error) {
  console.error(error.message)
}
```

**Command decorators provide security metadata:**

```python
from forge.bridge import requires_capability

@app.command
@requires_capability("filesystem")
def read_file(path: str) -> str:
    """This command requires filesystem permission."""
    return app.fs.read_text(path)
```

#### Events (Python ↔ JS bidirectional)

Events stream data from backend to frontend in real-time.

**Python side:**

```python
import threading
import time

def poll_data():
    """Background thread that emits updates."""
    while True:
        data = get_current_status()
        app.emit("status:update", {
            "timestamp": time.time(),
            "status": data
        })
        time.sleep(1)

# Start background thread
thread = threading.Thread(target=poll_data, daemon=True)
thread.start()
```

**JavaScript side:**

```javascript
import { on, emit } from './forge-api.js'

// Listen for events from Python
const unsubscribe = on("status:update", (payload) => {
  console.log("Status:", payload.status)
  console.log("Timestamp:", payload.timestamp)
})

// Clean up when done
return () => unsubscribe?.()

// Emit events back to Python
emit("user:action", { action: "clicked", target: "button" })
```

#### Protocol & Validation

All IPC messages follow a strict protocol:

```json
{
  "command": "my_command",
  "id": "msg-123",
  "protocol": "1.0",
  "args": { "param1": "value1" }
}
```

Responses include structured error information:

```json
{
  "type": "reply",
  "id": "msg-123",
  "result": null,
  "error": "Permission denied",
  "error_code": "permission_denied"
}
```

### Router (Modular Commands)

Group related commands using `Router` to avoid global state:

```python
from forge import Router

# Create a named router
user_router = Router("user")

@user_router.command("get")
def get_user(user_id: str) -> dict:
    return {"id": user_id, "name": "Alice"}

@user_router.command("create")
def create_user(name: str) -> dict:
    return {"id": "new-id", "name": name}

# Include into app
app.include_router(user_router)

# Commands are now namespaced: "user:get", "user:create"
```

### Event System

The `EventEmitter` provides thread-safe event handling with sync and async listeners:

```python
# Synchronous listener
@app.events.on("data:change")
def on_data_change(payload):
    print(f"Data changed: {payload}")

# Asynchronous listener
@app.events.on_async("background:task_complete")
async def on_task_complete(payload):
    await some_async_operation(payload)

# Emit from any thread
app.emit("data:change", {"field": "name", "value": "Alice"})

# Remove a specific listener
app.events.off("data:change", on_data_change)

# Clear all listeners for an event
app.events.off_all("data:change")
```

### Window Management

#### Single Window (Default)

```python
# Access main window
window = app.window

# Window state and properties
window.set_title("New Title")
window.set_size(1024, 768)
window.set_position(100, 100)
window.set_fullscreen(True)
window.set_always_on_top(True)

# Query state
is_visible = window.is_visible()
is_focused = window.is_focused()
position = window.position()  # {"x": ..., "y": ...}
state = window.state()

# Window control
window.show()
window.hide()
window.focus()
window.minimize()
window.maximize()
window.close()
```

#### Multi-Window Applications

```python
# Create additional windows
new_window = app.windows.create(
    label="child",
    title="Child Window",
    width=600,
    height=400,
    url="/admin"  # Optional: load specific URL
)

# List all windows
all_windows = app.windows.list()
for w in all_windows:
    print(f"Window {w['label']}: {w['title']}")

# Get specific window
child = app.windows.get("child")

# Control windows by label
app.windows.set_title("child", "Updated Title")
app.windows.set_size("child", 800, 600)
app.windows.focus("child")
app.windows.close("child")
```

**From JavaScript:**

```javascript
// Create window
const result = await invoke("__forge_window_create", {
  label: "settings",
  title: "Settings",
  width: 500,
  height: 400
})

// Close window
await invoke("__forge_window_close_label", { label: "settings" })

// Window events
on("window:focus", (payload) => {
  console.log("Window focused:", payload.label)
})
```

### State Management (AppState)

Type-safe, thread-safe shared state container similar to Tauri's `State<T>`:

```python
from forge.state import AppState
from dataclasses import dataclass

@dataclass
class AppData:
    user_name: str = "Alice"
    theme: str = "dark"
    count: int = 0

# Store in app
app.state.manage(AppData(user_name="Bob"))

# Auto-inject into commands
@app.command
def increment(state: AppState) -> int:
    """State is auto-injected."""
    data = state.get(AppData)
    data.count += 1
    return data.count

# Or typed injection (preferred)
@app.command
def get_user(data: AppData) -> str:
    """Typed injection - directly receives managed AppData."""
    return data.user_name

@app.command
def set_user(data: AppData, new_name: str) -> None:
    """Modify shared state."""
    data.user_name = new_name
```

**From JavaScript:**

```javascript
// Get state from runtime
const state = await invoke("__forge_runtime_get_state")
console.log(state)
```

---

## API Reference

### File System API (`app.fs`)

Provides sandboxed file system operations with strict access control.

**Configuration required in `forge.toml`:**

```toml
[permissions.filesystem]
read = ["$APPDATA/MyApp/**", "~/Documents/**"]
write = ["$APPDATA/MyApp/**"]
deny = [".git", "node_modules"]
```

**Python API:**

```python
# Reading
content = app.fs.read_text("config.json")
data = app.fs.read_file("image.png")  # bytes

# Writing
app.fs.write_text("data.json", '{"key": "value"}')
app.fs.write_file("output.bin", b"binary data")

# Directory operations
app.fs.create_dir("logs", recursive=True)
app.fs.remove_dir("temp", recursive=True)

# Listing
items = app.fs.list_dir("src")
for item in items:
    print(f"{item['name']} (dir={item['is_dir']})")

# File operations
app.fs.copy_file("src.txt", "dst.txt")
app.fs.move_file("old.txt", "new.txt")
app.fs.remove_file("deprecated.txt")

# Metadata
metadata = app.fs.metadata("file.txt")
# {"size": 1024, "modified": "2024-01-15T10:30:00Z", "is_dir": false}

exists = app.fs.exists("path/to/file")
```

**JavaScript API:**

```javascript
// Same API available in frontend
const content = await invoke("fs_read_text", { path: "config.json" })
const files = await invoke("fs_list_dir", { path: "src" })
```

### Shell API (`app.shell`)

Execute commands and open URLs safely.

**Configuration:**

```toml
[permissions.shell]
execute = ["git", "npm", "python"]
deny_execute = ["rm", "sudo"]
allow_urls = ["https://*", "mailto:*"]
deny_urls = ["file://*"]
```

**Python API:**

```python
# Execute a command
result = app.shell.execute("git", ["status"], cwd="/path/to/repo")
# {"stdout": "...", "stderr": "", "code": 0}

# Open URL
app.shell.open("https://example.com")

# Open file with default handler
app.shell.open("/path/to/image.png")
```

**JavaScript API:**

```javascript
const output = await invoke("shell_execute", {
  command: "npm",
  args: ["list"]
})
console.log(output.stdout)
```

### Dialog API (`app.dialog`)

Native file and message dialogs.

```python
# File open dialog
path = app.dialog.open_file(
    title="Open File",
    filters=[
        {"name": "JSON Files", "extensions": ["json"]},
        {"name": "All Files", "extensions": ["*"]}
    ]
)

# File save dialog
path = app.dialog.save_file(
    title="Save As",
    default_path="export.json",
    filters=[{"name": "JSON", "extensions": ["json"]}]
)

# Directory selection
directory = app.dialog.open_directory(title="Select Folder")

# Message dialogs
result = app.dialog.message(
    title="Confirm",
    message="Continue?",
    buttons=["Yes", "No"],
    destructive=1  # "No" is destructive
)
print(result.get("selected"))  # 0 for "Yes", 1 for "No"
```

### Notifications API (`app.notifications`)

Desktop notifications with actions.

```python
# Simple notification
app.notifications.show(
    title="Download Complete",
    body="File saved to Downloads"
)

# With ID (for updates)
app.notifications.show(
    id="progress-1",
    title="Uploading Files",
    body="2 of 5 files uploaded",
    icon="download.png"
)

# Close a notification
app.notifications.close("progress-1")
```

### Clipboard API (`app.clipboard`)

Read and write to system clipboard.

```python
# Read
text = app.clipboard.read_text()

# Write
app.clipboard.write_text("Hello, World!")

# Clear
app.clipboard.clear()
```

### System API (`app.system`)

System information and platform details.

```python
# System info
info = app.system.info()
# {
#   "os": "macos|windows|linux",
#   "arch": "x86_64|aarch64",
#   "cpu_count": 8,
#   "memory_total": 16000000000,
#   "memory_available": 8000000000,
#   "hostname": "machine.local"
# }

# App info
app_info = app.system.app_info()
# {"name": "MyApp", "version": "1.0.0"}

# Environment variables
value = app.system.env("HOME")
all_env = app.system.env_all()
```

### Notifications with Sound

```python
app.notifications.show(
    title="Alert",
    body="Important notification",
    sound="default"
)
```

### Screen API (`app.screen`)

Monitor and cursor information.

```python
# Get all monitors
monitors = app.screen.monitors()
for monitor in monitors:
    print(f"Monitor: {monitor['name']} ({monitor['width']}x{monitor['height']})")

# Get primary monitor
primary = app.screen.primary_monitor()

# Get cursor position
cursor = app.screen.cursor_position()
print(f"Cursor at: ({cursor['x']}, {cursor['y']})")
```

### Shortcuts API (`app.shortcuts`)

Register global keyboard shortcuts.

```python
# Register shortcut
app.shortcuts.register("CommandOrControl+Shift+D")

# Listen for shortcut in Python
@app.events.on("shortcut")
def on_shortcut(payload):
    print(f"Shortcut pressed: {payload['accelerator']}")

# Or from JavaScript
on("shortcut", (payload) => {
  console.log("Shortcut:", payload.accelerator)
})

# Unregister
app.shortcuts.unregister("CommandOrControl+Shift+D")
app.shortcuts.unregister_all()
```

### Menu API (`app.menu`)

Native application menu.

```python
app.menu.set([
    {
        "label": "File",
        "submenu": [
            {"label": "New", "accelerator": "CommandOrControl+N", "id": "file_new"},
            {"label": "Open", "accelerator": "CommandOrControl+O", "id": "file_open"},
            {"type": "separator"},
            {"label": "Exit", "accelerator": "CommandOrControl+Q", "role": "quit"}
        ]
    },
    {
        "label": "Help",
        "submenu": [
            {"label": "About", "id": "help_about"}
        ]
    }
])

# Handle menu selections from JavaScript
on("menu:select", (payload) => {
  console.log("Menu item:", payload.id)
  if (payload.id === "file_new") {
    handleNewFile()
  }
})
```

### OS Integration API (`app.os_integration`)

Platform-specific integrations.

```python
# Set progress bar on taskbar (Windows, macOS)
app.os_integration.set_progress_bar(0.5)  # 50%
app.os_integration.set_progress_bar(-1)   # Indeterminate

# Request user attention
app.os_integration.request_user_attention(is_critical=False)

# Clear attention
app.os_integration.clear_user_attention()
```

### Tray API (`app.tray`)

System tray icon and menu.

```python
app.tray.set_icon("assets/icon.png", "MyApp")

app.tray.set_menu([
    {"label": "Show", "id": "show"},
    {"label": "Hide", "id": "hide"},
    {"type": "separator"},
    {"label": "Quit", "id": "quit"}
])

# Handle tray actions
@app.events.on("tray:select")
def on_tray_select(payload):
    action = payload.get("action")
    if action == "show":
        app.window.show()
    elif action == "quit":
        app.window.close()
```

### Updater API (`app.updater`)

Check for and apply application updates.

**Configuration:**

```toml
[permissions]
updater = true

[updater]
enabled = true
endpoint = "https://releases.example.com/manifest.json"
channel = "stable"
check_on_startup = true
```

**Python API:**

```python
# Check for updates
update_info = app.updater.check()
# {"should_update": true, "version": "2.0.0", "url": "..."}

if update_info["should_update"]:
    # Download and install
    result = app.updater.download_and_install()
    # User will be prompted to restart
```

### Keychain API (`app.keychain`)

Secure credential storage.

```python
# Store credential
app.keychain.set("github_token", "ghp_xxxx")

# Retrieve credential
token = app.keychain.get("github_token")

# Delete credential
app.keychain.delete("github_token")
```

### Power API (`app.power`)

Battery and power state information.

```python
# Get battery info
battery = app.power.battery_info()
# {
#   "charging": true,
#   "percentage": 0.85,
#   "time_remaining": 3600  # seconds
# }
```

### Printing API (`app.printing`)

Print documents.

```python
# Get available printers
printers = app.printing.printers()

# Print file
app.printing.print(
    file_path="document.pdf",
    printer="Default Printer"
)
```

### Autostart API (`app.autostart`)

Configure application autostart on system boot.

```python
# Enable autostart
app.autostart.enable()

# Disable autostart
app.autostart.disable()

# Check if enabled
is_enabled = app.autostart.is_enabled()
```

### Window State API (`app.window_state`)

Persist window state across restarts.

```python
# Automatically persisted if enabled in forge.toml
# [window]
# remember_state = true

# Manual state management
state = app.window_state.get_state("main")
# {"width": 1200, "height": 800, "x": 100, "y": 100, "maximized": false}

# Save state
app.window_state.save_state("main", {
    "width": 1024,
    "height": 768,
    "x": 0,
    "y": 0,
    "maximized": False
})
```

### Deep Link API (`app.deep_links`)

Handle custom protocol URLs (e.g., `myapp://action`).

**Configuration:**

```toml
[protocol]
schemes = ["myapp"]
```

**Python API:**

```python
# Listen for deep links
@app.events.on("deep_link")
def on_deep_link(url: str):
    print(f"Deep link: {url}")
    # Parse and handle myapp://settings?theme=dark
```

**From command line:**

```bash
# macOS/Linux
open myapp://profile/123

# Windows
start myapp://profile/123
```

### Runtime API (`app.runtime`)

Application runtime introspection and diagnostics.

```python
# Get runtime health
health = app.runtime.health()
# {"status": "ok", "memory": 45.2, "cpu": 12.5}

# Get diagnostics
diags = app.runtime.diagnostics()
# Comprehensive runtime information

# Get command registry
commands = app.runtime.commands()

# Get protocol info
protocol_info = app.runtime.protocol()

# Last crash info
crash = app.runtime.last_crash()

# Runtime logs
logs = app.runtime.logs(limit=100)

# Current state
state = app.runtime.state()

# Navigation and reload
app.runtime.navigate("https://example.com")
app.runtime.reload()
app.runtime.go_back()
app.runtime.go_forward()

# Developer tools
app.runtime.open_devtools()
app.runtime.close_devtools()
app.runtime.toggle_devtools()

# Export support bundle for debugging
bundle_path = app.runtime.export_support_bundle()
```

---

## Configuration Guide

### `forge.toml` Structure

The `forge.toml` file defines all application configuration:

```toml
# Application Metadata
[app]
name = "My Desktop App"
version = "1.0.0"
description = "A powerful desktop application"
authors = ["Your Name <you@example.com>"]
main_html = "src/frontend/index.html"

# Window Configuration
[window]
title = "My Desktop App"
width = 1200
height = 800
resizable = true
fullscreen = false
min_width = 400
min_height = 300
decorations = true
always_on_top = false
transparent = false
# vibrancy = "mica"  # macOS/Windows native blur
remember_state = true  # Persist window size/position

# Build Configuration
[build]
entry = "src/main.py"
icon = "assets/icon.png"
output_dir = "dist"
single_binary = true

# Development Configuration
[dev]
frontend_dir = "src/frontend"
hot_reload = true
port = 5173
dev_server_cwd = "."
dev_server_timeout = 20

# API Permissions
[permissions]
filesystem = true  # or use detailed config below
clipboard = true
dialogs = true
notifications = true
screen = true
lifecycle = true
deep_link = true
os_integration = true
autostart = true
power = true
printing = true
window_state = true
drag_drop = true
menu = true

# Detailed Filesystem Permissions
[permissions.filesystem]
read = ["$APPDATA/MyApp/**", "~/Documents/**", "./data/**"]
write = ["$APPDATA/MyApp/**"]
deny = [".git", "node_modules", "*.tmp"]

# Shell Permissions
[permissions.shell]
execute = ["git", "npm", "python"]
deny_execute = ["rm", "sudo"]
allow_urls = ["https://*", "mailto:*"]
deny_urls = ["file://*"]

# Deep Link Schemes
[protocol]
schemes = ["myapp", "myapp-dev"]

# Security Configuration
[security]
allowed_commands = []  # Empty = allow all (if not in strict_mode)
denied_commands = []
expose_command_introspection = true
allowed_origins = ["https://example.com"]
strict_mode = false
rate_limit = 1000  # calls per second per connection, 0 = unlimited

# Window-scoped API access
[security.window_scopes]
main = ["filesystem", "shell"]      # Main window can access FS and Shell
settings = ["notification", "menu"] # Settings window limited access

# Update Configuration
[permissions]
updater = true

[updater]
enabled = true
endpoint = "https://releases.example.com/manifest.json"
channel = "stable"
check_on_startup = false
allow_downgrade = false
require_signature = true
staging_dir = ".forge-updater"

# Server Configuration (for `forge serve`)
[server]
host = "127.0.0.1"
port = 8000
workers = 4
cors_origins = ["*"]
auto_reload = false
# ssl_cert = "cert.pem"
# ssl_key = "key.pem"
log_level = "info"

# Database Configuration
[database]
url = "sqlite:///app.db"
pool_size = 5
echo = false

# Routes Configuration
[routes]
module = "src.routes"
api_prefix = "/api"

# Packaging Configuration
[packaging]
app_id = "com.example.myapp"
product_name = "My App"
formats = ["deb", "dmg", "nsis"]
category = "Utility"
```

### Environment Variables

Override configuration with environment variables:

```bash
# Dev server
export FORGE_DEV_SERVER_URL=http://127.0.0.1:5173

# Inspection mode (logs all IPC messages)
export FORGE_INSPECT=1

# Debug logging
export RUST_LOG=debug
```

---

## Security & Best Practices

### Security Architecture

Forge implements **defense in depth** with multiple security layers:

1. **Origin Validation** - Only `forge://` scheme and configured origins can access IPC
2. **Command Allowlisting** - Optionally restrict which commands are available
3. **Capability-Based Permissions** - APIs require explicit `forge.toml` permission
4. **Argument Validation** - Type checking via `msgspec` prevents injection attacks
5. **Path Traversal Prevention** - Filesystem API prevents `../` directory traversal
6. **Rate Limiting** - Configurable call-per-second limits prevent DoS
7. **Error Sanitization** - Errors sent to frontend have paths redacted

### Permission Model

Enable only required capabilities in `forge.toml`:

```toml
[permissions]
# Disable dangerous APIs
filesystem = false
shell = false
notifications = false  # Enable only if needed
```

### Path Security (Filesystem)

Use glob patterns with variable expansion:

```toml
[permissions.filesystem]
read = [
    "$APPDATA/MyApp/**",    # Platform-specific app data
    "~/Documents/**",        # User's documents
    "./public/**"            # Relative to project root
]
write = ["$APPDATA/MyApp/**"]
deny = [".git", "node_modules", "**/.env", "**/*.key"]
```

### Shell Command Security

Whitelist allowed commands:

```toml
[permissions.shell]
execute = ["git", "npm", "python"]
deny_execute = ["rm", "sudo", "rmdir"]

allow_urls = ["https://*", "mailto:*"]
deny_urls = ["file://*", "javascript:*"]
```

### IPC Security

Enable strict mode for maximum security:

```toml
[security]
strict_mode = true
# Requires explicit allowed_commands list
allowed_commands = ["get_data", "set_data"]
```

### Best Practices

✅ **DO:**
- Validate all user input on both backend and frontend
- Use HTTPS for external API calls
- Keep dependencies up to date
- Enable filesystem sandboxing with precise paths
- Use typed state containers for shared data
- Handle errors gracefully without exposing internals

❌ **DON'T:**
- Run untrusted code from the internet
- Use `eval()` or `exec()` with user input
- Hardcode credentials (use `keychain` API)
- Disable `strict_mode` unless necessary
- Expose sensitive paths in error messages
- Trust frontend validation alone

### CORS and Origin Validation

```python
# In forge.toml
[security]
allowed_origins = [
    "forge://app",                    # Built-in - always trusted
    "https://example.com",
    "http://127.0.0.1:5173"          # Dev server
]

# Check at runtime
if app.is_origin_allowed(origin):
    # Safe to process
    pass
```

---

## Advanced Topics

### Multi-threaded Applications

Python 3.14+ free-threaded benefits naturally translate to faster Forge apps:

```python
import threading
import time
from concurrent.futures import ThreadPoolExecutor

# Background data polling
def poll_updates():
    while True:
        data = fetch_data()
        app.emit("data:update", data)
        time.sleep(5)

thread = threading.Thread(target=poll_updates, daemon=True)
thread.start()

# CPU-intensive work with ThreadPoolExecutor
executor = ThreadPoolExecutor(max_workers=4)

@app.command
def compute_heavy(n: int) -> int:
    """Offload CPU work to thread pool."""
    future = executor.submit(expensive_calculation, n)
    return future.result(timeout=30)

def expensive_calculation(n: int) -> int:
    total = 0
    for i in range(n):
        total += i ** 2
    return total
```

### State Management with Type Injection

Use typed state for elegant component-based architecture:

```python
from dataclasses import dataclass
from forge.state import AppState

@dataclass
class Database:
    connection_string: str
    
    def query(self, sql: str):
        # Database query logic
        pass

@dataclass
class Logger:
    level: str = "info"
    
    def log(self, message: str):
        print(f"[{self.level.upper()}] {message}")

# Register instances
db = Database(connection_string="...")
logger = Logger()

app.state.manage(db)
app.state.manage(logger)

# Commands receive typed injection
@app.command
def fetch_data(db: Database) -> dict:
    """Database is auto-injected."""
    return db.query("SELECT * FROM users")

@app.command
async def background_task(logger: Logger) -> None:
    """Logger is auto-injected and available in async context."""
    logger.log("Starting task...")
    await asyncio.sleep(1)
    logger.log("Task complete")
```

### Plugins

Extend functionality via plugins:

```toml
[plugins]
enabled = true
modules = ["forge.plugins.analytics", "my_plugins.custom"]
```

```python
# my_plugins/custom.py
class MyPlugin:
    def __init__(self, app):
        self.app = app
        self.setup()
    
    def setup(self):
        # Register commands
        @self.app.command
        def plugin_command() -> str:
            return "From plugin"
        
        # Register event listeners
        @self.app.events.on("app:start")
        def on_app_start(payload):
            print("Plugin loaded!")

# Plugin will be loaded automatically
```

### Custom Configuration

Extend `forge.toml` with custom sections:

```python
import tomllib
from pathlib import Path

config_path = Path("forge.toml")
with open(config_path, "rb") as f:
    full_config = tomllib.load(f)

# Access custom sections
custom_section = full_config.get("myapp_config", {})
api_keys = custom_section.get("api_keys", {})
```

### Error Recovery and Crash Reporting

Forge includes automatic crash reporting:

```python
# Automatic crash capture
# Errors are logged to .forge-data/crashes/

# Manual crash recording
from forge.recovery import CrashReporter

crash = app._record_crash(
    exc_type=ValueError,
    exc_value=ValueError("Bad value"),
    exc_traceback=None,
    thread_name="worker",
    fatal=False
)

# Retrieve last crash
last_crash = app.runtime.last_crash()
print(f"Last crash: {last_crash['message']}")
```

### Circuit Breaker Pattern

Protect against cascading failures:

```python
from forge.recovery import CircuitBreaker

# Circuit breaker is built into the IPC bridge
# Tracks command failures and temporarily disables failing commands

# Check status
if app.bridge._circuit_breaker.is_allowed("risky_command"):
    # Safe to call
    pass
```

### Hot Reload

Both frontend and backend support hot reload during development:

```bash
# Automatically detects changes
python -m forge dev

# Frontend changes (via Vite)
# → Instant browser refresh

# Backend changes (`.py` files)
# → Background thread restarted via `os.execv`
```

### Logging

Configure comprehensive logging:

```python
import logging

# Set log level
logging.basicConfig(level=logging.DEBUG)

logger = logging.getLogger(__name__)

@app.command
def something() -> str:
    logger.info("Processing request")
    return "done"
```

Logs are:
- Printed to console during development
- Captured in runtime logs (accessible via `app.runtime.logs()`)
- Stored in `.forge-data/logs/` directory

### Window State Persistence

Automatically save and restore window state:

```toml
# Enable in forge.toml
[window]
remember_state = true
```

States are saved to `.forge-data/window-state.json` and restored on app startup.

### Support Bundle Export

Generate diagnostic bundle for debugging:

```python
# Export from Python
bundle_path = app.runtime.export_support_bundle("/tmp/forge-diagnostic.zip")

# Or from JavaScript
const path = await invoke("__forge_runtime_export_support_bundle")
```

Bundle includes:
- Logs
- System information
- Configuration (sanitized)
- Command registry
- Runtime state

---

## Code Examples

### Example 1: Todo Application

**Backend (`src/main.py`):**

```python
from forge import ForgeApp
from pathlib import Path
import json
from uuid import uuid4
from datetime import datetime

app = ForgeApp()
STORE_PATH = Path(".forge-data/todos.json")

class TodoStore:
    def __init__(self):
        STORE_PATH.parent.mkdir(exist_ok=True)
        if not STORE_PATH.exists():
            STORE_PATH.write_text(json.dumps([]))
    
    def get_all(self):
        return json.loads(STORE_PATH.read_text())
    
    def add(self, title: str) -> dict:
        todos = self.get_all()
        todo = {
            "id": str(uuid4()),
            "title": title,
            "completed": False,
            "created_at": datetime.now().isoformat()
        }
        todos.append(todo)
        STORE_PATH.write_text(json.dumps(todos))
        app.emit("todos:updated", todos)
        return todo
    
    def toggle(self, todo_id: str) -> dict:
        todos = self.get_all()
        for todo in todos:
            if todo["id"] == todo_id:
                todo["completed"] = not todo["completed"]
                break
        STORE_PATH.write_text(json.dumps(todos))
        app.emit("todos:updated", todos)
        return todo

store = TodoStore()

@app.command
def list_todos() -> list:
    return store.get_all()

@app.command
def add_todo(title: str) -> dict:
    return store.add(title)

@app.command
def toggle_todo(todo_id: str) -> dict:
    return store.toggle(todo_id)

if __name__ == "__main__":
    app.run()
```

**Frontend (`src/App.tsx`):**

```jsx
import React, { useState, useEffect } from 'react'
import { invoke, on } from './forge-api.js'

export default function TodoApp() {
  const [todos, setTodos] = useState([])
  const [input, setInput] = useState('')

  useEffect(() => {
    // Load initial todos
    loadTodos()
    
    // Listen for updates from backend
    const unsubscribe = on('todos:updated', (todos) => {
      setTodos(todos)
    })
    
    return () => unsubscribe?.()
  }, [])

  const loadTodos = async () => {
    const todos = await invoke('list_todos')
    setTodos(todos)
  }

  const addTodo = async () => {
    if (!input.trim()) return
    await invoke('add_todo', { title: input })
    setInput('')
  }

  const toggleTodo = async (id) => {
    await invoke('toggle_todo', { todo_id: id })
  }

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-4">My Todos</h1>
      
      <div className="flex gap-2 mb-6">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && addTodo()}
          placeholder="Add a todo..."
          className="flex-1 px-4 py-2 border rounded"
        />
        <button
          onClick={addTodo}
          className="px-6 py-2 bg-blue-500 text-white rounded"
        >
          Add
        </button>
      </div>

      <ul className="space-y-2">
        {todos.map((todo) => (
          <li
            key={todo.id}
            onClick={() => toggleTodo(todo.id)}
            className={`p-3 cursor-pointer rounded ${
              todo.completed ? 'bg-gray-200 line-through' : 'bg-white border'
            }`}
          >
            {todo.title}
          </li>
        ))}
      </ul>
    </div>
  )
}
```

### Example 2: Real-Time Data Dashboard

**Backend:**

```python
import threading
import time
import random

app = ForgeApp()

def emit_metrics():
    """Background thread emitting real-time metrics."""
    while True:
        cpu_usage = random.uniform(10, 80)
        memory_usage = random.uniform(30, 70)
        
        app.emit("metrics:update", {
            "cpu": cpu_usage,
            "memory": memory_usage,
            "timestamp": time.time()
        })
        
        time.sleep(1)

# Start background thread
threading.Thread(target=emit_metrics, daemon=True).start()

@app.command
def get_system_info() -> dict:
    return app.system.info()

if __name__ == "__main__":
    app.run()
```

**Frontend:**

```jsx
import { useState, useEffect } from 'react'
import { on, invoke } from './forge-api.js'
import { LineChart, Line, XAxis, YAxis, CartesianGrid } from 'recharts'

export default function Dashboard() {
  const [metrics, setMetrics] = useState([])
  const [systemInfo, setSystemInfo] = useState(null)

  useEffect(() => {
    loadSystemInfo()
    
    const unsubscribe = on('metrics:update', (data) => {
      setMetrics(prev => [...prev.slice(-59), data])
    })
    
    return () => unsubscribe?.()
  }, [])

  const loadSystemInfo = async () => {
    const info = await invoke('get_system_info')
    setSystemInfo(info)
  }

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">System Monitor</h1>
      
      {systemInfo && (
        <div className="grid grid-cols-2 gap-4 mb-6">
          <div className="bg-blue-50 p-4 rounded">
            <p className="text-sm text-gray-600">OS</p>
            <p className="text-lg font-bold">{systemInfo.os}</p>
          </div>
          <div className="bg-green-50 p-4 rounded">
            <p className="text-sm text-gray-600">CPU Cores</p>
            <p className="text-lg font-bold">{systemInfo.cpu_count}</p>
          </div>
        </div>
      )}

      <LineChart width={600} height={300} data={metrics}>
        <CartesianGrid />
        <XAxis dataKey="timestamp" />
        <YAxis domain={[0, 100]} />
        <Line type="monotone" dataKey="cpu" stroke="#8884d8" />
        <Line type="monotone" dataKey="memory" stroke="#82ca9d" />
      </LineChart>
    </div>
  )
}
```

### Example 3: File Manager with Permissions

**Backend:**

```python
from forge import ForgeApp
from pathlib import Path

app = ForgeApp()

@app.command
def list_files(directory: str = ".") -> list:
    try:
        path = app.fs.resolve_path(directory, mode="read")
        items = []
        for item in path.iterdir():
            items.append({
                "name": item.name,
                "is_dir": item.is_dir(),
                "size": item.stat().st_size if item.is_file() else 0,
                "modified": item.stat().st_mtime
            })
        return items
    except PermissionError:
        return {"error": "Permission denied"}

@app.command
def read_file(path: str) -> str:
    return app.fs.read_text(path)

@app.command
def write_file(path: str, content: str) -> bool:
    app.fs.write_text(path, content)
    return True

@app.command
def delete_file(path: str) -> bool:
    app.fs.remove_file(path)
    return True

if __name__ == "__main__":
    app.run()
```

**Configuration:**

```toml
[permissions.filesystem]
read = ["$APPDATA/MyApp/**", "~/Documents/**"]
write = ["$APPDATA/MyApp/**"]
deny = [".git", "node_modules", "*.key"]
```

---

## Troubleshooting

### Common Issues

#### "Module not found: forge_core"

**Cause:** PyO3 native extension not compiled.

**Solution:**
```bash
pip install maturin
maturin develop  # Rebuild from Rust source
```

#### IPC Commands Not Appearing

**Cause:** Commands decorators applied before `ForgeApp` instantiation.

**Solution:**
```python
# ❌ Wrong
@command
def my_func(): pass

app = ForgeApp()

# ✅ Correct
app = ForgeApp()

@app.command
def my_func(): pass
```

#### "Origin not allowed"

**Cause:** Frontend origin not in `allowed_origins`.

**Solution:**

```toml
[security]
allowed_origins = [
    "forge://app",              # Always included
    "http://127.0.0.1:5173",   # Your dev server
    "https://example.com"       # Production domain
]
```

#### Permission Denied on Filesystem

**Cause:** Path not in `[permissions.filesystem]` read/write lists.

**Solution:**

```toml
[permissions.filesystem]
read = ["$APPDATA/MyApp/**", "~/Documents/**"]  # Add your paths
write = ["$APPDATA/MyApp/**"]
```

#### Window Doesn't Appear

**Cause:** Frontend path misconfigured or index.html missing.

**Solution:**

```python
# Check configuration
print(app.config.dev.frontend_dir)  # Should exist
frontend = app.config.get_base_dir() / app.config.dev.frontend_dir
print(frontend.exists())  # Should be True
print((frontend / "index.html").exists())  # Should be True
```

#### Hot Reload Not Working

**Cause:** Dev server not running or misconfigured.

**Solution:**

```bash
# Run with debug flag
python -m forge dev --debug

# Check environment
echo $FORGE_DEV_SERVER_URL

# Manually start Vite
npm run dev
```

#### "Circuit breaker open" Error

**Cause:** Command failed repeatedly, temporarily disabled.

**Solution:**

```python
# Check circuit breaker status
breaker = app.bridge._circuit_breaker
is_allowed = breaker.is_allowed("command_name")

# Reset manually (not recommended - fix the underlying issue)
breaker._failures.pop("command_name", None)
```

#### State Injection Not Working

**Cause:** State container not properly initialized.

**Solution:**

```python
# Explicitly create and register state
from forge.state import AppState

app.state.manage(MyDataClass())

# Verify type checking
@app.command
def handler(data: MyDataClass) -> str:
    # Type must match managed instance
    return data.field
```

### Debug Mode

Enable comprehensive debugging:

```bash
export FORGE_INSPECT=1       # Log all IPC messages
export RUST_LOG=debug        # Verbose Rust logs
export RUST_BACKTRACE=1      # Full error stacks

python -m forge dev --debug
```

This will print:
- All IPC requests/responses
- Command timings
- Security denials
- State transitions

### Support Bundle

Export diagnostic information:

```python
bundle_path = app.runtime.export_support_bundle()
print(f"Bundle exported to: {bundle_path}")
```

The bundle contains:
- System information
- Application logs
- Configuration (sanitized)
- Last crash report
- Command registry

### Logging Best Practices

```python
import logging

# Configure before ForgeApp
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('.forge-data/app.log'),
        logging.StreamHandler()
    ]
)

logger = logging.getLogger(__name__)

@app.command
def operation():
    logger.info("Starting operation")
    try:
        result = do_work()
        logger.info(f"Operation succeeded: {result}")
        return result
    except Exception as e:
        logger.error(f"Operation failed: {e}", exc_info=True)
        raise
```

### Performance Profiling

```python
import cProfile
import pstats

@app.command
def slow_operation():
    profiler = cProfile.Profile()
    profiler.enable()
    
    result = expensive_calculation()
    
    profiler.disable()
    stats = pstats.Stats(profiler)
    stats.sort_stats('cumulative')
    stats.print_stats(10)
    
    return result
```

### Memory Leaks

Monitor for memory leaks:

```python
import tracemalloc

tracemalloc.start()

@app.command
def check_memory() -> dict:
    snapshot = tracemalloc.take_snapshot()
    top_stats = snapshot.statistics('lineno')
    
    return {
        "top_allocations": [
            f"{stat.size / 1024:.1f} KB - {stat}"
            for stat in top_stats[:5]
        ]
    }
```

---

## Advanced Architecture Patterns

### MVC Pattern with Forge

```python
# models.py
from dataclasses import dataclass

@dataclass
class UserModel:
    id: str
    name: str
    email: str

# controllers.py
class UserController:
    def __init__(self, app):
        self.app = app
    
    @app.command
    def get_user(self, user_id: str) -> dict:
        # Fetch from database
        user = self._fetch_from_db(user_id)
        return {"id": user.id, "name": user.name, "email": user.email}

# views (in React - see Examples section)
```

### Repository Pattern

```python
class UserRepository:
    def __init__(self, db):
        self.db = db
    
    def find_by_id(self, user_id: str):
        return self.db.query("SELECT * FROM users WHERE id = ?", (user_id,))
    
    def save(self, user):
        # Insert or update
        pass

app.state.manage(UserRepository(db_connection))

@app.command
def get_user(repo: UserRepository, user_id: str):
    return repo.find_by_id(user_id)
```

### Pub/Sub System

```python
class EventBus:
    def __init__(self):
        self.subscribers = {}
    
    def subscribe(self, event_name, handler):
        if event_name not in self.subscribers:
            self.subscribers[event_name] = []
        self.subscribers[event_name].append(handler)
    
    def publish(self, event_name, data):
        for handler in self.subscribers.get(event_name, []):
            handler(data)

event_bus = EventBus()
app.state.manage(event_bus)

# Use in handlers
@app.command
def do_action(bus: EventBus):
    bus.publish("action:completed", {"status": "success"})
```

---

## Deployment

### Building for Production

```bash
# Build desktop bundles
cargo tauri build

# Build installer
npm run build:windows  # or build:macos, build:linux
```

Output in `src-tauri/target/release/bundle/`:
- `deb/` - Linux Debian packages
- `dmg/` - macOS disk images  
- `nsis/` - Windows installers

### Code Signing

Configure in `forge.toml`:

```toml
[signing]
enabled = true
identity = "Developer ID Application: Your Name (XXXXXXXXXX)"
notarize = true
```

### Updater Configuration

```toml
[updater]
enabled = true
endpoint = "https://releases.example.com/latest.json"
channel = "stable"
```

Manifest format:

```json
{
  "version": "2.0.0",
  "url": "https://releases.example.com/app-2.0.0-x64.installer.exe",
  "signature": "signature-base64-encoded"
}
```

---

## Performance Optimization

### Command Optimization

```python
# ❌ Slow: IPC on every keystroke
@app.on("input:change")
def on_input_change(text):
    result = expensive_search(text)
    emit("results", result)

# ✅ Fast: Debounce on frontend, batch requests
# Frontend handles debouncing, then calls command once
```

### Batch Operations

```python
@app.command
def batch_update(items: list[dict]) -> list[dict]:
    """Process multiple items in one call."""
    results = []
    for item in items:
        results.append(process_item(item))
    return results
```

### Caching

```python
from functools import lru_cache

@lru_cache(maxsize=128)
@app.command
def get_config(key: str) -> str:
    # Only computed once per unique key
    return expensive_config_lookup(key)
```

### Async Operations

```python
@app.command
async def long_operation(data: str) -> str:
    """Non-blocking async command."""
    result1 = await fetch_data1()
    result2 = await fetch_data2()
    return process(result1, result2)
```

---

## Conclusion

Forge Framework enables you to build **modern, performant desktop applications** with the full power of Python and web technologies. By understanding the core concepts—commands, events, APIs, and security—you can create production-grade applications that rival native clients.

For more information and examples, visit:
- **GitHub:** https://github.com/your-org/forge
- **Documentation:** https://forge.dev/docs
- **Community:** https://community.forge.dev
- **Examples:** https://github.com/your-org/forge/tree/main/examples

Happy building! 🚀
