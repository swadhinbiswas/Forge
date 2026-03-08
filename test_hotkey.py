import os
import time
from forge import ForgeApp

app = ForgeApp()
app.config.permissions.global_shortcut = True
app.config.permissions.filesystem = True

@app.on_ready
def ready(_):
    print("READY! Registering CmdOrCtrl+Shift+X...")
    success = app.shortcuts.register("CmdOrCtrl+Shift+X", lambda: print("Shortcut triggered!"))
    print("Registration OK?", success)
    
    # Simulate a hotkey for testing, actually we physically can't easily,
    # but let's test if our parser unregisters it correctly.
    print("Unregistering...")
    unreg_success = app.shortcuts.unregister("CmdOrCtrl+Shift+X")
    print("Unregistration OK?", unreg_success)
    
    app.window.close()

if __name__ == "__main__":
    app.run()
