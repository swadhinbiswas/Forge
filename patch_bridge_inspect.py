import re
import os

with open("forge/bridge.py", "r") as f:
    bridge_code = f.read()

invoke_orig = """    def invoke_command(self, raw_message: str) -> str:
        \"\"\"
        Parse, validate, and execute an IPC command from a raw JSON string.

        This is the main entry point called by the Rust IPC handler.
        Returns a JSON string response."""

invoke_new = """    def invoke_command(self, raw_message: str) -> str:
        \"\"\"
        Parse, validate, and execute an IPC command from a raw JSON string.

        This is the main entry point called by the Rust IPC handler.
        Returns a JSON string response."""

# Wait I will just wrap `invoke_command` with a new method and rename the old to `_invoke_command_internal`.
# Or just rename it to `_invoke_command_internal` and make a new `invoke_command`.

replacement = """    def invoke_command(self, raw_message: str) -> str:
        inspect = os.environ.get("FORGE_INSPECT") == "1"
        if inspect:
            try:
                msg_data = json.loads(raw_message)
                cmd = msg_data.get('command') or msg_data.get('cmd') or 'unknown'
                print(f"\\n\\033[36m[IPC REQ]\\033[0m \\033[1m{cmd}\\033[0m: {raw_message[:500]}")
            except Exception:
                print(f"\\n\\033[36m[IPC REQ]\\033[0m \\033[1munknown\\033[0m: {raw_message[:500]}")
                
        result = self._invoke_command_internal(raw_message)
        
        if inspect:
            print(f"\\033[32m[IPC RES]\\033[0m {result[:500]}")
            
        return result

    def _invoke_command_internal(self, raw_message: str) -> str:
        \"\"\"
        Parse, validate, and execute an IPC command from a raw JSON string.

        This is the main entry point called by the Rust IPC handler.
        Returns a JSON string response."""

bridge_code = bridge_code.replace(invoke_orig, replacement)
with open("forge/bridge.py", "w") as f:
    f.write(bridge_code)

with open("forge/app.py", "r") as f:
    app_code = f.read()

emit_orig = """    def emit(self, event: str, payload: Any = None) -> None:
        \"\"\"
        Emit an event to the JavaScript frontend.

        Also dispatches to Python-side event listeners."""

emit_new = """    def emit(self, event: str, payload: Any = None) -> None:
        \"\"\"
        Emit an event to the JavaScript frontend.

        Also dispatches to Python-side event listeners.
        \"\"\"
        import os
        import json
        if os.environ.get("FORGE_INSPECT") == "1":
            try:
                encoded = json.dumps(payload)
            except Exception:
                encoded = str(payload)
            print(f"\\n\\033[35m[IPC EMT]\\033[0m \\033[1m{event}\\033[0m: {encoded[:500]}")

"""
app_code = app_code.replace(emit_orig + "\\n        \"\"\"", emit_new)
with open("forge/app.py", "w") as f:
    f.write(app_code)

