---
title: "Complete API Reference"
description: "Full reference for Forge Framework APIs, built-in commands, and advanced patterns"
---

## ForgeApp API

### Constructor

```python
from forge import ForgeApp

app = ForgeApp(config_path: Optional[str] = None)
```

**Parameters:**
- `config_path` - Optional path to `forge.toml`. If None, searches up from cwd.

### Command Registration

```python
@app.command(name: Optional[str] = None)
def my_command(param1: str, param2: int) -> dict:
    """Register a command endpoint"""
    return {"result": "value"}

# Custom command name
@app.command("custom_name")
def my_function():
    return "success"
```

### Event System

```python
# Emit event to frontend
app.emit(event_name: str, payload: Any = None) -> None

# Bind to event from command
@app.on_ready()
def on_app_ready():
    """Run when app window is ready"""
    pass

@app.on_close()
def on_app_closing():
    """Run before app shuts down"""
    pass
```

### State Management

```python
# Get state value
value = app.state.get(key: str, default: Any = None) -> Any

# Set state value
app.state.set(key: str, value: Any) -> None

# Check existence
exists = app.state.has(key: str) -> bool

# Delete state
app.state.delete(key: str) -> None

# Get all state
all_state = app.state.to_dict() -> dict
```

### Window API

```python
# Get main window
window = app.window

# Set window title
window.set_title(title: str)

# Get window size
size = window.size()  # Returns (width, height)

# Set window size
window.set_size(width: float, height: float)

# Get window position
pos = window.position()  # Returns (x, y)

# Set window position
window.set_position(x: float, y: float)

# Window visibility
window.show()
window.hide()
window.focus()

# Window state
window.minimize()
window.maximize()
window.is_visible() -> bool
window.is_focused() -> bool
window.is_minimized() -> bool
window.is_maximized() -> bool

# Window fullscreen
window.set_fullscreen(enabled: bool)

# Always on top
window.set_always_on_top(enabled: bool)

# Close window
window.close()
```

### Multi-Window API

```python
# Create new window
new_window = app.windows.create(
    title: str,
    url: str,
    width: int = 800,
    height: int = 600,
    label: Optional[str] = None
) -> WindowProxy

# List all windows
windows = app.windows.list() -> List[WindowInfo]

# Get specific window
window = app.windows.get(label: str) -> WindowProxy

# Close window
app.windows.close(label: str)

# Set window properties
app.windows.set_title(label: str, title: str)
app.windows.set_size(label: str, width: float, height: float)
app.windows.set_position(label: str, x: float, y: float)
app.windows.focus(label: str)
app.windows.minimize(label: str)
app.windows.maximize(label: str)
```

### Built-in APIs

#### File System

```python
# Requires: filesystem permission
app.fs  # FileSystemAPI

# Read file
content = await app.fs.read_text(path: str) -> str
binary = await app.fs.read_binary(path: str) -> bytes

# Write file
await app.fs.write_text(path: str, content: str)
await app.fs.write_binary(path: str, data: bytes)

# Directory operations
await app.fs.create_dir(path: str)
entries = await app.fs.list_dir(path: str) -> List[DirEntry]
await app.fs.remove(path: str)
await app.fs.copy(src: str, dst: str)
await app.fs.move(src: str, dst: str)
```

#### Dialogs

```python
# Requires: dialogs permission
app.dialog  # DialogAPI

# Open file dialog
path = await app.dialog.open_file(
    title: str = "Open File",
    filters: Optional[List[FileFilter]] = None
) -> Optional[str]

# Open files (multiple)
paths = await app.dialog.open_files() -> List[str]

# Save file dialog
path = await app.dialog.save_file(
    title: str = "Save File",
    default_name: Optional[str] = None
) -> Optional[str]

# Open directory
directory = await app.dialog.open_directory() -> Optional[str]
```

#### Notifications

```python
# Requires: notifications permission
app.notifications  # NotificationAPI

# Send notification
await app.notifications.show(
    title: str,
    body: Optional[str] = None,
    icon: Optional[str] = None
)
```

#### Clipboard

```python
# Requires: clipboard permission
app.clipboard  # ClipboardAPI

# Read clipboard
text = await app.clipboard.read_text() -> str

# Write clipboard
await app.clipboard.write_text(text: str)
```

#### System Info

