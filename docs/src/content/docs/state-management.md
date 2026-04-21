---
title: "State Management & Persistence"
description: "Managing application state, persistence, and backend data"
---

## Application State

Forge provides a thread-safe, global state container accessible from both Python and the frontend.

### Basic State Usage

```python
from forge import ForgeApp

app = ForgeApp()

# Set global state
@app.command("set_user_preference")
def set_user_preference(key: str, value: str):
    """Store user preference"""
    app.state.set(f"pref_{key}", value)
    return True

# Get global state
@app.command("get_user_preference")
def get_user_preference(key: str):
    """Retrieve user preference"""
    return app.state.get(f"pref_{key}")

# Check if key exists
@app.command("has_preference")
def has_preference(key: str):
    """Check if preference exists"""
    return app.state.has(f"pref_{key}")

# Delete from state
@app.command("clear_preference")
def clear_preference(key: str):
    """Remove user preference"""
    app.state.delete(f"pref_{key}")
    return True

app.run()
```

### Accessing State from Frontend

```jsx
import { invoke } from '../forge-api.js';

async function getUserTheme() {
  const theme = await invoke('get_user_preference', { key: 'theme' });
  return theme || 'light'; // Default to light
}

function App() {
  const [theme, setTheme] = useState('light');
  
  useEffect(() => {
    getUserTheme().then(setTheme);
  }, []);
  
  const handleThemeChange = async (newTheme) => {
    setTheme(newTheme);
    await invoke('set_user_preference', { 
      key: 'theme', 
      value: newTheme 
    });
  };
  
  return (
    <div className={`app-${theme}`}>
      <button onClick={() => handleThemeChange(
        theme === 'light' ? 'dark' : 'light'
      )}>
        Toggle Theme
      </button>
    </div>
  );
}
```

## Database Persistence

For persistence across application restarts, use SQLite or your preferred database.

### SQLite Setup

```python
# src/backend/db.py
import sqlite3
import threading
from pathlib import Path
from typing import List, Dict, Any

DB_PATH = Path.home() / ".app_cache" / "app.db"
DB_PATH.parent.mkdir(parents=True, exist_ok=True)

# Thread-local storage for connections
_thread_local = threading.local()

def get_connection():
    """Get database connection for current thread"""
    if not hasattr(_thread_local, 'connection'):
        _thread_local.connection = sqlite3.connect(
            DB_PATH,
            check_same_thread=False,
            timeout=10
        )
        _thread_local.connection.row_factory = sqlite3.Row
    return _thread_local.connection

def init_db():
    """Initialize database schema"""
    conn = get_connection()
    
    conn.executescript("""
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT DEFAULT 'pending',
            priority INTEGER DEFAULT 1,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    """)
    conn.commit()

def query(sql: str, params: tuple = ()) -> List[Dict]:
    """Execute SELECT query"""
    conn = get_connection()
    cursor = conn.execute(sql, params)
    return [dict(row) for row in cursor.fetchall()]

def query_one(sql: str, params: tuple = ()) -> Dict:
    """Execute SELECT query, return first result"""
    conn = get_connection()
    cursor = conn.execute(sql, params)
    row = cursor.fetchone()
    return dict(row) if row else None

def execute(sql: str, params: tuple = ()) -> int:
    """Execute INSERT/UPDATE/DELETE"""
    conn = get_connection()
    cursor = conn.execute(sql, params)
    conn.commit()
    return cursor.lastrowid

def execute_many(sql: str, params: List[tuple]) -> bool:
    """Execute multiple INSERT/UPDATE/DELETE"""
    conn = get_connection()
    conn.executemany(sql, params)
    conn.commit()
    return True
```

### Data Models

