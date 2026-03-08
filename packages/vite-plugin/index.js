export function forgeVitePlugin(options = {}) {
  const runtimeScriptPath = options.runtimeScriptPath || "/forge.js";
  const devServerUrl = options.devServerUrl || process.env.FORGE_DEV_SERVER_URL || null;

  return {
    name: "forge-vite-plugin",
    config() {
      return {
        define: {
          __FORGE_DEV_SERVER_URL__: JSON.stringify(devServerUrl),
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
