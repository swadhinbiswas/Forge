---
title: Learn ForgeDesk
description: Tutorials, courses, and step-by-step instructions to master building apps with ForgeDesk.
---

Welcome to the **ForgeDesk Learning Hub**! Whether you are completely new to building desktop applications or you are a seasoned Electron veteran looking for a lighter, faster, Python-based alternative, you are in the right place.

Here you will find full-length tutorials and concepts arranged from absolute beginner to advanced.

---

## 🐣 Beginner Path

Get your feet wet by understanding the absolute basics of spinning up a native application.

*   **[1. Your First App](/learn/hello-world)**  
    Start here! Learn how to initialize a project, understand the folder structure, and boot up your first `Hello World` app without touching a single complex configuration file.
*   **[2. Python & JavaScript Talking (IPC)](/learn/basic-ipc)**  
    Learn how to trigger Python functions (`invoke`) from a button click in your React/Astro frontend, and how to return that data securely.
*   **[3. Window Controls](/learn/window-management)**  
    How to resize, move, maximize, and minimize the native operating system window from both Python and the Webview.

---

## 🚀 Intermediate Path

Now that you have a functioning app, learn how to build real-world user experiences and state management.

*   **[4. State Management in Python](/learn/state-management)**  
    Learn how to construct a robust backend store to hold user preferences and data across multiple app reloads securely.
*   **[5. Pushing Events to the Webview](/learn/push-events)**  
    Instead of just waiting for the frontend to ask for data, learn how Python can "push" data (like download progress bars) proactively to the UI using the `listen` API.
*   **[6. Multi-Window Applications](/learn/multi-window)**  
    Need a settings window? Find out how to spawn secondary windows, manage their lifecycles, and securely share data between them.

---

## 🧠 Advanced Topics

Ready to ship a professional-grade application or extend the framework?

*   **[7. Reading & Writing Files Securely](/learn/safe-fs)**  
    A hands-on project utilizing the `@forgedesk/fs` plugin to safely interact with a user's filesystem using explicit scope permissions (`forge.toml`).
*   **[8. Building Custom Rust/Python Plugins](/learn/custom-plugins)**  
    The deepest dive. Learn how to scaffold an entirely new ForgeDesk plugin from scratch to expose proprietary C/Rust logic to your frontend over IPC.
*   **[9. Compiling for Release (CI/CD)](/learn/packaging-release)**  
    How to turn your beautiful code into a standalone `.exe` (Windows), `.app`/`.dmg` (macOS), or `.AppImage` (Linux) with auto-updates using GitHub Actions.

---

## Need a Quick Reference?

If you're already familiar with these concepts and just need an API signature, jump over to the **[References](/references/)** section! 
If you want to read more about the underlying philosophy, read the **[Core Concepts](/core-concepts/)**.
