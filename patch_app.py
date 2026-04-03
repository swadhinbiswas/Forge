import sys

with open("forge/app.py", "r") as f:
    text = f.read()

# Add signal handler setup
init_str = """        # Web Mode (ASGI) internal state
        self._asgi_adapter: Any = None"""
new_init_str = """        import signal
        if hasattr(signal, "SIGUSR1"):
            signal.signal(signal.SIGUSR1, self._handle_reload_signal)

        # Web Mode (ASGI) internal state
        self._asgi_adapter: Any = None"""

text = text.replace(init_str, new_init_str)

# Add methods
method_str = """    # ─── Command Registration (instance-level) ───"""
new_method_str = """    def _reload_backend(self) -> None:
        self._log_runtime_event("backend_reload_started")
        import sys
        import importlib
        from pathlib import Path

        # Clear existing commands so we don't duplicate or leak memory
        self.bridge._commands.clear()
        self._register_internal_runtime_commands()
        self._setup_apis()

        # Reload the user application entry point and any modules in the project
        project_dir = str(self.config.get_base_dir())
        
        # We must prevent app.run() from re-executing NativeWindow.run()
        # and blocking the thread or creating a new window on reload.
        original_run = self.__class__.run
        self.__class__.run = lambda *args, **kwargs: None
        
        try:
            for name, module in list(sys.modules.items()):
                if hasattr(module, "__file__") and module.__file__ and module.__file__.startswith(project_dir):
                    if "forge" not in name:
                        try:
                            importlib.reload(module)
                        except Exception as e:
                            logger.error(f"Failed to reload {name}: {e}")
        finally:
            self.__class__.run = original_run
            
        self._log_runtime_event("backend_reload_complete")
        
        # Also let frontend know backend reloaded
        self.emit("backend:reloaded")
        
    def _handle_reload_signal(self, signum, frame) -> None:
        self._reload_backend()

    # ─── Command Registration (instance-level) ───"""

text = text.replace(method_str, new_method_str)

with open("forge/app.py", "w") as f:
    f.write(text)
