import re

with open("forge/bridge.py", "r") as f:
    bridge_code = f.read()

invoke_body = """    def invoke_command(self, raw_message: str) -> str:
        msg_id: Any = None
        trace_requested = False
        command_name: Optional[str] = None
        start_time = time.perf_counter()
        
        inspect = os.environ.get("FORGE_INSPECT") == "1"

        try:
"""
bridge_code = bridge_code.replace("""    def invoke_command(self, raw_message: str) -> str:""", """    def invoke_command(self, raw_message: str) -> str:
        inspect = os.environ.get("FORGE_INSPECT") == "1"
""")
# Also find before return response to log it.
# Actually let's just write a regex or replace.

