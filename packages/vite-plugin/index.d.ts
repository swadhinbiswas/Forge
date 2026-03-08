export interface ForgeVitePluginOptions {
  runtimeScriptPath?: string;
  devServerUrl?: string | null;
}

export declare function forgeVitePlugin(options?: ForgeVitePluginOptions): {
  name: string;
  config(): {
    define: {
      __FORGE_DEV_SERVER_URL__: string;
    };
  };
  transformIndexHtml(html: string): string;
};

export default forgeVitePlugin;
