# Hello Forge

A demo application showcasing the Forge Framework's IPC capabilities.

## Features Demonstrated

- **Greeting**: Call Python functions from JavaScript
- **System Info**: Display OS, Python version, and platform details
- **Time Display**: Get current date/time from Python
- **Calculator**: Perform calculations in Python backend
- **Clipboard**: Read/write to system clipboard
- **Data Storage**: Save/load data using file system API
- **Progress Demo**: Real-time events from Python to JavaScript

## Running the Demo

```bash
cd examples/hello_forge
forge dev
```

## What You'll See

### Home Tab
- Welcome screen with greeting demo
- Quick action buttons to navigate features

### System Info Tab
- Click to fetch system information from Python
- Real-time clock display

### Calculator Tab
- Perform mathematical operations in Python
- See results with operation details

### Clipboard Tab
- Copy text to system clipboard
- Read clipboard contents

### Data Storage Tab
- Save key-value pairs to files
- Load previously saved data
- List all saved keys

### Progress Demo Tab
- Watch Python emit real-time events
- Progress bar updates from Python thread
- Start/Stop controls

## Architecture

```
┌─────────────────┐         ┌─────────────────┐
│   JavaScript    │◄───────►│     Python      │
│    Frontend     │   IPC   │     Backend     │
│                 │  Bridge │                 │
├─────────────────┤         ├─────────────────┤
│ • greet()       │         │ • greet(name)   │
│ • getSystemInfo │         │ • get_system_   │
│ • calculate()   │         │   info()        │
│ • clipboard     │         │ • calculate()   │
│ • save/load     │         │ • fs.read/write │
└─────────────────┘         └─────────────────┘
```

## Key Code Examples

### JavaScript → Python
```javascript
const result = await window.__forge__.invoke('greet', { name: 'Alice' });
console.log(result); // "Hello, Alice! From Python 🐍"
```

### Python → JavaScript (Events)
```python
# Python
app.emit("progress_update", {"value": 50})
```

```javascript
// JavaScript
window.__forge__.on("progress_update", (data) => {
    console.log(data.value); // 50
});
```

## Files

- `src/main.py` - Python backend with all commands
- `src/frontend/index.html` - UI structure
- `src/frontend/style.css` - Cyberpunk styling
- `src/frontend/main.js` - Frontend logic
- `forge.toml` - App configuration

---

**Forge the future. Ship with Python.** 🚀
