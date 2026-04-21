---
title: State Management
description: Learn how to manage and synchronize state across the Python backend and JavaScript frontend.
---

In a standard web application, all of your state lives in the browser's memory. However, **ForgeDesk utilizes a Dual-Process Architecture**, meaning you have *two* distinct places where state can live: the frontend WebView (JavaScript) and the native backend (Python).

Understanding how to manage, share, and synchronize state across this boundary is the key to building robust desktop applications.

---

## 🧠 Where should state live?

A good rule of thumb for ForgeDesk applications:

1.  **Strictly UI State (Frontend):** Is a modal open? What tab is currently selected? Is this button hovered? **Keep this in JavaScript.** Use standard tools like React `useState`, Zustand, Vuex, or Svelte Stores.
2.  **Application State (Backend):** Are we connected to the local database? What is the user's system configuration? What data did we just scrape from the machine? **Keep this in Python.**

---

## 🐍 Backend State (Python)

ForgeDesk provides a built-in state management system on the backend via the `app.state` object. This is a global, thread-safe dictionary where you can inject services, configuration, or in-memory caches when the app starts, and access them inside your IPC routes.

### 1. Initializing State

Inject your data during the `on_app_start` lifecycle event or right before calling `app.run()`.

```python
from forge import ForgeApp, router

app = ForgeApp()

@app.on_event("startup")
def setup_state():
    # Injecting a simple dictionary
    app.state.user_preferences = {"theme": "dark", "notifications": True}
    
    # You can also inject database connections!
    # app.state.db = SQLiteConnection("app.db")
```

### 2. Accessing State in Routes

When the frontend invokes a backend command, you can access the global state to read or mutate it.

```python
@router.command(namespace="prefs")
def get_theme(app) -> str:
    # Access the state we injected earlier
    return app.state.user_preferences["theme"]

@router.command(namespace="prefs")
def set_theme(app, new_theme: str) -> bool:
    app.state.user_preferences["theme"] = new_theme
    return True
```

---

## 🔄 Synchronizing State (Backend to Frontend)

Sometimes the backend mutates state entirely on its own (e.g., a background thread finishes downloading a file). Because the frontend doesn't know about this change automatically, the backend needs to explicitly **push** the update to the frontend using **Events**.

### Triggering an Event from Python

Use `app.window.emit()` to broadcast an event with data to the frontend.

```python
import time
import threading

def background_worker(app):
    time.sleep(5) # Simulate long work
    # Notify the frontend that the state has changed!
    app.window.emit("download_complete", {"filename": "report.pdf", "status": "success"})

@router.command(namespace="downloads")
def start_download(app, filename: str):
    # Start work in the background so we don't block the UI
    threading.Thread(target=background_worker, args=(app,)).start()
    return "started"
```

### Listening for the Event in JavaScript

On the frontend, use the `@forgedesk/api` event listener to catch the state update and update your React/Vue/Svelte components accordingly.

```typescript
import { listen } from '@forgedesk/api/events';
import { useEffect, useState } from 'react';

function DownloadManager() {
    const [downloads, setDownloads] = useState([]);

    useEffect(() => {
        // Subscribe to the backend event
        const unlisten = listen('download_complete', (event) => {
            console.log("Download finished:", event.payload.filename);
            
            // Update frontend state
            setDownloads(prev => [...prev, event.payload]);
        });

        // Cleanup the listener when the component unmounts
        return () => {
            unlisten.then(fn => fn());
        };
    }, []);

    return (
        <div>
            <h2>Finished Downloads</h2>
            {downloads.map(dn => <p key={dn.filename}>{dn.filename} - {dn.status}</p>)}
        </div>
    );
}
```

---

## 💾 Persistent State (Disk)

If you need state to survive an app restart (like saved configurations or local user data), you shouldn't rely on `app.state`, as it is cleared when the app closes. 

Instead, persist your data using native file storage:

1.  **Frontend-Driven:** Use the **`@forgedesk/plugin-fs`** (File System plugin) to read and write `.json` files to the user's local `AppData` or `Documents` folder.
2.  **Backend-Driven:** Use Python's built-in `sqlite3` or an ORM like `SQLAlchemy` to maintain a local database for complex, relationally structured state.