```python
# Always available
info = app.system.info() -> dict
# Returns: {
#   "os": "linux|darwin|windows",
#   "arch": "x86_64|aarch64",
#   "app_name": "My App",
#   "app_version": "1.0.0"
# }

# Exit app
app.system.exit(code: int = 0)
```

#### Shell Commands

```python
# Requires: shell permission
result = await app.shell.execute(
    command: str,
    cwd: Optional[str] = None,
    env: Optional[dict] = None
) -> ShellResult
# Returns: {
#   "code": 0,
#   "stdout": "...",
#   "stderr": "..."
# }

# Spawn process
process = await app.shell.spawn(command: str) -> Process
await process.wait()
```

#### Keyboard Shortcuts

```python
# Requires: system permission (implicit)
# Register global shortcut
registered = await app.shortcuts.register(
    accelerator: str  # e.g., "CommandOrControl+Shift+P"
) -> bool

# Unregister shortcut
await app.shortcuts.unregister(accelerator: str)

# Listen for shortcut
app.on("shortcut", lambda shortcut: print(f"Pressed: {shortcut}"))
```

#### System Tray

```python
# Requires: system_tray permission
app.tray  # TrayAPI

# Set icon
await app.tray.set_icon(icon_path: str)

# Add menu items
await app.tray.set_menu([
    {
        "id": "show",
        "label": "Show",
        "accelerator": None
    },
    {
        "id": "quit",
        "label": "Quit",
        "accelerator": None
    }
])

# Listen for tray menu clicks
app.on("tray:select", lambda action: print(f"Tray: {action}"))
```

#### Menu Bar

```python
# Create menu
await app.menu.set_menu([
    {
        "id": "file",
        "label": "File",
        "submenu": [
            {"id": "new", "label": "New"},
            {"id": "open", "label": "Open"},
            {"type": "separator"},
            {"id": "quit", "label": "Quit", "accelerator": "CommandOrControl+Q"}
        ]
    },
    {
        "id": "edit",
        "label": "Edit",
        "submenu": [
            {"id": "undo", "label": "Undo"},
            {"id": "redo", "label": "Redo"}
        ]
    }
])

# Handle menu clicks
app.on("menu:select", lambda action: print(f"Menu: {action}"))
```

#### Hardware Serial & USB

```python
# Requires: serial permission
app.serial  # SerialAPI

# List available ports
ports = app.serial.available_ports()
# [{"port": "/dev/ttyUSB0", "description": "Arduino Uno", ...}]

# Open connection
app.serial.open("/dev/ttyUSB0", baudrate=9600)

# Write data
app.serial.write("/dev/ttyUSB0", b"HELLO\n")

# Listen for serial data
app.on("serial_data", lambda data: print(data["port"], data["data"]))

# Close connection
app.serial.close("/dev/ttyUSB0")
```

#### OS Integration & Power

```python
# OS Integration API
app.os_integration.set_progress_bar(progress=0.5)  # Taskbar/Dock progress
app.os_integration.bounce_dock()  # macOS Dock bounce
app.os_integration.flash_frame()  # Windows Taskbar flash

# Power API
battery_info = app.power.get_battery_info()
power_state = app.power.get_power_state()
app.on("suspend", lambda _: print("System is going to sleep"))
```

#### Screen & Displays

```python
# Contains multi-monitor details, DPI scales, sizes
displays = app.screen.get_displays()
primary = app.screen.get_primary_display()

# Get mouse OS coordinates
cursor_pos = app.screen.get_cursor_position()
```

#### Shared Memory Buffers (Zero-Copy)

```python
from forge.memory import buffers

# Store raw bytes in the memory registry (no JSON serialization)
buffers["huge_data.bin"] = b'\x00\x01\x02\x03\x04\x05' * 1000

# On the frontend, fetch using the custom protocol:
# const bytes = await fetch('forge-memory://huge_data.bin').then(r => r.arrayBuffer());
```

#### Keychain & Secure Storage

```python
# Requires: keychain permission
app.keychain  # KeychainAPI

# Set password
app.keychain.set_password("my_app", "user1", "secret_pass")

# Get password
password = app.keychain.get_password("my_app", "user1")

# Delete password
app.keychain.delete_password("my_app", "user1")
```

#### App Lifecycle

