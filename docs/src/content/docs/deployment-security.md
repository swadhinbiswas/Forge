---
title: "Deployment, Security & Advanced Topics"
description: "Building for production: deployment strategies, security best practices, and advanced techniques"
---

## Production Builds

### Building Your Application

```bash
# Development (with debug tools)
forge dev

# Production build (optimized)
forge build --release

# Build for specific platform
forge build --release --platform linux
forge build --release --platform macos
forge build --release --platform windows

# Generate installers
forge build --release --installer
```

### Build Configuration

```toml
# forge.toml - Production settings

[app]
name = "My App"
version = "1.0.0"

[window]
title = "My App"
width = 1280
height = 720

[build]
entry = "src/main.py"
output_dir = "dist"
icon = "assets/icon.png"
single_binary = true

[dev]
frontend_dir = "src/frontend"

# Production security settings
[security]
strict_mode = true
allowed_origins = ["forge://app"]
denied_commands = ["os.system", "subprocess.call"]

[permissions]
clipboard = true
dialogs = true
notifications = true
system_tray = false

[permissions.filesystem]
# Explicitly whitelist and blacklist precise paths (Scopes)
allow = ["$APPDATA/my-app-name/**", "$DOCUMENTS/MyGameSaves/*.json"]
deny = ["$APPDATA/my-app-name/secrets.json"]

[permissions.shell]
execute = true
allow = ["python", "node"]
```

## Security Best Practices

### 1. Granular Scopes for Hardware and Filesystem

Instead of simple boolean flags that grant sweeping access, Forge allows and encourages granular **Scopes** via the `forge.toml` file.

**Filesystem Scoping:** Use cross-platform variables like `$APPDATA`, `$DOCUMENTS`, or `$DOWNLOAD` to define exactly where your application is allowed to read and write. If a compromised frontend attempts to read `~/.ssh/id_rsa`, it is instantly blocked by the Python/Rust IPC native bridge before any IO executes.

```toml
[permissions.filesystem]
allow = ["$APPDATA/My Secure App/**"]
deny = ["**/*.exe", "**/*.sh"]
```

**Custom Protocol Validation:** These scopes are heavily integrated into the `forge-asset://` zero-copy protocol. It validates all streaming file fetches against these exact glob patterns natively in Rust by calling back into Python's `ScopeValidator` via `PyO3`.

### 2. Input Validation

Always validate input on the backend:

```python
from pydantic import BaseModel, Field, validator

class UserInput(BaseModel):
    """Validate and sanitize user input"""
    username: str = Field(..., min_length=3, max_length=32)
    email: str = Field(..., regex=r"^[\w\.-]+@[\w\.-]+\.\w+$")
    age: int = Field(..., ge=0, le=150)
    
    @validator('username')
    def username_alphanumeric(cls, v):
        if not v.replace('_', '').isalnum():
            raise ValueError('Username must be alphanumeric')
        return v

@app.command("create_user")
def create_user(data: UserInput) -> dict:
    """Input is automatically validated"""
    # data is guaranteed to be valid
    user = models.User.create(data.username, data.email)
    return {"id": user.id}
```

### 2. Secure Storage

**Never store sensitive data in code:**

```python
import os
from pathlib import Path

# Load from environment or secure config
API_KEY = os.getenv('API_KEY')
DB_PATH = Path.home() / ".app" / "data.db"

# Encrypt sensitive data
from cryptography.fernet import Fernet

CIPHER = Fernet(os.getenv('ENCRYPTION_KEY').encode())

def encrypt_token(token: str) -> str:
    return CIPHER.encrypt(token.encode()).decode()

def decrypt_token(encrypted: str) -> str:
    return CIPHER.decrypt(encrypted.encode()).decode()

@app.command("store_token")
def store_token(token: str):
    """Securely store API token"""
    encrypted = encrypt_token(token)
    app.state.set("api_token", encrypted)
    return True
```

### 3. SQL Injection Prevention

**Always use parameterized queries:**

```python
# ❌ WRONG - Vulnerable to SQL injection
def bad_query(username: str):
    return db.query(f"SELECT * FROM users WHERE username = '{username}'")

# ✅ RIGHT - Safe parameterized query
def safe_query(username: str):
    return db.query("SELECT * FROM users WHERE username = ?", (username,))
```

### 4. Authentication & Authorization

```python
import hashlib
import hmac
from datetime import datetime, timedelta

def hash_password(password: str) -> str:
    """Securely hash password"""
    salt = os.urandom(32)
    key = hashlib.pbkdf2_hmac('sha256', password.encode(), salt, 100000)
    return (salt + key).hex()

def verify_password(password: str, hash_hex: str) -> bool:
    """Verify password against hash"""
    salt = bytes.fromhex(hash_hex[:64])
    stored = bytes.fromhex(hash_hex[64:])
    key = hashlib.pbkdf2_hmac('sha256', password.encode(), salt, 100000)
    return hmac.compare_digest(key, stored)

@app.command("register")
def register(username: str, password: str, email: str) -> dict:
    """Register user with hashed password"""
    if len(password) < 12:
        return {"error": "Password must be at least 12 characters"}
    
    try:
        hashed = hash_password(password)
        user_id = models.User.create(username, email, hashed)
        return {"success": True, "id": user_id}
    except Exception as e:
        return {"error": "Registration failed"}

@app.command("login")
def login(email: str, password: str) -> dict:
    """Authenticate user"""
    user = models.User.get_by_email(email)
    
    if not user or not verify_password(password, user['password_hash']):
        # Don't reveal whether email exists
        return {"error": "Invalid email or password"}
    
    session_id = sessions.create(user['id'])
    return {"success": True, "session_id": session_id}
```

### 5. Rate Limiting

