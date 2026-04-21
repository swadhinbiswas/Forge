---
title: "Building Applications with Forge"
description: "Complete guide to structuring and building production-grade Forge applications"
---

## Project Structure

A professional Forge application follows this recommended structure:

```
my-app/
├── forge.toml                    # Application manifest
├── src/
│   ├── main.py                   # Application entry point
│   ├── backend/
│   │   ├── api.py               # API endpoints (@app.command)
│   │   ├── services/            # Business logic
│   │   │   ├── data.py
│   │   │   ├── auth.py
│   │   │   └── worker.py
│   │   ├── models/              # Data models
│   │   │   ├── user.py
│   │   │   ├── task.py
│   │   │   └── schema.py
│   │   └── db/                  # Database layer
│   │       ├── connection.py
│   │       ├── migrations/
│   │       └── queries/
│   └── frontend/                # React/Vue/Svelte app
│       ├── index.html
│       ├── main.jsx
│       ├── forge-api.js         # IPC wrapper
│       ├── components/
│       ├── pages/
│       ├── hooks/
│       └── styles/
├── assets/
│   ├── icon.png
│   ├── icon.icns
│   └── icon.ico
├── package.json                 # Frontend dependencies
├── requirements.txt             # Python dependencies
└── README.md
```

## Setting Up Your Application

### 1. Configure `forge.toml`

```toml
[app]
name = "My Application"
version = "1.0.0"
description = "A powerful Forge application"
authors = ["Your Name"]

[window]
title = "My Application"
width = 1280
height = 720
min_width = 800
min_height = 600
resizable = true
fullscreen = false
decorations = true
transparent = false
always_on_top = false

[build]
entry = "src/main.py"
output_dir = "dist"
icon = "assets/icon.png"

[dev]
frontend_dir = "src/frontend"
port = 5199
hot_reload = true

[permissions]
filesystem = true
clipboard = true
dialogs = true
notifications = true
system_tray = false

[security]
strict_mode = false
allowed_origins = ["forge://app"]
```

### 2. Create Backend Entry Point

```python
# src/main.py
import logging
from forge import ForgeApp
from backend import api, services

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(name)s: %(message)s'
)

def main():
    # Initialize app
    app = ForgeApp()
    
    # Register API routes
    api.register_routes(app)
    
    # Setup lifecycle hooks
    @app.on_ready()
    def on_ready():
        logging.info("Application started and ready")
        # Initialize services, load data, etc.
        services.init_cache()
    
    @app.on_close()
    def on_close():
        logging.info("Application closing")
        # Cleanup, save state, etc.
        services.cleanup()
    
    # Run the app
    app.run()

if __name__ == "__main__":
    main()
```

### 3. Organize APIs with Modules

```python
# src/backend/api.py
import logging
from forge import ForgeApp
from . import services, models

logger = logging.getLogger(__name__)

def register_routes(app: ForgeApp):
    """Register all API endpoints"""
    
    # User Management
    @app.command("user_login")
    def user_login(email: str, password: str) -> dict:
        """Authenticate user"""
        try:
            user = services.auth.login(email, password)
            return {"success": True, "user": user.to_dict()}
        except Exception as e:
            logger.error(f"Login failed: {e}")
            return {"success": False, "error": str(e)}
    
    @app.command("user_get_profile")
    def user_get_profile(user_id: int) -> dict:
        """Fetch user profile"""
        user = services.user.get_by_id(user_id)
        return user.to_dict() if user else {}
    
    # Task Management
    @app.command("task_create")
    def task_create(title: str, description: str, priority: str = "medium") -> dict:
        """Create a new task"""
        task = models.Task(
            title=title,
            description=description,
            priority=priority
        )
        task.save()
        return {"success": True, "id": task.id}
    
    @app.command("task_list")
    def task_list(status: str = "all", limit: int = 50) -> list:
        """List tasks with optional filtering"""
        tasks = services.task.list(status=status, limit=limit)
        return [t.to_dict() for t in tasks]
    
    @app.command("task_update")
    def task_update(task_id: int, **updates) -> bool:
        """Update task fields"""
        task = models.Task.get_by_id(task_id)
        if not task:
            return False
        for key, value in updates.items():
            setattr(task, key, value)
        task.save()
        return True
    
    @app.command("task_delete")
    def task_delete(task_id: int) -> bool:
        """Delete a task"""
        return models.Task.delete_by_id(task_id)
```

