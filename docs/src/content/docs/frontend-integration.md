---
title: "Frontend Framework Integration"
description: "Guides for integrating Forge with React, Vue, Svelte, and other JavaScript frameworks"
---

## Overview

Forge works with any JavaScript framework. This guide covers best practices for popular choices.

## React Integration

### Setup

```bash
forge create my-app --template react
cd my-app
npm install
```

### Create Hooks for IPC

```jsx
// src/frontend/hooks/useForge.js
import { useCallback, useState, useEffect } from 'react';
import { invoke, on } from '../forge-api.js';

/**
 * Hook to call a Python command
 * Manages loading, error, and result states
 */
export const useForgeCommand = (command, initialPayload = {}) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [data, setData] = useState(null);
  
  const execute = useCallback(async (payload = {}) => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke(command, { ...initialPayload, ...payload });
      setData(result);
      return result;
    } catch (err) {
      setError(err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [command, initialPayload]);
  
  return { execute, loading, error, data };
};

/**
 * Hook to subscribe to Forge events
 */
export const useForgeEvent = (eventName, callback) => {
  useEffect(() => {
    const unsubscribe = on(eventName, callback);
    return () => unsubscribe?.();
  }, [eventName, callback]);
};

/**
 * Hook for debounced command execution
 */
export const useForgeCommandDebounced = (command, delay = 500) => {
  const [timeout, setTimeout_] = useState(null);
  const { execute, loading, error, data } = useForgeCommand(command);
  
  const executeDebounced = useCallback((payload) => {
    if (timeout) clearTimeout(timeout);
    
    const newTimeout = setTimeout(() => {
      execute(payload);
    }, delay);
    
    setTimeout_(newTimeout);
  }, [timeout, delay, execute]);
  
  return { executeDebounced, loading, error, data };
};
```

### Example Components

```jsx
// src/frontend/pages/Dashboard.jsx
import { useEffect, useState } from 'react';
import { useForgeCommand, useForgeEvent } from '../hooks/useForge.js';
import { Card } from '../components/Card.jsx';
import { TaskList } from '../components/TaskList.jsx';

function Dashboard({ userId }) {
  const [tasks, setTasks] = useState([]);
  const { execute: fetchTasks, loading } = useForgeCommand('task_list');
  
  // Load tasks on mount
  useEffect(() => {
    fetchTasks({ user_id: userId });
  }, [userId]);
  
  // Listen for task updates from backend
  useForgeEvent('task_updated', (task) => {
    setTasks(prev => 
      prev.map(t => t.id === task.id ? task : t)
    );
  });
  
  // Handle task creation
  const handleCreateTask = async (title, description) => {
    const result = await fetchTasks({ 
      user_id: userId,
      filter: 'all'
    });
    setTasks(result);
  };
  
  return (
    <div className="dashboard">
      <h1>My Tasks</h1>
      
      {loading ? (
        <p>Loading...</p>
      ) : (
        <Card>
          <TaskList 
            tasks={tasks}
            userId={userId}
            onTaskChange={() => fetchTasks({ user_id: userId })}
          />
        </Card>
      )}
    </div>
  );
}

export default Dashboard;
```

## Vue Integration

### Composables for Forge

```javascript
// src/frontend/composables/useForge.js
import { ref, reactive, onMounted, onUnmounted } from 'vue';
import { invoke, on } from '../forge-api.js';

export function useForgeCommand(command, payload = {}) {
  const loading = ref(false);
  const error = ref(null);
  const data = ref(null);
  
  const execute = async (overrides = {}) => {
    loading.value = true;
    error.value = null;
    
    try {
      const result = await invoke(command, { ...payload, ...overrides });
      data.value = result;
      return result;
    } catch (err) {
      error.value = err;
      throw err;
    } finally {
      loading.value = false;
    }
  };
  
  return { execute, loading, error, data };
}

export function useForgeEvent(eventName, handler) {
  onMounted(() => {
    handler && on(eventName, handler);
  });
  
  onUnmounted(() => {
    // Cleanup if needed
  });
}
```

### Vue Component Example

```vue
<!-- src/frontend/components/TaskForm.vue -->
<template>
  <form @submit.prevent="handleSubmit" class="task-form">
    <input 
      v-model="title" 
      placeholder="Task title" 
      required
    />
    <textarea 
      v-model="description"
      placeholder="Description"
    ></textarea>
    
    <button 
      type="submit" 
      :disabled="isLoading"
    >
      {{ isLoading ? 'Creating...' : 'Create Task' }}
    </button>
    
    <p v-if="error" className="error">{{ error }}</p>
  </form>
</template>

<script setup>
import { ref } from 'vue';
import { useForgeCommand } from '../composables/useForge.js';

const title = ref('');
const description = ref('');

const { 
  execute: createTask, 
  loading: isLoading, 
  error 
} = useForgeCommand('task_create');

async function handleSubmit() {
  await createTask({
    title: title.value,
    description: description.value
  });
  
  if (!error.value) {
    title.value = '';
    description.value = '';
  }
}
</script>

<style scoped>
.task-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
```

