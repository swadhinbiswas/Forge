export function forgeVitePlugin(options = {}) {
  const runtimeScriptPath = options.runtimeScriptPath || "/forge.js";
  const devServerUrl = options.devServerUrl || process.env.FORGE_DEV_SERVER_URL || null;
  const proxyUrl = options.proxyUrl || process.env.FORGE_ASGI_PROXY_URL || "http://127.0.0.1:8000";

  return {
    name: "forge-vite-plugin",
    config() {
      return {
        define: {
          __FORGE_DEV_SERVER_URL__: JSON.stringify(devServerUrl),
        },
        server: {
          proxy: {
            "/_forge/ipc": {
              target: proxyUrl.replace(/^http/, "ws"),
              ws: true,
            },
            [runtimeScriptPath]: {
              target: proxyUrl,
              changeOrigin: true,
            },
          },
        },
      };
    },
    transformIndexHtml(html) {
      if (html.includes(runtimeScriptPath)) {
        return html;
      }
      return html.replace(
        /<\/body>/i,
        `  <script src="${runtimeScriptPath}"></script>\n</body>`
      );
    },
  };
}

export default forgeVitePlugin;
