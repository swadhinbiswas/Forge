# ForgeDesk

Build small, fast, secure desktop applications with Python + native system WebViews.

![ForgeDesk Logo](branding/forgedesk-logo.svg)

![Version](https://img.shields.io/badge/version-2.0.2-blue)
![Python](https://img.shields.io/badge/python-3.10%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)

## Why ForgeDesk

- Native system WebView runtime (no bundled Chromium)
- Python-first backend with JavaScript bridge (`@forgedesk/api`)
- CLI-first developer workflow (`forge create`, `forge dev`, `forge build`)
- Security-focused runtime model with scoped capabilities
- Works with modern frontends (React, Vue, Svelte, Astro, and more)

## Brand Philosophy

ForgeDesk is built around **crafted power with discipline**.

- **Forge** means strength, precision, and reliable construction.
- **Desk** means practical developer workflow and day-to-day shipping velocity.

The identity reflects this: a forged gradient frame for trust and structure, with a spark core representing momentum and creation.

For complete usage rules, see [Brand Guide](branding/BRAND_GUIDE.md).

## Installation

### Python runtime

```bash
pip install forgedesk
```

### Node ecosystem (optional)

```bash
npm install @forgedesk/api
npm install -D @forgedesk/cli @forgedesk/vite-plugin vite
npx @forgedesk/create-forge-app@latest my-app
```

## Quick Start

```bash
forge create my-app --template plain
cd my-app
forge dev
```

Build release artifacts:

```bash
forge build
```

## Minimal Example

### Python (`src/main.py`)

```python
from forge import ForgeApp

app = ForgeApp()

@app.command
def greet(name: str) -> str:
    return f"Hello, {name}!"

app.run()
```

### Frontend

```ts
import { invoke } from "@forgedesk/api";

const msg = await invoke("greet", { name: "World" });
console.log(msg);
```

## Core CLI Commands

| Command | Purpose |
|---|---|
| `forge create <name>` | Scaffold a new project |
| `forge dev` | Run app in development mode with hot reload |
| `forge build` | Build production artifacts |
| `forge doctor` | Validate environment and project setup |
| `forge package` | Emit package/install metadata |
| `forge sign` | Run signing and verification workflow |
| `forge release` | Build + generate release manifest |

## Documentation

- Docs source: [docs](docs)
- Installation guide: [docs/src/content/docs/install](docs/src/content/docs/install)
- API reference: [docs/src/content/docs/api-reference.md](docs/src/content/docs/api-reference.md)
- Blog/release notes: [docs/src/content/docs/blog](docs/src/content/docs/blog)
- Branding assets: [branding](branding)

## Packages

- Python (PyPI): `forgedesk`
- npm: `@forgedesk/api`
- npm: `@forgedesk/cli`
- npm: `@forgedesk/vite-plugin`
- npm: `@forgedesk/create-forge-app`

## Development

```bash
git clone https://github.com/swadhinbiswas/ForgeDesk.git
cd ForgeDesk/forge-framework
pip install -e ".[dev]"
pytest
```

## License

MIT — see [LICENSE](LICENSE).
