# @forge/cli

Node wrapper for the Forge Python CLI.

## Install

```bash
npm install -D @forge/cli
```

## Usage

```bash
npx forge create my-app --template react
npx forge dev
npx forge build
npx forge package --result-format json
npx forge sign --result-format json
```

If the Python Forge package is missing, the wrapper attempts to install `forge-framework` with `pip` unless `FORGE_SKIP_AUTO_INSTALL=1` is set.