```python
# src/backend/models.py
from datetime import datetime
from typing import Optional, List
from . import db

class User:
    @staticmethod
    def create(username: str, email: str, password_hash: str):
        """Create new user"""
        user_id = db.execute(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)",
            (username, email, password_hash)
        )
        return user_id
    
    @staticmethod
    def get_by_id(user_id: int) -> Optional[dict]:
        """Fetch user by ID"""
        return db.query_one("SELECT * FROM users WHERE id = ?", (user_id,))
    
    @staticmethod
    def get_by_email(email: str) -> Optional[dict]:
        """Fetch user by email"""
        return db.query_one("SELECT * FROM users WHERE email = ?", (email,))
    
    @staticmethod
    def all() -> List[dict]:
        """Get all users"""
        return db.query("SELECT * FROM users ORDER BY created_at DESC")

class Task:
    @staticmethod
    def create(user_id: int, title: str, description: str = "", priority: int = 1) -> int:
        """Create new task"""
        task_id = db.execute(
            "INSERT INTO tasks (user_id, title, description, priority) VALUES (?, ?, ?, ?)",
            (user_id, title, description, priority)
        )
        return task_id
    
    @staticmethod
    def get_by_user(user_id: int, status: str = None) -> List[dict]:
        """Get tasks for user, optionally filtered by status"""
        if status:
            return db.query(
                "SELECT * FROM tasks WHERE user_id = ? AND status = ? ORDER BY priority DESC, created_at DESC",
                (user_id, status)
            )
        return db.query(
            "SELECT * FROM tasks WHERE user_id = ? ORDER BY priority DESC, created_at DESC",
            (user_id,)
        )
    
    @staticmethod
    def update(task_id: int, **fields) -> bool:
        """Update task fields"""
        allowed_fields = {'title', 'description', 'status', 'priority'}
        updates = {k: v for k, v in fields.items() if k in allowed_fields}
        
        if not updates:
            return False
        
        set_clause = ", ".join(f"{k} = ?" for k in updates.keys())
        db.execute(
            f"UPDATE tasks SET {set_clause}, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            (*updates.values(), task_id)
        )
        return True
    
    @staticmethod
    def delete(task_id: int) -> bool:
        """Delete task"""
        db.execute("DELETE FROM tasks WHERE id = ?", (task_id,))
        return True

class Settings:
    @staticmethod
    def get(key: str, default: str = None) -> str:
        """Get setting value"""
        result = db.query_one("SELECT value FROM settings WHERE key = ?", (key,))
        return result['value'] if result else default
    
    @staticmethod
    def set(key: str, value: str) -> bool:
        """Set setting value"""
        db.execute(
            "INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = ?",
            (key, value, value)
        )
        return True
    
    @staticmethod
    def all() -> dict:
        """Get all settings"""
        results = db.query("SELECT key, value FROM settings")
        return {r['key']: r['value'] for r in results}
```

### Using Models in API

```python
# src/backend/api.py
from forge import ForgeApp
from . import db, models

app = ForgeApp()
db.init_db()

@app.command("task_create")
def task_create(user_id: int, title: str, description: str = ""):
    """Create task"""
    task_id = models.Task.create(user_id, title, description)
    return {"id": task_id, "created": True}

@app.command("task_list")
def task_list(user_id: int, status: str = None):
    """List user tasks"""
    tasks = models.Task.get_by_user(user_id, status=status)
    return tasks

@app.command("task_update")
def task_update(task_id: int, **fields):
    """Update task"""
    success = models.Task.update(task_id, **fields)
    return {"updated": success}

@app.command("task_delete")
def task_delete(task_id: int):
    """Delete task"""
    success = models.Task.delete(task_id)
    return {"deleted": success}

@app.command("user_create")
def user_create(username: str, email: str, password: str):
    """Create user with password hashing"""
    import hashlib
    
    # Never store plain passwords!
    password_hash = hashlib.pbkdf2_hmac(
        'sha256',
        password.encode(),
        b'salt',  # Use proper salt in production
        100000
    ).hex()
    
    try:
        user_id = models.User.create(username, email, password_hash)
        return {"id": user_id, "created": True}
    except Exception as e:
        return {"error": str(e), "created": False}

@app.command("user_get")
def user_get(user_id: int):
    """Fetch user (without password hash)"""
    user = models.User.get_by_id(user_id)
    if user:
        del user['password_hash']  # Never expose
        return user
    return None
```

## Caching

### In-Memory Cache with TTL

