---
title: Security by Design
description: Understand how to build secure native desktop applications with ForgeDesk.
---

Welcome to the **ForgeDesk Security Guidelines**. Building a desktop application introduces unique security challenges compared to building a standard web app. Because a desktop application has direct access to a user's local operating system, security must be the primary consideration.

ForgeDesk was built with **Security by Design**, providing you with strong defaults to protect your users from malicious code execution, arbitrary file access, and broken access controls.

---

## 🛡️ Sandboxing & Context Isolation

Unlike older frameworks like Electron, ForgeDesk **does not** bundle a Node.js runtime into the frontend webview.

Your frontend code (HTML, CSS, JS, React, Astro) runs inside the operating system's native webview component (WebView2, WKWebView, WebKitGTK). This webview is completely sandboxed:
- It **cannot** execute arbitrary terminal commands.
- It **cannot** read or write files to the hard drive on its own.
- It **cannot** spawn unwanted background processes.

To accomplish native tasks, the frontend must explicitly pass JSON-RPC messages to the Python backend using the defined `@forgedesk/api` bridge.

---

## 🔒 Explicit Permissions (`forge.toml`)

One of the cornerstones of ForgeDesk security is the **Permission System**. 

By default, the frontend has **zero access** to native operating system APIs. To allow the frontend to interact with the file system, network, or deep system APIs, you must explicitly whitelist those capabilities inside your `forge.toml` configuration file.

### Example: Allowing File Read Access

```toml
# forge.toml

[permissions.fs]
read = true
write = false
scope = ["$APP_DATA", "$DOWNLOADS"]
```

In this example:
1. The frontend is allowed to **read** files via the `fs` plugin.
2. The frontend is **denied** the ability to write or delete files.
3. Operations are strictly isolated (`scope`) to the application's local data folder and the user's Downloads folder. Any attempt to read `/etc/passwd` or `C:\Windows\System32` from the frontend will be immediately rejected before hitting the Python backend.

---

## 🕸️ Content Security Policy (CSP)

A Content Security Policy protects your application from Cross-Site Scripting (XSS) attacks by dictating exactly where your application is allowed to load resources (like images, scripts, and fonts) from.

ForgeDesk makes it easy to configure your CSP in `forge.toml`:

```toml
[security]
csp = "default-src 'self'; img-src 'self' https://trusted-cdn.com; script-src 'self'"
```

**Best Practices:**
- Never use `'unsafe-eval'`.
- Never use `'unsafe-inline'` for scripts unless absolutely necessary during development.
- Serve all required UI assets locally whenever possible.

---

## 🚪 Safe External Navigation

If your UI contains links that navigate to external websites, you want to ensure they open in the user's **default web browser** (like Chrome or Firefox), rather than hijacking the application's internal webview.

ForgeDesk intercepts top-level `.href` changes and explicitly prevents navigating away from the local frontend server. 

When you use the `@forgedesk/api` to open links, it securely hands off the URL to the OS:

```javascript
import { shell } from '@forgedesk/api';

// Safely opens the trusted URL in Chrome/Safari/Firefox
await shell.openExternal('https://docs.forgedesk.com');
```

---

## 🔍 Input Validation and Type Generation

Your Python backend acts as the ultimate source of truth. Even though ForgeDesk secures the Inter-Process Communication (IPC) channel, you must still treat anything passed from the JavaScript frontend to Python as **untrusted user input**.

ForgeDesk helps with this by heavily utilizing **Pydantic** type hinting.

```python
from forge import router
from pydantic import BaseModel

class UserUpdate(BaseModel):
    user_id: int
    new_email: str

@router.command()
def update_user_email(payload: UserUpdate):
    # If the frontend passes a string instead of an int for user_id,
    # the router automatically rejects the request before it executes.
    pass
```

Because Forge uses strict type enforcement, malformed UI requests are instantly thwarted without crashing your backend.

---

## Next Steps

Security doesn't stop here. For practical implementations of these concepts:
- Review the **[Plugins](/plugins/)** section to see exactly how scoped plugin permissions are applied.
- Dive into **[API Reference](/references/)** for deep configurations.
