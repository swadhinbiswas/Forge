---
title: Keychain (Secrets)
description: Securely store API keys, tokens, and passwords using the native OS credential manager.
---

The **Keychain** provides a secure way to store and retrieve sensitive information like API tokens, OAuth credentials, and passwords. 

Instead of saving secrets in plain text inside a local database or localStorage (which is incredibly insecure), ForgeDesk uses the deeply integrated credential stores of the operating system:
*   **Windows:** Windows Credential Manager
*   **macOS:** Apple Keychain
*   **Linux:** Secret Service API (e.g., GNOME Keyring / KDE KWallet)

---

## ⚙️ Configuration

Before using the keychain API, you must explicitly enable it in your `forge.toml` file.

```toml
# forge.toml

[plugins.keychain]
enabled = true
```

Unlike the File System plugin, the Keychain does not require specific "scopes," as the OS manages process isolation for the stored secrets inherently.

---

## 💻 Usage (Frontend API)

All core features are bundled directly into the central `@forgedesk/api` package! Use the following methods to manage credentials securely from your React/Vue/Svelte components.

### Saving a Secret

To store a password, you need to provide a `service` name (usually your app's name or the API you are authenticating with) and an `account` (the username or identifier).

```typescript
import { setPassword } from '@forgedesk/api/keychain';

async function login(username, token) {
    // Save the authentication token natively
    await setPassword('MyForgeApp', username, token);
    console.log("Token securely saved!");
}
```

### Retrieving a Secret

When your app restarts, you can seamlessly retrieve the saved token to bypass the login screen.

```typescript
import { getPassword } from '@forgedesk/api/keychain';

async function checkLogin() {
    try {
        const token = await getPassword('MyForgeApp', 'user@example.com');
        if (token) {
            console.log("Automatically logged in!");
        }
    } catch (e) {
        console.log("No token found. User needs to log in.");
    }
}
```

### Deleting a Secret

When a user logs out, make sure to delete their credentials from the OS vault.

```typescript
import { deletePassword } from '@forgedesk/api/keychain';

async function logout() {
    await deletePassword('MyForgeApp', 'user@example.com');
    console.log("Credentials wiped securely from the OS.");
}
```

---

## 🧑‍💻 Usage on the Backend (Python)

If you need to access or modify these exact same secrets from your Python backend (for example, if a background thread needs the API key to sync data), you can use your typical native Python libraries like `keyring`.
