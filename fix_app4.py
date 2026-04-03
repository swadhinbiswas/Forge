with open("forge/app.py", "r") as f:
    text = f.read()

# 1. Fix evaluate_script calls
text = text.replace('self._proxy.evaluate_script(f"window.__forge__._handleMessage({data})")', 
                    'self._proxy.evaluate_script("main", f"window.__forge__._handleMessage({data})")')

# 2. Add include_router
include_router_code = """
    def include_router(self, router) -> None:
        for name, func_info in router._commands.items():
            self.bridge.register_command(
                name,
                func_info["func"],
                capability=func_info["capability"],
                version=func_info["version"],
                internal=func_info["internal"],
            )
"""
text = text.replace("    def register_command(self, name: str, func: Callable) -> None:", include_router_code + "\n    def register_command(self, name: str, func: Callable) -> None:")

# 3. Add shell to API setup
import_shell_code = """        from .api.shortcuts import ShortcutsAPI
        from .api.shell import ShellAPI"""
text = text.replace("        from .api.shortcuts import ShortcutsAPI", import_shell_code)

init_shell_code = """        self.power = PowerAPI(self)
        self.bridge.register_commands(self.power)

        self.shell = ShellAPI(self)
        self.bridge.register_commands(self.shell)"""
text = text.replace("        self.power = PowerAPI(self)\n        self.bridge.register_commands(self.power)", init_shell_code)

# 4. Remove duplicate _reload_backend
import re
text = re.sub(r'(\s+def _reload_backend\(self\) -> None:.*?\s+def _handle_reload_signal\(self, signum, frame\) -> None:\n\s+self._reload_backend\(\)\n)+', r'\1', text, flags=re.DOTALL)

with open("forge/app.py", "w") as f:
    f.write(text)
