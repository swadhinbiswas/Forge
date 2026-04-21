---
title: Window Manager
description: Control the native desktop window directly from your JavaScript frontend.
---

The **Window** API allows your frontend to send commands to the native OS window manager. This lets you build custom title bars, resize the window programmatically, minimize to the tray, or trigger fullscreen workflows.

---

## ⚙️ Configuration

Enable the window controls in your `forge.toml` file.

```toml
# forge.toml

[plugins.window]
enabled = true
```

---

## 💻 Usage (Frontend API)

All core window functionality is accessible directly through the `@forgedesk/api` package that is already installed in your project.

### Custom Title Bars

If you configure your app to be `frameless = true` in the `forge.toml` (removing the default OS minimize, maximize, and exact buttons), you can recreate those buttons in HTML and map them to these OS functions:

```typescript
import { 
    minimize, 
    maximize, 
    unmaximize, 
    close, 
    isMaximized 
} from '@forgedesk/api/window';

// Usage inside a React/Svelte button handler
async function handleMaximizeToggle() {
    const maxed = await isMaximized();
    if (maxed) {
        await unmaximize();
    } else {
        await maximize();
    }
}

// Close the entire Py/JS application natively
async function handleExit() {
    await close();
}
```

### Updating the Window Title Programmatically

You can dynamically update the window title, which shows up in the OS taskbar and task switcher.

```typescript
import { setTitle } from '@forgedesk/api/window';

await setTitle("My App - Unsaved Document *");
```

### Fullscreen and Visibility

```typescript
import { setFullscreen, hide, show } from '@forgedesk/api/window';

// Make the window go into true Fullscreen mode (great for games/kiosks)
await setFullscreen(true);

// Hide the window from the user (e.g., when minimized to the system tray)
await hide();

// Bring the window back and focus it
await show();
```
