---
title: "Best Practices & Common Patterns"
description: "Forge best practices, design patterns, and real-world examples"
---

## Application Structure

### Recommended Project Layout

```
my-app/
├── src/
│   ├── main.py                    # Entry point
│   ├── backend/
│   │   ├── __init__.py
│   │   ├── api/
│   │   │   ├── __init__.py
│   │   │   ├── users.py          # User endpoints
│   │   │   ├── tasks.py          # Task endpoints
│   │   │   └── settings.py       # Settings endpoints
│   │   ├── models/
│   │   │   ├── __init__.py
│   │   │   ├── user.py
│   │   │   ├── task.py
│   │   │   └── base.py           # Base model class
│   │   ├── services/
│   │   │   ├── __init__.py
│   │   │   ├── auth.py
│   │   │   ├── email.py
│   │   │   └── storage.py
│   │   ├── db/
│   │   │   ├── __init__.py
│   │   │   ├── connection.py
│   │   │   ├── queries.py
│   │   │   └── migrations/
│   │   ├── middleware/
│   │   │   ├── __init__.py
│   │   │   └── auth.py
│   │   └── utils/
│   │       ├── __init__.py
│   │       ├── validators.py
│   │       └── helpers.py
│   └── frontend/
│       ├── index.html
│       ├── main.jsx
│       ├── forge-api.js
│       ├── hooks/
│       │   ├── useForge.js
│       │   └── useAuth.js
│       ├── utils/
│       │   └── api-client.js
│       ├── context/
│       │   └── AuthContext.jsx
│       ├── components/
│       │   ├── Header.jsx
│       │   ├── Sidebar.jsx
│       │   └── common/
│       ├── pages/
│       │   ├── Dashboard.jsx
│       │   ├── Login.jsx
│       │   └── Settings.jsx
│       └── styles/
│           └── index.css
├── assets/
│   ├── icon.png
│   ├── icon.icns
│   └── icon.ico
├── tests/
│   ├── test_api.py
│   └── test_models.py
├── forge.toml
├── package.json
├── requirements.txt
├── .env.example
└── README.md
```

## Code Organization Patterns

### Module Pattern: Organizing Related Commands

```python
# src/backend/api/users.py
"""User management endpoints"""
import logging
from typing import Optional, List
from .. import models, services

logger = logging.getLogger(__name__)

def register_routes(app):
    """Register all user-related routes"""
    
    @app.command("user_create")
    def create_user(username: str, email: str, password: str) -> dict:
        """Create new user"""
        try:
            user_id = models.User.create(username, email, password)
            logger.info(f"User created: {username}")
            return {"success": True, "id": user_id}
        except ValueError as e:
            logger.warning(f"User creation failed: {e}")
            return {"success": False, "error": str(e)}
    
    @app.command("user_get")
    def get_user(user_id: int) -> Optional[dict]:
        """Get user by ID"""
        return models.User.get_by_id(user_id)
    
    @app.command("user_list")
    def list_users(limit: int = 50, offset: int = 0) -> List[dict]:
        """List users with pagination"""
        return models.User.list(limit=limit, offset=offset)
    
    @app.command("user_update")
    def update_user(user_id: int, **fields) -> dict:
        """Update user fields"""
        try:
            models.User.update(user_id, **fields)
            return {"success": True}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    @app.command("user_delete")
    def delete_user(user_id: int) -> bool:
        """Delete user"""
        return models.User.delete(user_id)

# src/main.py
from backend.api import users, tasks, settings

app = ForgeApp()

users.register_routes(app)
tasks.register_routes(app)
settings.register_routes(app)
```

## Error Handling Patterns

### Consistent Error Response

