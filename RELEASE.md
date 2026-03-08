# Release Publishing Guide

## Python package (PyPI)

Forge publishes the Python runtime as `forge-framework`.

### Requirements
- Python 3.14
- Rust toolchain
- PyPI Trusted Publishing configured for the repository
- Project name reserved on PyPI

### Manual local build

```bash
python -m pip install maturin build twine
python -m maturin build --release --out dist
python -m build --sdist --outdir dist
python -m twine check dist/*
```

### Automated publish
- Workflow: [.github/workflows/publish-python.yml](.github/workflows/publish-python.yml)
- Trigger: GitHub Release `published`
- Auth model: PyPI Trusted Publishing via GitHub OIDC

## Node packages (npm)

Forge publishes these npm packages:
- `@forge/api`
- `@forge/cli`
- `@forge/vite-plugin`
- `create-forge-app`

### Requirements
- Package names reserved on npm
- `NPM_TOKEN` configured in repository secrets
- Scoped packages published as public

### Manual local dry-run

```bash
cd packages/api && npm pack
cd ../cli && npm pack
cd ../vite-plugin && npm pack
cd ../create-forge-app && npm pack
```

### Automated publish
- Workflow: [.github/workflows/publish-npm.yml](.github/workflows/publish-npm.yml)
- Trigger: GitHub Release `published`
- Auth model: npm token + provenance

## Pre-publish checklist

- Run `cargo check`
- Run `pytest -q tests/test_cli.py tests/test_app.py tests/test_config.py`
- Run package/release validation workflows
- Confirm version numbers match across:
  - [pyproject.toml](pyproject.toml)
  - [forge_cli/__init__.py](forge_cli/__init__.py)
  - `packages/*/package.json`
- Confirm package names are available and ownership is configured
- Confirm PyPI and npm credentials/trusted publishing are configured

## Current publish readiness

### Ready now
- Python package build metadata exists in [pyproject.toml](pyproject.toml)
- npm package manifests exist under [packages](packages)
- automated publish workflows now exist

### Still required before first public publish
- replace placeholder project email/URLs if needed
- confirm final GitHub org/repo and package ownership
- reserve package names on PyPI/npm if not already reserved
- run one test release to TestPyPI and npm dry-run locally or in CI
