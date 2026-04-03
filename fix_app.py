with open("forge/app.py", "r") as f:
    lines = f.readlines()

new_lines = []
in_dupe = False
for i, line in enumerate(lines):
    if line.startswith("    def _reload_backend(self) -> None:") and i > 650:
        in_dupe = True
    if in_dupe and line.startswith("    def _handle_reload_signal(self, signum, frame) -> None:"):
        in_dupe = False # we'll skip to end of this function
        continue
    if in_dupe:
        continue
    new_lines.append(line)

with open("forge/app.py", "w") as f:
    f.writelines(new_lines)