### 4. Structure Business Logic

```python
# src/backend/services/auth.py
import hashlib
import secrets
from typing import Optional
from ..models import User, Session

class AuthService:
    """Authentication and authorization"""
    
    @staticmethod
    def login(email: str, password: str) -> User:
        """Verify credentials and create session"""
        user = User.find_by_email(email)
        if not user:
            raise ValueError("User not found")
        
        if not AuthService._verify_password(password, user.password_hash):
            raise ValueError("Invalid password")
        
        # Create session
        session = Session.create(user.id)
        return user
    
    @staticmethod
    def logout(session_id: str) -> bool:
        """Invalidate session"""
        session = Session.find_by_id(session_id)
        if session:
            session.delete()
            return True
        return False
    
    @staticmethod
    def _verify_password(password: str, hash: str) -> bool:
        """Securely verify password"""
        return hashlib.pbkdf2_hmac(
            'sha256',
            password.encode(),
            hash[:32],
            100000
        ) == hash[32:]

# Usage in API
auth_service = AuthService()
```

## Frontend Integration

### 1. Create the IPC Bridge Wrapper

```javascript
// src/frontend/forge-api.js
/**
 * Safe wrapper around Forge IPC runtime
 * Provides clean async interface for backend commands
 */

// Check if Forge runtime is available
const isForgeAvailable = () => typeof window !== 'undefined' && window.__forge__;

// Invoke a command on the Python backend
export const invoke = async (command, payload = {}) => {
  if (!isForgeAvailable()) {
    console.warn(`Forge runtime unavailable for: ${command}`);
    return null;
  }
  
  try {
    return await window.__forge__.invoke(command, payload);
  } catch (error) {
    console.error(`Command failed [${command}]:`, error);
    throw error;
  }
};

// Listen for events from Python backend
export const on = (eventName, handler) => {
  if (!isForgeAvailable()) {
    console.warn(`Forge runtime unavailable for event: ${eventName}`);
    return () => {}; // noop unsubscribe
  }
  
  return window.__forge__.on(eventName, (data) => {
    handler(data.payload);
  });
};

// Emit events from frontend
export const emit = async (eventName, data) => {
  if (!isForgeAvailable()) {
    console.warn(`Forge runtime unavailable to emit: ${eventName}`);
    return;
  }
  
  return await window.__forge__.emit(eventName, data);
};

// User API
export const user = {
  login: (email, password) => 
    invoke('user_login', { email, password }),
  getProfile: (userId) => 
    invoke('user_get_profile', { user_id: userId }),
};

// Task API
export const tasks = {
  create: (title, description, priority) =>
    invoke('task_create', { title, description, priority }),
  list: (status = 'all', limit = 50) =>
    invoke('task_list', { status, limit }),
  update: (id, updates) =>
    invoke('task_update', { task_id: id, ...updates }),
  delete: (id) =>
    invoke('task_delete', { task_id: id }),
};
```

### 2. Build React Components with IPC

```jsx
// src/frontend/hooks/useForgeCommand.js
import { useState, useCallback } from 'react';
import { invoke } from '../forge-api';

/**
 * Hook for calling Python backend commands
 * Handles loading, error, and result states
 */
export const useForgeCommand = (command, payload = {}) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [result, setResult] = useState(null);
  
  const execute = useCallback(async (overrides = {}) => {
    setLoading(true);
    setError(null);
    
    try {
      const res = await invoke(command, { ...payload, ...overrides });
      setResult(res);
      return res;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [command, payload]);
  
  return { execute, loading, error, result };
};
```

