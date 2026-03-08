# @forge/vite-plugin

Vite integration for Forge frontends.

## Install

```bash
npm install -D vite @forge/vite-plugin
```

## Usage

```js
import { defineConfig } from "vite";
import { forgeVitePlugin } from "@forge/vite-plugin";

export default defineConfig({
  plugins: [forgeVitePlugin()],
});
```

The plugin injects `forge.js` into `index.html` and exposes `__FORGE_DEV_SERVER_URL__` at build time.
