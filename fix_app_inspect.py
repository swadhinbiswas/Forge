with open("forge/app.py", "r") as f:
    text = f.read()

text = text.replace(
    '        self._proxy = proxy\n        self.window._apply_native_event("ready", None)',
    '''        self._proxy = proxy
        if os.environ.get("FORGE_INSPECT") == "1":
            try:
                self._proxy.evaluate_script("main", "window.__FORGE_INSPECT__ = true;")
            except Exception:
                pass
        self.window._apply_native_event("ready", None)'''
)

with open("forge/app.py", "w") as f:
    f.write(text)
