---
title: File System (FS)
description: Interact with the OS file system securely from your ForgeDesk frontend.
---

The **File System (`fs`)** API allows your frontend JavaScript to read, write, and manipulate files and directories on the host operating system. 

Because file system access is inherently dangerous, ForgeDesk uses a strict **Scope-Based Permission System** to ensure your app can only interact with explicitly authorized directories.

---

## ⚙️ Configuration & Security Scopes

Before you can use any file system API, you **must explicitly authorize** it and define its scope in your `forge.toml` project file.

Scopes restrict *where* the Chromium WebView is allowed to read or write files. You can use cross-platform Base Directory variables (like `$APPDATA`, `$DOCUMENTS`, `$DOWNLOAD`) so you don't have to worry about the differences between Windows, macOS, and Linux paths.

```toml
# forge.toml

[plugins.fs]
enabled = true

# Define explicitly which paths the UI is allowed to access
[[plugins.fs.scope]]
allow = ["$APPDATA/my-app-name/**", "$DOCUMENTS/MyGameSaves/*.json"]

# Optional: Explicitly deny specific sub-paths or file extensions
[[plugins.fs.scope]]
deny = ["$APPDATA/my-app-name/secrets.json", "**/*.exe"]
```

If the frontend attempts to read or write a file outside of these allowed scopes, the Python backend will block the request at the IPC level and immediately throw a `PermissionDeniedError`.

---

## 💻 Usage (Frontend API)

All core plugins are bundled directly into the central `@forgedesk/api` package, so there is no extra `npm install` needed! 

Below are the most common ways to interact with the file system directly from your React/Vue/Svelte components.

### Reading & Writing Files

```typescript
import { readTextFile, writeTextFile, BaseDir } from '@forgedesk/api/fs';

// Write a file to the AppData directory
await writeTextFile('my-app-name/config.json', JSON.stringify({ theme: 'dark' }), {
    dir: BaseDir.AppData
});

// Read the file back
const fileContent = await readTextFile('my-app-name/config.json', { 
    dir: BaseDir.AppData 
});
const config = JSON.parse(fileContent);
```

### Checking if a File or Directory Exists

```typescript
import { exists, BaseDir } from '@forgedesk/api/fs';

const isDatabaseInitialized = await exists('my-app-name/data.db', { dir: BaseDir.AppData });

if (!isDatabaseInitialized) {
    console.log("Database not found, running first-time setup...");
}
```

### Reading Directories (List Files)

```typescript
import { readDir, BaseDir } from '@forgedesk/api/fs';

// Returns an array of FileEntry objects
const entries = await readDir('MyGameSaves', { 
    dir: BaseDir.Documents,
    recursive: false // Set to true to read sub-folders
});

for (const entry of entries) {
    if (entry.children) {
        console.log(`Directory Found: ${entry.name}`);
    } else {
        console.log(`File Found: ${entry.name} at path (${entry.path})`);
    }
}
```

### Deleting Files

```typescript
import { removeFile, removeDir, BaseDir } from '@forgedesk/api/fs';

// Delete a single file
await removeFile('my-app-name/old-config.json', { dir: BaseDir.AppData });

// Delete an entire directory and all its contents
await removeDir('MyGameSaves/CorruptedSave', { dir: BaseDir.Documents, recursive: true });
```

---

## 🧑‍💻 File System on the Backend (Python)

It is important to remember that these constraints (like scoping and `# BaseDir`) exist strictly to protect the user from malicious **Javascript** attacks (e.g., Cross-Site Scripting reading your SSH keys).

If you are writing pure Python code on the backend (e.g. in `app.py` or inside your own custom plugins), you are natively executing on the OS and the IPC bridge limits do not apply to you. You can simply use Python's built-in `os.path`, `pathlib`, or `shutil` libraries as usual without interacting with this frontend package. 
