import re
with open("forge/bridge.py", "r") as f:
    text = f.read()

import_str = """
import logging
import pydantic_core
from pydantic_core import core_schema

# ─── IPC Request Schema (Pydantic-Core) ───
_META_SCHEMA = core_schema.typed_dict_schema(
    fields={
        "origin": core_schema.typed_dict_field(core_schema.str_schema(), required=False),
        "window_label": core_schema.typed_dict_field(core_schema.str_schema(), required=False),
    },
    extra_behavior="allow",
)

_IPC_REQUEST_SCHEMA = core_schema.typed_dict_schema(
    fields={
        "id": core_schema.typed_dict_field(core_schema.any_schema(), required=False),
        "command": core_schema.typed_dict_field(core_schema.str_schema(), required=False),
        "cmd": core_schema.typed_dict_field(core_schema.str_schema(), required=False),
        "args": core_schema.typed_dict_field(core_schema.dict_schema(), required=False),
        "meta": core_schema.typed_dict_field(_META_SCHEMA, required=False),
        "trace": core_schema.typed_dict_field(core_schema.bool_schema(), required=False),
        "include_meta": core_schema.typed_dict_field(core_schema.bool_schema(), required=False),
        "protocol": core_schema.typed_dict_field(core_schema.any_schema(), required=False),
        "protocolVersion": core_schema.typed_dict_field(core_schema.any_schema(), required=False),
    },
    extra_behavior="ignore"
)
_IPC_VALIDATOR = pydantic_core.SchemaValidator(_IPC_REQUEST_SCHEMA)
"""
text = text.replace("import logging", import_str)
with open("forge/bridge.py", "w") as f:
    f.write(text)

