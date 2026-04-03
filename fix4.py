with open("forge/app.py", "r") as f:
    lines = f.readlines()

new_lines = []
for i, line in enumerate(lines):
    if i == 1353: # zero-indexed line 1354
        new_lines.append(line.replace('capability=cap_req, version="1.0", internal=False', ''))
    else:
        new_lines.append(line)

with open("forge/app.py", "w") as f:
    f.writelines(new_lines)
