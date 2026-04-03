import json
import threading
from pathlib import Path
from typing import Dict, Any, Optional

class WindowStateAPI:
    """
    Manages multi-window state persistency (positions, sizes, etc.)
    preventing "window teleporting" across application restarts.
    """

    __forge_capability__ = "window_state"
    def __init__(self, app):
        self.app = app
        # Ensure FS API is initialized before accessing paths
        from .fs import _expand_path_var
        data_dir = _expand_path_var("$APPDATA") / app.config.app.name
        data_dir.mkdir(parents=True, exist_ok=True)
        self._state_file = data_dir / "window_state.json" 
        
        self._save_timer = None
        self._lock = threading.Lock()
        
        self._cache = self._load()
        
        # Hook window lifecycle events to keep state synced without JS
        self.app.events.on("resized", self._on_resized)
        self.app.events.on("moved", self._on_moved)
        self.app.events.on("ready", self._on_ready)

        self._hydrate_main_config()

    def _hydrate_main_config(self) -> None:
        """Inject saved width and height directly into the app config before the native engine reads it."""
        main_state = self.get_state("main")
        if "width" in main_state and main_state["width"]:
            self.app.config.window.width = int(main_state["width"])
        if "height" in main_state and main_state["height"]:
            self.app.config.window.height = int(main_state["height"])

    def try_hydrate_descriptor(self, descriptor: Dict[str, Any]) -> None:
        """Mutate a secondary window creation JSON descriptor inline before it's sent to Rust."""
        label = descriptor.get("label", "")
        if not label:
            return
            
        saved = self.get_state(label)
        if not saved:
            return
            
        if "width" in saved and saved["width"] is not None:
            descriptor["width"] = int(saved["width"])
        if "height" in saved and saved["height"] is not None:
            descriptor["height"] = int(saved["height"])
        if "x" in saved and saved["x"] is not None:
            descriptor["x"] = float(saved["x"])
        if "y" in saved and saved["y"] is not None:
            descriptor["y"] = float(saved["y"])

    def _load(self) -> Dict[str, Any]:
        if self._state_file.exists():
            try:
                with open(self._state_file, "r") as f:
                    return json.load(f)
            except Exception:
                return {}
        return {}

    def get_state(self, label: str) -> Dict[str, Any]:
        """Get the saved state for a given window label."""
        return self._cache.get(label, {})

    def _save_debounced(self):
        with self._lock:
            # Atomic write to avoid corruption
            temp_file = self._state_file.with_suffix(".tmp")
            try:
                with open(temp_file, "w") as f:
                    json.dump(self._cache, f)
                temp_file.replace(self._state_file)
            except Exception as e:
                pass # Silent fail if permissions block saving state

    def _trigger_save(self):
        with self._lock:
            if self._save_timer:
                self._save_timer.cancel()
            self._save_timer = threading.Timer(0.3, self._save_debounced)
            self._save_timer.start()

    def _on_resized(self, event):
        label = event.get("label", "main")
        if label not in self._cache:
            self._cache[label] = {}
        self._cache[label]["width"] = event.get("width")
        self._cache[label]["height"] = event.get("height")
        self._trigger_save()
        
    def _on_moved(self, event):
        label = event.get("label", "main")
        if label not in self._cache:
            self._cache[label] = {}
        self._cache[label]["x"] = event.get("x")
        self._cache[label]["y"] = event.get("y")
        self._trigger_save()

    def _on_ready(self, _event: Any) -> None:
        """Apply native positioning for the main window once the rust bridge is actually live."""
        main_state = self.get_state("main")
        if "x" in main_state and "y" in main_state:
            x = main_state["x"]
            y = main_state["y"]
            if x is not None and y is not None:
                self.app.window.set_position(x, y)
