# Forge API Explorer

An interactive developer tool for exploring and testing all Forge Framework APIs.

## Purpose

The API Explorer is designed to help you:
- Learn about all available Forge APIs
- Test API functionality interactively
- Understand request/response formats
- Debug IPC communication
- See real-time events from Python

## Running the Explorer

```bash
cd examples/api_explorer
forge dev
```

## Available APIs

### System APIs
- **System Info** - Get OS, Python version, hardware details
- **App Version** - Get application version
- **Timestamp** - Get current time in various formats

### Clipboard APIs
- **Read** - Read text from system clipboard
- **Write** - Write text to system clipboard

### File System APIs
- **List Directory** - List contents of a directory
- **Read File** - Read file contents
- **Write File** - Write content to a file
- **Check Exists** - Check if path exists
- **Create Directory** - Create a new directory

### Dialog APIs
- **Open File** - Native file open dialog
- **Save File** - Native file save dialog
- **Message** - Show message dialog (info/warning/error)

### Event APIs
- **Counter Demo** - Real-time counter with progress events
- **Custom Event** - Emit custom events from Python

### Utility APIs
- **Echo** - Echo back data (for testing)
- **Text Processor** - Various text operations
- **API List** - Get categorized list of all APIs

## Interface

```
┌─────────────────────────────────────────────────────────────┐
│  ⚡ API Explorer                              Forge v1.0.0  │
├──────────┬──────────────────────────────────────────────────┤
│ Sidebar  │  Main Content                                    │
│          │  ┌─────────────────┬─────────────────┐           │
│ System   │  │    Request      │    Response     │           │
│ ├ Info   │  │  ┌───────────┐  │  ┌───────────┐  │           │
│ ├ Version│  │  │ Parameters│  │  │  Result   │  │           │
│ └ Time   │  │  │           │  │  │           │  │           │
│          │  │  └───────────┘  │  └───────────┘  │           │
│ Clipboard│  └─────────────────┴─────────────────┘           │
│ File Sys │  ┌─────────────────────────────────────────┐     │
│ Dialogs  │  │           Event Log                      │     │
│ Events   │  │  [12:34:56] Received event: counter_update│     │
│ Utils    │  └─────────────────────────────────────────┘     │
└──────────┴──────────────────────────────────────────────────┘
```

## Features

### Request Builder
- Dynamic form generation based on API parameters
- Support for text, textarea, and select inputs
- JSON parsing for complex data types

### Response Viewer
- Syntax-highlighted JSON output
- Success/error color coding
- Copy to clipboard functionality

### Event Log
- Real-time event monitoring
- Timestamp for each event
- System, event, and error categorization

## Example Usage

### Testing File Write
1. Select "Write File" from File System section
2. Enter path: `test.txt`
3. Enter content: `Hello, Forge!`
4. Click Execute
5. See response: `true`

### Testing Events
1. Select "Counter Demo" from Events section
2. Click Execute
3. Watch progress updates in response and event log
4. See completion message at 100%

### Testing Clipboard
1. Select "Write Clipboard"
2. Enter text to copy
3. Click Execute
4. Paste elsewhere to verify

## Code Integration

Use the API Explorer as a reference for your own code:

```javascript
// From API Explorer source
async function callPython(command, params) {
    const result = await window.__forge__.invoke(command, params);
    console.log(result);
}

// Event listener
window.__forge__.on('my_event', (data) => {
    console.log('Event received:', data);
});
```

## Tips

1. **Start with Echo** - Use the Echo API to test basic connectivity
2. **Watch the Event Log** - All API calls and events are logged
3. **Copy Responses** - Use the Copy button to save results
4. **Try Dialogs** - File dialogs demonstrate native integration

---

**Built with Forge Framework** 🚀
