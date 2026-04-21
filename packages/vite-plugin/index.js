export function forgeVitePlugin(options = {}) {
  const runtimeScriptPath = options.runtimeScriptPath || "/forge.js";
  const devServerUrl = options.devServerUrl || process.env.FORGE_DEV_SERVER_URL || null;
  const proxyUrl = options.proxyUrl !== undefined ? options.proxyUrl : (process.env.FORGE_ASGI_PROXY_URL || "http://127.0.0.1:8000");
  const enableProxy = process.env.FORGE_WEB_MODE === "1" || process.env.FORGE_ASGI_PROXY_URL;
  const isDesktop = !enableProxy; // Desktop mode: no ASGI server, JS runtime injected natively

  const serverConfig = enableProxy ? {
    proxy: {
      "/_forge/ipc": {
        target: proxyUrl ? proxyUrl.replace(/^http/, "ws") : "",
        ws: true,
      },
      [runtimeScriptPath]: {
        target: proxyUrl,
        changeOrigin: true,
      },
    },
  } : {};

  return {
    name: "forge-vite-plugin",
    config() {
      return {
        define: {
          __FORGE_DEV_SERVER_URL__: JSON.stringify(devServerUrl),
        },
        server: serverConfig,
      };
    },
    transformIndexHtml(html) {
      // In desktop mode, the Forge JS runtime is injected natively via
      // wry's with_initialization_script. No need for a <script> tag
      // that would 500 trying to fetch from a non-existent ASGI server.
      if (isDesktop) {
        return html;
      }

      // In web/ASGI mode, inject the runtime script tag
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
