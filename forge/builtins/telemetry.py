"""In-process telemetry buffer (built-in extension)."""

from __future__ import annotations

import threading
import time
from collections import deque
from typing import Any, Deque, Dict, List, Optional

_CAP = "forge_extensions"

_MAX = 2000
_events: Deque[Dict[str, Any]] = deque(maxlen=_MAX)
_lock = threading.Lock()


class BuiltinTelemetryAPI:
    __forge_capability__ = _CAP

    def record(self, name: str, data: Optional[Dict[str, Any]] = None) -> bool:
        """Append a telemetry event (memory buffer only; export via ``flush``)."""
        with _lock:
            _events.append(
                {
                    "ts": time.time(),
                    "name": name,
                    "data": data or {},
                }
            )
        return True

    def flush(self, limit: int = 500) -> List[Dict[str, Any]]:
        """Return up to ``limit`` oldest events and remove them."""
        out: List[Dict[str, Any]] = []
        with _lock:
            while _events and len(out) < limit:
                out.append(_events.popleft())
        return out

    def peek(self, limit: int = 50) -> List[Dict[str, Any]]:
        """Inspect recent events without removing."""
        with _lock:
            snap = list(_events)[-limit:]
        return snap


def register(app: Any) -> None:
    app.bridge.register_commands(BuiltinTelemetryAPI())