## Svelte Integration

### Stores for Forge Commands

```javascript
// src/frontend/stores/forge.js
import { writable } from 'svelte/store';
import { invoke, on } from '../forge-api.js';

export function createForgeCommand(command) {
  const loading = writable(false);
  const error = writable(null);
  const data = writable(null);
  
  const execute = async (payload = {}) => {
    loading.set(true);
    error.set(null);
    
    try {
      const result = await invoke(command, payload);
      data.set(result);
      return result;
    } catch (err) {
      error.set(err);
      throw err;
    } finally {
      loading.set(false);
    }
  };
  
  return { loading, error, data, execute };
}

export function subscribeToForgeEvent(eventName, handler) {
  return on(eventName, handler);
}
```

### Svelte Component Example

```svelte
<!-- src/frontend/components/TaskInput.svelte -->
<script>
  import { createForgeCommand } from '../stores/forge.js';
  
  let title = '';
  let description = '';
  
  const { loading, error, data, execute } = createForgeCommand('task_create');
  
  async function handleSubmit() {
    await execute({ title, description });
    
    if (!$error) {
      title = '';
      description = '';
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <input 
    bind:value={title}
    placeholder="Task title" 
    required
  />
  <textarea 
    bind:value={description}
    placeholder="Description"
  ></textarea>
  
  <button type="submit" disabled={$loading}>
    {$loading ? 'Creating...' : 'Create Task'}
  </button>
  
  {#if $error}
    <p class="error">{$error}</p>
  {/if}
</form>

<style>
  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
```

## TypeScript Support

### Generate Types from Python

Forge can generate TypeScript types from your Python commands:

```bash
forge typegen --output src/frontend/types.ts
```

This generates:

```typescript
// src/frontend/types.ts
export interface TaskCreateRequest {
  title: string;
  description?: string;
  priority?: number;
}

export interface TaskCreateResponse {
  id: number;
  created: boolean;
}

export interface TaskListRequest {
  user_id: number;
  status?: string;
}

export interface Task {
  id: number;
  title: string;
  description: string;
  status: string;
  priority: number;
  created_at: string;
}

export type TaskListResponse = Task[];

// Commands
export const commands = {
  taskCreate: (req: TaskCreateRequest) => 
    invoke<TaskCreateResponse>('task_create', req),
  taskList: (req: TaskListRequest) => 
    invoke<TaskListResponse>('task_list', req),
};
```

Use in React:

```tsx
import { commands, type TaskCreateRequest } from '../types';

function CreateTask() {
  const [title, setTitle] = useState('');
  
  const handleCreate = async () => {
    const request: TaskCreateRequest = {
      title,
      description: 'My task'
    };
    
    const response = await commands.taskCreate(request);
    console.log(response.id); // TypeScript knows the type!
  };
  
  return <button onClick={handleCreate}>Create</button>;
}
```

## Framework-Specific Patterns

### React: Context for Global State

```jsx
// src/frontend/context/ForgeContext.jsx
import { createContext, useContext, useEffect, useState } from 'react';
import { invoke, on } from '../forge-api.js';

const ForgeContext = createContext();

export function ForgeProvider({ children }) {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    // Load user on mount
    loadUser();
    
    // Listen for auth events
    const unsubscribe = on('auth_changed', loadUser);
    return unsubscribe;
  }, []);
  
  const loadUser = async () => {
    try {
      const userData = await invoke('get_current_user');
      setUser(userData);
    } finally {
      setLoading(false);
    }
  };
  
  const login = async (email, password) => {
    const result = await invoke('login', { email, password });
    if (result.success) {
      await loadUser();
    }
    return result;
  };
  
  const logout = async () => {
    await invoke('logout');
    setUser(null);
  };
  
  return (
    <ForgeContext.Provider value={{ user, loading, login, logout }}>
      {children}
    </ForgeContext.Provider>
  );
}

export const useForge = () => {
  const context = useContext(ForgeContext);
  if (!context) {
    throw new Error('useForge must be used within ForgeProvider');
  }
  return context;
};
```

### Vue: Global Plugin

```javascript
// src/frontend/plugins/forge.js
import { invoke, on } from '../forge-api.js';

export default {
  install(app) {
    app.config.globalProperties.$forge = {
      invoke,
      on,
      async getCurrentUser() {
        return await invoke('get_current_user');
      }
    };
  }
};
```

Use in Vue components:

```vue
<template>
  <div>
    <p v-if="user">Welcome, {{ user.name }}</p>
    <button @click="logout">Logout</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useCurrentInstance } from 'vue';

const user = ref(null);
const instance = useCurrentInstance();
const $forge = instance.appContext.config.globalProperties.$forge;

onMounted(async () => {
  user.value = await $forge.getCurrentUser();
});

async function logout() {
  await $forge.invoke('logout');
  user.value = null;
}
</script>
```

