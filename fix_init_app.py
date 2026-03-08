with open("forge/api/__init__.py", "r") as f:
    text = f.read()

if "from .printing import PrintingAPI" not in text:
    text = text.replace("from .tray import TrayAPI", "from .tray import TrayAPI\nfrom .printing import PrintingAPI")
    text = text.replace('"TrayAPI",', '"TrayAPI",\n    "PrintingAPI",')

with open("forge/api/__init__.py", "w") as f:
    f.write(text)

with open("forge/app.py", "r") as f:
    text = f.read()
    
if "self.printing = PrintingAPI(self)" not in text:
    text = text.replace("self.drag_drop = DragDropAPI(self)", "self.drag_drop = DragDropAPI(self)\n        self.printing = PrintingAPI(self)")

with open("forge/app.py", "w") as f:
    f.write(text)
