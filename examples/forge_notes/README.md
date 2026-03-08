# Forge Notes

A feature-rich note-taking application built with Forge Framework, demonstrating file system operations, native dialogs, and real-time features.

## Features

- 📝 **Create, Edit, Delete Notes** - Full CRUD operations for notes
- 🔍 **Search Notes** - Search through note content
- 💾 **Auto-save** - Quick save with Ctrl+S
- 📁 **Import/Export** - Open and save files using native dialogs
- 📊 **Statistics** - View note count and total storage used
- ⚡ **Keyboard Shortcuts** - Fast navigation and actions
- 🎨 **Modern UI** - Dark theme with responsive design

## Running the App

```bash
cd examples/forge_notes
forge dev
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Save current note |
| `Ctrl+N` | Create new note |
| `Enter` (in search) | Search notes |

## Screenshots

### Main Interface
- Sidebar with note list
- Search functionality
- Editor with live counts

### Features Demonstrated

1. **File System API**
   - `list_notes()` - List all notes
   - `read_note()` - Read note content
   - `save_note()` - Save note content
   - `delete_note()` - Delete a note
   - `search_notes()` - Search by content

2. **Dialog API**
   - `open_note_dialog()` - Native file open dialog
   - `save_note_dialog()` - Native file save dialog

3. **System API**
   - `get_note_stats()` - Get storage statistics

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Forge Notes App                    │
├──────────────────┬──────────────────────────────────┤
│    Sidebar       │         Editor                   │
│  ┌────────────┐  │  ┌────────────────────────────┐  │
│  │ Search     │  │  │ Title Input                │  │
│  ├────────────┤  │  ├────────────────────────────┤  │
│  │ Note List  │  │  │                            │  │
│  │ • Note 1   │  │  │  Note Content              │  │
│  │ • Note 2   │  │  │  (Textarea)                │  │
│  │ • Note 3   │  │  │                            │  │
│  └────────────┘  │  └────────────────────────────┘  │
│  ┌────────────┐  │  ┌────────────────────────────┐  │
│  │ 5 notes    │  │  │ 1234 chars | 234 words     │  │
│  │ 15.2 KB    │  │  │ Last saved: Just now       │  │
│  └────────────┘  │  └────────────────────────────┘  │
└──────────────────┴──────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │   Python Backend │
                    │  ┌─────────────┐ │
                    │  │  notes/     │ │
                    │  │  • note1.txt│ │
                    │  │  • note2.txt│ │
                    │  └─────────────┘ │
                    └─────────────────┘
```

## File Structure

```
forge_notes/
├── forge.toml              # App configuration
├── src/
│   ├── main.py            # Python backend
│   └── frontend/
│       ├── index.html     # UI structure
│       ├── style.css      # Dark theme styles
│       └── main.js        # Frontend logic
├── notes/                 # Notes storage (created on first run)
└── README.md
```

## Code Examples

### Python Backend - List Notes
```python
@app.command
def list_notes() -> list[dict]:
    notes_dir = _get_notes_dir()
    notes = []
    for file in sorted(notes_dir.glob("*.txt")):
        stat = file.stat()
        notes.append({
            "name": file.stem,
            "size": stat.st_size,
            "modified": stat.st_mtime,
        })
    return notes
```

### JavaScript Frontend - Save Note
```javascript
async function saveCurrentNote() {
    const title = document.getElementById('noteTitle').value;
    const content = document.getElementById('noteEditor').value;
    
    const result = await window.__forge__.invoke('save_note', { 
        name: title, 
        content 
    });
    
    showToast('Note saved!', 'success');
}
```

## Tips

1. **Notes are stored locally** in the `./notes` directory as `.txt` files
2. **Use search** to quickly find notes by content
3. **Keyboard shortcuts** speed up your workflow
4. **Import existing notes** using the "Open File" button

---

**Built with Forge Framework** 🚀
