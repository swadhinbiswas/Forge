"""Media capabilities probe (built-in extension). Full capture requires OS permissions and native code."""

from __future__ import annotations

import shutil
import subprocess
from typing import Any, Dict

_CAP = "forge_extensions"


class BuiltinMediaAPI:
    __forge_capability__ = _CAP

    def capabilities(self) -> Dict[str, Any]:
        """Describe what is available on this machine (HTML5 playback in webview; optional ffmpeg)."""
        ffmpeg = shutil.which("ffmpeg")
        ffprobe = shutil.which("ffprobe")
        return {
            "html5_audio_video": True,
            "ffmpeg_path": ffmpeg,
            "ffprobe_path": ffprobe,
            "native_recording": False,
            "note": "Use Web APIs in the frontend for playback; use ffmpeg sidecar via shell permission for transcoding.",
        }

    def ffmpeg_version(self) -> Dict[str, Any]:
        """Return ffmpeg version string if installed."""
        exe = shutil.which("ffmpeg")
        if not exe:
            return {"ok": False, "error": "ffmpeg not on PATH"}
        try:
            out = subprocess.run([exe, "-version"], capture_output=True, text=True, timeout=5)
            return {"ok": out.returncode == 0, "version_line": (out.stdout or "").splitlines()[:1]}
        except Exception as exc:
            return {"ok": False, "error": str(exc)}


def register(_app: Any) -> None:
    _app.bridge.register_commands(BuiltinMediaAPI())
