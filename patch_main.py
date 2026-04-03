import re
with open("forge_cli/main.py", "r") as f:
    text = f.read()
# Let's see the _run_dev_loop code.
match = re.search(r"def _run_dev_loop.*?except KeyboardInterrupt:", text, re.DOTALL)
print(match.group(0))