```python
app.lifecycle  # AppLifecycleAPI

# Request single instance lock
is_primary = app.lifecycle.request_single_instance_lock()
if not is_primary:
    app.system.exit(0)

# Relaunch app
app.lifecycle.relaunch()
```

#### Autostart

```python
# Requires: autostart permission
app.autostart  # AutostartAPI

# Enable autostart on login
app.autostart.enable()

# Disable autostart
app.autostart.disable()

# Check if enabled
is_enabled = app.autostart.is_enabled()
```

#### Printing

```javascript
// Via Javascript frontend SDK:
import { printing } from '@forgedesk/api';

// Trigger native OS Print dialog
await printing.printPage();
```

## Frontend API

### Invoke Commands

```javascript
import { invoke } from '@forgedesk/api'

// Call Python command
const result = await invoke('command_name', {
  param1: 'value',
  param2: 123
})

// Error handling
try {
  await invoke('risky_command')
} catch (error) {
  console.error('Command failed:', error)
}
```

### Event Listeners

```javascript
import { on } from '@forgedesk/api'

// Subscribe to event
const unsubscribe = on('event_name', (data) => {
  console.log('Event received:', data)
})

// Unsubscribe
unsubscribe()
```

### Runtime Info

```javascript
import { runtime } from '@forgedesk/api'

// Get app info
const info = runtime.appInfo() // { version, name, os, arch }

// Reload app
runtime.reload()

// Open DevTools
runtime.openDevTools()

// Get state
const state = await runtime.getState()
```

## Configuration Reference

### forge.toml Structure

```toml
[app]
name = "Application Name"          # Required
version = "1.0.0"                 # Semantic version
description = "App description"    # Optional
authors = ["Author Name"]         # Optional

[window]
title = "Window Title"             # Window title
width = 1280                      # Initial width in pixels
height = 720                      # Initial height in pixels
min_width = 400                   # Minimum width
min_height = 300                  # Minimum height
resizable = true                  # Allow window resize
fullscreen = false                # Start fullscreen
decorations = true                # Show window frame
transparent = false               # Transparent background
always_on_top = false             # Always on top
vibrancy = null                   # macOS vibrancy effect

[build]
entry = "src/main.py"             # Python entry point
output_dir = "dist"               # Build output directory
icon = "assets/icon.png"          # App icon
single_binary = true              # Single-file binary

[dev]
frontend_dir = "src/frontend"     # Frontend directory
port = 5199                       # Dev server port
hot_reload = true                 # Enable hot reload
dev_server_command = "npm run dev"
dev_server_url = "http://127.0.0.1:5199"

[permissions]
filesystem = true                 # File system access
clipboard = true                  # Clipboard access
dialogs = true                    # File dialogs
notifications = true              # Show notifications
shell = false                     # Execute shell commands
system_tray = false               # System tray icon
keychain = false                  # Secure storage

[security]
strict_mode = false               # Enforce origin validation
allowed_origins = ["forge://app"] # Allowed IPC origins
denied_commands = []              # Block specific commands

[updater]
enabled = false                   # Enable auto-updates
endpoints = []                    # Update server endpoints
```

## Type Hints Guide

### Supported Python Types

```python
# Basic types
@app.command
def basic(s: str, n: int, f: float, b: bool) -> str:
    return "ok"

# Collections
from typing import List, Dict, Set, Tuple

@app.command
def collections(
    items: List[str],
    mapping: Dict[str, int],
    numbers: Set[float],
    pair: Tuple[str, int]
) -> List[dict]:
    return [{"item": i} for i in items]

# Optional types
from typing import Optional

@app.command
def optional(
    required: str,
    optional_param: Optional[str] = None
) -> Optional[dict]:
    return None

# Union types
from typing import Union

@app.command
def union(value: Union[str, int]) -> Union[dict, list]:
    return {"value": value}

# Pydantic models for validation
from pydantic import BaseModel, Field

class UserCreate(BaseModel):
    username: str = Field(..., min_length=3)
    email: str
    age: int = Field(0, ge=0, le=150)

@app.command
def create_user(user: UserCreate) -> dict:
    return user.dict()
```

### Type Errors to Avoid

```python
# ❌ Not serializable to JSON
@app.command
def bad_return() -> os.PathLike:  # Can't serialize Path objects
    return Path.home()

# ✅ Convert to JSON-serializable
@app.command
def good_return() -> str:
    return str(Path.home())

# ❌ Circular imports
from app import app  # Will fail if defined in same file

# ✅ Register commands after app creation
app = ForgeApp()

@app.command
def my_cmd():
    pass
```

