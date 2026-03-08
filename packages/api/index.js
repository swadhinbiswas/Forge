function missingRuntimeError() {
  return new Error(
    "Forge runtime is not available. Load forge.js in a Forge window or run inside Forge dev/build output."
  );
}

export function isForgeAvailable() {
  return typeof globalThis !== "undefined" && !!globalThis.__forge__;
}

export function getForge() {
  if (!isForgeAvailable()) {
    throw missingRuntimeError();
  }
  return globalThis.__forge__;
}

export function invoke(command, args = {}) {
  return getForge().invoke(command, args);
}

export function invokeDetailed(command, args = {}, options = {}) {
  return getForge().invokeDetailed(command, args, options);
}

export function on(eventName, handler) {
  return getForge().on(eventName, handler);
}

export function once(eventName, handler) {
  return getForge().once(eventName, handler);
}

export function off(eventName, handler) {
  return getForge().off(eventName, handler);
}

export const forge = new Proxy(
  {},
  {
    get(_target, property) {
      return getForge()[property];
    },
  }
);

export default forge;
