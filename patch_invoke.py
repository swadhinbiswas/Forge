import re
with open("forge/bridge.py", "r") as f:
    text = f.read()

old_str = """            # 2. Parse JSON
            try:
                data = json.loads(raw_message)
            except Exception:
                return self._error_response(None, "Invalid JSON")

            if not isinstance(data, dict):
                return self._error_response(None, "Request must be a JSON object")

            msg_id = data.get("id")
            trace_requested = bool(data.get("trace") or data.get("include_meta"))

            protocol = data.get("protocol") or data.get("protocolVersion")
            if protocol is not None and str(protocol) not in SUPPORTED_PROTOCOL_VERSIONS:
                return self._error_response(
                    msg_id,
                    f"Unsupported protocol version: {protocol!r}",
                    code="unsupported_protocol",
                    meta=self._build_trace_meta(start_time, trace_requested),
                )

            # 3. Required fields
            cmd_name = data.get("command") or data.get("cmd")
            command_name = cmd_name
            if not cmd_name:
                return self._error_response(
                    msg_id,
                    "Missing 'command' field",
                    code="missing_command",
                    meta=self._build_trace_meta(start_time, trace_requested),
                )

            # 4. Command name validation
            if not self._validate_command_name(cmd_name):
                return self._error_response(
                    msg_id,
                    f"Invalid command name: {cmd_name!r}",
                    code="invalid_command_name",
                    meta=self._build_trace_meta(start_time, trace_requested, command_name),
                )

            # 5. Args validation
            args = data.get("args", {})
            if not isinstance(args, dict):
                return self._error_response(
                    msg_id,
                    "Args must be a JSON object",
                    code="invalid_args",
                    meta=self._build_trace_meta(start_time, trace_requested, command_name),
                )

            meta = data.get("meta", {})
            if meta is None:
                meta = {}
            if not isinstance(meta, dict):
                return self._error_response(
                    msg_id,
                    "Meta must be a JSON object when provided",
                    code="invalid_meta",
                    meta=self._build_trace_meta(start_time, trace_requested, command_name),
                )
            origin = meta.get("origin") if isinstance(meta.get("origin"), str) else None
            window_label = meta.get("window_label") if isinstance(meta.get("window_label"), str) else None"""

new_str = """            # 2. Parse and validate JSON envelope via Pydantic-Core
            try:
                data = _IPC_VALIDATOR.validate_json(raw_message)
            except pydantic_core.ValidationError as e:
                errors = e.errors()
                if errors:
                    err = errors[0]
                    loc = err.get("loc", ())
                    if loc == ():
                        if err.get("type") in ("json_invalid", "json_type"):
                            return self._error_response(None, "Invalid JSON")
                        elif err.get("type") == "dict_type":
                            return self._error_response(None, "Request must be a JSON object")
                    elif loc == ("args",):
                        return self._error_response(None, "Args must be a JSON object", code="invalid_args")
                    elif loc == ("meta",):
                        return self._error_response(None, "Meta must be a JSON object when provided", code="invalid_meta")
                return self._error_response(None, "Invalid request payload")

            msg_id = data.get("id")
            trace_requested = bool(data.get("trace") or data.get("include_meta"))

            protocol = data.get("protocol") or data.get("protocolVersion")
            if protocol is not None and str(protocol) not in SUPPORTED_PROTOCOL_VERSIONS:
                return self._error_response(
                    msg_id,
                    f"Unsupported protocol version: {protocol!r}",
                    code="unsupported_protocol",
                    meta=self._build_trace_meta(start_time, trace_requested),
                )

            # 3. Required fields
            cmd_name = data.get("command") or data.get("cmd")
            command_name = cmd_name
            if not cmd_name:
                return self._error_response(
                    msg_id,
                    "Missing 'command' field",
                    code="missing_command",
                    meta=self._build_trace_meta(start_time, trace_requested),
                )

            # 4. Command name validation
            if not self._validate_command_name(cmd_name):
                return self._error_response(
                    msg_id,
                    f"Invalid command name: {cmd_name!r}",
                    code="invalid_command_name",
                    meta=self._build_trace_meta(start_time, trace_requested, command_name),
                )

            # 5. Extract validated args and meta
            args = data.get("args", {})
            meta = data.get("meta", {})
            origin = meta.get("origin")
            window_label = meta.get("window_label")"""

text = text.replace(old_str, new_str)
with open("forge/bridge.py", "w") as f:
    f.write(text)
