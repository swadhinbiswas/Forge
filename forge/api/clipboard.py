"""
Forge Clipboard API (v2.0).

Provides native OS clipboard read/write functionality for Forge applications.
"""

from __future__ import annotations

import logging
from typing import Dict, Any

from forge.bridge import command
from forge.forge_core import ClipboardManager

logger = logging.getLogger(__name__)

class ClipboardAPI:
    """
    Native OS Clipboard API for Forge applications.
    """

    __forge_capability__ = "clipboard"

    def __init__(self):
        self._manager = ClipboardManager()

    @command("clipboard_read")
    def read(self) -> Dict[str, Any]:
        try:
            text = self._manager.read_text()
            return {"success": True, "text": text}
        except Exception as e:
            logger.error(f"Failed to read clipboard: {e}")
            return {"success": False, "error": str(e)}

    @command("clipboard_write")
    def write(self, text: str) -> Dict[str, Any]:
        try:
            self._manager.write_text(text)
            return {"success": True}
        except Exception as e:
            logger.error(f"Failed to write clipboard: {e}")
            return {"success": False, "error": str(e)}

    @command("clipboard_clear")
    def clear(self) -> Dict[str, Any]:
        try:
            self._manager.write_text("")
            return {"success": True}
        except Exception as e:
            logger.error(f"Failed to clear clipboard: {e}")
            return {"success": False, "error": str(e)}
