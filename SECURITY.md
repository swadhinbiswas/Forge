# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 3.0.x   | ✅ Active support  |
| 2.x.x   | ⚠️ Security fixes only |
| < 2.0   | ❌ End of life     |

## Reporting a Vulnerability

**⚠️ DO NOT create a public GitHub issue for security vulnerabilities.**

### How to Report

1. **Email:** security@forgedesk.eu.cc
2. **Subject:** `[SECURITY] Brief description`
3. **PGP Key:** Available at [keys.forgedesk.eu.cc/security.asc](https://keys.forgedesk.eu.cc/security.asc)

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Suggested fix (if any)

### Response Timeline

| Stage | Timeline |
|-------|----------|
| Initial acknowledgment | Within 24 hours |
| Triage and assessment | Within 72 hours |
| Fix development | Within 7 days (critical), 30 days (other) |
| Public disclosure | After fix is released |

### Disclosure Policy

- We follow **Coordinated Disclosure** principles
- We will credit reporters in release notes (unless anonymity is requested)
- We request a **90-day disclosure window** from initial report

## Security Features

ForgeDesk includes multiple security layers:

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
- **Scope Validation**: Glob-based allow/deny patterns

```python
# Secure file operations - paths are validated
from forge.scope import ScopeValidator

validator = ScopeValidator(
    allow_patterns=["/home/user/appdata/**"],
    deny_patterns=["/home/user/appdata/secrets/**"]
)

# This will be denied
validator.is_path_allowed("/etc/passwd")  # False
validator.is_path_allowed("/home/user/appdata/secrets/key.pem")  # False
```

### 3. IPC Security

All inputs from JavaScript are validated:

- JSON structure is validated before processing
- Argument types are checked
- Request size is limited (10MB default)
- Error messages are sanitized to prevent information leakage
- Command injection prevention

### 4. Capability System

```toml
[permissions]
filesystem = true   # Enable/disable API access
shell = false       # Deny shell execution
clipboard = true    # Allow clipboard access
keychain = false    # Deny secure storage
dialogs = true      # Allow native dialogs
notifications = true # Allow notifications
```

### 5. Update Security

- Ed25519 signature verification for all updates
- SHA-256 checksums for all release artifacts
- Delta update integrity checks
- Certificate pinning support

### 6. XSS Prevention

Event data emitted from Python to JavaScript is properly escaped:

```python
# Safe - data is JSON-serialized and parsed safely in JS
app.emit("user_data", {"name": "<script>alert('xss')</script>"})

// JavaScript receives sanitized data
window.__forge__.on("user_data", (data) => {
    // data.name is safely parsed, not executed
});
```

## Security Best Practices for Users

1. **Always enable scope validation** for file system access
2. **Disable shell access** unless absolutely necessary
3. **Use keychain** for storing secrets (never hardcode)
4. **Validate all IPC inputs** in command handlers
5. **Keep ForgeDesk updated** to receive security patches
6. **Review permissions** in forge.toml before deployment

## Security Checklist for Forge Apps

When building a Forge application, review this checklist:

- [ ] Validate all user inputs in your commands
- [ ] Use parameterized queries if accessing databases
- [ ] Sanitize any HTML/JS content before displaying
- [ ] Limit file read sizes to prevent DoS
- [ ] Don't expose sensitive system commands via IPC
- [ ] Use HTTPS for any network requests
- [ ] Store secrets in environment variables or keychain
- [ ] Keep dependencies updated
- [ ] Review and test file access permissions
- [ ] Implement rate limiting for commands if needed

## Known Limitations

1. **WebView Security**: ForgeDesk uses the system's native webview. Security depends on the OS webview implementation (WKWebView, WebView2, WebKitGTK).

2. **Local File Access**: By design, ForgeDesk apps can access local files. Always validate file paths and use the permission system.

3. **JavaScript Injection**: The `window.__forge__` API is injected into the webview. Ensure your frontend code doesn't inadvertently expose this to untrusted content.

## Security Updates

Security updates will be released as patch versions (e.g., 3.0.1, 3.0.2). Critical security fixes may be released outside the normal release cycle.

To stay updated:
- Watch the GitHub repository for security advisories
- Subscribe to the security mailing list
- Check the CHANGELOG.md for security-related updates

## Contact

- **Security Email:** security@forgedesk.eu.cc
- **General Contact:** hello@forgedesk.eu.cc
- **PGP Key:** [keys.forgedesk.eu.cc/security.asc](https://keys.forgedesk.eu.cc/security.asc)
