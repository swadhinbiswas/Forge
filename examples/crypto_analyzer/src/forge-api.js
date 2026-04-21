export async function invoke(method, args = {}) {
  try {
    if (window.__forge__ && window.__forge__.invoke) {
      return await window.__forge__.invoke(method, args);
    } else {
      console.warn(`Forge API not found. Discarding invocation to '${method}'`);
      return null;
    }
  } catch (error) {
    console.error(`Error invoking forge method '${method}':`, error);
    throw error;
  }
}

export function onEvent(eventName, callback) {
  if (window.__forge__ && window.__forge__.on) {
    window.__forge__.on(eventName, callback);
  } else {
    // Standard window events fallback
    window.addEventListener(eventName, (e) => callback(e.detail));
  }
}

export function removeEvent(eventName, callback) {
  if (window.__forge__ && window.__forge__.off) {
    window.__forge__.off(eventName, callback);
  } else {
    window.removeEventListener(eventName, callback);
  }
}
