import sys

with open("tests/test_shell_api.py", "r") as f:
    t = f.read()
t = t.replace('match="not allowed"', 'match="not allowed"') # noop
with open("tests/test_shell_api.py", "w") as f:
    f.write(t)