## Advanced Patterns

### Plugin Architecture

```python
# src/backend/plugins/base.py
class ForgePlugin:
    """Base class for plugins"""
    
    def __init__(self, app):
        self.app = app
    
    def register(self):
        """Register plugin commands"""
        raise NotImplementedError
    
    def cleanup(self):
        """Cleanup on shutdown"""
        pass

# src/backend/plugins/analytics.py
class AnalyticsPlugin(ForgePlugin):
    def register(self):
        @self.app.command("track_event")
        def track_event(event_name: str, properties: dict):
            self.log(event_name, properties)
            return True
    
    def log(self, event: str, props: dict):
        # Send to analytics service
        pass

# src/main.py
analytics = AnalyticsPlugin(app)
analytics.register()
```

### Middleware Pattern

```python
from functools import wraps

def require_auth(func):
    """Middleware to require authentication"""
    @wraps(func)
    def wrapper(*args, session_id: str = None, **kwargs):
        if not session_id:
            return {"error": "Unauthorized"}
        
        user_id = validate_session(session_id)
        if not user_id:
            return {"error": "Session invalid"}
        
        return func(*args, session_id=session_id, **kwargs)
    return wrapper

@app.command
@require_auth
def protected_command(data: str, session_id: str = None):
    return {"data": data}
```

### Observable Pattern

```python
class Observable:
    """Simple observable for state changes"""
    
    def __init__(self, value):
        self.value = value
        self._listeners = []
    
    def observe(self, callback):
        """Subscribe to changes"""
        self._listeners.append(callback)
        return lambda: self._listeners.remove(callback)
    
    def set(self, value):
        """Update value and notify"""
        if value != self.value:
            self.value = value
            for listener in self._listeners:
                listener(value)

# Usage
user_count = Observable(0)

def on_count_change(count):
    app.emit("user_count_changed", {"count": count})

user_count.observe(on_count_change)

@app.command("add_user")
def add_user(name: str):
    user_count.set(user_count.value + 1)
    return {"count": user_count.value}
```

### Background Tasks API

The Background Tasks API allows executing long-running Python functions asynchronously without blocking the UI thread.

```python
# Create a task target function
def process_data(task_id: str, payload: dict):
    # Perform heavy computation
    app.tasks.report_progress(task_id, 50, {"step": "transforming"})
    return {"status": "success"}

# Dispatch the task
task_id = app.tasks.submit(process_data, payload={"input": "data"})
```

**JavaScript API:**

```javascript
// Submit task
const taskId = await window.__forge__.tasks.submit('process_data', { input: "data" })

// Check status
const status = await window.__forge__.tasks.status(taskId)

// Listen for progress (dispatched over Forge event bus)
window.__forge__.on(`task:progress:${taskId}`, (e) => {
  console.log(`Task ${taskId} progress:`, e.progress)
})
```

### Event Channels API

Channels are dedicated streams for pushing continuous or chunks of data (e.g., file downloads, log streaming) without clogging the main IPC bridge.

```python
# Create an event channel in Python
channel_id = app.channels.create("download-progress")

# Send data through the channel
app.channels.send(channel_id, {"bytes_downloaded": 1024, "total": 4096})

# Close channel when done
app.channels.close(channel_id)
```

**JavaScript API:**

```javascript
// Listed to the channel generically
window.__forge__.channel.on("download-progress", (data) => {
    console.log("Chunk received:", data)
})

// Stop listening
window.__forge__.channel.off("download-progress")
```

### WebSocket Client API

The WebSocket Client API enables Forge applications to connect to external WebSocket servers (e.g., chat servers, data feeds) and securely bridge the network stream down into the frontend.

**Configuration:**

```toml
[permissions.network]
websocket = true
```

**JavaScript API:**

```javascript
// Connect to a WebSocket server natively
const connId = await window.__forge__.ws.connect("wss://echo.websocket.events", true)

// Listen to incoming messages using the global bridge
window.__forge__.on(`ws:message:${connId}`, (e) => {
  console.log("Message received:", e.data)
})

// Send data
await window.__forge__.ws.send(connId, "Hello World from Forge!")

// Close connection
await window.__forge__.ws.close(connId)
```
