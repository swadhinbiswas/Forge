import re
import sys

with open("forge/app.py", "r") as f:
    content = f.read()

# Fix ShellAPI initialization
content = content.replace("self.shell = ShellAPI(self)", "self.shell = ShellAPI(self.config.shell)")

# Fix router._commands to router.commands
content = content.replace("router._commands.items()", "router.commands.items()")

# Fix load_url(url) to load_url("main", url)
content = content.replace("self._require_proxy().load_url(url)", "self._require_proxy().load_url(\"main\", url)")

with open("forge/app.py", "w") as f:
    f.write(content)