```python
# src/backend/cache.py
import time
from typing import Any, Optional, Callable

class TTLCache:
    """Simple in-memory cache with time-to-live"""
    
    def __init__(self):
        self._cache = {}
    
    def set(self, key: str, value: Any, ttl_seconds: int = 300):
        """Store value with TTL"""
        self._cache[key] = {
            'value': value,
            'expires_at': time.time() + ttl_seconds
        }
    
    def get(self, key: str) -> Optional[Any]:
        """Retrieve value if not expired"""
        if key not in self._cache:
            return None
        
        entry = self._cache[key]
        if entry['expires_at'] < time.time():
            del self._cache[key]
            return None
        
        return entry['value']
    
    def clear(self):
        """Clear all cache"""
        self._cache.clear()

# Usage
cache = TTLCache()

@app.command("get_expensive_data")
def get_expensive_data(resource_id: int):
    """Fetch data with caching"""
    cache_key = f"resource_{resource_id}"
    
    # Try cache first
    cached = cache.get(cache_key)
    if cached is not None:
        return {"data": cached, "cached": True}
    
    # Fetch from database
    data = fetch_from_database(resource_id)
    
    # Cache for 5 minutes
    cache.set(cache_key, data, ttl_seconds=300)
    
    return {"data": data, "cached": False}

@app.command("invalidate_cache")
def invalidate_cache(resource_id: int = None):
    """Clear cache"""
    if resource_id:
        cache.get(f"resource_{resource_id}")  # Trigger deletion
    else:
        cache.clear()
    return {"cleared": True}
```

## Session Management

```python
# src/backend/sessions.py
import uuid
import time
from typing import Optional

class SessionManager:
    """Manage user sessions"""
    
    def __init__(self, ttl_seconds: int = 86400):  # 24 hours
        self._sessions = {}
        self.ttl = ttl_seconds
    
    def create(self, user_id: int) -> str:
        """Create new session"""
        session_id = str(uuid.uuid4())
        self._sessions[session_id] = {
            'user_id': user_id,
            'created_at': time.time(),
            'expires_at': time.time() + self.ttl,
            'last_active': time.time()
        }
        return session_id
    
    def validate(self, session_id: str) -> Optional[int]:
        """Validate session and return user_id if valid"""
        if session_id not in self._sessions:
            return None
        
        session = self._sessions[session_id]
        
        # Check expiration
        if session['expires_at'] < time.time():
            del self._sessions[session_id]
            return None
        
        # Update last active
        session['last_active'] = time.time()
        return session['user_id']
    
    def destroy(self, session_id: str):
        """End session"""
        if session_id in self._sessions:
            del self._sessions[session_id]

sessions = SessionManager()

@app.command("login")
def login(email: str, password: str) -> dict:
    """Authenticate and create session"""
    user = models.User.get_by_email(email)
    if not user or not verify_password(password, user['password_hash']):
        return {"success": False, "error": "Invalid credentials"}
    
    session_id = sessions.create(user['id'])
    return {
        "success": True,
        "session_id": session_id,
        "user_id": user['id']
    }

@app.command("get_current_user")
def get_current_user(session_id: str) -> Optional[dict]:
    """Get current user from session"""
    user_id = sessions.validate(session_id)
    if not user_id:
        return None
    
    user = models.User.get_by_id(user_id)
    if user:
        del user['password_hash']
    return user

@app.command("logout")
def logout(session_id: str) -> bool:
    """End session"""
    sessions.destroy(session_id)
    return True
```

## Data Export & Import

```python
import json

@app.command("export_data")
def export_data(user_id: int, format: str = "json") -> dict:
    """Export all user data"""
    user = models.User.get_by_id(user_id)
    tasks = models.Task.get_by_user(user_id)
    
    data = {
        "user": user,
        "tasks": tasks,
        "exported_at": datetime.now().isoformat()
    }
    
    if format == "json":
        return data
    
    return {"error": "Unsupported format"}

@app.command("import_data")
def import_data(user_id: int, data: dict) -> bool:
    """Import user data from export"""
    try:
        for task in data.get("tasks", []):
            models.Task.create(
                user_id,
                task["title"],
                task.get("description", "")
            )
        return {"success": True}
    except Exception as e:
        return {"success": False, "error": str(e)}
```

