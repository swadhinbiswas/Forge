import re
import sys

with open("forge/app.py", "r") as f:
    content = f.read()

# Fix include_router
old_router = """    def include_router(self, router) -> None:
        for name, func_info in router.commands.items():
            self.bridge.register_command(
                name,
                func_info["func"],
                capability=func_info["capability"],
                version=func_info["version"],
                internal=func_info["internal"],
            )"""

new_router = """    def include_router(self, router) -> None:
        for name, func in router.commands.items():
            self.bridge.register_command(
                name,
                func,
                capability=getattr(func, "_forge_capability", None),
                version=getattr(func, "_forge_version", "1.0"),
            )"""
            
if old_router in content:
    content = content.replace(old_router, new_router)
else:
    print("Failed to replace include_router")

# Fix load_url in _on_window_ready
pattern = r'self\._proxy\.load_url\(([^,]+)\)'
content = re.sub(pattern, r'self._proxy.load_url("main", \1)', content)

with open("forge/app.py", "w") as f:
    f.write(content)

