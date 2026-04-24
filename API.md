# ForgeDesk API Reference

Complete API reference for ForgeDesk v3.0.0. All APIs are accessible from both Python backend and JavaScript frontend.

## Table of Contents

- [Core APIs](#core-apis)
  - [ForgeApp](#forgeapp)
  - [WindowAPI](#windowapi)
  - [WindowManagerAPI](#windowmanagerapi)
  - [IPCBridge](#ipcbridge)
  - [EventEmitter](#eventemitter)
  - [AppState](#appstate)
- [File System APIs](#file-system-apis)
  - [FileSystemAPI](#filesystemapi)
- [Dialog APIs](#dialog-apis)
  - [DialogAPI](#dialogapi)
- [System APIs](#system-apis)
  - [SystemAPI](#systemapi)
  - [ScreenAPI](#screenapi)
  - [PowerAPI](#powerapi)
- [UI APIs](#ui-apis)
  - [MenuAPI](#menuapi)
  - [TrayAPI](#trayapi)
  - [NotificationAPI](#notificationapi)
- [Security APIs](#security-apis)
  - [KeychainAPI](#keychainapi)
  - [ScopeValidator](#scopevalidator)
- [Network APIs](#network-apis)
  - [WebSocketAPI](#websocketapi)
  - [DeepLinkAPI](#deeplinkapi)
- [Utility APIs](#utility-apis)
  - [ClipboardAPI](#clipboardapi)
  - [ShortcutsAPI](#shortcutsapi)
  - [AutostartAPI](#autostartapi)
  - [UpdaterAPI](#updaterapi)
- [Built-in Plugins](#built-in-plugins)
  - [AI/ML Plugin](#aiml-plugin)
  - [Local LLM Plugin](#local-llm-plugin)
  - [Database Plugin](#database-plugin)
  - [Crypto Plugin](#crypto-plugin)

---

## Core APIs

### ForgeApp

The main application class that initializes and orchestrates all components.

```python
from forge import ForgeApp

app = ForgeApp()
```

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| `config` | `ForgeConfig` | Application configuration |
| `window` | `WindowAPI` | Main window control |
| `windows` | `WindowManagerAPI` | Multi-window management |
| `bridge` | `IPCBridge` | IPC communication |
| `events` | `EventEmitter` | Event system |
| `state` | `AppState` | Dependency injection |

#### Methods

| Method | Description |
|--------|-------------|
| `app.run()` | Start the application (blocks until closed) |
| `app.emit(event, data)` | Emit an event to all listeners |
| `app.on_close(callback)` | Register shutdown callback |

---

### WindowAPI

Control the main application window.

```python
# Window state
state = app.window.state()  # Dict with title, width, height, visible, etc.

# Window controls
app.window.set_title("My App")
app.window.set_size(1200, 800)
app.window.set_position(100, 200)
app.window.show()
app.window.hide()
app.window.focus()
app.window.minimize()
app.window.maximize()
app.window.set_fullscreen(True)
app.window.set_always_on_top(True)
app.window.set_vibrancy("mica")  # macOS/Windows blur effects
app.window.close()
```

#### State Properties

| Property | Type | Description |
|----------|------|-------------|
| `title` | `str` | Window title |
| `width` | `int` | Window width in pixels |
| `height` | `int` | Window height in pixels |
| `x` | `int\|None` | Window X position |
| `y` | `int\|None` | Window Y position |
| `visible` | `bool` | Whether window is visible |
| `focused` | `bool` | Whether window has focus |
| `minimized` | `bool` | Whether window is minimized |
| `maximized` | `bool` | Whether window is maximized |
| `fullscreen` | `bool` | Whether window is fullscreen |

---

### WindowManagerAPI

Manage multiple windows.

```python
# Create a new window
settings = app.windows.create(
    label="settings",
    title="Settings",
    route="/settings",
    width=600,
    height=400,
    parent="main"
)

# List all windows
window_list = app.windows.list()  # List of window descriptors

# Get specific window
window = app.windows.get("settings")

# Window operations
app.windows.set_title("settings", "New Title")
app.windows.set_size("settings", 800, 600)
app.windows.set_position("settings", 200, 300)
app.windows.focus("settings")
app.windows.minimize("settings")
app.windows.maximize("settings")
app.windows.close("settings")

# Broadcast to all windows
app.windows.broadcast("console.log('Hello from Python!')")
```

---

### IPCBridge

Communication between Python backend and JavaScript frontend.

```python
# Register a command
def greet(name: str) -> dict:
    return {"message": f"Hello, {name}!"}

app.bridge.register_command("greet", greet)

# Register with capability requirement
def read_file(path: str) -> dict:
    return {"content": "file data"}

app.bridge.register_command("read_file", read_file, capability="filesystem")

# Register async command
async def fetch_data(url: str) -> dict:
    return {"data": "..."}

app.bridge.register_command("fetch_data", fetch_data)
```

#### Frontend Usage (JavaScript)

```javascript
// Call Python command
const result = await window.__forge__.invoke("greet", { name: "World" });
console.log(result.message);  // "Hello, World!"

// Call with error handling
try {
    const data = await window.__forge__.invoke("read_file", { path: "/tmp/test.txt" });
} catch (error) {
    console.error(error.message);
}
```

---

### EventEmitter

Thread-safe event system.

```python
# Register listener
def on_ready(event):
    print("App is ready!", event)

app.events.on("ready", on_ready)

# Register with decorator
@app.events.on("data")
def on_data(event):
    print("Received data:", event)

# Emit event
app.events.emit("ready", {"status": "ok"})

# Remove listener
app.events.off("ready", on_ready)
```

---

### AppState

Type-safe dependency injection.

```python
from forge import AppState

class Database:
    def __init__(self, url: str):
        self.url = url

class Cache:
    def __init__(self, ttl: int = 300):
        self.ttl = ttl

# Register services
app.state.manage(Database("sqlite:///app.db"))
app.state.manage(Cache(ttl=60))

# Retrieve services
db = app.state.get(Database)
cache = app.state.get(Cache)

# Check if service exists
service = app.state.try_get(MissingService)  # Returns None
```

---

## File System APIs

### FileSystemAPI

Secure file system operations with scope validation.

```python
# Read file
content = app.fs.read_text("/path/to/file.txt")
data = app.fs.read_bytes("/path/to/file.bin")

# Write file
app.fs.write_text("/path/to/file.txt", "Hello, World!")
app.fs.write_bytes("/path/to/file.bin", binary_data)

# Directory operations
app.fs.create_dir("/path/to/dir")
files = app.fs.list_dir("/path/to/dir")
app.fs.remove("/path/to/file.txt")
app.fs.copy("/src.txt", "/dst.txt")
app.fs.move("/old.txt", "/new.txt")

# File info
exists = app.fs.exists("/path/to/file")
size = app.fs.size("/path/to/file")
is_file = app.fs.is_file("/path/to/file")
is_dir = app.fs.is_dir("/path/to/dir")
```

#### Frontend Usage

```javascript
// Read file
const content = await window.__forge__.fs.readText("/path/to/file.txt");

// Write file
await window.__forge__.fs.writeText("/path/to/file.txt", "Hello!");

// List directory
const files = await window.__forge__.fs.listDir("/path/to/dir");
```

---

## Dialog APIs

### DialogAPI

Native system dialogs.

```python
# Open file dialog
file_path = app.dialog.open_file(
    title="Select File",
    filters=[("Images", "*.png *.jpg *.gif"), ("All Files", "*.*")]
)

# Save file dialog
save_path = app.dialog.save_file(
    title="Save File",
    default_name="document.txt",
    filters=[("Text Files", "*.txt")]
)

# Select folder
folder_path = app.dialog.select_folder(title="Select Folder")

# Message dialog
app.dialog.message(
    title="Info",
    message="Operation completed successfully!",
    kind="info"  # "info", "warning", "error"
)

# Confirm dialog
confirmed = app.dialog.confirm(
    title="Confirm",
    message="Are you sure?",
    ok_label="Yes",
    cancel_label="No"
)
```

#### Frontend Usage

```javascript
// Open file
const filePath = await window.__forge__.dialog.openFile({
    title: "Select Image",
    filters: [{ name: "Images", extensions: ["png", "jpg", "gif"] }]
});

// Message dialog
await window.__forge__.dialog.message({
    title: "Success",
    message: "File saved!",
    kind: "info"
});
```

---

## System APIs

### SystemAPI

System information and operations.

```python
# Get system info
info = app.system.info()
# Returns: {"os": "Linux", "arch": "x86_64", "hostname": "...", ...}

# Open URL in default browser
app.system.open_url("https://example.com")

# Open file with default application
app.system.open_file("/path/to/document.pdf")

# Get environment variable
value = app.system.env("HOME")

# Get command line args
args = app.system.args()
```

### ScreenAPI

Monitor and display information.

```python
# Get all monitors
monitors = app.screen.monitors()
# Returns: [{"name": "HDMI-1", "width": 1920, "height": 1080, "scale": 1.0}, ...]

# Get primary monitor
primary = app.screen.primary_monitor()

# Get cursor position
pos = app.screen.cursor_position()
# Returns: {"x": 500, "y": 300}
```

### PowerAPI

Battery and power state.

```python
# Get battery info
battery = app.power.battery()
# Returns: {"level": 85, "charging": false, "time_remaining": 3600}

# Check if on battery power
on_battery = app.power.is_on_battery()
```

---

## UI APIs

### MenuAPI

Native application menus.

```python
# Set application menu
app.menu.set([
    {
        "label": "File",
        "children": [
            {"label": "New", "id": "new", "accelerator": "CmdOrCtrl+N"},
            {"label": "Open", "id": "open", "accelerator": "CmdOrCtrl+O"},
            {"type": "separator"},
            {"label": "Quit", "id": "quit", "role": "quit"}
        ]
    },
    {
        "label": "Edit",
        "children": [
            {"label": "Undo", "role": "undo"},
            {"label": "Redo", "role": "redo"},
            {"type": "separator"},
            {"label": "Cut", "role": "cut"},
            {"label": "Copy", "role": "copy"},
            {"label": "Paste", "role": "paste"}
        ]
    }
])

# Listen for menu events
@app.events.on("menu_selected")
def on_menu(event):
    print(f"Menu item clicked: {event['id']}")
```

### TrayAPI

System tray integration.

```python
# Create tray icon
app.tray.create(
    icon="path/to/icon.png",
    tooltip="My App",
    menu=[
        {"label": "Show", "id": "show"},
        {"type": "separator"},
        {"label": "Quit", "id": "quit"}
    ]
)

# Update tray
app.tray.set_tooltip("My App - Running")
app.tray.set_visible(True)
```

### NotificationAPI

Desktop notifications.

```python
# Show notification
app.notification.show(
    title="Update Available",
    body="A new version is ready to install.",
    icon="path/to/icon.png"
)
```

---

## Security APIs

### KeychainAPI

Secure credential storage.

```python
# Store secret
app.keychain.set("api_key", "sk-1234567890")

# Retrieve secret
api_key = app.keychain.get("api_key")

# Delete secret
app.keychain.delete("api_key")

# List stored keys
keys = app.keychain.keys()
```

### ScopeValidator

Path and URL access control.

```python
from forge.scope import ScopeValidator

validator = ScopeValidator(
    allow_patterns=["/home/user/appdata/**", "https://api.example.com/**"],
    deny_patterns=["/home/user/appdata/secret/**"]
)

# Check path
allowed = validator.is_path_allowed("/home/user/appdata/config.json")  # True
denied = validator.is_path_allowed("/home/user/appdata/secret/key.pem")  # False

# Check URL
allowed = validator.is_url_allowed("https://api.example.com/data")  # True
denied = validator.is_url_allowed("https://evil.com/steal")  # False
```

---

## Network APIs

### WebSocketAPI

WebSocket communication.

```python
# Create WebSocket server
app.websocket.listen(port=8080)

# Send message to client
app.websocket.send(client_id, {"type": "update", "data": {...}})

# Broadcast to all clients
app.websocket.broadcast({"type": "notification", "message": "Hello!"})

# Listen for messages
@app.events.on("websocket:message")
def on_message(event):
    print(f"Received: {event['data']}")
```

### DeepLinkAPI

Custom protocol handlers.

```python
# Register protocol
app.deep_links.register("myapp")

# Handle deep links
@app.events.on("deep_link")
def on_deep_link(event):
    url = event["url"]  # "myapp://settings?tab=general"
    print(f"Deep link: {url}")
```

---

## Utility APIs

### ClipboardAPI

System clipboard access.

```python
# Get clipboard content
text = app.clipboard.get_text()

# Set clipboard content
app.clipboard.set_text("Hello, World!")

# Check if clipboard has text
has_text = app.clipboard.has_text()
```

### ShortcutsAPI

Global keyboard shortcuts.

```python
# Register shortcut
app.shortcuts.register("CmdOrCtrl+Shift+X", "screenshot")

# Listen for shortcuts
@app.events.on("shortcut")
def on_shortcut(event):
    if event["id"] == "screenshot":
        take_screenshot()

# Unregister shortcut
app.shortcuts.unregister("CmdOrCtrl+Shift+X")

# Unregister all
app.shortcuts.unregister_all()
```

### AutostartAPI

Launch at system startup.

```python
# Enable autostart
app.autostart.enable()

# Disable autostart
app.autostart.disable()

# Check if enabled
enabled = app.autostart.is_enabled()
```

### UpdaterAPI

Application updates.

```python
# Check for updates
result = app.updater.check()
if result["update_available"]:
    print(f"New version: {result['latest_version']}")

# Download and apply update
app.updater.update()

# Generate manifest (for update server)
manifest = app.updater.generate_manifest(
    version="1.1.0",
    url="https://releases.example.com/app-1.1.0.zip",
    signature="...",
    checksum="sha256:..."
)
```

---

## Built-in Plugins

### AI/ML Plugin

Cloud AI integration.

```python
from forge.builtins.ai_ml import BuiltinAIMLAPI

ai = BuiltinAIMLAPI(app)

# OpenAI-compatible chat
result = ai.openai_chat(
    api_key="sk-...",
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)

# Check ONNX Runtime
onnx = ai.onnx_runtime_available()
```

### Local LLM Plugin

Offline AI inference with llama.cpp.

```python
from forge.builtins.llm_local import BuiltinLocalLLMAPI

llm = BuiltinLocalLLMAPI(app)

# Load model
result = llm.llm_load("/path/to/model.gguf", n_ctx=2048, n_gpu_layers=0)

# Chat completion
response = llm.llm_chat(
    messages=[{"role": "user", "content": "Hello!"}],
    max_tokens=512,
    temperature=0.7
)

# Text completion
result = llm.llm_complete("Once upon a time", max_tokens=100)

# Embeddings
embeddings = llm.llm_embed("Hello, world!")

# Unload model
llm.llm_unload()
```

### Database Plugin

Built-in database support.

```python
from forge.builtins.database import BuiltinDatabaseAPI

db = BuiltinDatabaseAPI(app)

# Connect
db.connect("sqlite:///app.db")

# Query
rows = db.query("SELECT * FROM users WHERE active = ?", [True])

# Execute
db.execute("INSERT INTO users (name, email) VALUES (?, ?)", ["John", "john@example.com"])

# Transaction
with db.transaction():
    db.execute("UPDATE accounts SET balance = balance - ? WHERE id = ?", [100, 1])
    db.execute("UPDATE accounts SET balance = balance + ? WHERE id = ?", [100, 2])
```

### Crypto Plugin

Cryptographic operations.

```python
from forge.builtins.crypto import BuiltinCryptoAPI

crypto = BuiltinCryptoAPI(app)

# Hash
hash_value = crypto.sha256("Hello, World!")

# HMAC
hmac_value = crypto.hmac_sha256("secret_key", "message")

# Encrypt/Decrypt (AES-GCM)
encrypted = crypto.encrypt("secret data", "password")
decrypted = crypto.decrypt(encrypted, "password")
```

---

## Configuration (forge.toml)

```toml
[app]
name = "My App"
version = "1.0.0"
description = "A Forge desktop application"

[window]
title = "My App"
width = 1200
height = 800
resizable = true
remember_state = true  # Persist window position/size across restarts

[build]
entry = "src/main.py"
icon = "assets/icon.png"

[permissions]
filesystem = true
shell = false
clipboard = true
dialogs = true
notifications = true
keychain = false
updater = false

[dev]
frontend_dir = "src/frontend"
port = 3000

[protocol]
schemes = ["myapp"]

[packaging]
app_id = "com.example.myapp"
formats = ["appimage", "deb"]  # Linux
# formats = ["dmg"]  # macOS
# formats = ["nsis", "msi"]  # Windows

[updater]
enabled = true
endpoint = "https://updates.example.com/manifest.json"
public_key = "..."
channel = "stable"
```

---

## Error Codes

| Code | Description |
|------|-------------|
| `invalid_request` | Malformed JSON or missing fields |
| `malformed_json` | JSON parsing failed |
| `request_too_large` | Request exceeds 10MB limit |
| `permission_denied` | Capability not enabled |
| `unknown_command` | Command not registered |
| `command_failed` | Command execution error |
| `command_timeout` | Command took too long |
| `circuit_open` | Too many failures, circuit breaker open |

---

## Thread Safety

ForgeDesk is designed for Python 3.14+ NoGIL (free-threaded) mode:

- **EventEmitter**: Thread-safe with `threading.Lock`
- **AppState**: Thread-safe with `threading.Lock`
- **IPCBridge**: Thread-safe command dispatch via `ThreadPoolExecutor`
- **WindowProxy**: Thread-safe via `EventLoopProxy` (Send+Sync)

All APIs can be safely called from any thread.
