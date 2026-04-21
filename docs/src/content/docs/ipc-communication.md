---
title: "IPC Communication & Commands"
description: "Complete guide to Inter-Process Communication between Python and JavaScript"
---

## Understanding IPC (Inter-Process Communication)

**IPC** is the bridge that lets your Python backend and JavaScript frontend talk to each other safely and efficiently. Forge uses:

- **Messages**: Serialized JSON passing between processes
- **Validation**: Pydantic type checking before execution
- **Threading**: Async execution without blocking the UI
- **Events**: One-way broadcasts from Python → JavaScript

## Commands: Python → JavaScript Communication

Commands are how your frontend **requests data or performs actions** on the backend.

### Basic Command

```python
# Backend: src/main.py
from forge import ForgeApp

app = ForgeApp()

@app.command("greet")
def greet(name: str) -> str:
    """Simple greeting command"""
    return f"Hello, {name}! 🐍"

app.run()
```

```javascript
// Frontend: src/main.jsx
import { invoke } from '../forge-api.js';

async function sayHello() {
  const message = await invoke('greet', { name: 'Alice' });
  console.log(message); // "Hello, Alice! 🐍"
}
```

### Commands with Type Hints

Forge automatically validates arguments based on Python type hints:

```python
from typing import List, Optional
from pydantic import BaseModel

class CreateTaskRequest(BaseModel):
    title: str
    description: Optional[str] = None
    tags: List[str] = []
    priority: int = 1

@app.command("create_task")
def create_task(request: CreateTaskRequest) -> dict:
    """
    Create a task with automatic validation
    
    - Validates all fields match types
    - Ensures required fields present
    - Applies defaults
    - Rejects unexpected fields
    """
    task = save_to_db(request)
    return {"id": task.id, "created_at": task.created_at.isoformat()}
```

```javascript
// Frontend will have type safety if using TypeScript
const response = await invoke('create_task', {
  title: 'Buy groceries',
  description: 'Milk, bread, eggs',
  tags: ['shopping', 'urgent'],
  priority: 2
});
```

### Return Types

Commands can return various Python types, automatically serialized to JSON:

```python
@app.command("get_config") 
def get_config() -> dict:
    """Dict/list/string/number/bool"""
    return {"theme": "dark", "language": "en"}

@app.command("count_items")
def count_items() -> int:
    return 42

@app.command("get_names")
def get_names() -> List[str]:
    return ["Alice", "Bob", "Charlie"]

@app.command("is_premium")
def is_premium(user_id: int) -> bool:
    return check_subscription(user_id)

# Complex nested structures
@app.command("get_dashboard")
def get_dashboard() -> dict:
    return {
        "user": {
            "id": 1,
            "name": "Alice",
            "score": 95.5
        },
        "tasks": [
            {"id": 1, "title": "Task 1", "done": True},
            {"id": 2, "title": "Task 2", "done": False}
        ]
    }
```

## Events: Backend → Frontend Communication

Events let the **backend push updates** to the frontend without the frontend asking.

### Emitting Events from Python

```python
import threading
import time

@app.command("start_long_task")
def start_long_task():
    """Start a task that emits progress updates"""
    
    def worker():
        for i in range(10):
            time.sleep(1)
            
            # Emit progress event to all listening clients
            app.emit("progress", {
                "step": i,
                "total": 10,
                "percent": (i / 10) * 100
            })
        
        # Signal completion
        app.emit("task_complete", {"message": "Done!"})
    
    thread = threading.Thread(target=worker, daemon=True)
    thread.start()
    return {"started": True}
```

### Listening for Events in React

```jsx
import { useEffect, useState } from 'react';
import { on } from '../forge-api.js';

function ProgressTracker() {
  const [progress, setProgress] = useState(0);
  const [complete, setComplete] = useState(false);
  
  useEffect(() => {
    // Subscribe to progress updates
    const unsubProgress = on('progress', (data) => {
      console.log(`Progress: ${data.percent}%`);
      setProgress(data.percent);
    });
    
    // Subscribe to completion
    const unsubComplete = on('task_complete', (data) => {
      console.log(data.message);
      setComplete(true);
    });
    
    // Cleanup subscriptions
    return () => {
      unsubProgress();
      unsubComplete();
    };
  }, []);
  
  return (
    <div>
      <div className="progress-bar">
        <div style={{ width: `${progress}%` }}></div>
      </div>
      <p>{progress}%</p>
      {complete && <p className="success">Task complete!</p>}
    </div>
  );
}
```

### Real-World Example: Live Data Streaming

```python
# Backend: Continuously emit market data
import requests
import time

@app.on_ready()
def start_market_stream():
    """Begin streaming market prices"""
    
    def stream_prices():
        while True:
            try:
                # Fetch latest price data
                prices = requests.get(
                    'https://api.coingecko.com/api/v3/simple/price',
                    params={'ids': 'bitcoin,ethereum', 'vs_currencies': 'usd'}
                ).json()
                
                # Emit to all listening frontends
                app.emit('price_update', prices)
                
                time.sleep(5)  # Every 5 seconds
            except Exception as e:
                print(f"Stream error: {e}")
                time.sleep(10)  # Retry after 10 seconds
    
    thread = threading.Thread(target=stream_prices, daemon=True)
    thread.start()
```

