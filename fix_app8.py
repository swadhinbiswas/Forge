with open("forge/app.py", "r") as f:
    content = f.read()

old_pattern = """            self.bridge.register_command(
                name,
                func,
                capability=getattr(func, "_forge_capability", None),
                version=getattr(func, "_forge_version", "1.0"),
            )"""
new_pattern = """            self.bridge.register_command(
                name,
                func,
                capability=getattr(func, "_forge_capability", None),
                version=getattr(func, "_forge_version", "1.0"),
                internal=getattr(func, "_forge_internal", False),
            )"""
if old_pattern in content:
    content = content.replace(old_pattern, new_pattern)
with open("forge/app.py", "w") as f:
    f.write(content)

