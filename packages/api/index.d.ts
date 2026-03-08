export interface InvokeDetailedOptions {
  detailed?: boolean;
  trace?: boolean;
}

export interface ForgeDialogApi {
  open(options?: Record<string, unknown>): Promise<unknown>;
  save(options?: Record<string, unknown>): Promise<unknown>;
  message(title: string, body: string, level?: string): Promise<unknown>;
  confirm(title: string, message: string, level?: string): Promise<unknown>;
  openDirectory(options?: Record<string, unknown>): Promise<unknown>;
}

export interface ForgeWindowApi {
  current(): Promise<unknown>;
  list(): Promise<unknown>;
  get(label: string): Promise<unknown>;
  create(options?: Record<string, unknown>): Promise<unknown>;
  closeLabel(label: string): Promise<unknown>;
  setTitle(title: string): Promise<unknown>;
  setPosition(x: number, y: number): Promise<unknown>;
  setSize(width: number, height: number): Promise<unknown>;
  setFullscreen(value: boolean): Promise<unknown>;
  setAlwaysOnTop(value: boolean): Promise<unknown>;
  position(): Promise<unknown>;
  state(): Promise<unknown>;
  isVisible(): Promise<boolean>;
  isFocused(): Promise<boolean>;
  isMinimized(): Promise<boolean>;
  isMaximized(): Promise<boolean>;
  show(): Promise<unknown>;
  hide(): Promise<unknown>;
  focus(): Promise<unknown>;
  minimize(): Promise<unknown>;
  unminimize(): Promise<unknown>;
  maximize(): Promise<unknown>;
  unmaximize(): Promise<unknown>;
  close(): Promise<unknown>;
}

export interface ForgeRuntimeApi {
  health(): Promise<unknown>;
  diagnostics(): Promise<unknown>;
  commands(): Promise<unknown>;
  protocol(): Promise<unknown>;
  logs(limit?: number): Promise<unknown>;
}

export interface ForgeApi {
  invoke(command: string, args?: Record<string, unknown>): Promise<unknown>;
  invokeDetailed(command: string, args?: Record<string, unknown>, options?: InvokeDetailedOptions): Promise<unknown>;
  on(eventName: string, handler: (payload: unknown) => void): unknown;
  once(eventName: string, handler: (payload: unknown) => void): unknown;
  off(eventName: string, handler: (payload: unknown) => void): unknown;
  fs: Record<string, (...args: any[]) => Promise<unknown>>;
  clipboard: Record<string, (...args: any[]) => Promise<unknown>>;
  dialog: ForgeDialogApi;
  runtime: ForgeRuntimeApi;
  window: ForgeWindowApi;
  notifications: Record<string, (...args: any[]) => Promise<unknown>>;
  updater: Record<string, (...args: any[]) => Promise<unknown>>;
  menu: Record<string, (...args: any[]) => Promise<unknown>>;
  tray: Record<string, (...args: any[]) => Promise<unknown>>;
  deepLink: Record<string, (...args: any[]) => Promise<unknown>>;
  app: Record<string, (...args: any[]) => Promise<unknown>>;
}

export declare function isForgeAvailable(): boolean;
export declare function getForge(): ForgeApi;
export declare function invoke(command: string, args?: Record<string, unknown>): Promise<unknown>;
export declare function invokeDetailed(
  command: string,
  args?: Record<string, unknown>,
  options?: InvokeDetailedOptions,
): Promise<unknown>;
export declare function on(eventName: string, handler: (payload: unknown) => void): unknown;
export declare function once(eventName: string, handler: (payload: unknown) => void): unknown;
export declare function off(eventName: string, handler: (payload: unknown) => void): unknown;
export declare const forge: ForgeApi;
export default forge;