```jsx
// Frontend: Real-time chart updates
import { LineChart, Line, XAxis, YAxis } from 'recharts';
import { useEffect, useState } from 'react';
import { on } from '../forge-api.js';

function PriceChart() {
  const [data, setData] = useState([]);
  
  useEffect(() => {
    const unsubscribe = on('price_update', (prices) => {
      const timestamp = new Date().toLocaleTimeString();
      setData(prev => [...prev, {
        time: timestamp,
        btc: prices.bitcoin.usd,
        eth: prices.ethereum.usd
      }].slice(-30)); // Keep last 30 data points
    });
    
    return unsubscribe;
  }, []);
  
  return (
    <LineChart data={data}>
      <XAxis dataKey="time" />
      <YAxis />
      <Line type="monotone" dataKey="btc" stroke="#f7931a" />
      <Line type="monotone" dataKey="eth" stroke="#627eea" />
    </LineChart>
  );
}
```

## Advanced Patterns

### 1. Request-Reply with State

```python
# Backend tracks individual requests
request_results = {}

@app.command("expensive_operation")
def expensive_operation(task_id: str, data: dict) -> dict:
    """Return immediately, compute in background, emit result"""
    
    def compute():
        try:
            result = process_data(data)
            app.emit("operation_result", {
                "task_id": task_id,
                "status": "success",
                "result": result
            })
        except Exception as e:
            app.emit("operation_result", {
                "task_id": task_id,
                "status": "error",
                "error": str(e)
            })
    
    thread = threading.Thread(target=compute, daemon=True)
    thread.start()
    
    return {"task_id": task_id, "status": "processing"}
```

```jsx
// Frontend waits for specific result
function ExpensiveOperation() {
  const [taskId, setTaskId] = useState(null);
  const [result, setResult] = useState(null);
  
  const startOperation = async () => {
    const id = Date.now().toString();
    setTaskId(id);
    
    // Set up listener before starting
    const unsubscribe = on('operation_result', (data) => {
      if (data.task_id === id) {
        setResult(data);
        unsubscribe();
      }
    });
    
    // Start the operation
    await invoke('expensive_operation', {
      task_id: id,
      data: { /* ... */ }
    });
  };
  
  return (
    <div>
      <button onClick={startOperation}>Start</button>
      {result && (
        result.status === 'success' ? (
          <p>Result: {JSON.stringify(result.result)}</p>
        ) : (
          <p className="error">Error: {result.error}</p>
        )
      )}
    </div>
  );
}
```

### 2. Error Handling

```python
# Backend: Consistent error responses
class CommandError(Exception):
    def __init__(self, message: str, code: str = "ERROR"):
        self.message = message
        self.code = code
        super().__init__(message)

@app.command("risky_operation")
def risky_operation(data: str) -> dict:
    try:
        if not data:
            raise CommandError("Data is required", "VALIDATION_ERROR")
        
        if len(data) > 1000:
            raise CommandError("Data too large", "SIZE_LIMIT")
        
        result = process(data)
        return {"success": True, "data": result}
    except CommandError as e:
        return {
            "success": False,
            "error": e.message,
            "code": e.code
        }
    except Exception as e:
        return {
            "success": False,
            "error": "Internal server error",
            "code": "INTERNAL_ERROR"
        }
```

```jsx
// Frontend: Centralized error handling
async function callWithErrorHandling(command, args) {
  try {
    const result = await invoke(command, args);
    
    if (!result.success) {
      throw new Error(result.error);
    }
    
    return result.data;
  } catch (error) {
    console.error(`Command ${command} failed:`, error);
    // Show user-friendly error message
    showErrorNotification(error.message);
    throw error;
  }
}
```

### 3. Streaming Large Data

For large responses, emit in chunks:

```python
@app.command("export_data")
def export_data(format: str = "json"):
    """Export all data as JSON, streaming to frontend"""
    
    def stream():
        total = count_records()
        
        for i, batch in enumerate(fetch_records_in_batches()):
            app.emit("export_progress", {
                "received": i * len(batch),
                "total": total,
                "percent": (i / (total / 100)) * 100
            })
            
            app.emit("export_data", {
                "batch": i,
                "records": batch
            })
    
    thread = threading.Thread(target=stream, daemon=True)
    thread.start()
    return {"started": True, "total": count_records()}
```

```jsx
// Frontend: Collect streamed data
function DataExporter() {
  const [data, setData] = useState([]);
  const [progress, setProgress] = useState(0);
  
  useEffect(() => {
    const unsubProgress = on('export_progress', (p) => {
      setProgress(p.percent);
    });
    
    const unsubData = on('export_data', (batch) => {
      setData(prev => [...prev, ...batch.records]);
    });
    
    return () => {
      unsubProgress();
      unsubData();
    };
  }, []);
  
  return (
    <div>
      <p>Progress: {progress.toFixed(1)}%</p>
      <p>Records: {data.length}</p>
    </div>
  );
}
```

## IPC Best Practices

### ✅ DO

- ✅ Use type hints for all parameters
- ✅ Return simple JSON-serializable types
- ✅ Keep commands focused and single-purpose
- ✅ Handle errors gracefully
- ✅ Use events for streaming/push updates
- ✅ Validate input on the backend
- ✅ Log important operations

### ❌ DON'T

- ❌ Pass file handles or non-serializable objects
- ❌ Create commands with 50+ parameters
- ❌ Block the Python thread (use threading for long ops)
- ❌ Emit events on every data change (batch updates)
- ❌ Trust frontend data validation alone
- ❌ Return raw Python objects
- ❌ Ignore errors silently

