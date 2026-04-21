---
title: Building Custom Plugins
description: Learn how to author and publish your own native plugins for ForgeDesk.
---

Absolutely! Extending ForgeDesk with your own **Custom Plugins** is one of the most powerful features of the framework. If you need to access a specific C++ SDK, a Rust system library, or a Python package that isn't included out-of-the-box, you can easily wrap it in a plugin.

---

## 🧩 The Anatomy of a Plugin

A complete ForgeDesk plugin consists of two main parts:
1. **The Backend Handler (Python/Rust):** Executes the native OS code and registers an Inter-Process Communication (IPC) route.
2. **The Frontend API (JavaScript/TypeScript):** Provides a clean, typed wrapper around the `invoke` command so your frontend developers can interact with it easily.

---

## 🛠️ Step 1: Create the Backend Logic (Python)

Let's build a simple custom plugin called `sysinfo` that returns the CPU usage of the host machine utilizing the Python `psutil` library.

Create a new Python file `forgedesk_sysinfo.py`:

```python
import psutil
from forge import Plugin, router

class SysInfoPlugin(Plugin):
    name = "sysinfo"

    def on_app_start(self, app):
        # Setup code when Forge boots up
        print(f"SysInfo Plugin Initialized!")

    # Expose this function to the frontend Javascript
    @router.command(namespace="sysinfo")
    def get_cpu_load(self) -> float:
        """Returns the current CPU usage percentage."""
        return psutil.cpu_percent(interval=None)
```

In this file:
- We inherit from `forge.Plugin` to hook into lifecycle events (`on_app_start`).
- We use `@router.command(namespace="sysinfo")` to expose our native Python functionality over the JSON-RPC bridge.

---

## 🌐 Step 2: Create the Frontend API (TypeScript)

Now, we want to give our React/Astro UI a clean way to fetch this data without having to type out manual `invoke` string calls every time.

Create a new Javascript/Typescript file `index.ts`:

```typescript
import { invoke } from '@forgedesk/api';

/**
 * Gets the current CPU load from the native Forge backend.
 */
export async function getCpuLoad(): Promise<number> {
    // The invoke string matches the namespace + function name from Python
    return await invoke('sysinfo:get_cpu_load');
}
```

Now, your frontend developers can simply `import { getCpuLoad } from 'forgedesk-sysinfo';` and use it perfectly typed!

---

## 🔒 Step 3: Enforcing Permissions

Remember, ForgeDesk uses **Security by Design**. Even if your plugin is installed, it will be blocked by default unless the developer explicitly allows it in `forge.toml`. 

To make your plugin respect the `forge.toml` permissions, you can check the app state inside your python function:

```python
from forge.exceptions import PermissionDeniedError

@router.command(namespace="sysinfo")
def get_cpu_load(app) -> float:
    # Check if the user authorized this plugin in forge.toml
    if not app.config.permissions.get("sysinfo", {}).get("enabled", False):
        raise PermissionDeniedError("SysInfo plugin is not enabled in forge.toml")
        
    return psutil.cpu_percent(interval=None)
```

Now, developers using your plugin will need to add this to their `forge.toml`:

```toml
[plugins.sysinfo]
enabled = true
```

---

## 💡 Ideas for New Plugins

The community is always looking for new native integrations! Here are some great ideas for plugins you could build and publish:

*   **🖨️ Printer API:** A plugin to list local printers and send PDF/HTML files directly to the print spooler.
*   **🎧 Audio/Mic Manager:** A plugin to get raw microphone streams, system volume levels, or enumerate audio output devices native to the OS.
*   **🔋 Battery / Power:** Expose battery percentages and AC power state.
*   **🌐 Bluetooth Manager:** Access local Bluetooth devices, scan targets, and pair natively.
*   **🖱️ Global Hotkeys:** A plugin to capture keyboard shortcuts even when the Forge app is minimized or out of focus.

---

## Publishing

Once you've built your plugin, you can publish the Python portion to **PyPI** (`pip install forgedesk-sysinfo`) and the TypeScript portion to **NPM** (`npm install @forgedesk/plugin-sysinfo`). 

Share it with the community on our GitHub discussions!
