"""
Command Router for Forge Framework.

Enables grouping IPC commands into modular routers that can be
included into the main ForgeApp, eliminating global state decorators.
"""

from typing import Any, Callable, Dict, Optional


class Router:
    """
    A router for grouping Forge IPC commands.

    Usage:
        router = Router("my_plugin")

        @router.command()
        def do_something():
            pass

        app.include_router(router)
    """

    def __init__(self, prefix: str = "") -> None:
        self.prefix = prefix
        self.commands: Dict[str, Callable] = {}

    def command(
        self,
        name: Optional[str] = None,
        capability: Optional[str] = None,
        version: str = "1.0",
    ) -> Callable:
        """
        Register a command within this router.
        """
        def decorator(func: Callable) -> Callable:
            cmd_name = name or getattr(func, "_forge_cmd", None) or func.__name__
            if self.prefix:
                cmd_name = f"{self.prefix}:{cmd_name}"
            
            if capability is not None:
                func._forge_capability = capability  # type: ignore[attr-defined]
            func._forge_version = version  # type: ignore[attr-defined]
            
            self.commands[cmd_name] = func
            return func

        return decorator