```python
# src/backend/errors.py
class APIError(Exception):
    """Base API error"""
    def __init__(self, message: str, code: str = "ERROR", status: int = 400):
        self.message = message
        self.code = code
        self.status = status
        super().__init__(message)

class ValidationError(APIError):
    def __init__(self, message: str):
        super().__init__(message, "VALIDATION_ERROR", 400)

class NotFoundError(APIError):
    def __init__(self, resource: str):
        super().__init__(f"{resource} not found", "NOT_FOUND", 404)

class UnauthorizedError(APIError):
    def __init__(self):
        super().__init__("Unauthorized", "UNAUTHORIZED", 401)

# src/backend/middleware/errors.py
def error_handler(func):
    """Decorator for consistent error handling"""
    from functools import wraps
    
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except ValidationError as e:
            return {
                "success": False,
                "error": e.message,
                "code": e.code
            }
        except NotFoundError as e:
            return {
                "success": False,
                "error": e.message,
                "code": e.code
            }
        except Exception as e:
            logger.error(f"Unexpected error in {func.__name__}: {e}")
            return {
                "success": False,
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }
    
    return wrapper

# Usage
@app.command("create_user")
@error_handler
def create_user(username: str) -> dict:
    if len(username) < 3:
        raise ValidationError("Username must be at least 3 characters")
    
    user_id = models.User.create(username)
    return {"success": True, "id": user_id}
```

## Frontend Patterns

### Data Fetching Pattern

```javascript
// src/frontend/utils/api-client.js
class APIClient {
  constructor() {
    this.cache = new Map()
    this.pending = new Map()
  }
  
  async fetch(endpoint, options = {}) {
    const { cache = true, ttl = 300 } = options
    const key = `${endpoint}:${JSON.stringify(options)}`
    
    // Return cached result if available
    if (cache && this.cache.has(key)) {
      const { data, expiry } = this.cache.get(key)
      if (expiry > Date.now()) return data
      this.cache.delete(key)
    }
    
    // Return pending request if in progress
    if (this.pending.has(key)) {
      return this.pending.get(key)
    }
    
    // Fetch new data
    const request = invoke(endpoint, options)
    this.pending.set(key, request)
    
    try {
      const data = await request
      
      // Cache result
      if (cache) {
        this.cache.set(key, {
          data,
          expiry: Date.now() + (ttl * 1000)
        })
      }
      
      return data
    } finally {
      this.pending.delete(key)
    }
  }
  
  invalidate(endpoint) {
    // Remove all cached entries for endpoint
    for (const [key] of this.cache) {
      if (key.startsWith(`${endpoint}:`)) {
        this.cache.delete(key)
      }
    }
  }
}

export const apiClient = new APIClient()
```

### Form Handling Pattern

```jsx
// src/frontend/hooks/useForm.js
import { useState, useCallback } from 'react'

export function useForm(initialValues, onSubmit) {
  const [values, setValues] = useState(initialValues)
  const [errors, setErrors] = useState({})
  const [loading, setLoading] = useState(false)
  
  const handleChange = useCallback((e) => {
    const { name, value, type, checked } = e.target
    setValues(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }))
    // Clear error on change
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: null }))
    }
  }, [errors])
  
  const handleSubmit = useCallback(async (e) => {
    e.preventDefault()
    setLoading(true)
    
    try {
      await onSubmit(values)
    } catch (error) {
      if (error.field) {
        setErrors(prev => ({
          ...prev,
          [error.field]: error.message
        }))
      } else {
        setErrors({ form: error.message })
      }
    } finally {
      setLoading(false)
    }
  }, [values, onSubmit])
  
  const reset = useCallback(() => {
    setValues(initialValues)
    setErrors({})
  }, [initialValues])
  
  return {
    values,
    errors,
    loading,
    handleChange,
    handleSubmit,
    reset,
    setValues,
    setErrors
  }
}

// Usage
function LoginForm() {
  const form = useForm(
    { email: '', password: '' },
    async (values) => {
      const result = await invoke('login', values)
      if (!result.success) {
        throw { field: 'email', message: result.error }
      }
    }
  )
  
  return (
    <form onSubmit={form.handleSubmit}>
      <input
        name="email"
        value={form.values.email}
        onChange={form.handleChange}
      />
      {form.errors.email && <span>{form.errors.email}</span>}
      
      <button type="submit" disabled={form.loading}>
        Login
      </button>
    </form>
  )
}
```

## Testing Patterns

### Backend Testing

```python
# tests/test_api.py
import pytest
from backend.api import users
from backend import models

@pytest.fixture
def app():
    """Create test app"""
    from forge import ForgeApp
    app = ForgeApp()
    users.register_routes(app)
    return app

@pytest.fixture
def client(app):
    """Create API client"""
    return APITestClient(app)

def test_user_create(client):
    """Test user creation"""
    result = client.call('user_create', {
        'username': 'testuser',
        'email': 'test@example.com',
        'password': 'secure123'
    })
    assert result['success'] == True
    assert 'id' in result

def test_user_create_invalid(client):
    """Test validation"""
    result = client.call('user_create', {
        'username': 'ab',  # Too short
        'email': 'invalid',
        'password': 'short'
    })
    assert result['success'] == False
    assert 'error' in result
```