```jsx
// src/frontend/pages/Dashboard.jsx
import React, { useState, useEffect } from 'react';
import { tasks, on } from '../forge-api';
import { useForgeCommand } from '../hooks/useForgeCommand';

export default function Dashboard() {
  const [taskList, setTaskList] = useState([]);
  const { execute: fetchTasks, loading } = useForgeCommand('task_list');
  
  // Load tasks on mount
  useEffect(() => {
    loadTasks();
  }, []);
  
  // Listen for task updates from backend
  useEffect(() => {
    const unsubscribe = on('task_updated', (task) => {
      setTaskList(prev => 
        prev.map(t => t.id === task.id ? task : t)
      );
    });
    
    return unsubscribe;
  }, []);
  
  const loadTasks = async () => {
    const data = await fetchTasks();
    setTaskList(data);
  };
  
  const handleCreateTask = async (e) => {
    e.preventDefault();
    const { title, description } = e.target.elements;
    
    const res = await tasks.create(
      title.value,
      description.value
    );
    
    if (res.success) {
      title.value = '';
      description.value = '';
      loadTasks();
    }
  };
  
  return (
    <div className="dashboard">
      <h1>Tasks</h1>
      
      <form onSubmit={handleCreateTask}>
        <input name="title" placeholder="Task title" required />
        <textarea name="description" placeholder="Description"></textarea>
        <button type="submit">Add Task</button>
      </form>
      
      {loading ? (
        <p>Loading...</p>
      ) : (
        <ul>
          {taskList.map(task => (
            <li key={task.id}>{task.title}</li>
          ))}
        </ul>
      )}
    </div>
  );
}
```

## Best Practices

### 1. Error Handling

```python
# Backend
from typing import Union

@app.command("safe_operation")
def safe_operation(value: int) -> Union[dict, str]:
    """
    Returns either success dict or error string
    """
    try:
        if value < 0:
            raise ValueError("Value must be positive")
        
        result = expensive_operation(value)
        return {"success": True, "data": result}
    except ValueError as e:
        return {"success": False, "error": str(e)}
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        return {"success": False, "error": "Internal server error"}
```

### 2. Type Annotations

```python
# Always use type hints
from typing import Optional, List
from pydantic import BaseModel

class UserRequest(BaseModel):
    email: str
    password: str
    remember_me: bool = False

@app.command("login")
def login(request: UserRequest) -> dict:
    """Fully typed, auto-validated command"""
    # Forge validates 'request' before calling this
    user = authenticate(request.email, request.password)
    return user.to_dict()
```

### 3. Background Tasks

```python
import threading
import time

def start_background_worker(app):
    """Long-running background task"""
    def worker():
        while True:
            try:
                # Poll for updates, sync data, etc.
                data = fetch_updates()
                app.emit("updates_available", data)
                time.sleep(30)  # Every 30 seconds
            except Exception as e:
                logger.error(f"Worker error: {e}")
    
    thread = threading.Thread(target=worker, daemon=True)
    thread.start()

@app.on_ready()
def on_ready():
    start_background_worker(app)
```

### 4. State Management

```python
# Use app.state for global app data
@app.command("get_cache")
def get_cache(key: str):
    return app.state.get(key)

@app.command("set_cache")
def set_cache(key: str, value: str):
    app.state.set(key, value)
    return True
```

## Deployment

### Build for Production

```bash
# Clean build
forge build --release

# Platform-specific
forge build --release --platform macos
forge build --release --platform windows
forge build --release --platform linux
```

### Distribution
- **macOS**: DMG installer with code signing
- **Windows**: EXE installer with MSIX support  
- **Linux**: AppImage, Flatpak, or .deb packages

### Signing & Updates

```toml
[updater]
enabled = true
endpoints = ["https://updates.example.com/releases"]
signature_key = "your-ed25519-key"
```

## Troubleshooting

### Commands Not Found
- Verify `@app.command` decorator is used
- Check command name matches JavaScript invocation
- Ensure `register_routes(app)` is called in `main.py`

### Type Validation Errors
- Use proper Python type hints: `str`, `int`, `float`, `bool`, `List[T]`
- Use `pydantic.BaseModel` for complex types
- Check argument names match exactly

### Frontend Can't Connect
- Verify `forge.toml` dev settings
- Check frontend Dev Tools console for errors
- Ensure `forge.js` is loaded before app initializes
- Check `forge-api.js` wrapper is imported

