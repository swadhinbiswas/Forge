with open("forge/app.py", "r") as f:
    lines = f.readlines()
# Find imports in setup_apis
import_line = False
new_lines = []
for line in lines:
    if "from .api.os_integration import OSIntegrationAPI" in line:
        import_line = True
        new_lines.append(line)
        new_lines.append("        from .api.printing import PrintingAPI\n")
        new_lines.append("        from .api.power import PowerAPI\n")
        new_lines.append("        from .api.window_state import WindowStateAPI\n")
        new_lines.append("        from .api.drag_drop import DragDropAPI\n")
        new_lines.append("        from .api.autostart import AutostartAPI\n")
    else:
        new_lines.append(line)

with open("forge/app.py", "w") as f:
    f.writelines(new_lines)