### Frontend Testing

```javascript
// __tests__/components/LoginForm.test.jsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { LoginForm } from '../../src/components/LoginForm'

// Mock forge-api
jest.mock('../../src/forge-api.js', () => ({
  invoke: jest.fn()
}))

test('renders login form', () => {
  render(<LoginForm />)
  expect(screen.getByPlaceholderText('Email')).toBeInTheDocument()
})

test('handles successful login', async () => {
  const { invoke } = require('../../src/forge-api.js')
  invoke.mockResolvedValue({ success: true })
  
  render(<LoginForm />)
  
  fireEvent.change(screen.getByPlaceholderText('Email'), {
    target: { value: 'test@example.com' }
  })
  
  fireEvent.click(screen.getByText('Login'))
  
  await waitFor(() => {
    expect(invoke).toHaveBeenCalledWith('login', expect.any(Object))
  })
})
```

## Performance Optimization

### Batch Updates

```python
# Collect updates and emit once per interval
import time
from collections import defaultdict

class BatchEmitter:
    def __init__(self, app, interval=1.0):
        self.app = app
        self.interval = interval
        self.batches = defaultdict(list)
        self.last_emit = defaultdict(int)
    
    def emit(self, event, data):
        """Queue data for batched emit"""
        self.batches[event].append(data)
        
        now = time.time()
        if now - self.last_emit[event] >= self.interval:
            self.app.emit(event, self.batches[event])
            self.batches[event] = []
            self.last_emit[event] = now

# Usage
batcher = BatchEmitter(app)

def on_data_change(data):
    batcher.emit('data_update', data)  # Batched updates
```

### Pagination Pattern

```python
# Backend
@app.command("list_items")
def list_items(page: int = 1, page_size: int = 20) -> dict:
    """Paginated listing"""
    total = models.Item.count()
    offset = (page - 1) * page_size
    
    items = models.Item.list(limit=page_size, offset=offset)
    
    return {
        "items": items,
        "page": page,
        "page_size": page_size,
        "total": total,
        "pages": (total + page_size - 1) // page_size
    }

# Frontend
function ItemList() {
  const [page, setPage] = useState(1)
  const [items, setItems] = useState([])
  const { execute: fetchItems, loading } = useForgeCommand('list_items')
  
  useEffect(() => {
    fetchItems({ page, page_size: 20 })
      .then(result => setItems(result.items))
  }, [page])
  
  return (
    <>
      {items.map(item => <Item key={item.id} item={item} />)}
      <button onClick={() => setPage(p => p + 1)}>Next</button>
    </>
  )
}
```

## Real-World Examples

### Todo Application

```python
# Backend - Clean separation
class TodoService:
    @staticmethod
    def create(title: str, description: str = "") -> int:
        """Create todo"""
        todo_id = models.Todo.create(title, description)
        app.emit("todo_created", {"id": todo_id, "title": title})
        return todo_id
    
    @staticmethod
    def complete(todo_id: int) -> bool:
        """Mark todo as complete"""
        success = models.Todo.update(todo_id, status="completed")
        if success:
            app.emit("todo_completed", {"id": todo_id})
        return success

# Register commands
@app.command("todo_create")
def create_todo(title: str):
    return {"id": TodoService.create(title)}

@app.command("todo_complete")
def complete_todo(todo_id: int):
    return {"success": TodoService.complete(todo_id)}
```

## Debugging Tips

1. **Enable verbose logging:**
   ```bash
   RUST_LOG=debug forge dev
   ```

2. **Inspect IPC traffic:**
   ```bash
   forge dev --inspect
   ```

3. **Profile Python performance:**
   ```python
   import cProfile
   cProfile.run('expensive_function()', 'profile_stats')
   ```

4. **Check React rendering:**
   ```javascript
   // Add to React components
   useEffect(() => {
     console.log('Component rendered/updated')
   })
   ```

5. **Monitor state changes:**
   ```python
   @app.on_ready()
   def debug_state():
     import logging
     logger = logging.getLogger('forge.state')
     logger.setLevel(logging.DEBUG)
   ```

