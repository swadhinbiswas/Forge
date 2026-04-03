from __future__ import annotations

import subprocess
import os
import sys
import logging
from pathlib import Path
from typing import TYPE_CHECKING, Dict, Any, List, Optional, Union

if TYPE_CHECKING:
    from ..config import ShellPermissions

logger = logging.getLogger(__name__)

class ShellAPI:
    """
    Strict shell execution capability.
    Only allows executing commands that match the allowlist defined in config.
    """

    __forge_capability__ = "shell"

    def __init__(
        self,
        base_dir: Path,
        permissions: Union[bool, ShellPermissions] = False,
    ) -> None:
        self._base_dir = base_dir
        self._permissions = permissions

    def _is_allowed(self, command: str) -> bool:
        if self._permissions is False:
            return False
        if self._permissions is True:
            # If globally allowed without strict lists
            return True
        
        # It's a ShellPermissions object
        return command in self._permissions.execute

    def _is_sidecar_allowed(self, name: str) -> bool:
        if self._permissions is False:
            return False
        if self._permissions is True:
            return True
        return hasattr(self._permissions, "sidecars") and name in self._permissions.sidecars

    def sidecar(self, name: str, args: Optional[List[str]] = None) -> Dict[str, Any]:
        """
        Execute a bundled sidecar binary securely.
        """
        if not self._is_sidecar_allowed(name):
            raise PermissionError(f"Execution of sidecar '{name}' is not allowed by shell permissions policy.")
        
        args = args or []
        ext = ".exe" if sys.platform == "win32" else ""
        
        # In runtime, sidecars are packaged in bin/, resolving from base_dir
        # Ensure we look exactly at bin/name.exe
        sidecar_path = self._base_dir / "bin" / f"{name}{ext}"
        
        if not sidecar_path.exists():
            raise FileNotFoundError(f"Sidecar binary '{name}' not found at {sidecar_path}")
            
        cmd_list = [str(sidecar_path)] + args
        try:
            result = subprocess.run(
                cmd_list,
                capture_output=True,
                text=True,
                check=False
            )
            return {
                "stdout": result.stdout,
                "stderr": result.stderr,
                "code": result.returncode
            }
        except Exception as e:
            logger.error(f"Sidecar execution failed {cmd_list}: {e}")
            raise RuntimeError(f"Sidecar execution failed: {e}")

    def execute(self, command: str, args: Optional[List[str]] = None) -> Dict[str, Any]:
        """
        Execute a native shell command securely according to the configured allowlist.
        """
        if not self._is_allowed(command):
            raise PermissionError(f"Execution of '{command}' is not allowed by shell permissions policy.")
        
        args = args or []
        cmd_list = [command] + args
        try:
            result = subprocess.run(
                cmd_list,
                capture_output=True,
                text=True,
                check=False
            )
            return {
                "stdout": result.stdout,
                "stderr": result.stderr,
                "code": result.returncode
            }
        except Exception as e:
            logger.error(f"Command execution failed {cmd_list}: {e}")
            raise RuntimeError(f"Command execution failed: {e}")

    def open(self, path: str) -> None:
        """
        Open a URL or path with the default system application.
        Requires general shell permission access.
        """
        if self._permissions is False:
            raise PermissionError("Shell access is disabled.")
            
        path = os.path.abspath(path) if not path.startswith("http") else path

        try:
            if sys.platform == "win32":
                os.startfile(path)
            elif sys.platform == "darwin":
                subprocess.Popen(["open", path])
            else:
                subprocess.Popen(["xdg-open", path])
        except Exception as e:
            logger.error(f"Failed to open '{path}': {e}")
            raise RuntimeError(f"Failed to open '{path}': {e}")
