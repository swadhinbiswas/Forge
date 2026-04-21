import re
with open('/home/swadhin/Forge/forge-framework/forge/api/shortcuts.py', 'r') as f:
    text = f.read()

text = re.sub(
    r"\s*# Expose specific string names for the IPC bridge so JS can call them cleanly.*?unregister_all._forge_cmd = \"shortcuts_unregister_all\"  # type: ignore\n",
    "\n",
    text,
    flags=re.DOTALL
)
with open('/home/swadhin/Forge/forge-framework/forge/api/shortcuts.py', 'w') as f:
    f.write(text)
