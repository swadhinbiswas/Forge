# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Forge seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: security@forge-framework.dev (placeholder)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

After the initial reply to your report, we will send you a more detailed response indicating the next steps in handling your report. After the initial reply, we will keep you informed of the progress toward a fix and full announcement, and may ask for additional information or guidance.

## Security Best Practices

When building applications with Forge, follow these security best practices:

### 1. Command Name Validation

All commands exposed to JavaScript are validated:
- Must start with a letter or underscore
- Can only contain alphanumeric characters and underscores
- Maximum length of 100 characters

```python
@app.command  # Validates: greet, get_system_info, etc.
def greet(name: str) -> str:
    return f"Hello, {name}!"
```

### 2. File System Security

The File System API includes multiple security layers:

- **Path Traversal Prevention**: All paths are resolved and validated against allowed directories
- **Null Byte Rejection**: Paths containing null bytes are rejected
- **Symlink Resolution**: Symlinks are resolved and validated
- **Size Limits**: File reads have default size limits to prevent DoS

```python
# Secure file operations - paths are validated
from forge.api.fs import FileSystemAPI

fs = FileSystemAPI(base_path=app_dir)

# This will raise ValueError if path is outside allowed directories
content = fs.read("../../../etc/passwd")  # BLOCKED
```

### 3. Input Validation

All inputs from JavaScript are validated:

- JSON structure is validated before processing
- Argument types are checked
- Request size is limited (10MB default)
- Error messages are sanitized to prevent information leakage

### 4. XSS Prevention

Event data emitted from Python to JavaScript is properly escaped:

```python
# Safe - data is JSON-serialized and parsed safely in JS
app.emit("user_data", {"name": "<script>alert('xss')</script>"})

// JavaScript receives sanitized data
window.__forge__.on("user_data", (data) => {
    // data.name is safely parsed, not executed
});
```

### 5. Error Handling

Errors are handled securely:

- Stack traces are not exposed to JavaScript
- File paths are redacted from error messages
- Error message length is limited

### 6. Permission System

Forge includes a permission system in `forge.toml`:

```toml
[permissions]
filesystem = true    # Enable file system API
clipboard = true     # Enable clipboard API
dialogs = true       # Enable dialog API
system_tray = false  # Disable system tray API
```

Disable APIs you don't need when creating your ForgeApp:

```python
app = ForgeApp(
    enable_fs_api=False,
    enable_clipboard_api=True,
    enable_dialog_api=True,
    enable_system_api=True,
)
```

## Security Checklist for Forge Apps

When building a Forge application, review this checklist:

- [ ] Validate all user inputs in your commands
- [ ] Use parameterized queries if accessing databases
- [ ] Sanitize any HTML/JS content before displaying
- [ ] Limit file read sizes to prevent DoS
- [ ] Don't expose sensitive system commands via IPC
- [ ] Use HTTPS for any network requests
- [ ] Store secrets in environment variables, not code
- [ ] Keep dependencies updated
- [ ] Review and test file access permissions
- [ ] Implement rate limiting for commands if needed

## Known Limitations

1. **WebView Security**: Forge uses the system's native webview. Security depends on the OS webview implementation (WKWebView, WebView2, WebKitGTK).

2. **Local File Access**: By design, Forge apps can access local files. Always validate file paths and use the permission system.

3. **JavaScript Injection**: The `window.__forge__` API is injected into the webview. Ensure your frontend code doesn't inadvertently expose this to untrusted content.

## Security Updates

Security updates will be released as patch versions (e.g., 1.0.1, 1.0.2). Critical security fixes may be released outside the normal release cycle.

To stay updated:
- Watch the GitHub repository for security advisories
- Subscribe to the security mailing list
- Check the CHANGELOG.md for security-related updates

## Security Audit History

| Date | Version | Finding | Status |
|------|---------|---------|--------|
| 2024-01-01 | 1.0.0 | Initial security review | Complete |

## Third-Party Dependencies

Forge depends on the following third-party libraries. Review their security policies:

- [pywebview](https://pywebview.flowrl.com/) - WebView binding
- [typer](https://typer.tiangolo.com/) - CLI framework
- [rich](https://rich.readthedocs.io/) - Terminal formatting

## Contact

For security-related questions not covered by this policy, contact:
- Email: security@forge-framework.dev (placeholder)
- GitHub: @forge-framework (placeholder)
