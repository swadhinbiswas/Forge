"""Built-in extension plugins (forge_extensions permission)."""

from __future__ import annotations

from pathlib import Path
from unittest.mock import MagicMock

from forge.builtins.registry import register_builtin_plugins
from forge.config import BuiltinPluginsConfig, ForgeConfig, PermissionsConfig


def _config_with_extensions(tmp_path: Path) -> ForgeConfig:
    cfg = ForgeConfig()
    cfg.config_path = tmp_path / "forge.toml"
    cfg.permissions = PermissionsConfig(forge_extensions=True)
    cfg.builtin_plugins = BuiltinPluginsConfig()
    return cfg


def test_builtin_plugins_skipped_without_permission(tmp_path: Path) -> None:
    cfg = ForgeConfig()
    cfg.config_path = tmp_path / "forge.toml"
    cfg.permissions = PermissionsConfig(forge_extensions=False)
    app = MagicMock()
    app.has_capability = MagicMock(return_value=False)
    app.config = cfg
    app.bridge = MagicMock()
    register_builtin_plugins(app)
    app.bridge.register_commands.assert_not_called()


def test_builtin_registers_sqlite_when_enabled(tmp_path: Path) -> None:
    cfg = _config_with_extensions(tmp_path)
    app = MagicMock()
    app.has_capability = lambda cap: cap == "forge_extensions"
    app.config = cfg

    bridge = MagicMock()
    app.bridge = bridge

    register_builtin_plugins(app)

    assert bridge.register_commands.called
    # At least database API registered
    names = [c.args[0].__class__.__name__ for c in bridge.register_commands.call_args_list]
    assert "BuiltinDatabaseAPI" in names


def test_forge_config_parses_builtin_plugins_table(tmp_path: Path) -> None:
    p = tmp_path / "forge.toml"
    p.write_text(
        """
[app]
name = "t"
version = "1.0.0"

[permissions]
forge_extensions = true

[builtin_plugins]
database = true
telemetry = false
""",
        encoding="utf-8",
    )
    cfg = ForgeConfig.from_file(p)
    assert cfg.permissions.forge_extensions is True
    assert cfg.builtin_plugins.database is True
    assert cfg.builtin_plugins.telemetry is False