```python
import time
from collections import defaultdict

class RateLimiter:
    """Simple rate limiter"""
    
    def __init__(self, max_requests: int = 100, window_seconds: int = 60):
        self.max_requests = max_requests
        self.window = window_seconds
        self.requests = defaultdict(list)
    
    def is_allowed(self, key: str) -> bool:
        """Check if request is allowed"""
        now = time.time()
        cutoff = now - self.window
        
        # Remove old requests
        self.requests[key] = [
            t for t in self.requests[key] if t > cutoff
        ]
        
        # Check limit
        if len(self.requests[key]) >= self.max_requests:
            return False
        
        # Record request
        self.requests[key].append(now)
        return True

limiter = RateLimiter(max_requests=20, window_seconds=60)

@app.command("expensive_operation")
def expensive_operation(user_id: int) -> dict:
    """Rate-limited endpoint"""
    if not limiter.is_allowed(f"user_{user_id}"):
        return {"error": "Rate limit exceeded"}
    
    # Proceed with operation
    return {"result": perform_operation()}
```

## Logging & Monitoring

### Structured Logging

```python
import logging
import json
from datetime import datetime

# Configure structured logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(Path.home() / '.app' / 'app.log'),
        logging.StreamHandler()
    ]
)

logger = logging.getLogger(__name__)

@app.command("audit_action")
def audit_action(action: str, user_id: int, details: dict):
    """Log user actions for audit trail"""
    logger.info(json.dumps({
        "timestamp": datetime.now().isoformat(),
        "action": action,
        "user_id": user_id,
        "details": details
    }))
    return True
```

### Error Tracking

```python
import sys
import traceback

def handle_error(error: Exception, context: str = ""):
    """Log errors with full context"""
    logger.error(
        f"Error in {context}",
        exc_info=True,
        extra={
            "error_type": type(error).__name__,
            "traceback": traceback.format_exc()
        }
    )

@app.command("risky_operation")
def risky_operation():
    """Handle errors gracefully"""
    try:
        return perform_risky_operation()
    except ValueError as e:
        handle_error(e, "risky_operation")
        return {"error": "Invalid operation"}
    except Exception as e:
        handle_error(e, "risky_operation")
        return {"error": "Internal error"}
```

## Performance Optimization

### Background Threads

```python
import threading
from concurrent.futures import ThreadPoolExecutor

# Use thread pool for concurrent operations
executor = ThreadPoolExecutor(max_workers=4)

@app.command("batch_process")
def batch_process(items: List[dict]) -> dict:
    """Process items in parallel"""
    
    def process_item(item):
        return expensive_operation(item)
    
    # Submit tasks to thread pool
    futures = [executor.submit(process_item, item) for item in items]
    
    # Wait for completion
    results = [f.result() for f in futures]
    
    return {"processed": len(results), "results": results}
```

### Caching Expensive Operations

```python
from functools import lru_cache
import apikey

@lru_cache(maxsize=128)
def expensive_calculation(param1: str, param2: int) -> dict:
    """Cache expensive calculations"""
    return do_expensive_work(param1, param2)

@app.command("get_calculation")
def get_calculation(param1: str, param2: int):
    """Automatically cached"""
    return expensive_calculation(param1, param2)
```

### Lazy Loading

```python
# Don't import heavy libraries at startup
import importlib

def load_sklearn():
    """Lazy load sklearn when needed"""
    global sklearn
    if 'sklearn' not in globals():
        sklearn = importlib.import_module('sklearn')
    return sklearn

@app.command("ml_predict")
def ml_predict(features: List[float]):
    """Load ML library only when needed"""
    sklearn = load_sklearn()
    model = sklearn.joblib.load('model.pkl')
    prediction = model.predict([features])
    return {"prediction": prediction[0]}
```

## Distribution

### Signing Your App

```toml
[build]
signature_key = "ed25519_private_key"  # Keep secret!
```

### Auto Updates

```toml
[updater]
enabled = true
endpoints = [
  "https://releases.example.com/updates",
  "https://backup.example.com/updates"
]
check_on_startup = true
interval_hours = 24
```

### Packaging

**macOS:**
```bash
forge build --release --installer --platform macos
# Produces: .dmg file with code signing
```

**Windows:**
```bash
forge build --release --installer --platform windows
# Produces: .exe installer
```

**Linux:**
```bash
forge build --release --installer --platform linux
# Produces: .AppImage, .deb, .rpm
```

## Troubleshooting

### Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| Commands not found | `@app.command` not used or routes not registered | Verify decorator, call `register_routes(app)` |
| Type validation error | Type hints don't match JavaScript data | Use proper hints: `str`, `int`, `List[T]`, etc. |
| IPC timeout | Backend blocked | Don't block in commands, use threading |
| Frontend can't connect | Dev URL wrong | Check `forge.toml` dev settings |
| State not persisting | Trying to use memory state | Use database or SQLite |
| Slow performance | Blocking operations | Move to thread pool executor |

### Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug forge dev

# Enable IPC tracing
forge dev --inspect
```

Monitor in DevTools:
- Python errors in Console
- Network/WebSocket in Network tab
- Performance in Performance tab

### Profiling Python Code

```python
import cProfile
import pstats
from io import StringIO

def profile_command(func):
    """Decorator to profile command execution"""
    def wrapper(*args, **kwargs):
        pr = cProfile.Profile()
        pr.enable()
        
        result = func(*args, **kwargs)
        
        pr.disable()
        s = StringIO()
        ps = pstats.Stats(pr, stream=s).sort_stats('cumulative')
        ps.print_stats(10)  # Top 10 functions
        
        logger.info(f"Profile for {func.__name__}:\n{s.getvalue()}")
        return result
    
    return wrapper

@app.command("slow_operation")
@profile_command
def slow_operation():
    return expensive_calculation()
```

