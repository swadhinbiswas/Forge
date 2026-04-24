# Contributing to ForgeDesk

Thank you for your interest in contributing to ForgeDesk! This guide will help you get started.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Code of Conduct](#code-of-conduct)

---

## Getting Started

### Prerequisites

- **Python 3.14+** (with NoGIL support)
- **Rust 1.75+** (via rustup)
- **Node.js 20+** (for frontend SDK)
- **pnpm 8+** (for NPM packages)
- **uv** (Python package manager)

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/forgedesk/forgedesk.git
cd forgedesk

# Install Python dependencies
uv pip install -e ".[dev]"

# Build Rust core
maturin develop

# Install Node dependencies
pnpm install

# Run tests
uv run pytest -v
cargo test --all-features
```

---

## Development Setup

### 1. Python Environment

```bash
# Create virtual environment
uv venv

# Activate
source .venv/bin/activate  # Linux/macOS
.venv\Scripts\activate     # Windows

# Install in development mode
uv pip install -e ".[dev]"
```

### 2. Rust Toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install components
rustup component add clippy rustfmt

# Build Rust core
maturin develop
```

### 3. Node.js Packages

```bash
# Install pnpm
npm install -g pnpm

# Install dependencies
pnpm install

# Build packages
pnpm run build --workspaces
```

### 4. System Dependencies (Linux)

```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    libxdo-dev \
    libssl-dev \
    libayatana-xdotool-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev
```

---

## Project Structure

```
forge-framework/
├── forge/              # Python framework core
│   ├── api/            # High-level API modules
│   ├── builtins/       # Built-in plugins
│   ├── js/             # JavaScript runtime
│   └── cli/            # CLI tools
├── src/                # Rust native core
│   ├── window/         # Window management
│   └── platform/       # Platform-specific code
├── packages/           # NPM packages
│   ├── api/            # TypeScript SDK
│   ├── vite-plugin/    # Vite integration
│   └── create-forge-app/  # Scaffolding tool
├── tests/              # Python test suite
├── docs/               # Documentation site
├── examples/           # Example applications
└── scripts/            # CI/CD scripts
```

### Key Files

| File | Purpose |
|------|---------|
| `forge/app.py` | Main ForgeApp class |
| `forge/bridge.py` | IPC bridge implementation |
| `forge/config.py` | Configuration parsing |
| `src/native_window.rs` | Native window management |
| `src/window/proxy.rs` | Thread-safe window handle |
| `packages/api/src/` | TypeScript SDK |

---

## Coding Standards

### Python

- **Style**: Follow PEP 8
- **Formatting**: Use `ruff format`
- **Linting**: Use `ruff check`
- **Type Hints**: Always use type hints
- **Docstrings**: Google-style docstrings

```python
def my_function(arg1: str, arg2: int = 0) -> dict[str, Any]:
    """Short description of the function.
    
    Longer description if needed.
    
    Args:
        arg1: Description of arg1.
        arg2: Description of arg2.
        
    Returns:
        Description of return value.
        
    Raises:
        ValueError: If arg1 is empty.
    """
    if not arg1:
        raise ValueError("arg1 cannot be empty")
    return {"result": arg1, "count": arg2}
```

### Rust

- **Style**: Follow Rust conventions
- **Formatting**: Use `cargo fmt`
- **Linting**: Use `cargo clippy`
- **Documentation**: Use `///` for public items

```rust
/// Process a command and return the result.
///
/// # Arguments
///
/// * `command` - The command name to execute.
/// * `args` - JSON-encoded arguments.
///
/// # Returns
///
/// JSON-encoded result or error.
pub fn process_command(command: &str, args: &str) -> Result<String, Error> {
    // Implementation
}
```

### TypeScript

- **Style**: Follow TypeScript conventions
- **Formatting**: Use Prettier
- **Linting**: Use ESLint
- **Types**: Always define return types

```typescript
/**
 * Invoke a command on the Python backend.
 * 
 * @param command - The command name.
 * @param args - Command arguments.
 * @returns Promise with the command result.
 */
async function invoke<T>(command: string, args: Record<string, unknown>): Promise<T> {
    // Implementation
}
```

---

## Testing

### Python Tests

```bash
# Run all tests
uv run pytest -v

# Run specific test file
uv run pytest tests/test_bridge.py -v

# Run with coverage
uv run pytest --cov=forge --cov-report=html

# Run specific test
uv run pytest tests/test_bridge.py::TestIPCBridge::test_valid_command -v
```

### Rust Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_window_descriptor

# Run with output
cargo test -- --nocapture
```

### Writing Tests

#### Python Test Example

```python
"""Tests for my feature."""

from __future__ import annotations

import pytest
from unittest.mock import MagicMock

from forge.bridge import IPCBridge


class TestMyFeature:
    """Tests for my feature."""

    @pytest.fixture
    def mock_app(self) -> MagicMock:
        """Create a mock app."""
        return MagicMock()

    @pytest.fixture
    def bridge(self, mock_app: MagicMock) -> IPCBridge:
        """Create a bridge instance."""
        return IPCBridge(mock_app, {})

    def test_valid_input(self, bridge: IPCBridge) -> None:
        """Test that valid input is accepted."""
        assert bridge._validate_command_name("greet") is True

    def test_invalid_input(self, bridge: IPCBridge) -> None:
        """Test that invalid input is rejected."""
        assert bridge._validate_command_name("") is False

    def test_edge_case(self, bridge: IPCBridge) -> None:
        """Test edge case handling."""
        # Test implementation
        pass
```

#### Rust Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_descriptor_defaults() {
        let json = r#"{"label": "test", "title": "Test", "width": 800, "height": 600}"#;
        let desc: WindowDescriptor = serde_json::from_str(json).unwrap();
        assert_eq!(desc.label, "test");
        assert!(desc.visible);
    }

    #[test]
    fn test_window_descriptor_missing_field() {
        let json = r#"{"label": "test"}"#;
        let result: Result<WindowDescriptor, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
```

### Test Coverage

We aim for:
- **Python**: >80% coverage
- **Rust**: >90% coverage
- **Critical paths**: 100% coverage (IPC, security, state)

---

## Pull Request Process

### 1. Branch Naming

```
feat/my-new-feature      # New feature
fix/my-bug-fix          # Bug fix
docs/my-docs-update     # Documentation
chore/my-maintenance    # Maintenance
```

### 2. Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new window state persistence
fix: resolve IPC timeout issue
docs: update API reference
chore: update dependencies
test: add E2E tests for multi-window
```

### 3. PR Checklist

Before submitting a PR:

- [ ] All tests pass (`uv run pytest -v`)
- [ ] Rust tests pass (`cargo test --all-features`)
- [ ] No linting errors (`uv run ruff check .`)
- [ ] Code formatted (`uv run ruff format .`)
- [ ] Type checking passes (`uv run mypy forge/`)
- [ ] Documentation updated
- [ ] Tests added for new features
- [ ] Breaking changes documented

### 4. PR Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

- [ ] Unit tests added/updated
- [ ] E2E tests added/updated
- [ ] Manual testing performed

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes
```

### 5. Review Process

1. **Automated checks** must pass
2. **At least one review** required
3. **Address all feedback** before merge
4. **Squash and merge** to main

---

## Issue Reporting

### Bug Reports

```markdown
## Bug Description

Clear description of the bug.

## Steps to Reproduce

1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior

What should happen.

## Actual Behavior

What actually happens.

## Environment

- OS: [e.g., Ubuntu 22.04]
- Python: [e.g., 3.14.0]
- ForgeDesk: [e.g., 3.0.0]

## Additional Context

Any other relevant information.
```

### Feature Requests

```markdown
## Feature Description

Clear description of the feature.

## Use Case

Why is this feature needed?

## Proposed Solution

How should this be implemented?

## Alternatives Considered

Other approaches considered.
```

---

## Code of Conduct

### Our Pledge

We are committed to making participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

**Positive behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community

**Unacceptable behavior:**
- Trolling, insulting/derogatory comments
- Public or private harassment
- Publishing others' private information
- Other unethical or unprofessional conduct

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the project team at conduct@forgedesk.eu.cc.

---

## Getting Help

- **Documentation**: [docs.forgedesk.eu.cc](https://docs.forgedesk.eu.cc)
- **Discord**: [discord.gg/forgedesk](https://discord.gg/forgedesk)
- **GitHub Discussions**: [github.com/forgedesk/forgedesk/discussions](https://github.com/forgedesk/forgedesk/discussions)
- **Email**: hello@forgedesk.eu.cc

---

## License

By contributing to ForgeDesk, you agree that your contributions will be licensed under the MIT License.
